use std::io::Write;

use clap::Parser;
use cli_app::args::Args;
use dsp_core::command_processor::CommandProcessor;

fn main() {
    let mut processor = CommandProcessor::create_processor();
    let init_args = Args::parse();
    if let Some(command) = init_args.command {
        processor.process_command(command);
    } else {
        run_repl(&mut processor);
    }
}

fn run_repl(processor: &mut CommandProcessor) {
    // loop {
    //     let user_input = read_line();
    //     let _result = processor
    //         .process_command(user_input.as_str())
    //         .map(|r| println!("{}", r));
    // }
}

fn read_line() -> String {
    use std::io::{self, Write};
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}
