pub fn lookup(hash_to_find: &[u8; 26], directory: &str) -> io::Result<Option<Record>> {
    let paths = std::fs::read_dir(directory)?;
}
