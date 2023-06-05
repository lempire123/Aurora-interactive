mod ascii_art;
mod commands;

// Standard library imports
use std::io::Result;
use std::process;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// External crate imports
use ctrlc::{self, set_handler};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

// Internal module imports
use commands::*;

fn main() -> Result<()> {
    println!("{}", ascii_art::ASCII_ART);

    println!("Aurora CLI allows simple intuitive interaction with the Aurora EVM smart contract.");
    println!();

    // Ask user to proceed or quit.
    let proceed = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Would you like to proceed to choose from the commands?")
        .interact()?;

    if !proceed {
        println!("Thanks for using Aurora CLI. Have a nice day!");
        return Ok(());
    }

    let selections = &[
        "CreateAccount",
        "ViewAccount",
        "DeployAurora",
        "Init",
        "GetChainId",
        "GetNonce",
        "GetBlockHash",
        "GetCode",
        "GetBalance",
        "GetUpgradeIndex",
        "GetVersion",
        "GetOwner",
        "SetOwner",
        "GetBridgeProver",
        "GetStorageAt",
        "RegisterRelayer",
        "PausePrecompiles",
        "ResumePrecompiles",
        "PausedPrecompiles",
        "FactoryUpdate",
        "FactorySetWnearAddress",
        "FundXccSubAccount",
        "StageUpgrade",
        "DeployUpgrade",
        "Deploy",
        "ViewCall",
        "Call",
        "EncodeAddress",
        "KeyPair",
    ];

    // Atomic boolean to handle graceful shutdown
    let running = Arc::new(AtomicBool::new(true));

    set_handler(move || {
        println!("\nProcess terminated. Thanks for using Aurora CLI. Have a good day!");
        process::exit(0);
    })
    .expect("Error setting Ctrl-C handler");

    // Main loop
    while running.load(Ordering::SeqCst) {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Please select a command")
            .default(0)
            .items(&selections[..])
            .interact()?;

        let command = match selections[selection] {
            "CreateAccount" => create_account()?,
            "ViewAccount" => view_account()?,
            "DeployAurora" => deploy_aurora()?,
            "Init" => commands::init()?,
            "GetChainId" => commands::get_chain_id(),
            "GetNonce" => commands::get_nonce()?,
            "GetBlockHash" => commands::get_block_hash()?,
            "GetCode" => commands::get_code()?,
            "GetBalance" => commands::get_balance()?,
            "GetUpgradeIndex" => commands::get_upgrade_index(),
            "GetVersion" => commands::get_version(),
            "GetOwner" => commands::get_owner(),
            "SetOwner" => commands::set_owner()?,
            "GetBridgeProver" => commands::get_bridge_prover(),
            "GetStorageAt" => commands::get_storage_at()?,
            "RegisterRelayer" => commands::register_relayer()?,
            "PausePrecompiles" => commands::pause_precompiles()?,
            "ResumePrecompiles" => commands::resume_precompiles()?,
            "PausedPrecompiles" => commands::paused_precompiles(),
            "FactoryUpdate" => commands::factory_update()?,
            "FactorySetWnearAddress" => commands::factory_set_wnear_address()?,
            "FundXccSubAccount" => commands::fund_xcc_sub_account()?,
            "StageUpgrade" => commands::stage_upgrade()?,
            "DeployUpgrade" => commands::deploy_upgrade(),
            "Deploy" => commands::deploy()?,
            "ViewCall" => commands::view_call()?,
            "Call" => commands::call()?,
            "EncodeAddress" => commands::encode_address()?,
            "KeyPair" => commands::key_pair()?,
            _ => unimplemented!(),
        };

        println!("You selected: {:?}", command);

        // Prompt for next command or exit
        let continue_running = Confirm::new()
            .with_prompt("Do you want to choose another command?")
            .interact()?;

        if !continue_running {
            break;
        }
    }

    println!("Thanks for using Aurora CLI. Have a nice day!");

    Ok(())
}
