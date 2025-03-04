use mpl_bubblegum::instructions::*;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use solana_sdk::{
    pubkey::Pubkey,
    signature::{Keypair, Signer},
    transaction::Transaction,
};

fn create_tree_config(
    payer_pubkey: String,
    tree_creator_pubkey: String,
    max_depth: u32,
    max_buffer_size: u32,
) -> Result<String, String> {
    let rpc_url = "https://api.devnet.solana.com";
    let client: RpcClient = RpcClient::new(rpc_url);

    let payer_pubkey = Pubkey::from_str(&payer_pubkey).unwrap();
    let tree_creator_pubkey = Pubkey::from_str(&tree_creator_pubkey).unwrap();

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
    let create_account_ix = solana_sdk::system_instruction::create_account(
        &payer_pubkey,
        &merkle_tree.pubkey(),
        rent_exempt_balance,
        space as u64,
        &mpl_bubblegum::ID,
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
    let tx = Transaction::new_signed_with_payer(
        &[create_account_ix, ix], // Ensure `create_account_ix` runs first
        Some(&payer_pubkey),
        &[&merkle_tree], // Merkle tree keypair signs for account creation
        recent_blockhash,
    );

    // Send transaction and return signature
    match client.send_and_confirm_transaction(&tx) {
        Ok(sig) => Ok(sig.to_string()),
        Err(err) => Err(err.to_string()),
    }
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
