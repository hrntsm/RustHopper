use serde_derive::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoResponseSchema {
    #[serde(rename = "Description")]
    pub description: Option<String>,
    #[serde(rename = "CacheKey")]
    pub cache_key: Option<String>,
    #[serde(rename = "InputNames")]
    pub input_names: Vec<String>,
    #[serde(rename = "OutputNames")]
    pub output_names: Vec<String>,
    #[serde(rename = "Icon")]
    pub icon: Option<String>,
    #[serde(rename = "Inputs")]
    pub inputs: Vec<InputParamSchema>,
    #[serde(rename = "Outputs")]
    pub outputs: Vec<IoParamSchema>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InputParamSchema {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "AtLeast")]
    pub at_least: f64,
    #[serde(rename = "AtMost")]
    pub at_most: f64,
    #[serde(rename = "Default")]
    pub default: String,
    #[serde(rename = "Minimum")]
    pub minimum: Value,
    #[serde(rename = "Maximum")]
    pub maximum: Value,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "ParamType")]
    pub param_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IoParamSchema {
    #[serde(rename = "Name")]
    pub name: Option<String>,
    #[serde(rename = "Nickname")]
    pub nickname: Option<String>,
    #[serde(rename = "ParamType")]
    pub param_type: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    pub absolutetolerance: f64,
    pub angletolerance: f64,
    pub algo: Option<String>,
    pub pointer: Option<String>,
    pub cachesolve: bool,
    pub recursionlevel: i64,
    pub values: Vec<DataTree>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DataTree {
    #[serde(rename = "ParamName")]
    pub param_name: String,
    #[serde(rename = "InnerTree")]
    pub inner_tree: HashMap<String, Vec<RestHopperObject>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RestHopperObject {
    #[serde(rename = "type")]
    pub object_type: String,
    pub data: String,
}
