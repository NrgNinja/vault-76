// use spdlog::info;
// use std::time::{Duration, Instant};

// pub struct ProgressTracker {
//     start_time: Instant,
//     total_records: u64,
//     records_processed: u64,
//     last_update_time: Instant,
//     update_interval: Duration,
//     current_stage: String,
// }

// impl ProgressTracker {
//     // Initializes a new ProgressTracker
//     pub fn new(total_records: u64, update_interval: Duration) -> Self {
//         Self {
//             total_records,
//             records_processed: 0,
//             last_update_time: Instant::now(),
//             start_time: Instant::now(),
//             update_interval,
//             current_stage: String::new(),
//         }
//     }

//     // Method to safely increment the records_processed field
//     pub fn update_records_processed(&mut self, processed: u64) {
//         self.records_processed += processed;
//     }

//     // Method to set the current stage and log progress
//     pub fn set_stage(&mut self, stage: &str) {
//         self.current_stage = stage.to_string();
//         self.log_progress_if_needed();
//     }

//     pub fn log_progress_if_needed(&mut self) {
//         let now = Instant::now();
//         let elapsed = self.start_time.elapsed().as_secs_f32();
//         let percentage = (self.records_processed as f32 / self.total_records as f32) * 100.0;
//         // let eta = if self.records_processed > 0
//         //     (elapsed / self.records_processed as f32)
//         //         * (self.total_records - self.records_processed) as f32

//         let eta = 0;

//         info!(
//             "Stage: {}, Progress: {:.2}% complete, ETA: {:.2} seconds",
//             self.current_stage, percentage, eta
//         );

//         self.last_update_time = now;
//     }

//     pub fn get_records_processed(&self) -> u64 {
//         self.records_processed
//     }
// }

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
            current_stage: Arc::new(Mutex::new(String::from("Initializing"))),
        });
        tracker.start_progress_thread();
        tracker
    }

    pub fn update_records_processed(&self, processed: u64) {
        let mut records = self.records_processed.lock().unwrap();
        *records += processed;
    }

    pub fn set_stage(&self, stage: &str) {
        let mut current_stage = self.current_stage.lock().unwrap();
        *current_stage = stage.to_string();
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
                let progress = (now_processed as f32 / total_records as f32) * 100.0;
                let throughput = (now_processed - last_processed) as f32 / interval.as_secs_f32();
                let eta = if now_processed > 0 {
                    (elapsed / progress) * (100.0 - progress)
                } else {
                    0.0
                };

                info!(
                    "Stage: {}, Progress: {:.2}% complete, ETA: {:.2} seconds, Throughput: {:.2} records/sec",
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
