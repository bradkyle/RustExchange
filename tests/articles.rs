//! Test snacks

mod common;

use common::*;
use rocket::http::{ContentType, Status};
use rocket::local::{Client, LocalResponse};

const snack_TITLE: &str = "Test snack";
const snack_BODY: &str = "This is obviously a test snack!";

#[test]
/// Test snack creation.
fn test_post_snacks() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token);

    let value = response_json_value(response);
    let title = value
        .get("snack")
        .expect("must have an 'snack' field")
        .get("title")
        .expect("must have a 'title' field")
        .as_str();

    assert_eq!(title, Some(snack_TITLE));
}

#[test]
/// Test snack retrieval.
fn test_get_snack() {
    let client = test_client();
    let response = &mut create_snack(&client, login(&client));

    let slug = snack_slug(response);
    // Slug can contain random prefix, thus `start_with` instead of `assert_eq`!
    assert!(slug.starts_with(&snack_TITLE.to_lowercase().replace(' ', "-")));

    let response = &mut client.get(format!("/api/snacks/{}", slug)).dispatch();

    let value = response_json_value(response);
    let body = value
        .get("snack")
        .and_then(|snack| snack.get("body"))
        .and_then(|body| body.as_str());

    assert_eq!(body, Some(snack_BODY));
}

#[test]
/// Test snack update.
fn test_put_snacks() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let new_desc = "Well, it's an updated test snack";
    let response = &mut client
        .put(format!("/api/snacks/{}", slug))
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
            "snack": {
                "description": new_desc,
                "tagList": ["test", "foo"]
            }
        }))
        .dispatch();

    let value = response_json_value(response);
    let description = value
        .get("snack")
        .and_then(|snack| snack.get("description"))
        .and_then(|description| description.as_str());

    assert_eq!(description, Some(new_desc));
}

#[test]
/// Test snack deletion.
fn test_delete_snack() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let response = &mut client
        .delete(format!("/api/snacks/{}", slug))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test that it's not possible to delete snack anonymously.
fn test_delete_snack_anonymously() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let response = &mut client.delete(format!("/api/snacks/{}", slug)).dispatch();

    assert_eq!(response.status(), Status::Forbidden);
}

#[test]
/// Test putting snack to favorites.
fn test_favorite_snack() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let response = &mut client
        .post(format!("/api/snacks/{}/favorite", slug))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test removing snack from favorites .
fn test_unfavorite_snack() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let response = &mut client
        .delete(format!("/api/snacks/{}/favorite", slug))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test getting multiple snacks.
fn test_get_snacks() {
    let client = test_client();
    let token = login(&client);
    create_snack(&client, token);

    let response = &mut client.get("/api/snacks").dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let num = value
        .get("snacksCount")
        .and_then(|count| count.as_i64())
        .expect("must have 'snacksCount' field");

    assert!(num > 0);
}

#[test]
/// Test getting multiple snacks with params.
fn test_get_snacks_with_params() {
    let client = test_client();
    let token = login(&client);
    create_snack(&client, token);

    let url = "/api/snacks?tag=foo&author=smoketest&favorited=smoketest&limit=1&offset=0";
    let response = &mut client.get(url).dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    value
        .get("snacksCount")
        .and_then(|count| count.as_i64())
        .expect("must have 'snacksCount' field");
}


#[test]
/// Test getting snacks feed.
fn test_get_snacks_fedd() {
    let client = test_client();
    let token = login(&client);

    let url = "/api/snacks/feed?limit=1&offset=0";
    let response = &mut client
        .get(url)
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    value.get("snacks").expect("must have 'snacks' field");
}

#[test]
/// Test posting and deleteing of comments.
fn test_commenting() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let response = &mut client
        .post(format!("/api/snacks/{}/comments", slug))
        .header(ContentType::JSON)
        .header(token_header(token.clone()))
        .body(json_string!({
            "comment": {
                "body": "Like!",
            }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let comment_id = value
        .get("comment")
        .and_then(|comment| comment.get("id"))
        .and_then(|id| id.as_i64())
        .expect("must have comment 'id' field");

    let response = client
        .delete(format!("/api/snacks/{}/comments/{}", slug, comment_id))
        .header(token_header(token))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);
}

#[test]
/// Test getting comments.
fn test_get_comment() {
    let client = test_client();
    let token = login(&client);
    let response = &mut create_snack(&client, token.clone());

    let slug = snack_slug(response);

    let response = &mut client
        .get(format!("/api/snacks/{}/comments", slug))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    let value = response_json_value(response);
    let comments_num = value
        .get("comments")
        .and_then(|comments| comments.as_array())
        .map(|comments| comments.len())
        .expect("must have 'comments' field");
    // Newly created snack must have no comments
    assert_eq!(comments_num, 0);
}

// Utility functions

fn snack_slug(response: &mut LocalResponse) -> String {
    response_json_value(response)
        .get("snack")
        .and_then(|snack| snack.get("slug"))
        .and_then(|slug| slug.as_str())
        .map(String::from)
        .expect("Cannot extract snack slug")
}

fn create_snack(client: &Client, token: Token) -> LocalResponse {
    let response = client
        .post("/api/snacks")
        .header(ContentType::JSON)
        .header(token_header(token))
        .body(json_string!({
                "snack": {
                    "title": snack_TITLE,
                    "description": "Well, it's a test snack",
                    "body": snack_BODY,
                    "tagList": ["test", "foo", "bar"]
                }
        }))
        .dispatch();

    assert_eq!(response.status(), Status::Ok);

    response
}
