/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.39
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct V0Period0Period39ReservationsResponse {
    #[serde(rename = "meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<Box<crate::models::V0Period0Period39Meta>>,
    /// Slurm errors
    #[serde(rename = "errors", skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<crate::models::V0Period0Period39Error>>,
    /// Slurm warnings
    #[serde(rename = "warnings", skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<crate::models::V0Period0Period39Warning>>,
    #[serde(rename = "reservations", skip_serializing_if = "Option::is_none")]
    pub reservations: Option<Vec<crate::models::V0Period0Period39ReservationInfo>>,
}

impl V0Period0Period39ReservationsResponse {
    pub fn new() -> V0Period0Period39ReservationsResponse {
        V0Period0Period39ReservationsResponse {
            meta: None,
            errors: None,
            warnings: None,
            reservations: None,
        }
    }
}


