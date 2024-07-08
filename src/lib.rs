mod abi;
mod pb;
use abi::nfts_contract::events::Transfer;
use helpers::format_hex;
use pb::contract::v1::{Mint, Mints};
use substreams::Hex;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::pb::eth::v2::Block;
use substreams_ethereum::Event;

mod helpers;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_mints(blk: Block) -> Result<Mints, substreams::errors::Error> {
    let mints = blk
        .logs()
        .filter_map(|log| {
            if format_hex(log.address())
                == "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D".to_lowercase()
            {
                if let Some(ape_transfer) = Transfer::match_and_decode(log) {
                    return Some(Mint {
                        token_id: (ape_transfer.token_id).to_string(),
                    });
                }
            }
            None
        })
        .collect::<Vec<Mint>>();

    Ok(Mints { mints })
}
