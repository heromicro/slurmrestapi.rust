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
pub struct V0039JobInfoPower {
    #[serde(rename = "flags", skip_serializing_if = "Option::is_none")]
    pub flags: Option<Vec<Flags>>,
}

impl V0039JobInfoPower {
    pub fn new() -> V0039JobInfoPower {
        V0039JobInfoPower {
            flags: None,
        }
    }
}

/// 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Flags {
    #[serde(rename = "EQUAL_POWER")]
    EqualPower,
}

impl Default for Flags {
    fn default() -> Flags {
        Self::EqualPower
    }
}

