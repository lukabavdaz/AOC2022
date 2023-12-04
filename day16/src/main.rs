use std::collections::{HashMap, HashSet};
type Mapping = HashMap<String, (i64, HashMap<String, i64>)>;

fn get_input() -> HashMap<String, (i64, Vec<String>)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(&[' ', ',', ';', '='])
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .map(|v| {
            (
                v[1].to_owned(),
                (
                    v[5].parse().unwrap(),
                    v[10..].iter().map(|&s| s.to_owned()).collect(),
                ),
            )
        })
        .collect()
}

fn connections(start: &str, input: &HashMap<String, (i64, Vec<String>)>) -> HashMap<String, i64> {
    let mut steps = 0;
    let mut distances = HashMap::new();
    let mut q = input.get(start).unwrap().1.clone();
    while !q.is_empty() {
        steps += 1;
        q.retain(|s| !distances.contains_key(s) && s != start);
        distances.extend(q.iter().map(|s| (s.clone(), steps)));
        q = q
            .iter()
            .flat_map(|s| input.get(s).unwrap().1.clone())
            .collect();
    }
    distances
        .into_iter()
        .filter(|(s, _)| input.get(s).unwrap().0 > 0)
        .collect()
}

fn get_mapping(input: &HashMap<String, (i64, Vec<String>)>) -> Mapping {
    input
        .iter()
        .filter(|&(s, (r, _))| r > &0 || s == "AA")
        .map(|(s, (r, v))| (s.clone(), (*r, connections(s, input))))
        .collect()
}

fn get_value_p2(t: i64, ps: Vec<(String, i64)>, visited: HashSet<String>, mapping: &Mapping) -> i64 {
    if t == 0 {
        return 0;
    }
    if let Some(i) = ps.iter().position(|&(_, w)| w == 0 ) {
        mapping
            .get(&ps[i].0)
            .unwrap()
            .1
            .iter()
            .filter(|&(s, &d)| !visited.contains(s) && d < t - 1)
            .map(|(s, &d)| {
                let mut new_ps = ps.clone();
                new_ps[i] = (s.clone(), d + 1);
                mapping.get(s).unwrap().0 * (t - d - 1)
                    + get_value_p2(
                    t,
                    new_ps,
                    &visited | &HashSet::from([s.clone()]),
                    mapping,
                )
            })
            .max()
            .unwrap_or(0)
    } else {
        //take a bigger step than 1? depends on largest w
        get_value_p2(t-1, ps.iter().map(|(p,w)| (p.clone(),w-1) ).collect(), visited, mapping)
    }
}

fn get_value_p1(t: i64, p: String, visited: HashSet<String>, mapping: &Mapping) -> i64 {
    if t == 0 {
        return 0;
    }
    mapping
        .get(&p)
        .unwrap()
        .1
        .iter()
        .filter(|&(s, &d)| !visited.contains(s) && d < t - 1)
        .map(|(s, &d)| {
            mapping.get(s).unwrap().0 * (t - d - 1)
                + get_value_p1(
                    t - d - 1,
                    s.clone(),
                    &visited | &HashSet::from([s.clone()]),
                    mapping,
                )
        })
        .max()
        .unwrap_or(0)
}

fn part1(input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
    let mapping = get_mapping(input);
    // get_value_p1(30, "AA".to_owned(), HashSet::new(), &mapping)
    //doesn't work if "AA" has a rate!
    get_value_p2(30, vec![("AA".to_owned(), 0)], HashSet::new(), &mapping)
}

fn part2(input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
    let mapping = get_mapping(input);
    //doesn't work if "AA" has a rate!
    get_value_p2(26, vec![("AA".to_owned(), 0), ("AA".to_owned(), 0)], HashSet::new(), &mapping)
}

// fn get_value(state, visited) -> i64 {
//     //get value of all the options
//     state.options.map(get_value).max() //with/without caching? probably fine without, no?
// }

// fn part2(input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
//     get_value(????)
// }

// fn get_value(t: i64, nodes: Vec<usize>, visited: &mut HashMap<usize>, rates:, connections: ) -> i64 {
//
// }

// fn part2() -> i64 {
//     get_value(26, vec![0,0], &mut HashMap::new(), )
// }

fn main() {
    let input = get_input();
    // println!("input: {:?}", input);
    // println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}

// let start_and_nonzero = input.iter().filter(|(s,&(rate,_))| rate > 0).map(|(s,&(rate,_))| rate ).collect();

//dijkstra to find map of distances: adds rate & vec of connections
//how to order properly?

// let non_zero_rates = input.iter().filter(|(_,&(rate,))| rate > 0).map(|(_,&(rate, _))| rate ).collect();
// let mut connections = HashMap::new();
// for (&from, &(rate, to)) in input {
//     if rate != 0 {
//         *connections.entry(from.to_owned()).or_insert((rate, vec![])).0 = rate;
//     }
//
//     *connections.entry(to.to_owned()).or_insert((0, vec![])).1
//
// }

//include the starting node at index 0?
//dijkstra for all nodes? shouldn't be that hard right?

// (rates, connections)

// fn get_input() -> HashMap<String, (i64, Vec<String>)> {
//     std::fs::read_to_string("input/test_input.txt")
//         .unwrap()
//         .lines()
//         .map(|l| {
//             l.split(&[' ', ',', ';', '='])
//                 .filter(|s| !s.is_empty())
//                 .collect::<Vec<_>>()
//         })
//         .map(|v| {
//             (
//                 v[1].to_owned(),
//                 (
//                     v[5].parse().unwrap(),
//                     v[10..].iter().map(|&s| s.to_owned()).collect(),
//                 ),
//             )
//         })
//         .collect()
// }

// #[derive(Clone, Hash, Eq, PartialEq, Debug)]
// struct State {
//     t_left: i64,
//     loc: String,
//     on: Vec<String>
// }
//
// #[derive(Clone, Hash, Eq, PartialEq, Debug)]
// struct State_p2 {
//     t_left: i64,
//     loc: Vec<String>,
//     on: Vec<String>
// }
//
//
// fn get_value(s: &State, cache: &mut HashMap<State,i64>, input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
//     if s.t_left == 0 {
//         0
//     } else if let Some(&v) = cache.get(s) {
//         v
//     } else {
//         let (rate, new_loc) = input.get(&s.loc).unwrap();
//         let mut possible_states: Vec<_> = new_loc.iter().map(|loc| (State { t_left: &s.t_left-1, loc: loc.clone(), on: s.on.clone()}, 0)).collect();
//         if rate != &0 && !s.on.contains(&s.loc) {
//             let mut new_on = s.on.clone();
//             new_on.push(s.loc.clone());
//             possible_states.push((State {t_left: &s.t_left-1, loc: s.loc.clone(), on: new_on}, (s.t_left-1) * rate))
//         }
//         let v = possible_states.iter().map(|(s,v)| get_value(s,cache,input) + v).max().unwrap();
//         cache.insert(s.clone(),v);
//         v
//     }
// }
//
// fn get_value_p2(s: &State_p2, cache: &mut HashMap<State_p2,i64>, input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
//     if s.t_left == 0 {
//         0
//     } else if s.t_left < 10 && s.on.len() < 8 {
//         0
//     } else if let Some(&v) = cache.get(s) {
//         v
//     } else {
//         let (rate0, new_l0) = input.get(&s.loc[0]).unwrap();
//         let (rate1, new_l1) = input.get(&s.loc[1]).unwrap();
//         let mut possible_states = vec![];
//         for nl0 in new_l0 {
//             for nl1 in new_l1 {
//                 let mut ll = vec![nl0.clone(),nl1.clone()];
//                 ll.sort();
//                 possible_states.push((State_p2 { t_left: s.t_left-1, loc: ll, on: s.on.clone()}, 0));
//             }
//         }
//         if rate0 != &0 && !s.on.contains(&s.loc[0]) {
//             let mut new_on = s.on.clone();
//             new_on.push(s.loc[0].clone());
//             for nl1 in new_l1 {
//                 let mut ll = vec![s.loc[0].clone(),nl1.clone()];
//                 ll.sort();
//                 possible_states.push((State_p2 {t_left: s.t_left-1, loc: ll, on: new_on.clone()}, (s.t_left-1) * rate0))
//             }
//         }
//         if s.loc[0] != s.loc[1] && rate1 != &0 && !s.on.contains(&s.loc[1]) {
//             let mut new_on = s.on.clone();
//             new_on.push(s.loc[1].clone());
//             for nl0 in new_l0 {
//                 let mut ll = vec![s.loc[1].clone(),nl0.clone()];
//                 ll.sort();
//                 possible_states.push((State_p2 {t_left: s.t_left-1, loc: ll, on: new_on.clone()}, (s.t_left-1) * rate1))
//             }
//             if rate0 != &0 && !s.on.contains(&s.loc[0]) {
//                 new_on.push(s.loc[0].clone());
//                 possible_states.push((State_p2 {t_left: s.t_left-1, loc: s.loc.clone(), on: new_on.clone()}, (s.t_left-1) * (rate0 + rate1)))
//             }
//         }
//         // println!("possible states: {:?}", possible_states);
//         let v = possible_states.iter().map(|(s,v)| get_value_p2(s,cache,input) + v).max().unwrap();
//         cache.insert(s.clone(),v);
//         println!("cache len: {}/2768044032, on len: {}, time: {}", cache.len(), s.on.len(), s.t_left);
//         v
//     }
// }
//
//
// fn part1(input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
//     get_value(&State {t_left: 30, loc: "AA".to_owned(), on: vec![]}, &mut HashMap::new(), input)
// }
//
// fn part2(input: &HashMap<String, (i64, Vec<String>)>) -> i64 {
//     get_value_p2(&State_p2 {t_left: 26, loc: vec!["AA".to_owned(),"AA".to_owned()], on: vec![]}, &mut HashMap::new(), input)
// }
//
// fn main() {
//     let input = get_input();
//     // println!("{:?}",input.len());
//     // println!("part1: {}", part1(&input));
//     println!("part2: {}", part2(&input));
// }
