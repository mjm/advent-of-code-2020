mod graph;

use std::env;
use std::fs;
use crate::graph::Graph;
use nom::{IResult, Finish};
use nom::bytes::complete::{tag};
use nom::sequence::{separated_pair, terminated};
use nom::character::complete::{alpha1, space1, digit1, newline, char};
use nom::combinator::{map, recognize, value, map_res, all_consuming};
use nom::multi::separated_list1;
use nom::branch::alt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let graph = parse_graph(&contents)
        .unwrap();

    part1(&graph);
}

fn part1(graph: &Graph) {
    let result = graph.nodes_reachable_from("shiny gold")
        .expect("Could not find any nodes reachable from shiny gold");
    println!("There are {} kinds of bags that can contain a shiny gold bag.", result);
}

fn parsed_digit1(s: &str) -> IResult<&str, i32> {
    map_res(digit1, |s: &str| s.parse::<i32>())(s)
}

fn bag_desc(s: &str) -> IResult<&str, &str> {
    recognize(separated_pair(alpha1, space1, alpha1))(s)
}

fn graph_parser(s: &str) -> IResult<&str, Graph> {
    let single_bag_desc = terminated(separated_pair(value(1, tag("1")), space1, bag_desc), tag(" bag"));
    let multi_bag_desc = terminated(separated_pair(parsed_digit1, space1, bag_desc), tag(" bags"));
    let counted_bag_desc = alt((
        single_bag_desc,
        multi_bag_desc,
    ));
    let graph_line = terminated(separated_pair(
        bag_desc,
        tag(" bags contain "),
        alt((
            value(vec![], tag("no other bags")),
            separated_list1(
                tag(", "),
                counted_bag_desc,
            ),
        )),
    ), char('.'));

    map(separated_list1(
        newline,
        graph_line,
    ), |lines| {
        let mut graph = Graph::new();

        for (from_desc, tos) in lines {
            for (quantity, to_desc) in tos {
                graph.add_edge(to_desc, from_desc, quantity);
            }
        }

        graph
    })(s)
}

fn parse_graph(contents: &str) -> Result<Graph, nom::error::Error<&str>> {
    all_consuming(graph_parser)(contents).finish().map(|(_, graph)| graph)
}
