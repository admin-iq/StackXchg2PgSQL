use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::XmlModel;

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a post in the system.
///
/// # Fields
///
/// * `id` - Unique identifier for the post.
/// * `post_type_id` - Type of post (1: Question, 2: Answer, etc.).
/// * `accepted_answer_id` - For questions, the ID of the accepted answer, if any.
/// * `creation_date` - When the post was created.
/// * `deletion_date` - When the post was deleted, if applicable.
/// * `score` - Net score of the post (upvotes minus downvotes).
/// * `view_count` - Number of times the post has been viewed.
/// * `body` - HTML body of the post.
/// * `owner_user_id` - ID of the user who created the post.
/// * `owner_display_name` - Display name of the post owner (for anonymous or deleted users).
/// * `last_editor_user_id` - ID of the user who last edited the post.
/// * `last_editor_display_name` - Display name of the last editor.
/// * `last_edit_date` - When the post was last edited.
/// * `last_activity_date` - When the post last had activity.
/// * `title` - Title of the post (for questions).
/// * `tags` - Tags associated with the post, stored as a string.
/// * `answer_count` - Number of answers (for questions).
/// * `comment_count` - Number of comments on the post.
/// * `favorite_count` - Number of users who favorited the post.
/// * `close_date` - When the post was closed, if applicable.
/// * `community_owned_date` - When the post became community owned, if applicable.
/// * `content_license` - License under which the content was posted.
pub struct Post {
    id: i32,
    post_type_id: Option<i16>,
    accepted_answer_id: Option<i32>,
    creation_date: Option<NaiveDateTime>,
    deletion_date: Option<NaiveDateTime>,
    score: Option<i32>,
    view_count: Option<i32>,
    body: Option<String>,
    owner_user_id: Option<i32>,
    owner_display_name: Option<String>,
    last_editor_user_id: Option<i32>,
    last_editor_display_name: Option<String>,
    last_edit_date: Option<NaiveDateTime>,
    last_activity_date: Option<NaiveDateTime>,
    title: Option<String>,
    tags: Option<String>,
    answer_count: Option<i32>,
    comment_count: Option<i32>,
    favorite_count: Option<i32>,
    close_date: Option<NaiveDateTime>,
    community_owned_date: Option<NaiveDateTime>,
    content_license: Option<String>,
}

impl XmlModel for Post {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut post = Post::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => post.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"PostTypeId" => {
                    post.post_type_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"AcceptedAnswerId" => {
                    post.accepted_answer_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"CreationDate" => {
                    post.creation_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"DeletionDate" => {
                    post.deletion_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"Score" => post.score = Some(String::from_utf8(attr.value.to_vec())?.parse()?),
                b"ViewCount" => {
                    post.view_count = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"Body" => post.body = Some(String::from_utf8(attr.value.to_vec())?),
                b"OwnerUserId" => {
                    post.owner_user_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"OwnerDisplayName" => {
                    post.owner_display_name = Some(String::from_utf8(attr.value.to_vec())?)
                }
                b"LastEditorUserId" => {
                    post.last_editor_user_id =
                        Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"LastEditorDisplayName" => {
                    post.last_editor_display_name = Some(String::from_utf8(attr.value.to_vec())?)
                }
                b"LastEditDate" => {
                    post.last_edit_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"LastActivityDate" => {
                    post.last_activity_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"Title" => post.title = Some(String::from_utf8(attr.value.to_vec())?),
                b"Tags" => post.tags = Some(String::from_utf8(attr.value.to_vec())?),
                b"AnswerCount" => {
                    post.answer_count = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"CommentCount" => {
                    post.comment_count = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"FavoriteCount" => {
                    post.favorite_count = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"CloseDate" => {
                    post.close_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"CommunityOwnedDate" => {
                    post.community_owned_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"ContentLicense" => {
                    post.content_license = Some(String::from_utf8(attr.value.to_vec())?)
                }
                _ => {}
            }
        }
        Ok(post)
    }
}

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::post_historys)]
/// Represents a history entry for a post in the system.
///
/// # Fields
///
/// * `id` - Unique identifier for this history entry.
/// * `post_history_type_id` - Identifies the type of history event (e.g., edit, title change).
/// * `post_id` - Reference to the post this history entry relates to.
/// * `revision_guid` - Globally unique identifier for this revision.
/// * `creation_date` - When this history entry was created.
/// * `user_id` - Identifier of the user who made this change.
/// * `user_display_name` - Display name of the user at the time of the change.
/// * `comment` - Optional comment explaining the change.
/// * `text` - The content that was changed or added.
/// * `content_license` - License under which the content is shared.
pub struct PostHistory {
    id: i32,
    post_history_type_id: Option<i16>,
    post_id: Option<i32>,
    revision_guid: Option<String>,
    creation_date: Option<NaiveDateTime>,
    user_id: Option<i32>,
    user_display_name: Option<String>,
    comment: Option<String>,
    text: Option<String>,
    content_license: Option<String>,
}

impl XmlModel for PostHistory {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut post_history = PostHistory::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => post_history.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"PostHistoryTypeId" => {
                    post_history.post_history_type_id =
                        Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"PostId" => {
                    post_history.post_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"RevisionGUID" => {
                    post_history.revision_guid = Some(String::from_utf8(attr.value.to_vec())?)
                }
                b"CreationDate" => {
                    post_history.creation_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"UserId" => {
                    post_history.user_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                b"UserDisplayName" => {
                    post_history.user_display_name = Some(String::from_utf8(attr.value.to_vec())?)
                }
                b"Comment" => post_history.comment = Some(String::from_utf8(attr.value.to_vec())?),
                b"Text" => post_history.text = Some(String::from_utf8(attr.value.to_vec())?),
                b"ContentLicense" => {
                    post_history.content_license = Some(String::from_utf8(attr.value.to_vec())?)
                }
                _ => {}
            }
        }
        Ok(post_history)
    }
}

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::post_links)]
/// Represents a link or relationship between two posts.
///
/// # Fields
///
/// * `id` - Unique identifier for the post link.
/// * `creation_date` - The date and time when the link was created.
/// * `post_id` - The identifier of the source post.
/// * `related_post_id` - The identifier of the target/related post.
/// * `link_type_id` - The type of relationship between the posts.
pub struct PostLink {
    id: i32,
    creation_date: NaiveDateTime,
    post_id: i32,
    related_post_id: i32,
    link_type_id: i16,
}

impl XmlModel for PostLink {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut post_link = PostLink::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => post_link.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"CreationDate" => {
                    post_link.creation_date = NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?
                }
                b"PostId" => post_link.post_id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"RelatedPostId" => {
                    post_link.related_post_id = String::from_utf8(attr.value.to_vec())?.parse()?
                }
                b"LinkTypeId" => {
                    post_link.link_type_id = String::from_utf8(attr.value.to_vec())?.parse()?
                }
                _ => {}
            }
        }
        Ok(post_link)
    }
}
