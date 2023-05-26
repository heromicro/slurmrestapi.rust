/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0037JobReservation : Reservation usage details



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0037JobReservation {
    /// Database id of reservation
    #[serde(rename = "id", skip_serializing_if = "Option::is_none")]
    pub id: Option<i32>,
    /// Name of reservation
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<i32>,
}

impl Dbv0037JobReservation {
    /// Reservation usage details
    pub fn new() -> Dbv0037JobReservation {
        Dbv0037JobReservation {
            id: None,
            name: None,
        }
    }
}


