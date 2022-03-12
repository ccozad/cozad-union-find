use std::fs::File;
use std::io::{BufReader, BufRead};
use clap::Parser;
mod union_find;
use union_find::client::BulkConnection;
use union_find::client::Client;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the file with node names by index
    #[clap(short, long, value_name = "FILE")]
    nodes: String,

    /// Name of the file with node connections by index
    #[clap(short, long, value_name = "FILE")]
    connections: String,
}

fn main() {
    let args = Args::parse();
    let mut client = Client::new();

    println!("\nNode File: {}", args.nodes);
    let mut nodes: Vec<String> = vec![];

    let node_file = File::open(args.nodes).unwrap();
    let node_reader = BufReader::new(node_file);

    println!("Processing nodes file...");
    for line in node_reader.lines() {
        nodes.push(line.unwrap());
    }
    println!("Nodes file processed");

    println!("Bulk adding nodes...");
    client.add_nodes_bulk(nodes);
    println!("Nodes bulk added");

    println!("\nConnections File: {}", args.connections);
    let mut connections: Vec<BulkConnection> = vec![];

    let connection_file = File::open(args.connections).unwrap();
    let connection_reader = BufReader::new(connection_file);

    println!("Processing connections file...");
    for line in connection_reader.lines() {
        connections.push(convert_connection(line.unwrap()))
    }
    println!("Connections file processed");

    println!("Bulk connecting nodes...");
    client.connect_nodes_bulk(connections);
    println!("Nodes bulk connected");

    println!("\nDisjoint sets found: {}", client.disjoint_set_count())
}

fn convert_connection(line: String) -> BulkConnection {
    let connection_raw = line.split_once(",");
    let connection = connection_raw.unwrap();
    let a = connection.0.parse::<usize>().unwrap();
    let b = connection.1.parse::<usize>().unwrap();

    BulkConnection::new(a, b)
}