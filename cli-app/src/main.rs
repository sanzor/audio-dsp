
use cli_app::{

    processor::Processor,

};
use dsp_core::{api::create_command_processor};

fn main() {
    let mut processor = Processor::new(create_command_processor());
    loop {
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
