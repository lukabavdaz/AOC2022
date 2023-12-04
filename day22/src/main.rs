fn get_input() -> (Vec<Vec<char>>, Vec<String>) {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let mut parts = file.split("\n\n");
    let mut map: Vec<Vec<char>> = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let path = parts
        .next()
        .unwrap()
        .split_inclusive(char::is_alphabetic)
        .map(|s| s.split_at(s.len() - 1))
        .flat_map(|s| [s.0.trim().to_owned(), s.1.trim().to_owned()])
        .filter(|s| !s.is_empty())
        .collect();
    let map_x_size = map.iter().map(|v| v.len() ).max().unwrap();
    for v in &mut map {
        v.resize(map_x_size, ' ');
    }
    (map, path)
}

// fn part1((map, path): &(Vec<Vec<char>>, Vec<String>)) -> usize {
//     let mut p = (map[0].iter().position(|c| !c.is_whitespace()).unwrap(), 0);
//     let mut direction = 0;
//     for c in path {
//         match &c[..] {
//             "R" => direction = (direction + 1) % 4,
//             "L" => direction = (direction + 3) % 4,
//             n => {
//                 let num = n.parse().unwrap();
//                 'outer: for _ in 0..num {
//                     let mut new_p = p;
//                     'inner: loop {
//                         let d = match direction {
//                             0 => (1,0),
//                             1 => (0,1),
//                             2 => (-1,0),
//                             _ => (0,-1),
//                         };
//
//                         new_p.1 = (new_p.1 as i64 + d.1).rem_euclid(map.len() as i64) as usize; //this is wrong wrapping for part of the grid!
//                         new_p.0 = (new_p.0 as i64 + d.0).rem_euclid(map[new_p.1].len() as i64) as usize;
//
//                         match map[new_p.1][new_p.0] {
//                             '.' => {
//                                 p = new_p;
//                                 println!("{:?}", p);
//                                 break 'inner;
//                             },
//                             '#' => break 'outer,
//                             _ => (),
//                         }
//                     }
//                 }
//             }
//         }
//     }
//     (p.1 + 1) * 1000 + (p.0 + 1) * 4 + direction
// }

fn wrap_p_direction(p: (i64,i64), direction: usize) -> ((usize, usize), usize) {
    let ((a,b),c) = match (p.0,p.1) {
        (x, -1) if x >= 50 && x < 100 => ((0,x+100), 0),
        (x, -1) if x >= 100 && x < 150 => ((x-100, 199) , 3),
        (49, y) if y < 50 => ((0,149-y) , 0),
        (49, y) if y >= 50 && y < 100 => ((y-50,100) , 1),
        (150, y) if y < 50 => ((99,149-y), 2),
        (x, 50) if x >= 100 && x < 150 => ((99, x-50), 2),
        (100, y) if y >= 50 && y < 100 => ((y + 50,49), 3),
        (100, y) if y >= 100 && y < 150 => ((149, 149-y), 2),
        (x, 99) if x < 50 => ((50,50+x), 1),
        (-1, y) if y >= 100 && y < 150 => ((50,149-y), 0),
        (-1, y) if y >= 150 && y < 200 => ((y - 100,0), 1),
        (x, 150) if x >= 50 && x < 100 => ((49, x+100), 2),
        (50, y) if y >= 150 && y < 200 => ((y-100, 149),2),
        (x, 200) if x < 50 => ((x+100, 0),1),
        (x, y) => ((x,y),direction),
    };
    if a < 0 || b < 0 {
        println!("a: {} b: {} c: {}",a,b,c);
    }
    ((a as usize, b as usize), c)
}

fn part2((map, path): &(Vec<Vec<char>>, Vec<String>)) -> usize {
    let mut p = (map[0].iter().position(|c| !c.is_whitespace()).unwrap(), 0);
    let mut direction = 0;
    for c in path {
        match &c[..] {
            "R" => direction = (direction + 1) % 4,
            "L" => direction = (direction + 3) % 4,
            n => {
                let num = n.parse().unwrap();
                'outer: for _ in 0..num {
                    let mut new_p = p;
                    let mut new_direction = direction;
                    'inner: loop {
                        let d = match direction {
                            0 => (1,0),
                            1 => (0,1),
                            2 => (-1,0),
                            _ => (0,-1),
                        };

                        (new_p, new_direction) = wrap_p_direction((new_p.0 as i64 + d.0, new_p.1 as i64 + d.1), new_direction);

                        match map[new_p.1][new_p.0] {
                            '.' => {
                                p = new_p;
                                direction = new_direction;
                                break 'inner;
                            },
                            '#' => break 'outer,
                            _ => (),
                        }
                    }
                }
            }
        }
    }
    (p.1 + 1) * 1000 + (p.0 + 1) * 4 + direction
}

fn main() {
    let input = get_input();
    // println!("{:?}", input.0.len());
    // println!("part1: {}", part1(&input));
    // let input = get_input2();
    println!("part2: {}", part2(&input));
}
