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
            "Init" => init()?,
            "GetChainId" => get_chain_id(),
            "GetNonce" => get_nonce()?,
            "GetBlockHash" => get_block_hash()?,
            "GetCode" => get_code()?,
            "GetBalance" => get_balance()?,
            "GetUpgradeIndex" => get_upgrade_index(),
            "GetVersion" => get_version(),
            "GetOwner" => get_owner(),
            "SetOwner" => set_owner()?,
            "GetBridgeProver" => get_bridge_prover(),
            "GetStorageAt" => get_storage_at()?,
            "RegisterRelayer" => register_relayer()?,
            "PausePrecompiles" => pause_precompiles()?,
            "ResumePrecompiles" => resume_precompiles()?,
            "PausedPrecompiles" => paused_precompiles(),
            "FactoryUpdate" => factory_update()?,
            "FactorySetWnearAddress" => factory_set_wnear_address()?,
            "FundXccSubAccount" => fund_xcc_sub_account()?,
            "StageUpgrade" => stage_upgrade()?,
            "DeployUpgrade" => deploy_upgrade(),
            "Deploy" => deploy()?,
            "ViewCall" => view_call()?,
            "Call" => call()?,
            "EncodeAddress" => encode_address()?,
            "KeyPair" => key_pair()?,
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
