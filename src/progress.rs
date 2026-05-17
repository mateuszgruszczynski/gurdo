pub trait ProgressReporter: Send + Sync {
    fn stage(&self, name: &str);
    fn tick(&self, current: u64, total: Option<u64>);
    #[allow(dead_code)]
    fn message(&self, msg: &str);
    fn finish(&self, ok: bool, summary: &str);
}

pub struct NullProgress;

impl ProgressReporter for NullProgress {
    fn stage(&self, _: &str) {}
    fn tick(&self, _: u64, _: Option<u64>) {}
    fn message(&self, _: &str) {}
    fn finish(&self, _: bool, _: &str) {}
}

#[cfg(test)]
pub struct RecordingReporter(pub std::sync::Mutex<Vec<String>>);

#[cfg(test)]
impl RecordingReporter {
    pub fn new() -> Self { Self(std::sync::Mutex::new(vec![])) }
    pub fn events(&self) -> Vec<String> { self.0.lock().unwrap().clone() }
}

#[cfg(test)]
impl ProgressReporter for RecordingReporter {
    fn stage(&self, name: &str) {
        self.0.lock().unwrap().push(format!("stage:{}", name));
    }
    fn tick(&self, cur: u64, total: Option<u64>) {
        self.0.lock().unwrap().push(format!("tick:{}/{}", cur, total.unwrap_or(0)));
    }
    fn message(&self, msg: &str) {
        self.0.lock().unwrap().push(format!("msg:{}", msg));
    }
    fn finish(&self, ok: bool, summary: &str) {
        self.0.lock().unwrap().push(format!("finish:{}/{}", ok, summary));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recording_reporter_captures_events_in_order() {
        let r = RecordingReporter::new();
        r.stage("loading");
        r.tick(1, Some(10));
        r.tick(5, Some(10));
        r.finish(true, "done");
        assert_eq!(r.events(), vec![
            "stage:loading",
            "tick:1/10",
            "tick:5/10",
            "finish:true/done",
        ]);
    }
}
