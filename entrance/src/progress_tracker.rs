// this file serves prints the progress of each stage of the vault operations to the command line
use spdlog::info;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const RECORD_SIZE: usize = 32; // 6 bytes for nonce + 26 bytes for hash

pub struct ProgressTracker {
    start_time: Instant,
    total_records: u64,
    records_processed: Arc<Mutex<u64>>,
    stage_records_processed: Arc<Mutex<u64>>,
    update_interval: Duration,
    current_stage: Arc<Mutex<String>>,
    last_update_time: Arc<Mutex<Instant>>,
    last_processed_count: Arc<Mutex<u64>>,
}

impl ProgressTracker {
    pub fn new(total_records: u64, update_interval: Duration) -> Arc<Self> {
        let tracker = Arc::new(Self {
            start_time: Instant::now(),
            total_records,
            records_processed: Arc::new(Mutex::new(0)),
            stage_records_processed: Arc::new(Mutex::new(0)),
            update_interval,
            current_stage: Arc::new(Mutex::new(String::from("[START]"))),
            last_update_time: Arc::new(Mutex::new(Instant::now())),
            last_processed_count: Arc::new(Mutex::new(0)),
        });
        tracker.start_progress_thread();
        tracker
    }

    pub fn update_records_processed(&self, processed: u64) {
        let mut total_records = self.records_processed.lock().unwrap();
        let mut stage_records = self.stage_records_processed.lock().unwrap();
        *total_records = (*total_records + processed).min(self.total_records);
        *stage_records += processed;
    }

    pub fn set_stage(&self, stage: &str) {
        let mut current_stage = self.current_stage.lock().unwrap();
        *current_stage = stage.to_string();

        // reset stage records processed when a new stage starts
        if stage == "[SORTING]" {
            *self.stage_records_processed.lock().unwrap() = 0;
        }
    }

    // this one outputs the large throughput numbers
    fn start_progress_thread(&self) {
        let interval = self.update_interval;
        let total_records = self.total_records;
        let current_stage = self.current_stage.clone();
        let stage_records = self.stage_records_processed.clone();
        let _last_update_time = self.last_update_time.clone();
        let last_processed_count = self.last_processed_count.clone();

        thread::spawn(move || {
            let mut last_time = Instant::now();
            let mut last_logged_progress: Option<(f32, f64, f64)> = None;

            while Arc::strong_count(&stage_records) > 1 {
                thread::sleep(interval);
                let now_processed = *stage_records.lock().unwrap();
                let now = Instant::now();
                let elapsed = now.duration_since(last_time).as_secs_f64(); // time since last update
                let progress = (now_processed as f32 / total_records as f32) * 100.0;

                let eta = if now_processed > 0 && progress < 100.0 {
                    (elapsed / progress as f64) * (100.0 - progress) as f64 * 10 as f64
                } else {
                    0.0
                };

                let bytes_processed = (now_processed - *last_processed_count.lock().unwrap())
                    as f64
                    * RECORD_SIZE as f64;
                let throughput = bytes_processed / (1024.0 * 1024.0) / elapsed;

                // check if the current progress, ETA, or throughput is different from the last logged values
                let current_log = (progress, eta, throughput);
                if last_logged_progress.map_or(true, |last| last != current_log) {
                    if current_stage.lock().unwrap().clone() == "[SYNCING]" {
                        info!("{}", current_stage.lock().unwrap().clone());
                    } else {
                        info!(
                            "{}: {:.2}% complete, ETA: {:.1} seconds, Throughput: {:.2} MB/sec",
                            *current_stage.lock().unwrap(),
                            progress,
                            eta,
                            throughput
                        );
                    }

                    last_logged_progress = Some(current_log); // update the last logged values
                }

                *last_processed_count.lock().unwrap() = now_processed;
                last_time = now;
            }
        });
    }

    // // this one outputs the smaller throughput numbers
    // fn start_progress_thread(&self) {
    //     let interval = self.update_interval;
    //     let total_records = self.total_records;
    //     let current_stage = self.current_stage.clone();
    //     let stage_records = self.stage_records_processed.clone();
    //     let last_update_time = self.last_update_time.clone();
    //     let last_processed_count = self.last_processed_count.clone();

    //     thread::spawn(move || {
    //         let mut last_time = Instant::now();
    //         let mut last_logged_progress: Option<(f32, f64, f64)> = None;

    //         while Arc::strong_count(&stage_records) > 1 {
    //             thread::sleep(interval);
    //             let now_processed = *stage_records.lock().unwrap();
    //             let now = Instant::now();
    //             let elapsed = now.duration_since(last_time).as_secs_f64(); // time since last update
    //             let progress = (now_processed as f32 / total_records as f32) * 100.0;

    //             let eta = if now_processed > 0 && progress < 100.0 {
    //                 (elapsed / progress as f64) * (100.0 - progress) as f64 * 10 as f64
    //             } else {
    //                 0.0
    //             };

    //             let bytes_processed = (now_processed - *last_processed_count.lock().unwrap())
    //                 as f64
    //                 * RECORD_SIZE as f64;
    //             let elapsed_since_last_update =
    //                 last_update_time.lock().unwrap().elapsed().as_secs_f64();
    //             let throughput = if elapsed_since_last_update > 0.0 {
    //                 bytes_processed / (1024.0 * 1024.0) / elapsed_since_last_update
    //             } else {
    //                 0.0
    //             };

    //             // check if the current progress, ETA, or throughput is different from the last logged values
    //             let current_log = (progress, eta, throughput);
    //             if last_logged_progress.map_or(true, |last| last != current_log) {
    //                 if current_stage.lock().unwrap().clone() == "[SYNCING]" {
    //                     info!("{}", current_stage.lock().unwrap().clone());
    //                 } else {
    //                     info!(
    //                         "{}: {:.2}% complete, ETA: {:.1} seconds, Throughput: {:.2} MB/sec",
    //                         *current_stage.lock().unwrap(),
    //                         progress,
    //                         eta,
    //                         throughput
    //                     );
    //                 }

    //                 last_logged_progress = Some(current_log); // update the last logged values
    //             }

    //             *last_processed_count.lock().unwrap() = now_processed;
    //             last_time = now; // update the last time to current
    //         }
    //     });
    // }

    // all this function does is reports the progress at the given time it is called
    pub fn report_progress(&self) {
        let records = self.records_processed.lock().unwrap();
        let start_time = self.start_time;
        let total_records = self.total_records;
        let last_update_time = self.last_update_time.lock().unwrap();
        let last_count = self.last_processed_count.lock().unwrap();

        let now_processed = *records;
        let elapsed = start_time.elapsed().as_secs_f32();
        let progress = (now_processed as f32 / total_records as f32 * 100.0).min(100.0);
        let elapsed_since_last_update = last_update_time.elapsed().as_secs_f32();

        let eta = if now_processed > 0 && progress < 100.0 {
            (elapsed / (progress / 100.0)) * (100.0 - progress)
        } else {
            0.0
        };

        let throughput = if elapsed_since_last_update > 0.0 {
            (now_processed - *last_count) as f32 / elapsed_since_last_update / (1024.0 * 1024.0)
        } else {
            0.0
        };

        info!(
            "{}: {:.2}% complete, ETA: {:.1} seconds, Throughput: {:.2} MB/sec",
            *self.current_stage.lock().unwrap(),
            progress,
            eta,
            throughput
        );
    }
}
