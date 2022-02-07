mod grasshopper;
mod io;

use std::collections::HashMap;

const URL: &str = "http://localhost:6500/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let gh_path = "definitions/sum.gh";

    let mut input_tree: Vec<io::DataTree> = Vec::new();

    let mut tree = HashMap::new();
    tree.insert(
        String::from("{0}"),
        vec![io::RestHopperObject {
            object_type: String::from("System.Double"),
            data: String::from("1.0"),
        }],
    );
    input_tree.push(io::DataTree {
        param_name: String::from("A"),
        inner_tree: tree,
    });

    let mut tree = HashMap::new();
    tree.insert(
        String::from("{0}"),
        vec![io::RestHopperObject {
            object_type: String::from("System.Double"),
            data: String::from("2.0"),
        }],
    );
    input_tree.push(io::DataTree {
        param_name: String::from("B"),
        inner_tree: tree,
    });

    let output = grasshopper::evaluate_definition(gh_path, input_tree).await?;

    let errors = output.errors;
    if !errors.is_empty() {
        println!("Errors:");
        for error in errors {
            println!("{}", error);
        }
    }

    let warnings = output.warnings;
    if !warnings.is_empty() {
        println!("Warnings:");
        for warning in warnings {
            println!("{}", warning);
        }
    }

    let values = output.values;
    for value in values {
        let name = value.param_name;
        let inner_tree = value.inner_tree;
        println!("{}", name);
        for (key, value) in inner_tree {
            println!("{}", key);
            for v in value {
                println!("{}", v.data);
            }
        }
    }

    Ok(())
}
