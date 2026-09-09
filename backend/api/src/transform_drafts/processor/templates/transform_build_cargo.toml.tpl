[package]
name = "transform_build"
version = "0.0.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
transform-sdk = { path = __SDK_PATH__ }

[profile.release]
opt-level = "s"
lto = true
panic = "abort"

[workspace]
