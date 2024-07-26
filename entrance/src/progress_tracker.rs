use spdlog::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

pub struct ProgressTracker {
    start_time: Instant,
    total_records: u64,
    records_processed: Arc<Mutex<u64>>,
    update_interval: Duration,
    current_stage: Arc<Mutex<String>>,
}

impl ProgressTracker {
    // Initializes a new ProgressTracker
    pub fn new(total_records: u64, update_interval: Duration) -> Arc<Self> {
        let tracker = Arc::new(Self {
            start_time: Instant::now(),
            total_records,
            records_processed: Arc::new(Mutex::new(0)),
            update_interval,
            current_stage: Arc::new(Mutex::new(String::from("[START]"))),
        });
        tracker.start_progress_thread();
        tracker
    }

    pub fn update_records_processed(&self, processed: u64) {
        let mut records = self.records_processed.lock().unwrap();
        *records = (*records + processed).min(self.total_records);
    }

    pub fn set_stage(&self, stage: &str) {
        let mut current_stage = self.current_stage.lock().unwrap();
        *current_stage = stage.to_string();
        info!("Stage: {}", *current_stage);
    }

    fn start_progress_thread(&self) {
        let interval = self.update_interval;
        let records = self.records_processed.clone();
        let start_time = self.start_time;
        let total_records = self.total_records;
        let current_stage = self.current_stage.clone();

        thread::spawn(move || {
            let mut last_processed = 0;
            while Arc::strong_count(&records) > 1 {
                thread::sleep(interval);
                let now_processed = *records.lock().unwrap();
                let elapsed = start_time.elapsed().as_secs_f32();
                let throughput = (now_processed - last_processed) as f32 / interval.as_secs_f32();
                let progress = (now_processed as f32 / total_records as f32).min(1.0) * 100.0;
                let eta = if now_processed > 0 && progress < 100.0 {
                    (elapsed / progress) * (100.0 - progress)
                } else {
                    0.0
                };

                info!(
                    "Stage: {}, Progress: {:.2}% complete, ETA: {:.2} seconds, Throughput: {:.2} MB/sec",
                    *current_stage.lock().unwrap(),
                    progress,
                    eta,
                    throughput
                );

                last_processed = now_processed;
            }
        });
    }

    pub fn get_records_processed(&self) -> u64 {
        *self.records_processed.lock().unwrap()
    }
}
