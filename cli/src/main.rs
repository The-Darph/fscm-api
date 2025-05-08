use clap::{Arg, Command};
use cli::commands;

fn main() -> anyhow::Result<()> {
    let mut command = Command::new("Skeleton API manager & configurator");
    command = commands::configure(command);

    let matches = command.get_matches();
    commands::handle(&matches)?;

    Ok(())
}
