# cozad-union-find
A Rust implementation of the union-find disjoint set graph algorithm

![MIT License](https://img.shields.io/github/license/ccozad/cozad-union-find) ![Build Status](https://img.shields.io/github/workflow/status/ccozad/cozad-union-find/Build) ![Code Size](https://img.shields.io/github/languages/code-size/ccozad/cozad-union-find)

# Quick Start

## Run as a CLI
``` bash
cargo build
cd target/debug
cozad-union-find -n ../../data/nodes_small.txt -c ../../connections_small.txt

```

## Run the tests

``` bash
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
