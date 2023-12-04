use std::collections::VecDeque;

fn get_input() -> Vec<Vec<i64>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| l.split(',').map(|s| s.parse().unwrap()).collect())
        .collect()
}

fn part1(input: &[Vec<i64>]) -> usize {
    let mut set = std::collections::HashSet::new();
    let mut overlaps = 0;
    for p in input {
        overlaps += [
            (0, 0, 1),
            (0, 0, -1),
            (0, 1, 0),
            (0, -1, 0),
            (1, 0, 0),
            (-1, 0, 0),
        ]
        .iter()
        .filter(|(x, y, z)| set.contains(&(x + p[0], y + p[1], z + p[2])))
        .count();
        set.insert((p[0], p[1], p[2]));
    }
    input.len() * 6 - 2 * overlaps
}

fn part2(input: &[Vec<i64>]) -> usize {
    let mut set = std::collections::HashMap::<(i64,i64,i64),bool>::from_iter(input.iter().map(|v|((v[0],v[1],v[2]),true)));
    let adjacent = [
        (0, 0, 1),
        (0, 0, -1),
        (0, 1, 0),
        (0, -1, 0),
        (1, 0, 0),
        (-1, 0, 0),
    ];

    //floodfill
    let x_min = set.keys().map(|(x,_,_)| x).min().unwrap() - 1;
    let y_min = set.keys().map(|(_,y,_)| y).min().unwrap() - 1;
    let z_min = set.keys().map(|(_,_,z)| z).min().unwrap() - 1;
    let x_max = set.keys().map(|(x,_,_)| x).max().unwrap() + 1;
    let y_max = set.keys().map(|(_,y,_)| y).max().unwrap() + 1;
    let z_max = set.keys().map(|(_,_,z)| z).max().unwrap() + 1;

    let mut edges = 0;
    let mut fill_queue = vec![(x_min,y_min,z_min)];
    // fill_queue.push_back();
    while let Some(p) = fill_queue.pop() {
        if set.contains_key(&p) {
            continue
        }
        println!("p: {p:?}");
        println!("len set: {}", set.len());
        edges += adjacent.iter().filter(|(x,y,z)| *set.get(&(x + p.0, y + p.1, z + p.2)).unwrap_or(&false)).count();
        set.insert(p,false);
        fill_queue.extend(adjacent.iter().filter(|(x,y,z)| x + p.0 >= x_min && y + p.1 >= y_min && z + p.2 >= z_min && x + p.0 <= x_max && y + p.1 <= y_max && z + p.2 <= z_max && !set.contains_key(&(x + p.0, y + p.1, z + p.2)) ).map(|(x,y,z)| (x + p.0, y + p.1, z + p.2)));
        println!("edges: {edges}");
    }
    edges
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
