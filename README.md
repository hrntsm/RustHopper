# RustHopper

This is a crate to run grasshopper with RhinoCompute from rust.  
The input data can be created by entering into main.rs the same Python code that the Hops component generates to run on RhinoCompute.

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
