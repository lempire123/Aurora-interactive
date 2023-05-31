use clap::{App, Arg, SubCommand};
use dialoguer::{theme::ColorfulTheme, Select};
use std::io::Result;

fn main() -> Result<()> {
    let matches = App::new("Aurora")
        .version("1.0")
        .author("Lance H. <lance.henderson@aurora.dev>")
        .about("Interactive Aurora CLI")
        .arg(
            Arg::new("selection")
                .short('s')
                .long("selection")
                .value_name("SELECTION")
                .help("Selects a default option")
                .takes_value(true),
        )
        .subcommand(
            SubCommand::with_name("Option 1").about("Do something").arg(
                Arg::with_name("subselection")
                    .short('s')
                    .long("subselection")
                    .help("Selects a default sub option for Option 1")
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("Option 2")
                .about("Do something else")
                .arg(
                    Arg::with_name("subselection")
                        .short('s')
                        .long("subselection")
                        .help("Selects a default sub option for Option 2")
                        .takes_value(true),
                ),
        )
        .get_matches();

    println!("Welcome to Aurora");

    let selections = &["Option 1: Do something", "Option 2: Do something else"];

    let default_selection = matches
        .value_of("selection")
        .and_then(|s| s.parse().ok())
        .unwrap_or(0);

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Please select an action")
        .default(default_selection)
        .items(&selections[..])
        .interact()?;

    match selection {
        0 => {
            let subselections = &["Sub Option 1.1", "Sub Option 1.2"];
            let subselection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select a sub action for Option 1")
                .default(0)
                .items(&subselections[..])
                .interact()?;
            match subselection {
                0 => println!("You selected Sub Option 1.1"),
                1 => println!("You selected Sub Option 1.2"),
                _ => unreachable!(),
            }
        }
        1 => {
            let subselections = &["Sub Option 2.1", "Sub Option 2.2"];
            let subselection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Please select a sub action for Option 2")
                .default(0)
                .items(&subselections[..])
                .interact()?;
            match subselection {
                0 => println!("You selected Sub Option 2.1"),
                1 => println!("You selected Sub Option 2.2"),
                _ => unreachable!(),
            }
        }
        _ => unreachable!(), // We don't allow the user to input an option out of range
    }

    Ok(())
}
