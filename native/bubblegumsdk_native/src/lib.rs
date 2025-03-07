use mpl_bubblegum::instructions::*;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

#[derive(Debug)]
enum BubblegumError {
    PubkeyError(String),
    RpcError(String),
    TransactionError(String),
}

impl std::error::Error for BubblegumError {}

impl std::fmt::Display for BubblegumError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BubblegumError::PubkeyError(msg) => write!(f, "Pubkey error: {}", msg),
            BubblegumError::RpcError(msg) => write!(f, "RPC error: {}", msg),
            BubblegumError::TransactionError(msg) => write!(f, "Transaction error: {}", msg),
        }
    }
}

#[rustler::nif]
fn create_tree_config(
    _payer_pubkey: String,
    _tree_creator_pubkey: String,
    _merkleTree: Box<dyn Signer>,
    _merkle_tree_size: u32,
    _maxbuffer_size: u32,
) -> String {
    let rpc_url = "https://api.devnet.solana.com";
    let client: RpcClient = RpcClient::new(rpc_url);

    let payer_pubkey = parse_pubkey(&_payer_pubkey)?;
    let tree_creator_pubkey = parse_pubkey(&_tree_creator_pubkey)?;

    // Generate a new keypair for the Merkle Tree account
    let merkle_tree = Keypair::new();

    // Derive the Tree Config PDA
    let (tree_config_pubkey, _) =
        Pubkey::find_program_address(&[merkle_tree.pubkey().as_ref()], &mpl_bubblegum::ID);

    // Define additional required accounts
    let system_program = solana_program::system_program::ID;
    let log_wrapper = spl_noop::ID;
    let compression_program = spl_account_compression::ID;

    // Determine the correct space allocation for the Merkle tree
    let space = 8 + 32 * (1 << max_depth);
    let rent_exempt_balance = client
        .get_minimum_balance_for_rent_exemption(space)
        .unwrap();

    // Create the account for the Merkle tree
    let create_account_ix = create_account_instruction(
        &payer_pubkey,
        &merkle_tree.pubkey(),
        rent_exempt_balance,
        space as u64,
        &ID,
    );

    // Create the tree configuration instruction
    let ix = CreateTreeConfigBuilder::new()
        .merkle_tree(merkle_tree.pubkey())
        .tree_config(tree_config_pubkey)
        .payer(payer_pubkey)
        .tree_creator(tree_creator_pubkey)
        .max_depth(max_depth)
        .max_buffer_size(max_buffer_size)
        .log_wrapper(log_wrapper)
        .compression_program(compression_program)
        .system_program(system_program)
        .instruction();

    // Fetch recent blockhash
    let recent_blockhash = client.get_latest_blockhash().unwrap();

    // Construct and sign transaction with both instructions
    let tx = construct_transaction(
        &payer_pubkey,
        &merkle_tree,
        &create_account_ix,
        &ix,
        &recent_blockhash,
    )?;

    // Send transaction and return signature
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => Ok(sig.to_string()),
        Err(err) => Err(err.to_string()),
    }
}

fn parse_pubkey(pubkey_str: &str) -> Result<Pubkey, BubblegumError> {
    Pubkey::from_str(pubkey_str).map_err(|e| BubblegumError::PubkeyError(e.to_string()))
}

fn get_minimum_balance_for_rent_exemption(
    client: &RpcClient,
    space: u64,
) -> Result<u64, BubblegumError> {
    client
        .get_minimum_balance_for_rent_exemption(space)
        .map_err(|e| BubblegumError::RpcError(e.to_string()))
}

fn create_account_instruction(
    payer: &Pubkey,
    new_account: &Pubkey,
    lamports: u64,
    space: u64,
    program_id: &Pubkey,
) -> solana_sdk::system_instruction::CreateAccount {
    system_instruction::create_account(payer, new_account, lamports, space, program_id)
}

fn construct_transaction(
    payer: &Pubkey,
    signer: &Keypair,
    create_account_ix: &solana_sdk::system_instruction::CreateAccount,
    ix: &solana_sdk::instruction::Instruction,
    recent_blockhash: &str,
) -> Result<Transaction, BubblegumError> {
    Transaction::new_signed_with_payer(
        &[create_account_ix.clone(), ix.clone()], // Ensure `create_account_ix` runs first
        Some(payer),
        &[signer],
        recent_blockhash,
    )
    .map_err(|e| BubblegumError::TransactionError(e.to_string()))
}

pub struct MintV1 {
    pub tree_config: Pubkey,

    pub leaf_owner: Pubkey,

    pub leaf_delegate: Pubkey,

    pub merkle_tree: Pubkey,

    pub payer: Pubkey,

    pub tree_creator_or_delegate: Pubkey,

    pub log_wrapper: Pubkey,

    pub compression_program: Pubkey,

    pub system_program: Pubkey,
}

rustler::init!("Elixir.BubblegumElixirClient");
