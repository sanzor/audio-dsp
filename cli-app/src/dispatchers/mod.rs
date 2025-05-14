pub mod load;
pub use load::LoadDispatcher;

pub mod copy;
pub use copy::CopyDispatcher;

pub mod delete;
pub use delete::DeleteDispatcher;

pub mod list;
pub use list::ListDispatcher;

pub mod info;
pub use info::InfoDispatcher;

pub mod upload;
pub use upload::UploadDispatcher;

pub mod gain;
pub use gain::GainDispatcher;

pub mod normalize;
pub use normalize::NormalizeDispatcher;

pub mod high_pass;
pub use high_pass::HighPassDispatcher;

pub mod low_pass;
pub use low_pass::LowPassDispatcher;
