/*
 * Slurm Rest API
 *
 * API to access and control Slurm.
 *
 * The version of the OpenAPI document: 0.0.37
 * Contact: sales@schedmd.com
 * Generated by: https://openapi-generator.tech
 */

/// Dbv0037QosLimitsMaxAccruing : Limits on accruing priority



#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct Dbv0037QosLimitsMaxAccruing {
    #[serde(rename = "per", skip_serializing_if = "Option::is_none")]
    pub per: Option<Box<crate::models::Dbv0037QosLimitsMaxAccruingPer>>,
}

impl Dbv0037QosLimitsMaxAccruing {
    /// Limits on accruing priority
    pub fn new() -> Dbv0037QosLimitsMaxAccruing {
        Dbv0037QosLimitsMaxAccruing {
            per: None,
        }
    }
}


