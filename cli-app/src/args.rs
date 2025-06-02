use dsp_domain::dsp_message::DspMessage;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<DspMessage>,
}
