# cozad-union-find
A Rust implementation of the union-find disjoint set graph algorithm

![MIT License](https://img.shields.io/github/license/ccozad/cozad-union-find) ![Build Status](https://img.shields.io/github/workflow/status/ccozad/cozad-union-find/Build) ![Code Size](https://img.shields.io/github/languages/code-size/ccozad/cozad-union-find) ![Top Language](https://img.shields.io/github/languages/top/ccozad/cozad-union-find)
![Crates.io](https://img.shields.io/crates/v/cozad_union_find)

# Quick Start

## Instalation
Add the following to your Cargo.toml file

```
cozad-union-find = "1.1.0"
```

## Using the named node interfaces
For relatively small networks you can simply interact with nodes by name.

``` rust
extern crate cozad_union_find;
use cozad_union_find::union_find::client as ufclient;

fn main() {
    let mut client = ufclient::Client::new();

    client.add_node("A");
    client.add_node("B");
    client.add_node("C");
    client.add_node("D");
    client.add_node("E");
    client.add_node("F");
    client.add_node("G");
    client.add_node("H");
    client.add_node("I");
    client.add_node("J");


    client.connect_nodes("E", "D");
    client.connect_nodes("D", "I");
    client.connect_nodes("G", "F");
    client.connect_nodes("J", "E");
    client.connect_nodes("C", "B");
    client.connect_nodes("I", "J");
    client.connect_nodes("F", "A");
    client.connect_nodes("H", "B");
    client.connect_nodes("G", "B");
    client.connect_nodes("B", "A");
    client.connect_nodes("G", "H");

    println!("\nDisjoint sets found: {}", client.disjoint_set_count());
}
```

Output
```
Disjoint sets found: 2
```

## Using the bulk interfaces

When you have a large volume of connections to process you can skip the lookups that occur with named nodes and use the bulk interfaces. The process involves giving a vector of node names and then specifying connections between nodes by index.

``` rust
extern crate cozad_union_find;
use cozad_union_find::union_find::client as ufclient;
use cozad_union_find::union_find::client::BulkConnection as ufconnection;

fn main() {

    let mut bulk_client = ufclient::Client::new();
    let nodes = vec![
        String::from("A"), 
        String::from("B"), 
        String::from("C"),
        String::from("D"),
        String::from("E"),
        String::from("F"), 
        String::from("G"), 
        String::from("H"), 
        String::from("I"), 
        String::from("J")
    ];
    bulk_client.add_nodes_bulk(nodes);

    let connections = vec![
        ufconnection { a: 4, b: 3 },
        ufconnection { a: 3, b: 8 },
        ufconnection { a: 6, b: 5 },
        ufconnection { a: 9, b: 4 },
        ufconnection { a: 2, b: 1 },
        ufconnection { a: 8, b: 9 },
        ufconnection { a: 5, b: 0 },
        ufconnection { a: 7, b: 2 },
        ufconnection { a: 6, b: 1 },
        ufconnection { a: 1, b: 0 },
        ufconnection{ a: 6, b: 7 }
    ];
    bulk_client.connect_nodes_bulk(connections);

    println!("\nDisjoint sets found: {}", bulk_client.disjoint_set_count());
}
```

Output
```
Disjoint sets found: 2
```

## Run as a CLI

```
cargo build
cd target/debug
./cozad-union-find -n ../../data/nodes_small.txt -c ../../data/connections_small.txt

```

Example Output
```
Node File: ../../data/nodes_small.txt
Processing nodes file...
Nodes file processed
Bulk adding nodes...
Nodes bulk added

Connections File: ../../data/connections_small.txt
Processing connections file...
Connections file processed
Bulk connecting nodes...
Nodes bulk connected

Disjoint sets found: 2
```

## Run the tests

```
cargo test
```

# Concepts
 - What is a disjoint set?
   - Disjoint sets have no items in common between each set
   - https://en.wikipedia.org/wiki/Disjoint_sets
 - Why would I use this?
   - You have a large un-directed graph and you want to find non overlapping sets, such as for
   - 2D and 3D Percolation
   - Disease exposure
   - Contact tracing
   - Labeling clusters
 - How can I learn more?
   - https://algs4.cs.princeton.edu/15uf/
   - Purchase access to the full support videos
     - Includes detailed coverage of theory, code, and tests
     - Coming soon!

# Support
 - How do I request a change?
   - Please submit an issue or a pull request
 - How fast will my request be added?
   - Probably not very fast for requests outside of a support package because this repo is maintained by a working professional
   - If you require fast, predictable responses, please purchase a support package
 - Can support package be purchased?
   - Yes, various support packages can be purchased and customized for your needs. Support areas available include:
   - On demand support videos
   - 1:1 and team coaching
   - New features and other modifications

## License

Licensed under

 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
