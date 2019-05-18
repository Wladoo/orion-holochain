use hdk::{
    self,
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    error::ZomeApiError
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


#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub struct Order {
    exchange_addr: HashString,
    broker_addr: HashString,
    base_asset_code: String,
    quoted_asset_code: String,
    direction: Direction,
    quoted_price_per_unit: f64,
    amount: f64,
    inserted_at: u64,
    status: Status
}

#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub enum Direction {
    Buy,
    Sell
}

//todo: think
#[derive(Serialize, Deserialize, Debug, Clone, DefaultJson)]
pub enum Status {
    New,
    Pending,
    Approved,
    Closed
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
          Ok(())
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

  // todo
  fn new(base_asset_code: String, quoted_asset_code: String, direction: Direction, quoted_price_per_unit: f64, amount: f64) -> Self {
    let ts = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    Order{
      exchange_addr: HashString::default(),
      broker_addr: HashString::default(),
      base_asset_code: base_asset_code,
      quoted_asset_code: quoted_asset_code,
      direction: direction,
      quoted_price_per_unit: quoted_price_per_unit,
      amount: amount,
      inserted_at: ts,
      status: Status::New
    }
  }

  fn calculate_total_price(self) -> f64 {
      self.amount * self.quoted_price_per_unit
  }
}

pub fn handle_get_single(addr: HashString) -> Result<Order, ZomeApiError> {
    match hdk::get_entry(&addr) {
        Ok(Some(Entry::App(_, entry_json_str)))  => {
            let res = Order::try_from(entry_json_str)?;
            Ok(res)
        }

        //todo
        // Ok(None) | Err(err) => {
        _ => {
          unimplemented!()
        }
    }
}

// todo
// todo: replace Result<(), ZomeApiError> with ZomeApiResult<HashString> ?
pub fn handle_approve(addr: HashString) -> Result<(), ZomeApiError> {
    match hdk::get_entry(&addr) {
      Ok(Some(Entry::App(_, orig_entry_json_str))) => {
        let orig_order = Order::try_from(orig_entry_json_str)?;
        let mut upd_order = Order::new(orig_order.base_asset_code, orig_order.quoted_asset_code, orig_order.direction, orig_order.quoted_price_per_unit, orig_order.amount);
        upd_order.status = Status::Approved;
        let updated_order_entry = Entry::App("order".into(), upd_order.into());
        hdk::update_entry(updated_order_entry, &addr);
        Ok(())
      }

      // todo
      _ => {
        unimplemented!()
      }
    }
}

pub fn handle_create(base_asset_code: String, quoted_asset_code: String, direction: Direction, quoted_price_per_unit: f64, amount: f64) -> Result<HashString, ZomeApiError> {
    let ord1 = Order::new(base_asset_code, quoted_asset_code, direction, quoted_price_per_unit, amount);
    let ord1_ent = Entry::App("order".into(), ord1.into());
    Ok(hdk::commit_entry(&ord1_ent)?)
}










// impl From<Option<Order>> for hdk::holochain_core_types::json::JsonString {
//   fn from(order: Order) -> Self {
//     JsonString::empty_object()
//   }
// }


// impl TryFrom<EntryType> for Order {
//     type Error = HolochainError;
//     fn try_from(entry_type: EntryType) -> Result<Self, Self::Error> {
//         match entry_type {
//             EntryType::App(app_entry_type) => Ok(app_entry_type),
//             _ => Err(HolochainError::ErrorGeneric(format!(
//                 "Attempted to convert {:?} EntryType to an Order",
//                 entry_type
//             ))),
//         }
//     }
// }
