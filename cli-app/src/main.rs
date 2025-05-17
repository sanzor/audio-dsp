use clap::Parser;
use cli_app::args::Args;
use dsp_core::command_processor::CommandProcessor;





fn main() {
    let processor=CommandProcessor::new(DispatcherProvider::new(),create)
    loop {
        let args=Args::parse();

       
        let user_input = read_line();
        let _result = processor
            .process(user_input.as_str())
            .map(|r| println!("{}", r));
    }
}

fn read_line() -> String {
    use std::io::{self, Write};
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}
