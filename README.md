# RustHopper
<p align="center">
    <img width="50%" src="https://raw.githubusercontent.com/hrntsm/RustHopper/6a0671479ac20414cf5054117cbb80cd56938ad9/image/home-og.svg">
</p>

This is a crate to run grasshopper with RhinoCompute from rust.  
The input data can be created by entering into main.rs the same Python code that the Hops component generates to run on RhinoCompute.

# LICENSE

This library is released under the MIT License.

Rust icon : Rust Foundation, CC 4.0, https://commons.wikimedia.org/w/index.php?curid=40715219

# How to use

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
    input_tree.push(io::DataTree {i
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

# More details

Rust is a popular language these days, and I'm sure there are many people who would like to try it.
However, most of the architectural software is provided with Python or C# SDKs,
and you will rarely come into contact with a language like Rust if you don't use it in your work.

If you can use Rust to manipulate something in architectural software, you will have a chance to touch Rust.
So this repo describes how to use RhinoCompute with Rust, which allows you to use Rhinoceros features through HTTP communication with Rhinoceros.

In this article, we will first deepen our understanding of communicating with RhinoCompute to see how it should be implemented before touching Rust.
After that, we will create a runtime environment in Rust based on that.

Technically, it is possible to get Rhino data and do rhino3dm equivalent operations in Rust.
However, the implementation is time consuming, so the goal is to post Grasshopper data and use RhinoCompute to retrieve the results.

### Environment

- Rhinoceros 7
- Grasshopper
  - Hops v0.11
- Rust (rustc 1.58.1)

## What is the content of the communication with RhinoCompute?

It is difficult to understand the RhinoCompute implementation from the beginning,
so let's first check how the communication is done using the Hops component.

Create a simple Grasshopper data to calculate A+B and load it into Hops as shown below.
In v0.11 and later versions of Hops,
REST API output has been added to help you understand what is being communicated.

![hops.jpg](https://hiron.dev/article-images/deep-dive-into-rhinocompute/hops.jpg)

For example, Last IO Request is as follows.

The `algo` part stands out in particular.
It is omitted in the following, but I think it is actually a very long string.
The content here is a Grasshopper file converted to Base64, which is a format that does not cause problems in communication.

```json
{
  "URL": "http://localhost:6500/io",
  "content": {
    "absolutetolerance": 0.0,
    "angletolerance": 0.0,
    "algo": "7VgJUBNZGk4gQEIIId.....",
    "pointer": null,
    "cachesolve": false,
    "recursionlevel": 0,
    "values": [],
    "warnings": [],
    "errors": []
  }
}
```

The following is the response from the above post.
You can see the contents of the Input and Output and the breakdown of the Input settings.

The important thing here is the value of `CacheKey`. This is the key to the cache of the Grasshopper data you posted.

```json
{
  "Description": "",
  "CacheKey": "md5_77996BBE6275E0EA0564BF666AF66C32",
  "InputNames": ["A", "B"],
  "OutputNames": ["RH_OUT:result"],
  "Icon": null,
  "Inputs": [
    {
      "Description": "",
      "AtLeast": 1,
      "AtMost": 1,
      "Default": "1",
      "Minimum": null,
      "Maximum": null,
      "Name": "A",
      "Nickname": null,
      "ParamType": "Number"
    },
    {
      "Description": "",
      "AtLeast": 1,
      "AtMost": 1,
      "Default": "1",
      "Minimum": null,
      "Maximum": null,
      "Name": "B",
      "Nickname": null,
      "ParamType": "Number"
    }
  ],
  "Outputs": [
    {
      "Name": "RH_OUT:result",
      "Nickname": null,
      "ParamType": "Number"
    }
  ]
}
```

Now that you've uploaded Grasshopper by posting to /io,
you can get the results in RhinoCompute by posting Input to it.

The request to Solve is as follows.
The point to check here is the value of `pointer`.
The CacheKey obtained in the response from IO is used.
The tolerance value required for actual calculation is also specified.
If Hops is set to cache the result, the value of `cachesolve` is set to true.
If you don't want to cache the results, set this to false.

```json
{
  "URL": "http://localhost:6500/grasshopper",
  "content": {
    "absolutetolerance": 0.001,
    "angletolerance": 1.0,
    "algo": null,
    "pointer": "md5_77996BBE6275E0EA0564BF666AF66C32",
    "cachesolve": true,
    "recursionlevel": 0,
    "values": [
      {
        "ParamName": "A",
        "InnerTree": {
          "0": [
            {
              "type": "System.Double",
              "data": "1.0"
            }
          ]
        }
      },
      {
        "ParamName": "B",
        "InnerTree": {
          "0": [
            {
              "type": "System.Double",
              "data": "1.0"
            }
          ]
        }
      }
    ],
    "warnings": [],
    "errors": []
  }
}
```

If you send base64 Grasshopper data to algo without using pointer, the result will be returned.
However, the file size becomes about 4/3 times larger when base64 is used due to the feature of the algorithm, and the data is cached in RhinoCompute.
If you run the same file several times,
it's fine, but if you run it hundreds of times, the memory footprint of the accumulated Grasshopper data will be quite large.
So, if possible, it is better to cache the data once in /io and use the cache as described above.

The returned result is below.
Since I created it to do addition, RH_OUT:result returns 2 which is the result of 1+1.

```json
{
  "absolutetolerance": 0.0,
  "angletolerance": 0.0,
  "algo": "",
  "pointer": "md5_77996BBE6275E0EA0564BF666AF66C32",
  "cachesolve": false,
  "recursionlevel": 0,
  "values": [
    {
      "ParamName": "RH_OUT:result",
      "InnerTree": {
        "{0}": [
          {
            "type": "System.Double",
            "data": "2.0"
          }
        ]
      }
    }
  ],
  "warnings": [],
  "errors": []
}
```

Now that we have a general understanding of the behavior in Hops, we can check the actual implementation.
The schema used can be found in mcneel's compute.rhino3d repository following.

- [compute.rhino3d/src/compute.geometry/IO/Schema.cs](https://github.com/mcneel/compute.rhino3d/blob/master/src/compute.geometry/IO/Schema.cs)

If you check the value of `warnings` and `errors`,
which are empty arrays that do not return any value in this case,
the implementation returns a list of strings.

The following is an excerpt from the relevant section.

```cs
public class Schema
{
  [JsonProperty(PropertyName = "warnings")]
  public List<string> Warnings { get; set; } = new List<string>();

  [JsonProperty(PropertyName = "errors")]
  public List<string> Errors { get; set; } = new List<string>();
}
```

Now you know what kind of data you need to post to run Grasshopper.
I'll describe the details of the implementation in the Rust part later on.

## Implementing RhinoCompute Execution with Rust

First, create a new package using cargo.
If you name it "rusthopper", it will look like this.

```bash
cargo new rusthopper
```

I'm going to use this to create it now.

### Creating structure for IO with Json

Before creating the communication part, we create a structure to easily exchange Json in Input/Output.
Note that there is no class in Rust.
The crate that supports serialization/deserialization of Json is mainly serde in Rust.
Dependencies can be resolved in Cargo.toml.

```toml
[dependencies]
serde = "1.0.136"
serde_derive = "1.0.136"
serde_json = "1.0.78"
```

Create an io.rs file in the src directory and create a structure for I/O Json in it.

This structure is based on [Schema.cs](https://github.com/mcneel/compute.rhino3d/blob/master/src/compute.geometry/IO/Schema.cs) in RhinoCompute repository.
As mentioned above, RhinoCompute uses this class for processing, so you can exchange data smoothly by following this Schema.

However, implementing this from scratch is tedious, so it's easier to use an automatic implementation.
For example, [transform.tools](https://transform.tools/json-to-rust-serde) can create a Rust structure from Json,
so I recommend you to use it as a base and modify the missing parts by hand.

Use Json output from Hops. As an example, the conversion of Json to IO posted by Hops is as follows.

Before

```json
{
  "absolutetolerance": 0.0,
  "angletolerance": 0.0,
  "algo": "7VgJUBNZGk4gQ.....",
  "pointer": null,
  "cachesolve": false,
  "recursionlevel": 0,
  "values": [],
  "warnings": [],
  "errors": []
}
```

After

```rs
use serde_derive::Deserialize;
use serde_derive::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub absolutetolerance: f64,
    pub angletolerance: f64,
    pub algo: String,
    pub pointer: Value,
    pub cachesolve: bool,
    pub recursionlevel: i64,
    pub values: Vec<Value>,
    pub warnings: Vec<Value>,
    pub errors: Vec<Value>,
}
```

The parts of Json that are null or empty arrays are not automatically typed, so let's fix them by looking at the Schema.
The result of the modification is as follows.

```rs
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
```

Since `algo` and `pointer` can be null (or None in Rust), we use Option Enum to handle None or Some(T).

`warnings` and `errors` are Vec\<String\> arrays of strings

The `values` structure `DataTree` is created separately to contain the information of Grasshopper's tree.
In C# Schema files, DataTree is handled differently, but for simplicity, we use a different structure here.
A DataTree consists of a `param_name`, which is the name of the parameter,
a String that is the path to the value of the parameter,
and a HashMap of the `inner_tree` that contains the actual value.

The Response structure is created in the same way. The following is the created content. This is the same as C#'s Schema.cs but rewritten into a Rust structure.

```rs
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
    pub minimum: f64,
    #[serde(rename = "Maximum")]
    pub maximum: f64,
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
```

### Creating the communication part

Now that the structure for IO has been created, the next step is to create the part that posts them.
As necessary dependencies, add base64 which converts binary (gh file) to Base64 for communication,
reqwest for communication and tokio which is necessary for asynchronization to the dependencies.

```toml
[dependencies]
base64 = "0.13.0"
reqwest = { version = "0.11", features = ["json"] }
serde = "1.0.136"
serde_derive = "1.0.136"
serde_json = "1.0.78"
tokio =  { version = "1", features = ["full"] }
```

First, create a part that posts a Grasshopper file to /io.
The implementation is to take the path to the gh file as an argument with &str, and return the result of the post as Result.
Also, `async` is added at the beginning to make the function asynchronous because it is a communication.

Since it's a post to /io, I'm expecting algo to contain the data from the gh file,
and the response to return the cache_key for that data.

```rs
use base64::encode;
use std::fs::File;
use std::io::Read;

use crate::{io, URL};

async fn upload_definition(
    gh_path: &str,
) -> Result<io::IoResponseSchema, Box<dyn std::error::Error>> {
    // create io URL
    let io_url = URL.to_owned() + "io";

    // encode to Base64
    let mut gh_file = File::open(gh_path).unwrap();
    let mut buf = Vec::new();
    let _ = gh_file.read_to_end(&mut buf);
    let encoded: &str = &encode(&buf);

    // serialize Json
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

    // post & deserialize its result
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
```

Rust is often said to have a strict compiler. In the above example,
I put the string "0.0" and run `cargo check`, even though the absolutetolerance is f64.

```rs
let io_schema = io::Schema {
    absolutetolerance: "0.0",
```

The result is as follows, the compiler will tell you in detail what is wrong and how it is wrong.
Basically, if you follow the compiler's instructions honestly, the code will be finished.

```bash
error[E0308]: mismatched types
  --> src\grasshopper.rs:15:28
   |
15 |         absolutetolerance: "0.0",
   |                            ^^^^^ expected `f64`, found `&str`
```

The response from /io will return the cache key of the gh file you posted,
so we'll use that to create the part that actually evaluates the file.

The implementation follows mcneel's Python compute-rhino3d implementation,
taking a gh file path and a DataTree and processing it.

```rs
pub async fn evaluate_definition(
    gh_path: &str,
    data_tree: Vec<io::DataTree>,
) -> Result<io::Schema, Box<dyn std::error::Error>> {

    // get cache_key
    let cache_key = upload_definition(gh_path).await?.cache_key;

    // create io URL
    let solve_url = URL.to_owned() + "grasshopper";

    // Serialize to Json
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

    // post json & deserialize its result
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
```

This completes the part of the Grasshopper file that executes and retrieves the values.

### Creating main.rs

Now that the IO part and the communication part have been created,
we will put them together to create the part that actually creates the data to be posted and displays the results.

```rs
mod grasshopper;
mod io;

use std::collections::HashMap;

// create base URL
const URL: &str = "http://localhost:6500/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // set .gh file path
    let gh_path = "definitions/sum.gh";

    // create DateTree for input
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

    // post date to RhinoCompute & get result
    let output = grasshopper::evaluate_definition(gh_path, input_tree).await?;

    // Show result
    // error
    let errors = output.errors;
    if !errors.is_empty() {
        println!("Errors:");
        for error in errors {
            println!("{}", error);
        }
    }

    // warning
    let warnings = output.warnings;
    if !warnings.is_empty() {
        println!("Warnings:");
        for warning in warnings {
            println!("{}", warning);
        }
    }

    // result in RH_OUT
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
```

Once you've done all that, run `cargo check` to make sure everything is ok.
If everything is fine, start RhinoCompute and do `cargo run` to run the code and make sure it returns the correct results.

## Summary

How did you like the example of running RhinoCompute in Rust?
It's a very different language from C# or Python, so I'm sure you had a hard time with it,
but I'm sure you felt the power of Rust's powerful compiler when you actually wrote the code as you went along.

I don't think we'll be using Rust much in the architecture field,
but if you get the chance, please give Rust a try.

