use std::fmt;

use petgraph::{
    dot::{Config, Dot},
    Graph,
};

#[derive(Debug, Copy, Clone)]
struct Person<'a> {
    name: &'a str,
    age: u8,
}

#[derive(Debug, Copy, Clone)]
enum Relationship {
    Friend,
    Parent,
    Sibling,
    Child,
    Boss,
}

impl<'a> fmt::Display for Person<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}, {}", self.name, self.age)
    }
}

impl fmt::Display for Relationship {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn main() {
    println!("digraph  {{");

    let mut social_graph: Graph<Person, Relationship> = Graph::new();

    let bob = social_graph.add_node(Person {
        name: "Bob",
        age: 37,
    });
    let alice = social_graph.add_node(Person {
        name: "Alice",
        age: 17,
    });
    social_graph.add_edge(bob, alice, Relationship::Parent);

    let lilly = social_graph.add_node(Person {
        name: "Lilly",
        age: 50,
    });
    social_graph.add_edge(lilly, bob, Relationship::Boss);

    let george = social_graph.add_node(Person {
        name: "George",
        age: 16,
    });
    social_graph.add_edge(george, alice, Relationship::Friend);
    social_graph.add_edge(lilly, george, Relationship::Parent);

    let fred = social_graph.add_node(Person {
        name: "Fred",
        age: 16,
    });
    social_graph.add_edge(george, fred, Relationship::Friend);
    social_graph.add_edge(alice, fred, Relationship::Friend);

    println!(
        "{:?}",
        Dot::with_config(&social_graph, &[Config::GraphContentOnly])
    );
    println!("}}");
}
