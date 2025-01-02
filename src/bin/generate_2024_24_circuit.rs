use advent_of_code_rust::_2024::_24::parse;
use itertools::Itertools;
use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    // Define the input file path (modify this to reflect your actual file location)
    let input = include_str!("../../input/2024/24.txt");
    let output_file = "circuit.dot";

    let gates = parse(input).gates;
    let z = gates
        .iter()
        .filter(|g| g.output.starts_with("z"))
        .map(|g| g.output.clone())
        .sorted()
        .join("->");
    let x = z.replace("z", "x");
    let y = z.replace("z", "y");

    let and = gates
        .iter()
        .filter(|g| g.operation_str == "AND")
        .map(|g| g.output.clone())
        .join(" ");

    let or = gates
        .iter()
        .filter(|g| g.operation_str == "OR")
        .map(|g| g.output.clone())
        .join(" ");

    let xor = gates
        .iter()
        .filter(|g| g.operation_str == "XOR")
        .map(|g| g.output.clone())
        .join(" ");

    // Open a file for writing
    let mut file = File::create(output_file)?;

    write!(
        file,
        "digraph G {{
subgraph output_z {{
   node [style=filled,color=green]
    {z}
}}
subgraph input_x {{
    node [style=filled,color=gray]
    {x}
}}
subgraph input_y {{
    node [style=filled,color=gray]
    {y}
}}
subgraph gates_and {{
    node [style=filled,color=pink]
    {and}
}}
subgraph gates_or {{
    node [style=filled,color=yellow];
    {or}
}}
subgraph gates_xor {{
    node [style=filled,color=lightblue];
    {xor}
}}
"
    )?;

    // Write outputs to the file
    for gate in gates {
        writeln!(
            file,
            "    {} -> {}; {} -> {};",
            gate.input_a, gate.output, gate.input_b, gate.output
        )?;
    }

    writeln!(file, "}}")?;

    Ok(())
}
