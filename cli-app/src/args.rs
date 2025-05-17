use dsp_domain::dsp_command::DspCommand;

#[derive(clap::Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    command: DspCommand,
}
