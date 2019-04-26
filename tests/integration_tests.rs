use google_api3::{FieldSelector, Leaf};
use serde::Deserialize;

#[derive(Deserialize, FieldSelector)]
#[serde(rename_all = "camelCase")]
struct File {
    id: String,
    mime_type: String,
    sharing_user: Option<UserInfo>,
}

#[derive(Deserialize, FieldSelector)]
#[serde(rename_all = "camelCase")]
struct UserInfo {
    me: bool,
    email_address: String,
}

#[test]
fn basic() {
    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        next_page_token: String,
        files: Vec<File>,
    }

    assert_eq!(
        Response::field_selector(),
        "nextPageToken,files(id,mimeType,sharingUser/me,sharingUser/emailAddress)"
    );
}

#[test]
fn generic_with_flatten() {
    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct Response<T>
    where
        T: FieldSelector,
    {
        next_page_token: String,
        #[serde(flatten)]
        payload: T,
    }

    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct ListFiles {
        files: Vec<File>,
    }
    assert_eq!(
        Response::<ListFiles>::field_selector(),
        "nextPageToken,files(id,mimeType,sharingUser/me,sharingUser/emailAddress)"
    );
}

#[test]
fn external_types() {
    use chrono::{DateTime, Utc};

    #[derive(Deserialize, FieldSelector)]
    struct File {
        id: String,
        viewed_by_me_time: Leaf<DateTime<Utc>>,
    }

    #[derive(Deserialize, FieldSelector)]
    #[serde(rename_all = "camelCase")]
    struct Response {
        next_page_token: String,
        files: Vec<File>,
    }

    assert_eq!(
        Response::field_selector(),
        "nextPageToken,files(id,viewed_by_me_time)"
    );
}
