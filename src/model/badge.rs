use chrono::NaiveDateTime;
use diesel::prelude::*;

use super::XmlModel;

#[derive(Clone, Debug, Default, Insertable)]
#[diesel(table_name = crate::schema::badges)]
#[diesel(check_for_backend(diesel::pg::Pg))]
/// Represents a badge awarded to a user in the system.
///
/// # Fields
///
/// * `id` - Unique identifier for the badge
/// * `user_id` - Identifier of the user who earned the badge
/// * `name` - The name of the badge
/// * `date` - The date and time when the badge was awarded
/// * `class` - The class/tier of the badge (e.g., 1=gold, 2=silver, 3=bronze)
/// * `tag_based` - Indicates whether this badge is associated with a specific tag
pub struct Badge {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub date: NaiveDateTime,
    pub class: i16,
    pub tag_based: bool,
}

impl XmlModel for Badge {
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, super::XmlError> {
        let mut badge = Badge::default();

        for attr in element.attributes() {
            let attr = match attr {
                Ok(attr) => attr,
                Err(error) => return Err(super::XmlError::XmlParse(error.into())),
            };

            match attr.key.as_ref() {
                b"Id" => badge.id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"UserId" => badge.user_id = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"Name" => badge.name = String::from_utf8(attr.value.to_vec())?,
                b"Date" => {
                    let s = String::from_utf8(attr.value.to_vec())?;
                    badge.date = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M:%S%.f")?;
                }
                b"Class" => badge.class = String::from_utf8(attr.value.to_vec())?.parse()?,
                b"TagBased" => badge.tag_based = String::from_utf8(attr.value.to_vec())? == "1",
                _ => {}
            }
        }
        Ok(badge)
    }
}
