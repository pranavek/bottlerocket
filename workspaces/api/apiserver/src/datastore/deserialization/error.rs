use serde::de;
use snafu::{IntoError, NoneError as NoSource, Snafu};

use crate::datastore::ScalarError;

/// Potential errors from deserialization.
#[derive(Debug, Snafu)]
#[snafu(visibility = "pub")]
pub enum Error {
    // This error variant is required to implement ser::Error for serde.
    #[snafu(display("Error during deserialization: {}", msg))]
    Message { msg: String },

    #[snafu(display("Error deserializing scalar value: {}", source))]
    DeserializeScalar { source: ScalarError },

    #[snafu(display(
        "Data store deserializer must be used on a struct, or you must give a prefix"
    ))]
    BadRoot {},
}

pub type Result<T> = std::result::Result<T, Error>;

impl de::Error for Error {
    fn custom<T: std::fmt::Display>(msg: T) -> Self {
        Message {
            msg: msg.to_string(),
        }
        .into_error(NoSource)
    }
}
