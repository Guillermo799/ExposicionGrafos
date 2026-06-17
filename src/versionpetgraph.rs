use petgraph::graph::{UnGraph, NodeIndex};
use petgraph::algo::astar;
//importar el BFS para poder darle un poco mas de dinamismo a la version de petgraph.
use petgraph::visit::Bfs;
pub fn ejecutar() {

    println!("=================================");
    println!("IMPLEMENTACION PETGRAPH");
    println!("=================================");

    // Se crea un grafo NO dirigido donde cada nodo almacena un &str (nombre de la estación)
    // y las aristas no llevan ningún dato asociado (de ahí el tipo () como peso).
    let mut red = UnGraph::<&str, ()>::new_undirected();

    // ESTACIONES
    // add_node() agrega un nodo al grafo y devuelve un NodeIndex,
    // que es el identificador interno que usaremos luego para crear las conexiones (aristas).
    let auroria  = red.add_node("Auroria");
    let velstrom = red.add_node("Velstrom");
    let nexara   = red.add_node("Nexara");
    let dralion  = red.add_node("Dralion");
    let korveth  = red.add_node("Korveth");
    let myrenth  = red.add_node("Myrenth");
    let caldrix  = red.add_node("Caldrix");
    let zentova  = red.add_node("Zentova");
    let pyloran  = red.add_node("Pyloran");
    let thornex  = red.add_node("Thornex");

    // VIAS
    // add_edge(a, b, peso) crea una conexión entre los nodos a y b.
    // El peso () significa que no nos interesa guardar información extra en la arista
    red.add_edge(auroria, velstrom, ());
    red.add_edge(velstrom, nexara, ());
    red.add_edge(nexara, dralion, ());
    red.add_edge(dralion, korveth, ());
    red.add_edge(korveth, myrenth, ());
    red.add_edge(myrenth, caldrix, ());
    red.add_edge(caldrix, zentova, ());
    red.add_edge(zentova, auroria, ());

    // se conecta con 4 estaciones distintas del anillo principal,
    // funcionando como un atajo entre partes opuestas de la red.
    red.add_edge(pyloran, auroria, ());
    red.add_edge(pyloran, nexara, ());
    red.add_edge(pyloran, korveth, ());
    red.add_edge(pyloran, caldrix, ());

    red.add_edge(thornex, dralion, ());
    red.add_edge(thornex, velstrom, ());

    println!("\nESTADISTICAS");
    // node_count() devuelve el número total de nodos (estaciones) en el grafo.
    println!("Nodos   : {}", red.node_count());
    // edge_count() devuelve el número total de aristas (vías/conexiones) en el grafo.
    println!("Aristas : {}", red.edge_count());

    // Busca y muestra la ruta más corta entre Zentova y Nexara usando A*
    buscar_ruta(
        &red,
        zentova,
        nexara
    );

// Muestra la lista de adyacencia completa del grafo
mostrar_adyacencia(&red);
// Recorre el grafo en anchura (BFS) comenzando desde Zentova
mostrar_bfs(&red, zentova);

}

// Imprime, para cada nodo del grafo, la lista de sus vecinos directos
// (es decir, las estaciones con las que tiene una conexión directa).
fn mostrar_adyacencia(
    red: &UnGraph<&str, ()>
) {

    println!("\nLISTA DE ADYACENCIA\n");

    for nodo in red.node_indices() {

        print!("{} -> ", red[nodo]);

        let vecinos: Vec<_> =
            red.neighbors(nodo)
               .map(|v| red[v])
               .collect();

        println!("{}", vecinos.join(", "));
    }
}

// Realiza un recorrido BFS (Breadth-First Search / Búsqueda en Anchura)
// a partir de un nodo de origen, mostrando el orden en que se visitan los nodos.
fn mostrar_bfs(
    red: &UnGraph<&str, ()>,
    origen: NodeIndex,
) {

    println!("\nRECORRIDO BFS\n");

    let mut bfs = Bfs::new(red, origen);

    let mut paso = 1;

    while let Some(nx) = bfs.next(red) {

        println!(
            "{}. {}",
            paso,
            red[nx]
        );

        paso += 1;
    }
}

// Busca la ruta más corta (en número de saltos) entre un nodo origen y un nodo destino,
// usando el algoritmo A* (A-star), y la muestra en pantalla paso a paso.
fn buscar_ruta(
    red: &UnGraph<&str, ()>,
    origen: NodeIndex,
    destino: NodeIndex,
) {

    // Devuelve Some((costo_total, camino)) si existe una ruta, o None si no hay conexión.
    if let Some((_costo, camino)) = astar(
        red,
        origen,
        |n| n == destino,
        |_| 1,
        |_| 0,
    ) {

        println!();
        
    println!("\nRUTA MINIMA\n");

    println!("Origen  : {}", red[origen]);
    println!("Destino : {}", red[destino]);

    println!();

    for nodo in &camino {

    println!("{}", red[*nodo]);

    if *nodo != destino {
        println!("  ↓");
    }
    }

    println!();
    println!("Saltos: {}", camino.len() - 1);

        println!();
    }
    // Nota: si astar devuelve None (no hay ruta), esta función no imprime nada,
    // podría ser útil agregar un mensaje de "ruta no encontrada" en ese caso.
}