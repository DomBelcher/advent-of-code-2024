use std::{collections::{HashMap, HashSet}, fs, i32};
use std::cmp::{min, max};

const FILENAME: &str = "./inputs/input.txt";
const LAN_SIZE: usize = 3;

fn main() {
    let connections = parse_input();
    println!("{}", connections.len());
    
    let mut all_lans: Vec<LAN> = vec![];
    // let mut complete_lans = vec![];

    let mut graph: HashMap<[char; 2], HashSet<[char; 2]>> = HashMap::new();

    let mut cliques = vec![];

    // let mut lan_mapping = HashMap::new();
    for (idx, conn) in connections.iter().enumerate() {
        let mut clique = HashSet::new();
        clique.insert(conn.c1);
        clique.insert(conn.c2);
        cliques.push(clique);

        if graph.contains_key(&conn.c1) {
            graph.get_mut(&conn.c1).unwrap().insert(conn.c2);
        } else {
            let arcs = HashSet::from([conn.c2; 1]);
            // arcs.insert(conn.c2);
            graph.insert(conn.c1, arcs);
        }

        if graph.contains_key(&conn.c2) {
            graph.get_mut(&conn.c2).unwrap().insert(conn.c1);
        } else {
            let arcs = HashSet::from([conn.c1; 1]);
            // arcs.insert(conn.c1);
            graph.insert(conn.c2, arcs);
        }

        // let mut new_lans: Vec<LAN> = vec![];
        // let mut new_complete_lans = vec![];
        // for lan in all_lans.iter() {
        //     let new_lan = lan.try_insert(conn);
        //     if new_lan.is_none() {
        //         continue;
        //     }
        //     let nl = new_lan.unwrap();
        //     if nl.is_complete() {
        //         new_complete_lans.push(nl);
        //     } else {
        //         new_lans.push(nl);
        //     }
        // }
        // all_lans.append(&mut new_lans);
        // complete_lans.append(&mut new_complete_lans);

        // all_lans.push(LAN::with_conn(LAN_SIZE, conn));
    }

    // println!("{:?}", graph.get(&['a', 'd']).unwrap());
    let max_connections = graph.iter().map(|(source, targets)| targets.len()).max().unwrap();
    println!("Max connections: {}", max_connections);

    println!("Total lans: {}", all_lans.len());

    // let mut complete_lans = 0;
    let mut t_lans = 0;

    let mut upper_bound = max_connections + 1;
    let mut largest_clique_size = 0;
    let mut largest_clique = HashSet::new();

    for (source, targets) in graph.iter() {
        let mut nodes = targets.clone();
        nodes.insert(*source);

        if is_clique(&nodes, &graph) {
            println!("clique:");
            print_clique(&nodes);
            panic!();
        } else {
            let largest_subclique = find_largest_clique(&nodes, &graph, 2);
            if largest_subclique.len() > largest_clique_size {
                largest_clique_size = largest_subclique.len();
                largest_clique = largest_subclique;
                println!("current largest clique size: {}", largest_clique_size);
                print_clique(&largest_clique);
            }
        }
    }
    // println!("complete lans: {}", complete_lans.len());

    // for lan in complete_lans.iter() {
    //     if lan.nodes.iter().any(|n| n[0] == 't') {
    //         t_lans += 1;
    //     }
    // }

    // println!("T lans: {}", t_lans);

    let mut clique_size = 2;
    loop {
        println!("Clique size: {}", clique_size);
        println!("total cliques: {}", cliques.len());
        if cliques.len() == 1 {
            println!("{:?}", cliques[0]);
            print_clique(&cliques[0]);
            break
        }
        if cliques.len() < 1 {
            println!("No cliques of size {}", clique_size);
            break;
        }
        clique_size += 1;
        println!("Searching for cliques of size {}", clique_size);

        let mut new_cliques = vec![];

        for (source, targets) in graph.iter() {
            // println!("node: {:?} | {} targets", source, targets.len());
            for clique in cliques.iter_mut() {
                if clique.iter().all(|c: &[char; 2]| targets.contains(c)) {
                    let mut new_clique = clique.clone();
                    new_clique.insert(*source);
                    // clique.insert(*source);
                    new_cliques.push(new_clique);
                }
            }
        }

        println!("new cliques: {}", new_cliques.len());
        // cliques = new_cliques;

        cliques = new_cliques.into_iter().filter(|clique| clique.len() == clique_size).collect::<Vec<_>>();
        println!("total cliques: {}", cliques.len());
    }

    let mut lan_size = LAN_SIZE + 1;
    // // loop {
    //     all_lans = complete_lans.iter().map(|lan| lan.expand()).collect::<Vec<_>>();
    //     complete_lans = vec![];
    //     println!("Current lan size: {}", lan_size);
    //     println!("Currently {} lans", all_lans.len());

    //     for (idx, conn) in connections.iter().enumerate() {
    //         if idx % 100 == 0 {
    //             println!("{}", idx);
    //         }
    //         let mut new_lans: Vec<LAN> = vec![];
    //         let mut new_complete_lans = vec![];
    //         for lan in all_lans.iter() {
    //             let new_lan = lan.try_insert(conn);
    //             if new_lan.is_none() {
    //                 continue;
    //             }
    //             let nl = new_lan.unwrap();
    //             if nl.is_complete() {
    //                 new_complete_lans.push(nl);
    //             } else {
    //                 new_lans.push(nl);
    //             }
    //         }
    //         all_lans.append(&mut new_lans);
    //         complete_lans.append(&mut new_complete_lans);
    //     }

    //     if complete_lans.len() == 0 {
    //         break;
    //     }
    //     lan_size += 1;
    // }

    // let biggest_lan = complete_lans.last().unwrap();
    // println!("Biggest lan: {}", biggest_lan.len());
    // println!("{:?}", biggest_lan.nodes);
}

fn is_clique (nodes: &HashSet<[char; 2]>, graph: &HashMap<[char; 2], HashSet<[char; 2]>>) -> bool {
    let mut is_clique = true;
    for node in nodes {
        let targets = graph.get(node).unwrap();
        is_clique = is_clique && nodes.iter().all(|n| n == node || targets.contains(n));
        if !is_clique {
            break
        }
    }

    return is_clique;
}

fn find_largest_clique (nodes: &HashSet<[char; 2]>, graph: &HashMap<[char; 2], HashSet<[char; 2]>>, largest_known_clique: usize) -> HashSet<[char; 2]> {
    let clique_size = nodes.len();
    // println!("testing clique of size: {}", clique_size);
    let mut largest_clique_size = largest_known_clique;
    let mut largest_clique = HashSet::new();

    if nodes.len() <= largest_clique_size {
        return largest_clique;
    }

    if is_clique(&nodes, &graph) {
        return nodes.clone();
    }

    for node in nodes {
        let mut possible_clique = nodes.clone();
        possible_clique.remove(node);
        let largest_subclique = find_largest_clique(&possible_clique, graph, largest_clique_size);
        if largest_subclique.len() > largest_clique_size {
            largest_clique_size = largest_subclique.len();
            largest_clique = largest_subclique;
            // println!("current largest clique size: {}", largest_clique_size);
            // print_clique(&largest_clique);
        }
    }

    return largest_clique;
}

fn print_clique (clique: &HashSet<[char; 2]>) {
    let mut clique_vec = clique.iter().map(|chars| chars.iter().collect::<String>()).collect::<Vec<String>>();
    clique_vec.sort();
    println!("{:?}", clique_vec.join(","))
}

fn parse_input () -> Vec<Connection> {
    let mut connections = vec![];

    for line in fs::read_to_string(FILENAME).unwrap().lines() {
        // secrets.push(line.parse::<i64>().unwrap())
        connections.push(Connection::from_input(line));
    }

    return connections;
}

#[derive(PartialEq)]
#[derive(Eq)]
#[derive(Clone)]
#[derive(Debug)]
#[derive(Hash)]
struct Connection {
    c1: [char; 2],
    c2: [char; 2]
}

impl Connection {
    fn from_input (input: &str) -> Connection {
        let sections = input.split('-').collect::<Vec<&str>>();
        let c1 = sections[0].chars().collect::<Vec<char>>();
        let c2 = sections[1].chars().collect::<Vec<char>>();

        let ca = [c1[0], c1[1]];
        let cb = [c2[0], c2[1]];

        return Connection {
            c1: min(ca, cb),
            c2: max(ca, cb)
        }
    }
}

#[derive(Debug)]
struct LAN {
    nodes: HashSet<[char; 2]>,
    connections: HashSet<Connection>,
    size: usize
}

impl LAN {
    fn new (size: usize) -> LAN {
        return LAN {
            nodes: HashSet::new(),
            connections: HashSet::new(),
            size
        }
    }

    fn insert (&self, conn: &Connection) -> LAN {
        let mut new_nodes = self.nodes.clone();
        let mut new_conns = self.connections.clone();
        new_nodes.insert(conn.c1);
        new_nodes.insert(conn.c2);
        new_conns.insert(conn.clone());

        return LAN {
            nodes: new_nodes,
            connections: new_conns,
            size: self.size
        }
    }

    fn with_conn (size: usize, conn: &Connection) -> LAN {
        let mut lan = LAN::new(size);
        lan.connections.insert(conn.clone());
        lan.nodes.insert(conn.c1);
        lan.nodes.insert(conn.c2);

        return lan
    }

    fn try_insert (&self, conn: &Connection) -> Option<LAN> {
        if self.connections.contains(conn) {
            return None;
        }

        if self.nodes.contains(&conn.c1) && self.nodes.contains(&conn.c2) {
            return Some(self.insert(conn));
        }

        if self.nodes.len() == self.size {
            return None
        }

        if self.nodes.contains(&conn.c1) || self.nodes.contains(&conn.c2) {
            return Some(self.insert(conn));
        }

        return None
    }

    fn expand (&self) -> LAN {
        return LAN {
            nodes: self.nodes.clone(),
            connections: self.connections.clone(),
            size: self.size + 1
        }
    }

    fn len (&self) -> usize {
        return self.nodes.len()
    }
    
    fn is_complete (&self) -> bool {
        return self.len() == self.size && self.connections.len() == (self.size * (self.size - 1)) / 2;
    }
}