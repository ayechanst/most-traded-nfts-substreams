mod abi;
mod pb;
use abi::nfts_contract::events::Transfer;
use abi::nfts_contract::functions::MintApe;
use helpers::format_hex;
use pb::contract::v1::{Mint, Mints, TokensTransfer, TokensTransfers};
use substreams::pb::substreams::module::input;
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::{Event, Function};

mod helpers;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

pub const BAYC: &str = "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D";
pub const ZERO_ADDRESS: &str = "0x0000000000000000000000000000000000000000";
pub const MINT_APE: &str = "0xa723533e0000000000000000000000000000000000000000000000000000000000000002";

#[substreams::handlers::map]
fn map_ape_mints(blk: Block) -> Result<Mints, substreams::errors::Error> {
    let ape_mints = blk
        .logs()
        .filter_map(|log_view| {
            if format_hex(&log_view.log.address) == BAYC.to_lowercase() {
                if format_hex(&log_view.log.topics[0] == MINT_APE.to_lowercase()) {
                    // left off here. we want to find the value of a variable in the Mint_Ape function
                    if let Some(value) = &log_view.
                    Some(Mint {
                        num_of_tokens: value.
                    })
                }
                // if let Some(mint) = MintApe::match_and_decode(log_view) {
                    //     let token_id = mint.number_of_tokens.to_string();
                //     Some(mint);
                // }
                None
            }
            None
        })
        .map(|mint| Mint {
            num_of_tokens: mint.number_of_tokens,
        })
        .collect::<Vec<Mint>>();
    Ok(Mints { mints: ape_mints })
}

#[substreams::handlers::map]
fn map_transfers(blk: Block) -> Result<TokensTransfers, substreams::errors::Error> {
    let transfers = blk
        .logs()
        .filter_map(|log| {
            if format_hex(log.address()) == BAYC.to_lowercase() {
                if let Some(event) = Transfer::match_and_decode(log) {
                    return Some(TokensTransfer {
                        from: event.from,
                        to: event.to,
                        token_id: event.token_id.to_string(),
                    });
                }
            }
            None
        })
        .collect::<Vec<TokensTransfer>>();

    Ok(TokensTransfers {
        tokens_transfers: transfers,
    })
}
