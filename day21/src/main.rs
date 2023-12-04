fn get_input() -> Vec<(String, Yell)> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(' ')
                .map(|s| s.trim_end_matches(':'))
                .collect::<Vec<_>>()
        })
        .map(|v| match v[..] {
            [name, n] => (name.to_owned(), Yell::Number(n.parse().unwrap())),
            [name, n1, c, n2] => (
                name.to_owned(),
                Yell::Operation(
                    n1.to_owned(),
                    n2.to_owned(),
                    match c {
                        "+" => |a, b| a + b,
                        "-" => |a, b| a - b,
                        "*" => |a, b| a * b,
                        "/" => |a, b| a / b,
                        _ => panic!(),
                    },
                ),
            ),
            _ => panic!(),
        })
        .collect()
}

#[derive(Debug)]
enum Yell {
    Number(i64),
    Operation(String, String, fn(i64, i64) -> i64),
}

fn part1(input: &[(String, Yell)], result: &str) -> i64 {
    let mut answers = std::collections::HashMap::new();
    for (s,y) in input.iter().cycle() {
        match y {
            Yell::Number(n) => drop(answers.insert(s,*n)),
            Yell::Operation(s1, s2, op) => {
                match (answers.get(s1), answers.get(s2)) {
                    (Some(n1), Some(n2)) if s == result => return op(*n1,*n2),
                    (Some(n1), Some(n2)) => drop(answers.insert(s, op(*n1,*n2))),
                    _ => ()
                }
            }
        }
    }
    panic!();
}

fn test_number(input: &[(String, Yell)], result: &str, humn: i64) -> i64 {
    let binding = "humn".to_owned();
    let mut answers = std::collections::HashMap::from([(&binding,humn)]);
    for (s,y) in input.iter().cycle() {
        match y {
            Yell::Number(n) if s != "humn" => drop(answers.insert(s,*n)),
            Yell::Operation(s1, s2, op) => {
                match (answers.get(s1), answers.get(s2)) {
                    (Some(n1), Some(n2)) if s == result => return op(*n1,*n2),
                    (Some(n1), Some(n2)) => drop(answers.insert(s, op(*n1,*n2))),
                    _ => ()
                }
            }
            _ => ()
        }
    }
    panic!();
}

fn part2(input: &[(String, Yell)]) -> i64 {
    for i in (3721298272000..3721298273000) {
        println!("i: {}, result: {}", i, test_number(input, "tlpd",i));
    }
    4
    // (0..).step_by(1000).find(|i| test_number(input, *i)).unwrap()
}

fn part2v2(input: &[(String, Yell)]) -> i64 {
    let mut calc_s = "humn";
    while let Some(s) = input.iter().find_map(|(s,y)| match y {
        Yell::Operation(s1, s2, _) if (s1 == calc_s || s2 == calc_s) && s != "root" => Some(s),
        _ => None,
    }) {
        calc_s = s;
    }
    let to_equal = test_number(input, "jjmw",0);


    //reverse calcs???

    println!("to equal: {}", to_equal);

    // println!("s: {}", current_string);
    4
    //find what number humn is not required in?
}

fn main() {
    let input = get_input();
    // println!("part1: {}", part1(&input, "root"));
    println!("part2: {}", part2(&input));
    // println!("{:?}", input);
    // println!("{}", input[0].2(4,3));
    // println!("Hello, world!");
}
