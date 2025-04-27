mod badge;
mod comment;
mod post;
mod tag;
mod user;
mod vote;

use std::{fmt::Debug, fs::File, io::BufReader};
use thiserror::Error;

pub use badge::Badge;
pub use comment::Comment;
pub use post::{Post, PostHistory, PostLink};
pub use tag::Tag;
pub use user::User;
pub use vote::Vote;

/// A trait for types that can be deserialized from an XML element.
///
/// Implementors of this trait can convert XML elements into Rust structures.
/// This is typically used for parsing XML data from external sources.
///
/// # Examples
///
/// ```
/// // Implementation example would go here
/// ```
///
/// # Errors
///
/// Returns an `XmlError` if the XML element cannot be properly parsed or
/// if it contains invalid data for the target type.
pub(crate) trait XmlModel {
    /// Constructs a new instance from an XML element.
    ///
    /// # Parameters
    ///
    /// * `element` - A reference to the XML element to parse
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` - A successfully parsed instance
    /// * `Err(XmlError)` - If parsing fails
    fn from_xml_element(element: &quick_xml::events::BytesStart) -> Result<Self, XmlError>
    where
        Self: Sized;
}
/// Implements the `Iterator` trait for parsing XML content.
///
/// # Type Parameters
///
/// * `T`: A type that can be constructed from XML elements using `from_xml_element`
pub(crate) struct XmlModelIterator<T>
where
    T: XmlModel + Debug,
{
    reader: quick_xml::Reader<BufReader<File>>,
    buffer: Vec<u8>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> XmlModelIterator<T>
where
    T: XmlModel + Debug,
{
    /// Creates a new `XmlModelIterator` from a file path.
    ///
    /// # Parameters
    ///
    /// * `source` - A string slice representing the path to the XML file
    ///
    /// # Returns
    ///
    /// * `Ok(XmlModelIterator<T>)` - A new iterator instance
    /// * `Err(XmlError)` - If the file cannot be opened or read
    pub(crate) fn new(source: &str) -> Result<Self, quick_xml::Error> {
        let mut reader = quick_xml::Reader::from_file(source)?;
        reader.config_mut().trim_text(true);

        Ok(Self {
            reader,
            buffer: Vec::new(),
            _phantom: std::marker::PhantomData,
        })
    }
}

impl<T> Iterator for XmlModelIterator<T>
where
    T: XmlModel + Debug,
{
    type Item = Result<T, XmlError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            self.buffer.clear();

            match self.reader.read_event_into(&mut self.buffer) {
                Err(error) => return Some(Err(XmlError::from(error))),
                Ok(quick_xml::events::Event::Eof) => return None,
                Ok(quick_xml::events::Event::Empty(element)) => {
                    if element.name().as_ref() == b"row" {
                        return Some(T::from_xml_element(&element));
                    }
                }
                _ => continue,
            }
        }
    }
}

#[derive(Debug, Error)]
pub enum XmlError {
    #[error("XML parsing error: {0}")]
    XmlParse(#[from] quick_xml::Error),

    #[error("UTF-8 conversion error: {0}")]
    Utf8Conversion(#[from] std::string::FromUtf8Error),

    #[error("Integer parse error: {0}")]
    IntegerParse(#[from] std::num::ParseIntError),

    #[error("Date parsing error: {0}")]
    DateParse(#[from] chrono::ParseError),
}
