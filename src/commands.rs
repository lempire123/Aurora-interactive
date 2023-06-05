use dialoguer::{Confirm, Input};
use std::io::Result;

#[derive(Debug)]
pub enum Command {
    CreateAccount {
        account: String,
        balance: f64,
    },
    ViewAccount {
        account: String,
    },
    DeployAurora {
        path: String,
    },
    Init {
        chain_id: u64,
        owner_id: Option<String>,
        bridge_prover_id: Option<String>,
        upgrade_delay_blocks: Option<u64>,
        custodian_address: Option<String>,
        ft_metadata_path: Option<String>,
    },
    GetChainId,
    GetNonce {
        address: String,
    },
    GetBlockHash {
        height: u64,
    },
    GetCode {
        address: String,
    },
    GetBalance {
        address: String,
    },
    GetUpgradeIndex,
    GetVersion,
    GetOwner,
    SetOwner {
        account_id: String,
    },
    GetBridgeProver,
    GetStorageAt {
        address: String,
        key: String,
    },
    RegisterRelayer {
        address: String,
    },
    PausePrecompiles {
        mask: u32,
    },
    ResumePrecompiles {
        mask: u32,
    },
    PausedPrecompiles,
    FactoryUpdate {
        path: String,
    },
    FactorySetWnearAddress {
        address: String,
    },
    FundXccSubAccount {
        target: String,
        wnear_account_id: Option<String>,
        deposit: f64,
    },
    StageUpgrade {
        path: String,
    },
    DeployUpgrade,
    Deploy {
        code: String,
        args: Option<String>,
        abi_path: Option<String>,
        aurora_secret_key: Option<String>,
    },
    ViewCall {
        address: String,
        function: String,
        args: Option<String>,
        abi_path: String,
    },
    Call {
        address: String,
        function: String,
        args: Option<String>,
        abi_path: String,
        value: Option<String>,
        aurora_secret_key: Option<String>,
    },
    EncodeAddress {
        account: String,
    },
    KeyPair {
        random: bool,
        seed: Option<u64>,
    },
}

pub fn create_account() -> Result<Command> {
    let account: String = Input::new()
        .with_prompt("Please enter the account")
        .interact()?;
    let balance: f64 = Input::new()
        .with_prompt("Please enter the balance")
        .interact()?;
    Ok(Command::CreateAccount { account, balance })
}

pub fn view_account() -> Result<Command> {
    let account: String = Input::new()
        .with_prompt("Please enter the account")
        .interact()?;
    Ok(Command::ViewAccount { account })
}

pub fn deploy_aurora() -> Result<Command> {
    let path: String = Input::new()
        .with_prompt("Please enter the path to the WASM file")
        .interact()?;
    Ok(Command::DeployAurora { path })
}

pub fn init() -> Result<Command> {
    let chain_id: u64 = Input::new()
        .with_prompt("Please enter the chain ID")
        .interact()?;

    let owner_id_input: String = Input::new()
        .with_prompt("Please enter the owner ID (optional)")
        .default("".to_string())
        .interact()?;
    let owner_id = if owner_id_input.is_empty() {
        None
    } else {
        Some(owner_id_input)
    };

    let bridge_prover_id_input: String = Input::new()
        .with_prompt("Please enter the bridge prover ID (optional)")
        .default("".to_string())
        .interact()?;
    let bridge_prover_id = if bridge_prover_id_input.is_empty() {
        None
    } else {
        Some(bridge_prover_id_input)
    };

    let upgrade_delay_blocks_input: String = Input::new()
        .with_prompt("Please enter the upgrade delay blocks (optional)")
        .default("".to_string())
        .interact()?;
    let upgrade_delay_blocks = upgrade_delay_blocks_input.parse::<u64>().ok();

    let custodian_address_input: String = Input::new()
        .with_prompt("Please enter the custodian address (optional)")
        .default("".to_string())
        .interact()?;
    let custodian_address = if custodian_address_input.is_empty() {
        None
    } else {
        Some(custodian_address_input)
    };

    let ft_metadata_path_input: String = Input::new()
        .with_prompt("Please enter the FT metadata path (optional)")
        .default("".to_string())
        .interact()?;
    let ft_metadata_path = if ft_metadata_path_input.is_empty() {
        None
    } else {
        Some(ft_metadata_path_input)
    };

    Ok(Command::Init {
        chain_id,
        owner_id,
        bridge_prover_id,
        upgrade_delay_blocks,
        custodian_address,
        ft_metadata_path,
    })
}

pub fn get_chain_id() -> Command {
    Command::GetChainId
}

pub fn get_nonce() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    Ok(Command::GetNonce { address })
}

pub fn get_block_hash() -> Result<Command> {
    let height: u64 = Input::new()
        .with_prompt("Please enter the block height")
        .interact()?;
    Ok(Command::GetBlockHash { height })
}

pub fn get_code() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    Ok(Command::GetCode { address })
}

pub fn get_balance() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    Ok(Command::GetBalance { address })
}

pub fn get_upgrade_index() -> Command {
    Command::GetUpgradeIndex
}

pub fn get_version() -> Command {
    Command::GetVersion
}

pub fn get_owner() -> Command {
    Command::GetOwner
}

pub fn set_owner() -> Result<Command> {
    let account_id: String = Input::new()
        .with_prompt("Please enter the account ID")
        .interact()?;
    Ok(Command::SetOwner { account_id })
}

pub fn get_bridge_prover() -> Command {
    Command::GetBridgeProver
}

pub fn get_storage_at() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    let key: String = Input::new()
        .with_prompt("Please enter the key")
        .interact()?;
    Ok(Command::GetStorageAt { address, key })
}

pub fn register_relayer() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    Ok(Command::RegisterRelayer { address })
}

pub fn pause_precompiles() -> Result<Command> {
    let mask: u32 = Input::new()
        .with_prompt("Please enter the mask")
        .interact()?;

    Ok(Command::PausePrecompiles { mask })
}

pub fn resume_precompiles() -> Result<Command> {
    let mask: u32 = Input::new()
        .with_prompt("Please enter the mask")
        .interact()?;
    Ok(Command::ResumePrecompiles { mask })
}

pub fn paused_precompiles() -> Command {
    Command::PausedPrecompiles
}

pub fn factory_update() -> Result<Command> {
    let path: String = Input::new()
        .with_prompt("Please enter the path to the update file")
        .interact()?;
    Ok(Command::FactoryUpdate { path })
}

pub fn factory_set_wnear_address() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the WNEAR address")
        .interact()?;
    Ok(Command::FactorySetWnearAddress { address })
}

pub fn fund_xcc_sub_account() -> Result<Command> {
    let target: String = Input::new()
        .with_prompt("Please enter the target account")
        .interact()?;
    let wnear_account_id_input: String = Input::new()
        .with_prompt("Please enter the WNEAR account ID (optional)")
        .interact()?;
    let wnear_account_id = if wnear_account_id_input.is_empty() {
        None
    } else {
        Some(wnear_account_id_input)
    };
    let deposit: f64 = Input::new()
        .with_prompt("Please enter the deposit amount")
        .interact()?;
    Ok(Command::FundXccSubAccount {
        target,
        wnear_account_id,
        deposit,
    })
}

pub fn stage_upgrade() -> Result<Command> {
    let path: String = Input::new()
        .with_prompt("Please enter the path to the upgrade file")
        .interact()?;
    Ok(Command::StageUpgrade { path })
}

pub fn deploy_upgrade() -> Command {
    Command::DeployUpgrade
}

pub fn deploy() -> Result<Command> {
    let code: String = Input::new()
        .with_prompt("Please enter the code")
        .interact()?;
    let args_input: String = Input::new()
        .with_prompt("Please enter the arguments (optional)")
        .interact()?;
    let args = if args_input.is_empty() {
        None
    } else {
        Some(args_input)
    };
    let abi_path_input: String = Input::new()
        .with_prompt("Please enter the ABI path (optional)")
        .interact()?;
    let abi_path = if abi_path_input.is_empty() {
        None
    } else {
        Some(abi_path_input)
    };
    let aurora_secret_key_input: String = Input::new()
        .with_prompt("Please enter the Aurora secret key (optional)")
        .interact()?;
    let aurora_secret_key = if aurora_secret_key_input.is_empty() {
        None
    } else {
        Some(aurora_secret_key_input)
    };
    Ok(Command::Deploy {
        code,
        args,
        abi_path,
        aurora_secret_key,
    })
}

pub fn view_call() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    let function: String = Input::new()
        .with_prompt("Please enter the function")
        .interact()?;
    let args_input: String = Input::new()
        .with_prompt("Please enter the arguments (optional)")
        .interact()?;
    let args = if args_input.is_empty() {
        None
    } else {
        Some(args_input)
    };
    let abi_path: String = Input::new()
        .with_prompt("Please enter the ABI path")
        .interact()?;
    Ok(Command::ViewCall {
        address,
        function,
        args,
        abi_path,
    })
}

pub fn call() -> Result<Command> {
    let address: String = Input::new()
        .with_prompt("Please enter the address")
        .interact()?;
    let function: String = Input::new()
        .with_prompt("Please enter the function")
        .interact()?;
    let args_input: String = Input::new()
        .with_prompt("Please enter the arguments (optional)")
        .interact()?;
    let args = if args_input.is_empty() {
        None
    } else {
        Some(args_input)
    };
    let abi_path: String = Input::new()
        .with_prompt("Please enter the ABI path")
        .interact()?;
    let value_input: String = Input::new()
        .with_prompt("Please enter the value (optional)")
        .interact()?;
    let value = if value_input.is_empty() {
        None
    } else {
        Some(value_input)
    };
    let aurora_secret_key_input: String = Input::new()
        .with_prompt("Please enter the Aurora secret key (optional)")
        .interact()?;
    let aurora_secret_key = if aurora_secret_key_input.is_empty() {
        None
    } else {
        Some(aurora_secret_key_input)
    };
    Ok(Command::Call {
        address,
        function,
        args,
        abi_path,
        value,
        aurora_secret_key,
    })
}

pub fn encode_address() -> Result<Command> {
    let account: String = Input::new()
        .with_prompt("Please enter the account")
        .interact()?;
    Ok(Command::EncodeAddress { account })
}

pub fn key_pair() -> Result<Command> {
    let random: bool = Confirm::new()
        .with_prompt("Generate a random key pair?")
        .interact()?;
    let seed: Option<u64> = if !random {
        let seed: u64 = Input::new()
            .with_prompt("Please enter the seed")
            .interact()?;
        Some(seed)
    } else {
        None
    };
    Ok(Command::KeyPair { random, seed })
}
