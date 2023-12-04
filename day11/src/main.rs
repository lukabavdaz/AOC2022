fn get_input() -> Vec<Monkey> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| Monkey::new_from_lines(s))
        .collect()
}

#[derive(Debug,Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: Op,
    test: Vec<usize>,
}

#[derive(Debug,Copy,Clone)]
enum Op {
    Mul(i64),
    Sum(i64),
    Sq,
}

impl Monkey {
    fn new_from_lines(s: &str) -> Monkey {
        let mut it = s.lines().skip(1);
        let items = it
            .next()
            .unwrap()
            .split(|c: char| !c.is_numeric())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();
        let operation = match it.next().unwrap().split_once("= old ").unwrap().1.split_once(' ').unwrap() {
            ("*", "old") => Op::Sq,
            ("*", n) => Op::Mul(n.parse().unwrap()),
            ("+", n) => Op::Sum(n.parse().unwrap()),
            _ => panic!()
        };
        let test = it.map(|s| s[s.len()-2..].trim().parse().unwrap()).collect();
        Monkey {
            items,
            operation,
            test,
        }
    }
}

// fn part1(input: &[Monkey]) -> i64 {
//     let mut inspects = vec![0;input.len()];
//
//     let mut monkeys = input.to_vec();
//     for a in 0..20 {
//         // println!("{a}");
//         for m in 0..monkeys.len() {
//             let current_monkey = monkeys[m].clone();
//             for &i in &current_monkey.items {
//                 inspects[m] += 1;
//                 let expr: meval::Expr = current_monkey.operation.parse().unwrap();
//                 let func = expr.bind("old").unwrap();
//                 let new_value = (func(i as f64) / 3.0).floor() as i64;
//                 if new_value%(current_monkey.test[0] as i64) == 0 {
//                     monkeys[current_monkey.test[1]].items.push(new_value);
//                 } else {
//                     monkeys[current_monkey.test[2]].items.push(new_value);
//                 }
//                 // println!("worry level: {i}, value after div: {new_value}");
//             }
//             monkeys[m].items = vec![];
//         }
//     }
//     println!("{:?}", inspects);
//     34
// }

fn part2(input: &[Monkey]) -> i64 {
    let mut inspects = vec![0;input.len()];
    let full_divisor = input.iter().map(|m| m.test[0]).product::<usize>();

    let mut monkeys = input.to_vec();
    for a in 0..10000 {
        println!("{a}");
        for m in 0..monkeys.len() {
            let current_monkey = monkeys[m].clone();
            for i in current_monkey.items {
                inspects[m] += 1;
                let mut new_value = match current_monkey.operation {
                    Op::Sq => i * i,
                    Op::Mul(n) => i * n,
                    Op::Sum(n) => i + n
                };
                new_value %= full_divisor as i64;

                if new_value%current_monkey.test[0] as i64 == 0{
                    monkeys[current_monkey.test[1]].items.push(new_value);
                } else {
                    monkeys[current_monkey.test[2]].items.push(new_value);
                }
                // println!("worry level: {i}, value after div: {new_value}");
            }
            monkeys[m].items = vec![];
        }
    }
    println!("{:?}", inspects);
    34
}

fn main() {
    let input = get_input();
    // println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
