use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::{bail, Context, Result};
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chrono::Utc;
use rand::Rng;
use rcgen::{BasicConstraints, CertificateParams, IsCa, KeyPair, SanType};
use rustls::pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::time::{timeout, Duration};
use tokio_rustls::TlsAcceptor;
use tracing::info;

use crate::config::Config;
use super::models::{StoredToken, TokenResponse};

// Register at https://developer.spotify.com — create an app, add
// https://127.0.0.1:8888/callback as a redirect URI, paste the client ID here.
const CLIENT_ID: &str = "b5e0f935d5b74b1cb7c2fc40a0e9b45e";

const SPOTIFY_AUTH_URL: &str = "https://accounts.spotify.com/authorize";
const SPOTIFY_TOKEN_URL: &str = "https://accounts.spotify.com/api/token";
const SCOPES: &str = "user-read-private user-read-playback-state user-modify-playback-state \
                      playlist-read-private playlist-read-collaborative \
                      user-library-read user-library-modify";

// ── PKCE helpers ─────────────────────────────────────────────────────────────

fn generate_code_verifier() -> String {
    let bytes: Vec<u8> = (0..64).map(|_| rand::thread_rng().r#gen::<u8>()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    URL_SAFE_NO_PAD.encode(hasher.finalize())
}

fn generate_state() -> String {
    let bytes: Vec<u8> = (0..16).map(|_| rand::thread_rng().r#gen::<u8>()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

// ── Certificate ───────────────────────────────────────────────────────────────

/// Generates a self-signed localhost certificate if one does not already exist.
/// The cert is reused across sessions so the browser only prompts once per install.
pub fn ensure_localhost_cert(config: &Config) -> Result<()> {
    let cert_path = config.cert_der_path();
    let key_path  = config.key_der_path();

    if cert_path.exists() && key_path.exists() {
        return Ok(());
    }

    info!("Generating localhost certificate for OAuth callback...");

    let mut params = CertificateParams::new(vec!["localhost".to_string()])?;
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.subject_alt_names.push(SanType::IpAddress(
        std::net::IpAddr::V4(std::net::Ipv4Addr::new(127, 0, 0, 1)),
    ));
    let key_pair = KeyPair::generate()?;
    let cert     = params.self_signed(&key_pair)?;

    std::fs::write(&cert_path, cert.der())?;
    std::fs::write(&key_path,  key_pair.serialize_der())?;

    Ok(())
}

// ── OAuth flow ────────────────────────────────────────────────────────────────

pub async fn run_oauth_flow(config: &Config) -> Result<()> {
    let redirect_uri = &config.spotify.redirect_uri;
    let port         = config.spotify.callback_port;

    ensure_localhost_cert(config)?;

    let verifier  = generate_code_verifier();
    let challenge = generate_code_challenge(&verifier);
    let state     = generate_state();

    let auth_url = format!(
        "{}?client_id={}&response_type=code&redirect_uri={}&scope={}&state={}&code_challenge={}&code_challenge_method=S256",
        SPOTIFY_AUTH_URL,
        CLIENT_ID,
        urlencoding::encode(redirect_uri),
        urlencoding::encode(SCOPES),
        state,
        challenge,
    );

    if open::that(&auth_url).is_err() {
        info!("Could not open browser automatically.");
    }

    info!("Waiting for Spotify OAuth callback on port {}...", port);
    let code = wait_for_callback(config, port, &state).await?;

    info!("Exchanging auth code for tokens...");
    let token = exchange_code(CLIENT_ID, &code, &verifier, redirect_uri).await?;
    save_token(config, &token)?;

    info!("Spotify login successful.");
    Ok(())
}

// ── HTTPS callback server ─────────────────────────────────────────────────────

async fn wait_for_callback(config: &Config, port: u16, expected_state: &str) -> Result<String> {
    let cert_der = std::fs::read(config.cert_der_path())
        .context("Certificate not found — this is a bug, ensure_localhost_cert should have run first")?;
    let key_der = std::fs::read(config.key_der_path())
        .context("Private key not found — this is a bug, ensure_localhost_cert should have run first")?;

    let tls_config = rustls::ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(
            vec![CertificateDer::from(cert_der)],
            PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(key_der)),
        )?;
    let acceptor = TlsAcceptor::from(Arc::new(tls_config));

    let addr: SocketAddr = format!("127.0.0.1:{}", port).parse()?;
    let listener = TcpListener::bind(addr).await
        .with_context(|| format!("Cannot bind to port {}", port))?;

    let result = timeout(Duration::from_secs(300), async {
        let (stream, _) = listener.accept().await?;
        let mut tls = acceptor.accept(stream).await
            .context("TLS handshake failed — in your browser click Advanced → Proceed to 127.0.0.1")?;

        let mut buf = vec![0u8; 8192];
        let n = tls.read(&mut buf).await?;
        let request = String::from_utf8_lossy(&buf[..n]);

        let path = request.lines().next().unwrap_or("")
            .split_whitespace().nth(1).unwrap_or("");

        let (code, state) = parse_callback_params(path)?;

        let html = r#"<!DOCTYPE html><html>
<head><title>Gurdo</title>
<style>body{font-family:sans-serif;display:flex;align-items:center;justify-content:center;
height:100vh;margin:0;background:#191414}h1{color:#1db954}p{color:#fff}</style></head>
<body><div style="text-align:center"><h1>Gurdo</h1>
<p>Spotify connected. You can close this tab.</p></div></body></html>"#;

        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            html.len(), html
        );
        tls.write_all(response.as_bytes()).await?;
        let _ = tls.shutdown().await;

        Ok::<(String, String), anyhow::Error>((code, state))
    })
    .await
    .context("OAuth callback timed out — did the browser redirect complete?")??;

    if result.1 != expected_state {
        bail!("State mismatch — try connecting again");
    }

    Ok(result.0)
}

fn parse_callback_params(path: &str) -> Result<(String, String)> {
    let query = path.split('?').nth(1).unwrap_or("");
    let mut code  = None;
    let mut state = None;

    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some("code"),  Some(v)) => code  = Some(urlencoding::decode(v)?.into_owned()),
            (Some("state"), Some(v)) => state = Some(urlencoding::decode(v)?.into_owned()),
            (Some("error"), Some(e)) => bail!("Spotify denied authorization: {}", e),
            _ => {}
        }
    }

    match (code, state) {
        (Some(c), Some(s)) => Ok((c, s)),
        _ => bail!("Callback missing code or state parameters"),
    }
}

// ── Token exchange ────────────────────────────────────────────────────────────

async fn exchange_code(
    client_id: &str,
    code: &str,
    verifier: &str,
    redirect_uri: &str,
) -> Result<StoredToken> {
    let client = reqwest::Client::new();
    let resp = client
        .post(SPOTIFY_TOKEN_URL)
        .form(&[
            ("grant_type",    "authorization_code"),
            ("code",          code),
            ("redirect_uri",  redirect_uri),
            ("client_id",     client_id),
            ("code_verifier", verifier),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!("Token exchange failed: {}", body);
    }

    let token: TokenResponse = resp.json().await?;
    let refresh_token = token.refresh_token
        .context("Spotify did not return a refresh token")?;

    Ok(StoredToken {
        access_token:  token.access_token,
        refresh_token,
        expires_at: Utc::now().timestamp() + token.expires_in as i64 - 60,
    })
}

async fn refresh_access_token(refresh_token: &str) -> Result<StoredToken> {
    let client = reqwest::Client::new();
    let resp = client
        .post(SPOTIFY_TOKEN_URL)
        .form(&[
            ("grant_type",    "refresh_token"),
            ("refresh_token", refresh_token),
            ("client_id",     CLIENT_ID),
        ])
        .send()
        .await?;

    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        bail!("Token refresh failed: {}", body);
    }

    let token: TokenResponse = resp.json().await?;
    Ok(StoredToken {
        access_token:  token.access_token,
        refresh_token: token.refresh_token.unwrap_or_else(|| refresh_token.to_string()),
        expires_at: Utc::now().timestamp() + token.expires_in as i64 - 60,
    })
}

// ── Token storage ─────────────────────────────────────────────────────────────

fn save_token(config: &Config, token: &StoredToken) -> Result<()> {
    let path = config.token_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::write(&path, serde_json::to_string_pretty(token)?)?;
    Ok(())
}

fn load_token(config: &Config) -> Result<Option<StoredToken>> {
    let path = config.token_path();
    if !path.exists() {
        return Ok(None);
    }
    let json = std::fs::read_to_string(&path)
        .with_context(|| format!("Cannot read token file: {}", path.display()))?;
    Ok(Some(serde_json::from_str(&json)?))
}

pub async fn load_or_refresh_token(config: &Config) -> Result<Option<String>> {
    let Some(token) = load_token(config)? else {
        return Ok(None);
    };

    if Utc::now().timestamp() < token.expires_at {
        return Ok(Some(token.access_token));
    }

    info!("Access token expired, refreshing...");
    let refreshed = refresh_access_token(&token.refresh_token).await?;
    save_token(config, &refreshed)?;
    Ok(Some(refreshed.access_token))
}
