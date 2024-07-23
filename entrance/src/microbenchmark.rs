use std::fs::OpenOptions;

pub fn writing_to_disk() {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open("../output/output.bin")
        .unwrap();
}

pub fn reading_from_disk() {
    let file = OpenOptions::new()
        .read(true)
        .open("../output/output.bin")
        .unwrap();

}