use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::XmlModel;

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a user entity in the system.
///
/// # Fields
///
/// * `id` - Unique identifier for the user
/// * `reputation` - User's reputation score
/// * `creation_date` - Date and time when the user account was created
/// * `display_name` - User's displayed name (optional)
/// * `last_access_date` - Date and time of the user's most recent access
/// * `website_url` - URL to the user's website (optional)
/// * `location` - User's geographical location (optional)
/// * `about_me` - User's self-description or bio (optional)
/// * `views` - Number of profile views (optional)
/// * `up_votes` - Number of upvotes cast by the user (optional)
/// * `down_votes` - Number of downvotes cast by the user (optional)
/// * `profile_image_url` - URL to the user's profile image (optional)
/// * `email_hash` - Hash of the user's email, typically for Gravatar (optional)
/// * `account_id` - Associated account identifier (optional)
pub struct User {
    id: i32,
    reputation: i32,
    creation_date: NaiveDateTime,
    display_name: Option<String>,
    last_access_date: NaiveDateTime,
    website_url: Option<String>,
    location: Option<String>,
    about_me: Option<String>,
    views: Option<i32>,
    up_votes: Option<i32>,
    down_votes: Option<i32>,
    profile_image_url: Option<String>,
    email_hash: Option<String>,
    account_id: Option<i32>,
}

impl XmlModel for User {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut user = User::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => user.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"Reputation" => {
                    user.reputation = String::from_utf8(attr.value.to_vec())?.parse()?
                }
                b"CreationDate" => {
                    let date_str = String::from_utf8(attr.value.to_vec())?;
                    user.creation_date =
                        NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f")?;
                }
                b"DisplayName" => user.display_name = Some(String::from_utf8(attr.value.to_vec())?),
                b"LastAccessDate" => {
                    let date_str = String::from_utf8(attr.value.to_vec())?;
                    user.last_access_date =
                        NaiveDateTime::parse_from_str(&date_str, "%Y-%m-%dT%H:%M:%S%.f")?;
                }
                b"WebsiteUrl" => user.website_url = Some(String::from_utf8(attr.value.to_vec())?),
                b"Location" => user.location = Some(String::from_utf8(attr.value.to_vec())?),
                b"AboutMe" => user.about_me = Some(String::from_utf8(attr.value.to_vec())?),
                b"Views" => user.views = Some(String::from_utf8(attr.value.to_vec())?.parse()?),
                b"UpVotes" => {
                    user.up_votes = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"DownVotes" => {
                    user.down_votes = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"ProfileImageUrl" => {
                    user.profile_image_url = Some(String::from_utf8(attr.value.to_vec())?)
                }
                b"EmailHash" => user.email_hash = Some(String::from_utf8(attr.value.to_vec())?),
                b"AccountId" => {
                    user.account_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                _ => {}
            }
        }
        Ok(user)
    }
}
