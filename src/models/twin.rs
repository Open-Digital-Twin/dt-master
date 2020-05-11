// use cdrs::query::*;
use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct Twin {
  id: i64,
  name: String
}

// impl Twin {
//   fn into_query(self) -> QueryValues {
//     query_values!("id" => self.id, "name" => self.name)
//   }
// }
