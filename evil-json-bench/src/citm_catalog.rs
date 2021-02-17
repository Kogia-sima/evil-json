use crate::common::{Empty, PrimStr};

use std::collections::BTreeMap as Map;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(simd_json_derive::Serialize)]
pub struct CitmCatalog {
    pub area_names: Map<IdStr, String>,
    pub audience_sub_category_names: Map<IdStr, String>,
    pub block_names: Map<IdStr, String>,
    pub events: Map<IdStr, Event>,
    pub performances: Vec<Performance>,
    pub seat_category_names: Map<IdStr, String>,
    pub sub_topic_names: Map<IdStr, String>,
    pub subject_names: Map<IdStr, String>,
    pub topic_names: Map<IdStr, String>,
    pub topic_sub_topics: Map<IdStr, Vec<Id>>,
    pub venue_names: Map<String, String>,
}

pub type Id = u32;
pub type IdStr = PrimStr<u32>;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(simd_json_derive::Serialize)]
pub struct Event {
    pub description: (),
    pub id: Id,
    pub logo: Option<String>,
    pub name: String,
    pub sub_topic_ids: Vec<Id>,
    pub subject_code: (),
    pub subtitle: (),
    pub topic_ids: Vec<Id>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(simd_json_derive::Serialize)]
pub struct Performance {
    pub event_id: Id,
    pub id: Id,
    pub logo: Option<String>,
    pub name: (),
    pub prices: Vec<Price>,
    pub seat_categories: Vec<SeatCategory>,
    pub seat_map_image: (),
    pub start: u64,
    pub venue_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(simd_json_derive::Serialize)]
pub struct Price {
    pub amount: u32,
    pub audience_sub_category_id: Id,
    pub seat_category_id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(simd_json_derive::Serialize)]
pub struct SeatCategory {
    pub areas: Vec<Area>,
    pub seat_category_id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
#[derive(simd_json_derive::Serialize)]
pub struct Area {
    pub area_id: Id,
    pub block_ids: Empty,
}
