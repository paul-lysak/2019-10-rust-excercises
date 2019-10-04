#[macro_use]
extern crate clap;

//use clap::{Arg, App, SubCommand};
use clap::App;


#[derive(Debug)]
struct CmdLine {
    items: i32,
}

fn parse_cmd_line() -> CmdLine {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();
    println!("Matches: {:?}", matches);
    //TODO wrap an error
    let items = matches.value_of("items").unwrap().parse::<i32>().unwrap();
    CmdLine { items: items }
}

fn main() {
    let cmd_line = parse_cmd_line();
    println!("Command line args: {:?}", cmd_line);
}
