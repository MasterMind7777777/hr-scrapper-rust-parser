use crate::schema::{element_selector, elements, pages, selectors};
use chrono::NaiveDateTime;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = pages)]
pub struct Page {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub html: String,
    pub html_hash: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[table_name = "pages"]
pub struct NewPage {
    pub name: String,
    pub url: String,
    pub html: String,
    pub html_hash: String,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Page))]
#[diesel(table_name = elements)]
pub struct Element {
    pub id: Uuid,
    pub page_id: Uuid,
    pub name: String,
    pub html: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = elements)]
pub struct NewElement<'a> {
    pub page_id: Uuid,
    pub name: &'a str,
    pub html: &'a str,
}

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = selectors)]
pub struct Selector {
    pub id: Uuid,
    pub path: String,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = selectors)]
pub struct NewSelector<'a> {
    pub path: &'a str,
}

#[derive(Queryable, Identifiable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Element))]
#[diesel(belongs_to(Selector))]
#[diesel(table_name = element_selector)]
pub struct ElementSelector {
    pub id: Uuid,
    pub element_id: Uuid,
    pub selector_id: Uuid,
    pub matches: i32,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = element_selector)]
pub struct NewElementSelector {
    pub element_id: Uuid,
    pub selector_id: Uuid,
    pub matches: i32,
}
