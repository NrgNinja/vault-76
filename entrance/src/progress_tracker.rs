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
    done_stage_started: Arc<Mutex<bool>>,
    last_update_time: Arc<Mutex<Instant>>,
    last_processed_count: Arc<Mutex<u64>>,
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
            done_stage_started: Arc::new(Mutex::new(false)),
            last_update_time: Arc::new(Mutex::new(Instant::now())),
            last_processed_count: Arc::new(Mutex::new(0)),
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

        if stage == "[DONE]" {
            let mut done_stage_started = self.done_stage_started.lock().unwrap();
            *done_stage_started = true;
        }
    }

    fn start_progress_thread(&self) {
        let interval = self.update_interval;
        let records = self.records_processed.clone();
        let start_time = self.start_time;
        let total_records = self.total_records;
        let current_stage = self.current_stage.clone();
        let done_stage_started = self.done_stage_started.clone();
        let last_update_time = self.last_update_time.clone();
        let last_processed_count = self.last_processed_count.clone();

        thread::spawn(move || {
            while Arc::strong_count(&records) > 1 {
                thread::sleep(interval);
                let now_processed = *records.lock().unwrap();
                let elapsed = start_time.elapsed().as_secs_f32();
                let mut last_update = last_update_time.lock().unwrap();
                let mut last_count = last_processed_count.lock().unwrap();

                let progress = {
                    let done_stage_started = *done_stage_started.lock().unwrap();
                    if done_stage_started {
                        100.0
                    } else {
                        (now_processed as f32 / total_records as f32).min(1.0) * 100.0
                    }
                };
                let elapsed_since_last_update = last_update.elapsed().as_secs_f32();
                let eta = if now_processed > 0 && progress < 100.0 {
                    (elapsed / progress) * (100.0 - progress)
                } else {
                    0.0
                };
                let throughput = (now_processed - *last_count) as f32 * (16.0 / (1024.0 * 1024.0))
                    / elapsed_since_last_update;

                info!(
                    "Stage: {}, Progress: {:.2}% complete, ETA: {:.1} seconds, Throughput: {:.2} MB/sec",
                    *current_stage.lock().unwrap(),
                    progress,
                    eta,
                    throughput
                );

                *last_update = Instant::now();
                *last_count = now_processed;
            }
        });
    }

    // pub fn get_records_processed(&self) -> u64 {
    //     *self.records_processed.lock().unwrap()
    // }
}
