mod command;
mod command_parser;
mod scheduler;
use command::{Command, CommandResult};
use command_parser::*;


fn main() {
   loop{
        let user_input=read_line();

        let _result=
                parse_command(user_input.as_str())
                .map(dispatch)
                .map(|r|println!("{}",r));
        
   }
}


fn read_line()->String{
    use std::io::{self,Write};
    print!("> ");
    std::io::stdout().flush().unwrap();
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("failed to read");
    input.trim().to_string()
}


