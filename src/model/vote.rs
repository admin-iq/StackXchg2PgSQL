use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::XmlModel;

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::votes)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a vote on a post in the system.
///
/// # Fields
///
/// * `id` - Unique identifier for the vote
/// * `post_id` - Identifier of the post this vote belongs to
/// * `vote_type_id` - Type of vote (e.g., upvote=2, downvote=3, etc.)
/// * `user_id` - Optional identifier of the user who cast the vote
/// * `creation_date` - When the vote was created
/// * `bounty_amount` - For bounty-related votes, the reputation amount offered
pub struct Vote {
    pub id: i32,
    pub post_id: i32,
    pub vote_type_id: i16,
    pub user_id: Option<i32>,
    pub creation_date: Option<NaiveDateTime>,
    pub bounty_amount: Option<i32>,
}

impl XmlModel for Vote {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut vote = Vote::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => vote.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"PostId" => vote.post_id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"VoteTypeId" => {
                    vote.vote_type_id = String::from_utf8(attr.value.to_vec())?.parse()?
                }
                b"UserId" => vote.user_id = Some(String::from_utf8(attr.value.to_vec())?.parse()?),
                b"CreationDate" => {
                    vote.creation_date = Some(NaiveDateTime::parse_from_str(
                        &String::from_utf8(attr.value.to_vec())?,
                        "%Y-%m-%dT%H:%M:%S%.f",
                    )?)
                }
                b"BountyAmount" => {
                    vote.bounty_amount = Some(String::from_utf8(attr.value.to_vec())?.parse()?)
                }
                _ => {}
            }
        }
        Ok(vote)
    }
}
