#[cfg(test)]
#[path = "client_tests.rs"]
mod client_tests;

use std::collections::HashMap;

/// A node in the graph
#[derive(Hash, Eq, PartialEq, Debug)]
struct Node {
    /// Unique ID of the node
    pub uuid: String,
    /// Index for the node's parent
    pub parent_index: usize,
    /// Index where the node is stored
    pub index: usize,
    /// Number items in chain
    pub size: usize
}

#[derive(Hash, Eq, PartialEq, Debug)]
/// A connection between two node indexes
pub struct BulkConnection {
    /// Index of first connection
    pub a: usize,
    /// Index of second connection
    pub b: usize
}

#[derive(Debug)]
/// A client that manages a graph of nodes and their connections
///
/// # Examples
///
/// Named node interfaces
///
/// ``` rust
/// extern crate cozad_union_find;
/// use cozad_union_find::union_find::client as ufclient;
///
/// fn main() {
///    let mut client = ufclient::Client::new();
///
///    client.add_node("A");
///    client.add_node("B");
///    client.add_node("C");
///    client.add_node("D");
///    client.add_node("E");
///    client.add_node("F");
///    client.add_node("G");
///    client.add_node("H");
///    client.add_node("I");
///    client.add_node("J");
///
///    client.connect_nodes("E", "D");
///    client.connect_nodes("D", "I");
///    client.connect_nodes("G", "F");
///    client.connect_nodes("J", "E");
///    client.connect_nodes("C", "B");
///    client.connect_nodes("I", "J");
///    client.connect_nodes("F", "A");
///    client.connect_nodes("H", "B");
///    client.connect_nodes("G", "B");
///    client.connect_nodes("B", "A");
///    client.connect_nodes("G", "H");
///
///    println!("\nDisjoint sets found: {}", client.disjoint_set_count());
/// }
///```
///
/// Bulk interfaces
///
/// ``` rust
/// extern crate cozad_union_find;
/// use cozad_union_find::union_find::client as ufclient;
/// use cozad_union_find::union_find::client::BulkConnection as ufconnection;
///
/// fn main() {
///
///    let mut bulk_client = ufclient::Client::new();
///    let nodes = vec![
///        String::from("A"), 
///        String::from("B"), 
///        String::from("C"),
///        String::from("D"),
///        String::from("E"),
///        String::from("F"), 
///        String::from("G"), 
///        String::from("H"), 
///        String::from("I"), 
///        String::from("J")
///    ];
///    bulk_client.add_nodes_bulk(nodes);
///
///    let connections = vec![
///        ufconnection { a: 4, b: 3 },
///        ufconnection { a: 3, b: 8 },
///        ufconnection { a: 6, b: 5 },
///        ufconnection { a: 9, b: 4 },
///        ufconnection { a: 2, b: 1 },
///        ufconnection { a: 8, b: 9 },
///        ufconnection { a: 5, b: 0 },
///        ufconnection { a: 7, b: 2 },
///        ufconnection { a: 6, b: 1 },
///        ufconnection { a: 1, b: 0 },
///        ufconnection{ a: 6, b: 7 }
///    ];
///    bulk_client.connect_nodes_bulk(connections);
///
///    println!("\nDisjoint sets found: {}", bulk_client.disjoint_set_count());
/// }
/// ```
pub struct Client {
    /// Storage for nodes in the graph
    nodes: Vec<Node>,
    /// Map of names to index
    node_map: HashMap<String, usize>,
    /// Number of disjoint sets
    set_count: usize
}

impl BulkConnection {
    /// Constructs a new `BulkConnection`
    /// 
    /// # Arguments
    ///
    /// * `a` - Index of first connection
    /// * `b` - Index of second connection
    pub fn new(a: usize, b: usize) -> Self {
        BulkConnection {
            a,
            b
        }
    }
}

impl Client {
    /// Constructs a new `Client`
    pub fn new() -> Self {
        let node_map = HashMap::new();
        let mut nodes = Vec::new();

        let root_node = Node { 
            uuid: String::from("root"), 
            parent_index: 0, 
            index: 0,
            size: 0
        };
        nodes.push(root_node);

        Client {
            nodes,
            node_map,
            set_count: 0
        }
    }

    /// Adds a node with given unique id
    ///
    /// # Arguments
    ///
    /// * `uuid` - Unique ID of node
    ///
    #[allow(dead_code)]
    pub fn add_node(&mut self, uuid: &str) {
        if !self.node_exists(uuid) {
            let node = Node { 
                uuid: String::from(uuid), 
                parent_index: self.nodes.len(), 
                index: self.nodes.len(),
                size: 1
            };
            self.node_map.insert(String::from(uuid), node.index);
            self.nodes.push(node);
            self.set_count += 1;
        }
    }

    /// Adds a multiple nodes with a single call
    /// 
    /// # Arguments
    ///
    /// * `uuid_list` - Collection of unique IDs
    /// 
    pub fn add_nodes_bulk(&mut self, uuid_list: Vec<String>) {
        for uuid in uuid_list.iter() {
            let node = Node { 
                uuid: String::from(uuid), 
                parent_index: self.nodes.len(), 
                index: self.nodes.len(),
                size: 1
            };
            self.node_map.insert(String::from(uuid), node.index);
            self.nodes.push(node);
            self.set_count += 1;
        }
    }

    /// Connects two nodes using their unique id
    /// 
    /// # Arguments
    ///
    /// * `uuid_a` - Unique id first node
    /// * `uuid_b` - Unique id second node
    ///
    #[allow(dead_code)]
    pub fn connect_nodes(&mut self, uuid_a: &str, uuid_b: &str) {
        let uuid_a_root = self.find_root_index(uuid_a);
        let uuid_b_root = self.find_root_index(uuid_b);

        if uuid_a_root == uuid_b_root {
            return
        } else {
            let node_slice = &mut self.nodes[..];

            if node_slice[uuid_a_root].size < node_slice[uuid_b_root].size {
                node_slice[uuid_a_root].parent_index = uuid_b_root;
                node_slice[uuid_b_root].size += node_slice[uuid_a_root].size;
            } else {
                node_slice[uuid_b_root].parent_index = uuid_a_root;
                node_slice[uuid_a_root].size += node_slice[uuid_b_root].size;
            }

            self.set_count -= 1;
        }
    }

    /// Connects a collection of nodes using node indexes to avoid node lookups by name
    ///
    /// # Arguments
    ///
    /// * `connections` - Collection of graph connections
    /// 
    pub fn connect_nodes_bulk(&mut self, connections: Vec<BulkConnection>) {
        for connection in connections.iter() {
            let uuid_a_root = self.find_root_index_bulk(connection.a + 1);
            let uuid_b_root = self.find_root_index_bulk(connection.b + 1);

            if uuid_a_root == uuid_b_root {
                //do nothing
            } else {
                let node_slice = &mut self.nodes[..];

                if node_slice[uuid_a_root].size < node_slice[uuid_b_root].size {
                    node_slice[uuid_a_root].parent_index = uuid_b_root;
                    node_slice[uuid_b_root].size += node_slice[uuid_a_root].size;
                } else {
                    node_slice[uuid_b_root].parent_index = uuid_a_root;
                    node_slice[uuid_a_root].size += node_slice[uuid_b_root].size;
                }
    
                self.set_count -= 1;
            }
        }
    }

    /// The number of sets that share no connection with another set
    pub fn disjoint_set_count(&self) -> usize {
        self.set_count
    }

    /// Finds the connected node with no parent
    /// 
    /// # Arguments
    ///
    /// * `uuid` - Unique ID of node to find root of
    /// 
    pub fn find_root_index(&self, uuid: &str) -> usize {
        let node_index = self.node_index(uuid);
        if node_index > 0 {
            let mut node = self.nodes.get(node_index).unwrap();
            while node.parent_index != node.index {
                node = self.nodes.get(node.parent_index).unwrap();
            }
            node.parent_index
        } else {
            0
        }
    }

    /// Finds the connected node with no parent, optimized to reduce lookups
    /// 
    /// # Arguments
    ///
    /// * `node_index` - Index of node to find the root of
    /// 
    pub fn find_root_index_bulk(&self, node_index: usize) -> usize {
        let mut node = self.nodes.get(node_index).unwrap();
        while node.parent_index != node.index {
            node = self.nodes.get(node.parent_index).unwrap();
        }
        node.parent_index
    }

    /// Determines if two nodes are connected through any path
    /// 
    /// # Arguments
    ///
    /// * `uuid_a` - Unique ID of first connection
    /// * `uuid_b` - Unique ID of second connection
    ///
    #[allow(dead_code)] 
    pub fn nodes_connected(&self, uuid_a: &str, uuid_b: &str) -> bool {
        let uuid_a_root = self.find_root_index(uuid_a);
        let uuid_b_root = self.find_root_index(uuid_b);
        
        uuid_a_root > 0 && uuid_a_root == uuid_b_root
    }

    /// The number of unique nodes in the graph
    #[allow(dead_code)]
    pub fn node_count(&self) -> usize {
        self.nodes.len() - 1
    }

    /// Determines if a node exists by the given name
    /// 
    /// # Arguments
    ///
    /// * `uuid` - Unique ID of node
    /// 
    pub fn node_exists(&self, uuid: &str) -> bool {
        let node_uuid = String::from(uuid);
        self.node_map.contains_key(&node_uuid)
    }

    /// Gets the index for a node with a given unique ID
    /// 
    /// # Arguments
    ///
    /// * `uuid` - Unique ID of node
    /// 
    pub fn node_index(&self, uuid: &str) -> usize {
        let node_uuid = String::from(uuid);
        if self.node_map.contains_key(&node_uuid) {
            *self.node_map.get(&node_uuid).unwrap()
        } else {
            0
        }
    }
}