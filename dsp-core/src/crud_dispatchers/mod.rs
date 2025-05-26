pub(crate) mod run_script;
pub(crate) use run_script::RunScriptDispatcher;

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
