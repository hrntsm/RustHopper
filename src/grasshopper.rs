use base64::encode;
use std::fs::File;
use std::io::Read;

use crate::{io, URL};

pub async fn evaluate_definition(
    gh_path: &str,
    data_tree: Vec<io::DataTree>,
) -> Result<io::Schema, Box<dyn std::error::Error>> {
    let cache_key = upload_definition(gh_path).await?.cache_key;

    let solve_url = URL.to_owned() + "grasshopper";
    let solve_schema = io::Schema {
        absolutetolerance: 0.001,
        angletolerance: 1.0,
        cachesolve: false,
        algo: None,
        pointer: cache_key,
        recursionlevel: 0,
        values: data_tree,
        warnings: Vec::new(),
        errors: Vec::new(),
    };

    let solve_body = serde_json::to_string(&solve_schema)?;
    let solve_client = reqwest::Client::new();

    let solve_res = solve_client
        .post(solve_url)
        .body(solve_body)
        .send()
        .await?
        .json::<io::Schema>()
        .await?;

    Ok(solve_res)
}

async fn upload_definition(
    gh_path: &str,
) -> Result<io::IoResponseSchema, Box<dyn std::error::Error>> {
    let io_url = URL.to_owned() + "io";

    let mut gh_file = File::open(gh_path).unwrap();
    let mut buf = Vec::new();
    let _ = gh_file.read_to_end(&mut buf);
    let encoded: &str = &encode(&buf);

    let io_schema = io::Schema {
        absolutetolerance: 0.0,
        angletolerance: 0.0,
        algo: Some(encoded.to_owned()),
        pointer: None,
        cachesolve: false,
        recursionlevel: 0,
        values: Vec::new(),
        warnings: Vec::new(),
        errors: Vec::new(),
    };
    let io_body = serde_json::to_string(&io_schema)?;

    let client = reqwest::Client::new();
    let res = client
        .post(io_url)
        .body(io_body)
        .send()
        .await?
        .json::<io::IoResponseSchema>()
        .await?;

    Ok(res)
}
