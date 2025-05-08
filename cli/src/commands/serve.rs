use clap::{ArgMatches, Command};

pub const COMMAND_NAME: &str = "serve";

pub fn configure() -> Command {
    Command::new(COMMAND_NAME).about("Start HTTP server")
}

pub fn handle(_matches: &ArgMatches) -> anyhow::Result<()> {
 
    println!("This will start the server when implemented and take another argument to specify port number");

    Ok(())
}
