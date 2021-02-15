use std::borrow::Cow;
use std::collections::BTreeMap as Map;

use crate::common::Empty;

pub type Id = u32;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CitmCatalog<'a> {
    #[serde(borrow)]
    pub area_names: Map<Id, &'a str>,
    #[serde(borrow)]
    pub audience_sub_category_names: Map<Id, &'a str>,
    #[serde(borrow)]
    pub block_names: Map<Id, &'a str>,
    #[serde(borrow)]
    pub events: Map<Id, Event<'a>>,
    #[serde(borrow)]
    pub performances: Vec<Performance<'a>>,
    #[serde(borrow)]
    pub seat_category_names: Map<Id, &'a str>,
    #[serde(borrow)]
    pub sub_topic_names: Map<Id, &'a str>,
    #[serde(borrow)]
    pub subject_names: Map<Id, &'a str>,
    #[serde(borrow)]
    pub topic_names: Map<Id, &'a str>,
    pub topic_sub_topics: Map<Id, Vec<Id>>,
    #[serde(borrow)]
    pub venue_names: Map<&'a str, &'a str>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Event<'a> {
    pub description: (),
    pub id: Id,
    #[serde(borrow)]
    pub logo: Option<&'a str>,
    #[serde(borrow)]
    pub name: Cow<'a, str>,
    pub sub_topic_ids: Vec<Id>,
    pub subject_code: (),
    pub subtitle: (),
    pub topic_ids: Vec<Id>,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Performance<'a> {
    pub event_id: Id,
    pub id: Id,
    #[serde(borrow)]
    pub logo: Option<&'a str>,
    pub name: (),
    pub prices: Vec<Price>,
    pub seat_categories: Vec<SeatCategory>,
    pub seat_map_image: (),
    pub start: u64,
    #[serde(borrow)]
    pub venue_code: &'a str,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Price {
    pub amount: u32,
    pub audience_sub_category_id: Id,
    pub seat_category_id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SeatCategory {
    pub areas: Vec<Area>,
    pub seat_category_id: Id,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Area {
    pub area_id: Id,
    pub block_ids: Empty,
}
