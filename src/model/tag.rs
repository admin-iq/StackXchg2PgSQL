use diesel::prelude::*;

use super::XmlModel;

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::tags)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a tag in the system.
///
/// # Fields
///
/// * `id` - The unique identifier for the tag.
/// * `tag_name` - The name of the tag (e.g., "rust", "postgresql").
/// * `count` - The number of posts that use this tag.
/// * `excerpt_post_id` - ID of the post containing the tag's usage guidance/excerpt.
/// * `wiki_post_id` - ID of the post containing the tag's detailed wiki information.
/// * `is_moderator_only` - Flag indicating if the tag can only be used by moderators (1 for true, 0 for false).
/// * `is_required` - Flag indicating if the tag is required for certain posts (1 for true, 0 for false).
pub struct Tag {
    pub id: i32,
    pub tag_name: Option<String>,
    pub count: i32,
    pub excerpt_post_id: Option<i32>,
    pub wiki_post_id: Option<i32>,
    pub is_moderator_only: Option<i16>,
    pub is_required: Option<i16>,
}

impl XmlModel for Tag {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut tag = Tag::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => tag.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"TagName" => tag.tag_name = Some(String::from_utf8(attr.value.to_vec())?),
                b"Count" => tag.count = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"ExcerptPostId" => {
                    tag.excerpt_post_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"WikiPostId" => {
                    tag.wiki_post_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"IsModeratorOnly" => {
                    tag.is_moderator_only = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"IsRequired" => {
                    tag.is_required = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                _ => {}
            }
        }
        Ok(tag)
    }
}
