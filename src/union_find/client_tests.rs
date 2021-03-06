use crate::union_find::client as ufclient;
use crate::union_find::client::BulkConnection as ufconnection;

#[test]
fn constructor() {
    let client = ufclient::Client::new();

    assert_eq!(1, client.nodes.len());
    assert_eq!(0, client.node_map.len());
}

#[test]
fn add_node() {
    let mut client = ufclient::Client::new();
    client.add_node("A");

    assert_eq!(2, client.nodes.len());
    assert_eq!(1, client.node_count());
}

#[test]
fn duplicate_adds_ignored() {
    let mut client = ufclient::Client::new();
    client.add_node("A");
    client.add_node("A");

    assert_eq!(2, client.nodes.len());
    assert_eq!(1, client.node_count());
}

#[test]
fn node_exists_positive() {
    let mut client = ufclient::Client::new();
    client.add_node("A");

    assert_eq!(true, client.node_exists("A"));
}

#[test]
fn node_exists_negative() {
    let mut client = ufclient::Client::new();
    client.add_node("A");

    assert_eq!(false, client.node_exists("foo"));
}

#[test]
fn node_index_positive() {
    let mut client = ufclient::Client::new();
    client.add_node("A");

    assert_eq!(1, client.node_index("A"));
}

#[test]
fn node_index_negative() {
    let mut client = ufclient::Client::new();
    client.add_node("A");

    assert_eq!(0, client.node_index("foo"));
}

#[test]
fn connect_nodes_positive() {
    let mut client = ufclient::Client::new();
    client.add_node("A");
    client.add_node("B");
    client.connect_nodes("A", "B");

    assert_eq!(true, client.nodes_connected("A", "B"));
}

#[test]
fn connect_nodes_negative() {
    let mut client = ufclient::Client::new();
    client.add_node("A");
    client.add_node("B");
    client.add_node("C");
    client.connect_nodes("A", "B");

    assert_eq!(false, client.nodes_connected("A", "C"));
}

#[test]
fn disjoint_set_count() {
    let mut client = ufclient::Client::new();
    client.add_node("A");
    client.add_node("B");
    client.add_node("C");
    assert_eq!(3, client.disjoint_set_count());
    client.connect_nodes("A", "B");
    assert_eq!(2, client.disjoint_set_count());
    client.connect_nodes("B", "C");
    assert_eq!(1, client.disjoint_set_count());
    client.connect_nodes("B", "C");
    assert_eq!(1, client.disjoint_set_count());
    client.connect_nodes("A", "A");
    assert_eq!(1, client.disjoint_set_count());
}

#[test]
fn add_nodes_bulk() {
    let mut client = ufclient::Client::new();
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
    client.add_nodes_bulk(nodes);

    assert_eq!(10, client.node_count());
}

#[test]
fn connect_nodes_bulk() {
    let mut client = ufclient::Client::new();
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
    client.add_nodes_bulk(nodes);

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
    client.connect_nodes_bulk(connections);

    assert_eq!(10, client.node_count());
    assert_eq!(2, client.disjoint_set_count());
}