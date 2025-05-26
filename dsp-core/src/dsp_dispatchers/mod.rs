pub(crate) mod gain;
pub(crate) use gain::GainDispatcher;

pub(crate) mod normalize;
pub(crate) use normalize::NormalizeDispatcher;

pub(crate) mod high_pass;
pub(crate) use high_pass::HighPassDispatcher;

pub(crate) mod low_pass;
pub(crate) use low_pass::LowPassDispatcher;
