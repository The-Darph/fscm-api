use clap::{Arg, Command};
use cli::commands;
use cli::settings::Settings;
use dotenv::dotenv;


fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let mut command = Command::new("API manager and configurator CLI")
            .arg(
                Arg::new("config")
                    .short('c')
                    .long("config")
                    .help("Configuration file location")
                    .default_value("config.json"),
            );

    command = commands::configure(command);

    let matches = command.get_matches();

    let config_location = matches
        .get_one("config")
        .map(|s: &String| Some(s.as_str()))
        .unwrap_or(None);

    // let config_location = matches
    //     .get_one::<String>("config")
    //     .map(|s| s.as_str())
    //     .unwrap_or("");

    let settings = Settings::new(config_location, "APP")?;

    commands::handle(&matches, &settings)?;

    Ok(())
}
