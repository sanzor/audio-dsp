use std::path::PathBuf;

pub fn test_data(path:&str)->PathBuf{
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
    .join("tests/test_data")
    .join(path)
}