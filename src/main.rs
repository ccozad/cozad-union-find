use clap::Parser;

/// Simple program to greet a person
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

    println!("Node File: {}!", args.nodes);
    println!("Connections File: {}!", args.connections);
}