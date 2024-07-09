mod abi;
mod pb;
use abi::nfts_contract::events::Transfer;
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

#[substreams::handlers::map]
fn map_ape_mints(blk: Block) -> Result<Mints, substreams::errors::Error> {
    let ape_mints = blk
        .calls()
        .filter_map(|call| {
            if format_hex(&call.call.address) == BAYC.to_lowercase() {
                match &call.call.input {
                    Some(input_data) => {
                        substreams::log::info!("{:?}", &input_data);
                        Some(Mint {
                            num_of_tokens: input_data,
                        })
                    }
                    None => None,
                }
            }
            None
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
