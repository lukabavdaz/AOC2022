use std::cmp::Ordering;
use std::iter::Peekable;

fn get_input() -> Vec<Vec<List>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.lines()
                .map(|l| parse_list(&mut l.chars().skip(1).peekable()))
                .collect()
        })
        .collect()
}

#[derive(Clone)]
enum List {
    Lists(Vec<List>),
    Number(i64),
}

fn parse_list<T: Iterator<Item = char>>(it: &mut Peekable<T>) -> List {
    let mut content = vec![];
    while let Some(c) = it.next() {
        match (c, it.peek()) {
            ('[', _) => content.push(parse_list(it)),
            (']', _) => return List::Lists(content),
            (',', _) => (),
            ('1', Some(&'0')) => content.push(List::Number(10)),
            (n, _) => content.push(List::Number(n.to_digit(10).unwrap().into())),
        }
    }
    panic!()
}

fn order(left: &List, right: &List) -> Ordering {
    return match (left, right) {
        (List::Number(nl), List::Number(nr)) => nl.cmp(&nr),
        (List::Lists(vl), List::Lists(vr)) => vl
            .iter()
            .zip(vr)
            .map(|(l, r)| order(l, r))
            .find(|o| o.is_ne())
            .unwrap_or(vl.len().cmp(&vr.len())),
        (List::Number(_), r) => order(&List::Lists(vec![left.clone()]), r),
        (l, List::Number(_)) => order(l, &List::Lists(vec![right.clone()])),
    };
}

fn part1(input: &[Vec<List>]) -> usize {
    input
        .iter()
        .enumerate()
        .filter(|(_, v)| order(&v[0], &v[1]).is_le())
        .map(|(i, _)| i + 1)
        .sum()
}

fn part2(input: &[Vec<List>]) -> usize {
    let mut packets: Vec<List> = input.iter().flat_map(|v| v.iter()).cloned().collect();
    let divs = ["[[2]]", "[[6]]"].map(|l| parse_list(&mut l.chars().skip(1).peekable()));
    packets.extend_from_slice(&divs);
    packets.sort_by(order);
    divs.map(|d| packets.iter().position(|l| order(l, &d).is_eq()).unwrap() + 1)
        .iter()
        .product()
}

fn main() {
    let input = get_input();
    println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
