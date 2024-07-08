mod abi;
mod pb;
use helpers::format_hex;
use hex_literal::hex;
use pb::contract::v1 as contract;
use substreams::Hex;
use substreams_database_change::pb::database::DatabaseChanges;
use substreams_database_change::tables::Tables as DatabaseChangeTables;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_entity_change::tables::Tables as EntityChangesTables;
use substreams_ethereum::pb::eth::v2 as eth;
use substreams_ethereum::Event;
mod helpers;

#[allow(unused_imports)]
use num_traits::cast::ToPrimitive;
use std::str::FromStr;
use substreams::scalar::BigDecimal;

substreams_ethereum::init!();

#[substreams::handlers::map]
fn map_nft_transfers(blk: &eth::Block) -> Result<Transfers, substreams::errors::Error> {
    let transfers = blk.logs().filter_map(|log| {
        if format_hex(&log.log.address)
            == "0xBC4CA0EdA7647A8aB7C2061c2E118A18a936f13D".to_lowercase()
        {
            if let Some(transfer) = Transfer::match_and_decode(log) {
                let tokenID = 
            }
        } else {
            None;
        }
    });
}
