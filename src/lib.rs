//! This crate provides structs to serialize and deserialize bwHC DTOs.
//! The base struct is `MtbFile`.

#![allow(
    clippy::needless_doctest_main,
)]

pub use crate::mtbfile::MtbFile;
use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::str::FromStr;

mod mtbfile;

#[derive(Debug)]
pub struct SerdeError(String);

impl Display for SerdeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "MtbFile Serde Error: {}", self.0)
    }
}

impl Error for SerdeError {}

impl FromStr for MtbFile {
    type Err = SerdeError;

    /// Deserialize an instance of `MtbFile` from a string of JSON text.
    ///
    /// # Example
    ///
    /// ```
    /// use bwhc_dto::MtbFile;
    ///
    /// fn main() {
    ///     const MTBFILE_JSON: &str = include_str!("../tests/fake_MTBFile.json");
    ///
    ///     let mtb_file = MtbFile::from_str(MTBFILE_JSON).unwrap();
    ///     println!("{:#?}", mtb_file);
    /// }
    /// ```
    ///
    /// # Errors
    ///
    /// If the conversion fails, an `SerdeError` will be returned.
    fn from_str(value: &str) -> Result<MtbFile, SerdeError> {
        serde_json::from_str(value).map_err(|err| SerdeError(err.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const MTBFILE_JSON: &str = include_str!("../tests/fake_MTBFile.json");

    #[test]
    fn should_deserialize_json_string() {
        let mtbfile = MtbFile::from_str(MTBFILE_JSON);
        assert!(mtbfile.is_ok())
    }
}
