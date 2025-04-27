use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::XmlModel;

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::comments)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a comment in the system.
///
/// # Fields
///
/// * `id` - Unique identifier for the comment.
/// * `post_id` - Optional reference to the post this comment belongs to.
/// * `score` - Optional score/rating of the comment.
/// * `text` - The actual content/body of the comment.
/// * `creation_date` - Timestamp when the comment was created.
/// * `user_display_name` - Optional display name of the user who created the comment.
/// * `user_id` - Optional identifier of the user who created the comment.
/// * `content_license` - Optional license information for the comment content.
pub struct Comment {
    pub id: i32,
    pub post_id: Option<i32>,
    pub score: Option<i32>,
    pub text: String,
    pub creation_date: NaiveDateTime,
    pub user_display_name: Option<String>,
    pub user_id: Option<i32>,
    pub content_license: Option<String>,
}

impl XmlModel for Comment {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut comment = Comment::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => comment.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"PostId" => {
                    comment.post_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"Score" => comment.score = Some(String::from_utf8(attr.value.to_vec())?.parse()?),
                b"Text" => comment.text = String::from_utf8(attr.value.to_vec())?,
                b"CreationDate" => {
                    comment.creation_date = NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?
                }
                b"UserDisplayName" => {
                    comment.user_display_name = Some(String::from_utf8(attr.value.to_vec())?)
                }
                b"UserId" => {
                    comment.user_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"ContentLicense" => {
                    comment.content_license = Some(String::from_utf8(attr.value.to_vec())?)
                }
                _ => {}
            }
        }
        Ok(comment)
    }
}
