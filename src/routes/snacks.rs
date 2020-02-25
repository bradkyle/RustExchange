use crate::auth::Auth;
use crate::db;
use crate::db::snacks::{Feedsnacks, Findsnacks};
use crate::errors::{Errors, FieldValidator};
use rocket::request::Form;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize)]
pub struct Newsnack {
    snack: NewsnackData,
}

#[derive(Deserialize, Validate)]
pub struct NewsnackData {
    #[validate(length(min = 1))]
    title: Option<String>,
    #[validate(length(min = 1))]
    description: Option<String>,
    #[validate(length(min = 1))]
    body: Option<String>,
    #[serde(rename = "tagList")]
    tag_list: Vec<String>,
}

#[post("/snacks", format = "json", data = "<new_snack>")]
pub fn post_snacks(
    auth: Auth,
    new_snack: Json<Newsnack>,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_snack = new_snack.into_inner().snack;

    let mut extractor = FieldValidator::validate(&new_snack);
    let title = extractor.extract("title", new_snack.title);
    let description = extractor.extract("description", new_snack.description);
    let body = extractor.extract("body", new_snack.body);
    extractor.check()?;

    let snack = db::snacks::create(
        &conn,
        auth.id,
        &title,
        &description,
        &body,
        &new_snack.tag_list,
    );
    Ok(json!({ "snack": snack }))
}

/// return multiple snacks, ordered by most recent first
#[get("/snacks?<params..>")]
pub fn get_snacks(params: Form<Findsnacks>, auth: Option<Auth>, conn: db::Conn) -> JsonValue {
    let user_id = auth.map(|x| x.id);
    let snacks = db::snacks::find(&conn, &params, user_id);
    json!({ "snacks": snacks.0, "snacksCount": snacks.1 })
}

#[get("/snacks/<slug>")]
pub fn get_snack(slug: String, auth: Option<Auth>, conn: db::Conn) -> Option<JsonValue> {
    let user_id = auth.map(|x| x.id);
    db::snacks::find_one(&conn, &slug, user_id).map(|snack| json!({ "snack": snack }))
}

#[delete("/snacks/<slug>")]
pub fn delete_snack(slug: String, auth: Auth, conn: db::Conn) {
    db::snacks::delete(&conn, &slug, auth.id);
}

#[post("/snacks/<slug>/favorite")]
pub fn favorite_snack(slug: String, auth: Auth, conn: db::Conn) -> Option<JsonValue> {
    db::snacks::favorite(&conn, &slug, auth.id).map(|snack| json!({ "snack": snack }))
}

#[delete("/snacks/<slug>/favorite")]
pub fn unfavorite_snack(slug: String, auth: Auth, conn: db::Conn) -> Option<JsonValue> {
    db::snacks::unfavorite(&conn, &slug, auth.id).map(|snack| json!({ "snack": snack }))
}

#[derive(Deserialize)]
pub struct Updatesnack {
    snack: db::snacks::UpdatesnackData,
}

#[put("/snacks/<slug>", format = "json", data = "<snack>")]
pub fn put_snacks(
    slug: String,
    snack: Json<Updatesnack>,
    auth: Auth,
    conn: db::Conn,
) -> Option<JsonValue> {
    // TODO: check auth
    db::snacks::update(&conn, &slug, auth.id, snack.into_inner().snack)
        .map(|snack| json!({ "snack": snack }))
}

#[derive(Deserialize)]
pub struct NewComment {
    comment: NewCommentData,
}

#[derive(Deserialize, Validate)]
pub struct NewCommentData {
    #[validate(length(min = 1))]
    body: Option<String>,
}

#[post("/snacks/<slug>/comments", format = "json", data = "<new_comment>")]
pub fn post_comment(
    slug: String,
    new_comment: Json<NewComment>,
    auth: Auth,
    conn: db::Conn,
) -> Result<JsonValue, Errors> {
    let new_comment = new_comment.into_inner().comment;

    let mut extractor = FieldValidator::validate(&new_comment);
    let body = extractor.extract("body", new_comment.body);
    extractor.check()?;

    let comment = db::comments::create(&conn, auth.id, &slug, &body);
    Ok(json!({ "comment": comment }))
}

#[delete("/snacks/<slug>/comments/<id>")]
pub fn delete_comment(slug: String, id: i32, auth: Auth, conn: db::Conn) {
    db::comments::delete(&conn, auth.id, &slug, id);
}

#[get("/snacks/<slug>/comments")]
pub fn get_comments(slug: String, conn: db::Conn) -> JsonValue {
    let comments = db::comments::find_by_slug(&conn, &slug);
    json!({ "comments": comments })
}

#[get("/snacks/feed?<params..>")]
pub fn get_snacks_feed(params: Form<Feedsnacks>, auth: Auth, conn: db::Conn) -> JsonValue {
    let snacks = db::snacks::feed(&conn, &params, auth.id);
    let snacks_count = snacks.len();
    json!({ "snacks": snacks, "snacksCount": snacks_count })
}
