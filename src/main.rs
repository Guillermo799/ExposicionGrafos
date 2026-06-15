mod versionmanual;
mod versionpetgraph;

use std::env;

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {

        println!("Uso:");
        println!("cargo run manual");
        println!("cargo run petgraph");

        return;
    }

    match args[1].as_str() {

        "manual" => versionmanual::ejecutar(),

        "petgraph" => versionpetgraph::ejecutar(),

        _ => println!("Opción inválida"),
    }
}