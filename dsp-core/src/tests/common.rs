use std::path::PathBuf;

pub fn test_data(path: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("src")
        .join("tests")
        .join("test_data")
        .join(path)
}
