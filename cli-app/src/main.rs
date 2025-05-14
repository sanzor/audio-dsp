use cli_app::command_processor::CommandProcessor;
use cli_app::{
    dispatch_provider::DispatchProvider,
    processor::Processor,
    state::{create_shared_state, SharedState},
};

fn main() {
    let mut processor = Processor::new(CommandProcessor::new(
        DispatchProvider::new(),
        initialize_state(),
    ));
    loop {
        let user_input = read_line();
        let _result = processor
            .process(user_input.as_str())
            .map(|r| println!("{}", r));
    }
}

fn initialize_state() -> SharedState {
    create_shared_state()
}
fn read_line() -> String {
    use std::io::{self, Write};
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}
