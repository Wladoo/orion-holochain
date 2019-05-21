#![allow(unused_variables)]
#![feature(try_from)]
#[macro_use]
extern crate hdk;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
#[macro_use]
extern crate holochain_core_types_derive;

use hdk::holochain_core_types::{
    cas::content::Address,
    error::HolochainError,
    json::JsonString,
    hash::HashString
};
use hdk::{
    error::ZomeApiError,
    error::ZomeApiResult
};

pub mod broker;
pub mod trade;
pub mod order;
pub mod balance;


define_zome! {
    entries: [
        broker::definition(),
        balance::definition(),
        order::definition(),
        trade::definition()
    ]

    genesis: || {
        Ok(())
    }

    functions: [
        create_broker: {
            inputs: |name: String|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: broker::handle_create
        }

        initialize_order: {
            inputs: |base_asset_code: String, quoted_asset_code: String, direction: order::Direction, quoted_price_per_unit: f64, amount: f64|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: order::handle_create
        }

        get_order: {
            inputs: |addr: HashString|,
            outputs: |result: Result<order::Order, ZomeApiError>|,
            handler: order::handle_get_single
        }

        approve_order: {
            inputs: |addr: HashString|,
            outputs: |result: ZomeApiResult<Address>|,
            handler: order::handle_approve
        }

        create_trade: {
            inputs: |/*todo*/|,
            outputs: |result: Result<HashString, ZomeApiError>|,
            handler: trade::handle_create
        }
    ]

    traits: {
        hc_public [
            create_broker,
            create_broker2,
            initialize_order,
            get_order,
            approve_order,
            create_trade
        ]
    }
}
