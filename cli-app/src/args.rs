use dsp_domain::message::Message;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Message>,
}
