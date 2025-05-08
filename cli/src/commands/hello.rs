use clap::{ArgMatches, Command};

pub const COMMAND_NAME: &str = "hello";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("This is the example Hello command")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
    println!("You requested a greeting: Hello.");

    Ok(())
}
