pub(crate) mod load;
pub(crate) use load::LoadDispatcher;

pub(crate) mod copy;
pub(crate) use copy::CopyDispatcher;

pub(crate) mod delete;
pub(crate) use delete::DeleteDispatcher;

pub(crate) mod list;
pub(crate) use list::ListDispatcher;

pub(crate) mod info;
pub(crate) use info::InfoDispatcher;

pub(crate) mod upload;
pub(crate) use upload::UploadDispatcher;

pub(crate) mod gain;
pub(crate) use gain::GainDispatcher;

pub(crate) mod normalize;
pub(crate) use normalize::NormalizeDispatcher;

pub(crate) mod high_pass;
pub(crate) use high_pass::HighPassDispatcher;

pub(crate) mod low_pass;
pub(crate) use low_pass::LowPassDispatcher;
