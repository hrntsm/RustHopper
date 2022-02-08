# RustHopper

This is a crate to run grasshopper with RhinoCompute from rust.

The input data can be created by entering into main.rs the same Python code that the Hops component generates to run on RhinoCompute.


# Sample

This is a sample code to run sum.gh in the definitions directory.
It returns the result of A+B.

You can see that it is written in a very similar way to python code which output from hops.

```rs
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
        "0".to_string(),
        vec![io::RestHopperObject {
            object_type: "System.Double".to_string(),
            data: "1.0".to_string(),
        }],
    );
    input_tree.push(io::DataTree {
        param_name: "A".to_string(),
        inner_tree: tree,
    });

    let mut tree = HashMap::new();
    tree.insert(
        "0".to_string(),
        vec![io::RestHopperObject {
            object_type: "System.Double".to_string(),
            data: "2.0".to_string(),
        }],
    );
    input_tree.push(io::DataTree {
        param_name: "B".to_string(),
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

// this settings return below values
// RH_OUT:result
// {0}
// 3.0
```