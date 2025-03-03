use mpl_bubblegum::instructions::*;
use mpl_bubblegum::types::*;
use solana_program::pubkey::Pubkey;

#[rustler::nif]
fn add() -> String {
    let val: String = String::from("Hello world from elixir");
    return val;
}

#[rustler::nif]
fn mint_compressed_nft(merkle_tree_config: MintV1) -> String {

    let metadata = MetadataArgs {
        name: "Example NFT".to_string(),
        symbol: "ENFT".to_string(),
        uri: "https://example.com/nft.json".to_string(),
        seller_fee_basis_points: 500, 
        primary_sale_happened: false, 
        is_mutable: true,             
        edition_nonce: None,          
        token_standard: None,         
        collection: None,             
        uses: None,                   
        token_program_version: TokenProgramVersion::Token2022, 
        creators: vec![Creator {
            address: merkle_tree_config.payer,
            verified: true,
            share: 100,
        }],
    };

    let mint_ix = MintV1Builder::new()
        .tree_config(merkle_tree_config.tree_config)
        .leaf_owner(merkle_tree_config.leaf_owner)
        .leaf_delegate(merkle_tree_config.leaf_delegate)
        .merkle_tree(merkle_tree_config.merkle_tree)
        .payer(merkle_tree_config.payer)
        .tree_creator_or_delegate(merkle_tree_config.tree_creator_or_delegate)
        .metadata(metadata)
        .instruction();

    let tx_id: String = format!(
        "Transferred nft {} from {} to {}",
        nft_id, from_pubkey, to_pubkey
    );

    return tx_id;
}

pub struct MintV1 {
    pub tree_config: solana_program::pubkey::Pubkey,

    pub leaf_owner: solana_program::pubkey::Pubkey,

    pub leaf_delegate: solana_program::pubkey::Pubkey,

    pub merkle_tree: solana_program::pubkey::Pubkey,

    pub payer: solana_program::pubkey::Pubkey,

    pub tree_creator_or_delegate: solana_program::pubkey::Pubkey,

    pub log_wrapper: solana_program::pubkey::Pubkey,

    pub compression_program: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

rustler::init!("Elixir.BubblegumElixirClient");
