use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    error::ZomeApiError,
    AGENT_ADDRESS
};
use hdk::holochain_core_types::{
    cas::content::Address,
    hash::HashString,
    entry::Entry,
    error::HolochainError,
    json::{JsonString,RawString},
    dna::entry_types::Sharing
};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{collections::BTreeMap, convert::TryFrom};

const ORION_MAIN_AGENT_ADDRESS: &'static str = "orion123_test-----------------------------------------------------------------------------fdsafdsafds";

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Order {
    exchange_addr: HashString,
    broker_addr: HashString,
    base_asset_code: String,
    quoted_asset_code: String,
    direction: Direction,
    quoted_price_per_unit: f64,
    amount: f64,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub enum Direction {
    Buy,
    Sell,
}

//todo: think
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub enum Status {
    New,
    Pending,
    Approved,
    Closed,
}

pub fn definition() -> ValidatingEntryType {
    entry!(
      name: "order",
      description: "",
      sharing: Sharing::Public,
      validation_package: || {
        hdk::ValidationPackageDefinition::Entry
      },
      validation: |validation_data: hdk::EntryValidationData<Order>| {
          //todo:
          // https://developer.holochain.org/api/latest/hdk/api/fn.property.html
          // hdk::property("public_key")

          match validation_data {
            // only match if the entry is being created (not modified or deleted)
            EntryValidationData::Create {entry, validation_data} => 


            // 1
            // {
            //     let game_proposal = GameProposal::from(entry);
            //     if validation_data.sources().contains(&game_proposal.agent) {
            //         Ok(())
            //     } else {
            //         Err("Cannot author a proposal from another agent".into())
            //     }
            // }

              // 2
                // {
                //     // **Initial Validation**
                //     // Check that the origin is from a valid device
                //     // i.e. the agent is linked from RootHash
                //     let source = &validation_data.package.chain_header.provenances()[0].0;
                //     match validation_source(source,_r.keyset_root){
                //         Ok(v) => {
                //             if v {return Ok(())}
                //             else {return Err("Could not Validate Rules: Source is not equal to the provenances".to_string())}
                //         }
                //         _=> Err("Could not Validate Rules: Source is not equal to the provenances".to_string())
                //     }
                    // **On Update**
                    // Check if signed by Prior Revocation Key on Update
                    // (field not required on Create)
                    // Ok(())



                // 3
                // when using (in validation_package) hdk::ValidationPackageDefinition::Entry
                // the chain_header for the entry is returned, providing some useful
                // metadata for validating against
                let chain_header = &validation_data.package.chain_header;
                // provenances() returns an array, since there can be multiple authors/signers for a single Entry
                let first_author = &chain_header.provenances()[0];
                // first_author is a tuple (agent_address, agent_signature)
                let first_author_agent_address = first_author.0.to_string();
                // if self is not alice, and entry author is alice, don't hold the Entry
                if hdk::AGENT_ADDRESS.to_string() != ORION_MAIN_AGENT_ADDRESS && first_author_agent_address == ORION_MAIN_AGENT_ADDRESS {
                    Err("No one but Alice will hold Alice's entries".to_string())
                }
                else {
                    Ok(())
                }

},

            ,
            _ => {
                Err("Cannot modify, only create and delete".into())
            }
          }



      },

      links: [
          to!(
              "transaction",
              tag: "transactions",
              validation_package: || {
                  hdk::ValidationPackageDefinition::Entry
              },
              validation: |_validation_data: hdk::LinkValidationData| {
                  Ok(())
              }
          ),
          from!(
            "broker",
            tag: "broker",
            validation_package: || {
                hdk::ValidationPackageDefinition::Entry
            },
            validation: |_validation_data: hdk::LinkValidationData| {
                Ok(())
            }
          )
      ]
    )
}

impl Order {
    fn new(base_asset_code: String, quoted_asset_code: String, direction: Direction, quoted_price_per_unit: f64, amount: f64) -> Self {
        Order {
          exchange_addr: HashString::default(),
          broker_addr: HashString::default(),
          base_asset_code: base_asset_code,
          quoted_asset_code: quoted_asset_code,
          direction: direction,
          quoted_price_per_unit: quoted_price_per_unit,
          amount: amount,
          status: Status::New,
        }
    }

  fn calculate_total_price(self) -> f64 {
      self.amount * self.quoted_price_per_unit
  }
}

pub fn handle_get_single(addr: HashString) -> ZomeApiResult<Order> {
    match hdk::get_entry(&addr) {
        Ok(Some(Entry::App(_, entry_json_str)))  => {
            let res = Order::try_from(entry_json_str)?;
            Ok(res)
        },
        _ => {
          Err(hdk::error::ZomeApiError::HashNotFound)
        }
    }
}

pub fn handle_approve(addr: HashString) -> ZomeApiResult<Address> {
    match hdk::get_entry(&addr) {
        Ok(Some(Entry::App(_, orig_entry_json_str))) => {
            let orig_order = Order::try_from(orig_entry_json_str)?;
            let mut upd_order = Order::new(orig_order.base_asset_code, orig_order.quoted_asset_code, orig_order.direction, orig_order.quoted_price_per_unit, orig_order.amount);
            upd_order.status = Status::Approved;
            let updated_order_entry = Entry::App("order".into(), upd_order.into());
            hdk::update_entry(updated_order_entry, &addr)
      },
        _ => {
            Err(hdk::error::ZomeApiError::HashNotFound)
        }
    }
}

pub fn handle_create(base_asset_code: String, quoted_asset_code: String, direction: Direction, quoted_price_per_unit: f64, amount: f64) -> ZomeApiResult<Address> {
    let ord1 = Order::new(base_asset_code, quoted_asset_code, direction, quoted_price_per_unit, amount);
    let ord1_ent = Entry::App("order".into(), ord1.into());
    hdk::commit_entry(&ord1_ent)
}
