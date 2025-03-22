//! This crate provides structs to serialize and deserialize bwHC DTOs.
//! The base struct is `MtbFile`.

#![allow(clippy::needless_doctest_main)]

pub use crate::mtbfile::*;
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
    /// use std::str::FromStr;
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

impl MtbFile {
    /// Creates "dummy" MtbFile with consent status `REJECTED`.
    /// The created MtbFile does not contain all information, just enough to contain the
    /// information, that the patient with given ID has rejected the consent.
    pub fn new_with_consent_rejected(patient_id: &str) -> MtbFile {
        MtbFile {
            care_plans: None,
            claim_responses: None,
            claims: None,
            consent: Consent {
                id: "".to_string(),
                patient: patient_id.to_string(),
                status: ConsentStatus::Rejected,
            },
            diagnoses: None,
            ecog_status: None,
            episode: Episode {
                id: "".to_string(),
                patient: patient_id.to_string(),
                period: EpisodePeriod {
                    end: None,
                    start: "".to_string(),
                },
            },
            family_member_diagnoses: None,
            genetic_counselling_requests: None,
            histology_reevaluation_requests: None,
            histology_reports: None,
            last_guideline_therapies: None,
            molecular_pathology_findings: None,
            molecular_therapies: None,
            ngs_reports: None,
            patient: Patient {
                birth_date: None,
                date_of_death: None,
                gender: Gender::Unknown,
                id: patient_id.to_string(),
                insurance: None,
                managing_zpm: None,
            },
            previous_guideline_therapies: None,
            rebiopsy_requests: None,
            recommendations: None,
            responses: None,
            specimens: None,
            study_inclusion_requests: None,
        }
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
