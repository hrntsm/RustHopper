use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

use base64::encode;
use std::collections::HashMap;
use std::fs::File;

use std::io::Read;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gh_path = "definitions/sum.gh";
    let base_url = String::from("http://localhost:6500");
    let io_url = base_url.clone() + "/io";
    let solve_url = base_url.clone() + "/grasshopper";

    let mut gh_file = File::open(gh_path).unwrap();
    let mut buf = Vec::new();
    let _ = gh_file.read_to_end(&mut buf);

    let encoded: &str = &encode(&buf);

    let mut map = HashMap::new();
    map.insert("algo", encoded);
    map.insert("absolutetolerance", "0.0");
    map.insert("angletolerance", "0.0");
    map.insert("cachesolve", "false");

    let client = reqwest::Client::new();
    let res = client
        .post(io_url)
        .json(&map)
        .send()
        .await?
        // .text()
        .json::<IOResponse>()
        .await?;

    println!("response /io \n{:#?}", res.cache_key);

    let mut tree_a = HashMap::new();
    tree_a.insert(
        String::from("0"),
        vec![InnerData {
            type_field: String::from("System.Double"),
            data: String::from("1.0"),
        }],
    );

    let mut tree_b = HashMap::new();
    tree_b.insert(
        String::from("0"),
        vec![InnerData {
            type_field: String::from("System.Double"),
            data: String::from("2.0"),
        }],
    );

    let gh_values: Vec<GHValue> = vec![
        GHValue {
            param_name: String::from("A"),
            inner_tree: tree_a,
        },
        GHValue {
            param_name: String::from("B"),
            inner_tree: tree_b,
        },
    ];

    let post_solve = Root {
        absolutetolerance: 0.001,
        angletolerance: 1.0,
        cachesolve: false,
        algo: None,
        pointer: res.cache_key,
        recursionlevel: 0,
        values: gh_values,
        warnings: Vec::new(),
        errors: Vec::new(),
    };

    let solve_body = serde_json::to_string(&post_solve)?;

    println!("solve body: \n{}", solve_body);

    let solve_client = reqwest::Client::new();
    let solve_res = solve_client
        .post(solve_url)
        .body(solve_body)
        .send()
        .await?
        // .text()
        .json::<Root>()
        .await?;

    let sum_result: &str = &solve_res.values[0].inner_tree["{0}"][0].data;
    let sum_float: f64 = sum_result.parse().unwrap();

    println!(
        "response /grasshopper \n{:#?}",
        sum_float
    );

    Ok(())
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IOResponse {
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "CacheKey")]
    pub cache_key: String,
    #[serde(rename = "InputNames")]
    pub input_names: Vec<String>,
    #[serde(rename = "OutputNames")]
    pub output_names: Vec<String>,
    #[serde(rename = "Icon")]
    pub icon: Value,
    #[serde(rename = "Inputs")]
    pub inputs: Vec<Input>,
    #[serde(rename = "Outputs")]
    pub outputs: Vec<Output>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Input {
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
    pub nickname: Value,
    #[serde(rename = "ParamType")]
    pub param_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Output {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Nickname")]
    pub nickname: Value,
    #[serde(rename = "ParamType")]
    pub param_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub absolutetolerance: f64,
    pub angletolerance: f64,
    pub algo: Option<String>,
    pub pointer: String,
    pub cachesolve: bool,
    pub recursionlevel: i64,
    pub values: Vec<GHValue>,
    pub warnings: Vec<String>,
    pub errors: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GHValue {
    #[serde(rename = "ParamName")]
    pub param_name: String,
    #[serde(rename = "InnerTree")]
    pub inner_tree: HashMap<String, Vec<InnerData>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InnerData {
    #[serde(rename = "type")]
    pub type_field: String,
    pub data: String,
}
