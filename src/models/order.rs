use crate::config::DATE_FORMAT;
use crate::models::user::User;
use crate::models::instrument::Instrument;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Queryable)]
pub struct Order {
    pub id : i32,
    pub userid : i32,
    pub instrumentid : i32,
    pub side : String,
    pub ord_status : String,
    pub ord_type : String,
    pub exec_inst : String,
    pub time_in_force : String,
    pub initial_qty : i32,
    pub leaves_qty : i32,
    pub price : f32,
    pub created_at : DateTime<Utc>,
    pub updated_at : DateTime<Utc>,
}

impl Order {
    pub fn attach(self, author: User, favorited: bool) : OrderJson {
        OrderJson {
            id: self.id,
            slug: self.slug,
            title: self.title,
            description: self.description,
            body: self.body,
            author,
            tag_list: self.tag_list,
            created_at: self.created_at.format(DATE_FORMAT).to_string(),
            updated_at: self.updated_at.format(DATE_FORMAT).to_string(),
            favorites_count: self.favorites_count,
            favorited,
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderJson {
    id : i32,
    userid : i32,
    instrumentid : i32,
    side : String,
    ord_status : String,
    ord_type : String,
    exec_inst : String,
    time_in_force : String,
    initial_qty : i32,
    leaves_qty : i32,
    price : f32,
    created_at : DateTime<Utc>,
    updated_at : DateTime<Utc>,
}
