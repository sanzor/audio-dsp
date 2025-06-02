use std::io::Write;

use clap::Parser;
use cli_app::{args::Args, processor::Processor};
use dsp_domain::dsp_message_result::DspMessageResult;

#[tokio::main]
async fn main() -> Result<(), String> {
    let mut processor = Processor::create_processor();
    let init_args = Args::parse();

    if let Some(command) = init_args.command {
        let result = processor.process_command(command).await?;
        let json = serde_json::to_string_pretty(&result.output).map_err(|e| e.to_string())?;
        println!("{json}");
        Ok(())
    } else {
        run_repl(&mut processor).await?;
        Ok(())
    }
}

async fn run_repl(processor: &mut Processor) -> Result<(), String> {
    loop {
        let user_input = read_line();

        match processor.process(user_input.as_str()).await {
            Ok(DspMessageResult {
                should_exit: true,
                output: _,
            }) => break,
            Ok(result) => match serde_json::to_string_pretty(&result.output) {
                Ok(json) => println!("{json}"),
                Err(e) => eprintln!("Failed to serialize result: {e}"),
            },
            Err(e) => eprintln!("Error: {e}"),
        }
    }
    Ok(())
}

fn read_line() -> String {
    use std::io::{self, Write};
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}
