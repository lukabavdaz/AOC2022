use std::collections::HashSet;

fn get_input() -> Vec<char> {
    std::fs::read_to_string("input/input.txt").unwrap().trim().chars().collect()
}

fn get_shape(i: usize) -> Vec<(i64,i64)> {
    match i%5 {
        0 => vec![(0,0),(1,0),(2,0),(3,0)],
        1 => vec![(1,0),(0,1),(1,1),(2,1),(1,2)],
        2 => vec![(0,0),(1,0),(2,0),(2,1),(2,2)],
        3 => vec![(0,0),(0,1),(0,2),(0,3)],
        _ => vec![(0,0),(0,1),(1,0),(1,1)]
    }
}

// fn part1(input: &[char]) -> i64 {
//     let mut cave = std::collections::HashSet::new();
//     let mut jets = input.iter().cycle();
//     let mut rocks = 0;
//     let mut rock = get_shape(0);
//     for (x,y) in &mut rock {
//         *y += 3 + 1;
//         *x += 2;
//     }
//     while rocks < 2022 {
//         // println!("{rock:?}");
//         // println!("{cave:?}");
//         let m = match jets.next().unwrap() {
//             '<' => -1,
//             _ => 1,
//         };
//         if rock.iter().map(|(x,y)| (x+m,*y)).all(|p| p.0 >= 0 && p.0 < 7 && !cave.contains(&p) ) {
//             for (x,_) in &mut rock {
//                 *x += m;
//             }
//         }
//         if rock.iter().map(|(x,y)| (*x,y-1)).all(|p| p.1 > 0 && !cave.contains(&p) ) {
//             for (_,y) in &mut rock {
//                 *y -= 1;
//             }
//         } else {
//             rocks += 1;
//             for r in rock {
//                 cave.insert(r);
//             }
//             rock = get_shape(rocks);
//             let y_max = cave.iter().map(|(_,y)| y).max().unwrap();
//             for (x,y) in &mut rock {
//                 *y += y_max + 3 + 1;
//                 *x += 2;
//             }
//         }
//     }
//     // println!("{cave:?}");
//     println!("{}", cave.iter().filter(|(x,y)| y==&6).count());
//     *cave.iter().map(|(_,y)| y).max().unwrap()
// }

fn part2(input: &[char]) -> i64 {
    let mut cave: HashSet<(i64,i64)> = std::collections::HashSet::new();
    let mut jets = input.iter().cycle().enumerate();
    let mut rocks = 0;
    let mut rock = get_shape(0);
    for (x,y) in &mut rock {
        *y += 3 + 1;
        *x += 2;
    }
    while rocks < (1748 + 1007) {//1000000000000 {
        // println!("{rock:?}");
        // println!("{cave:?}");
        let (i, c) = jets.next().unwrap();
        let cave_height = *cave.iter().map(|(_,y)| y).max().unwrap_or(&0);
        let hash: i64 = (0..7).map(|j| 7_i64.pow(j) * (cave_height - cave.iter().filter(|&&p| p.0 == j.into()).map(|p| p.1).max().unwrap_or(0))).sum();
        // println!("{}: {}", i/input.len(), hash);
        if i%input.len() == 0 {
            // println!("{}: rocks: {}", i/input.len(), rocks);
            // println!("{}:\trocks: {}\theight: {}", i/input.len(), rocks, cave.iter().map(|(_,y)| y).max().unwrap_or(&0));
            // let hash: i64 = (0..7).map(|j| 7_i64.pow(j) * cave.iter().filter(|&&p| p.0 == j.into()).map(|p| p.0).max().unwrap_or(0)).sum();
            println!("{}:\trocks: {}\theight: {}\thash: {}\tcurrent_rock: {}", i/input.len(), rocks, cave_height, hash, rocks%5);
        }
        let m = match c {
            '<' => -1,
            _ => 1,
        };
        if rock.iter().map(|(x,y)| (x+m,*y)).all(|p| p.0 >= 0 && p.0 < 7 && !cave.contains(&p) ) {
            for (x,_) in &mut rock {
                *x += m;
            }
        }
        if rock.iter().map(|(x,y)| (*x,y-1)).all(|p| p.1 > 0 && !cave.contains(&p) ) {
            for (_,y) in &mut rock {
                *y -= 1;
            }
        } else {
            rocks += 1;
            for r in rock {
                cave.insert(r);
            }
            rock = get_shape(rocks);
            let y_max = cave.iter().map(|(_,y)| y).max().unwrap();
            for (x,y) in &mut rock {
                *y += y_max + 3 + 1;
                *x += 2;
            }
            //remove bottom of cave

        }
    }
    // println!("{cave:?}");
    *cave.iter().map(|(_,y)| y).max().unwrap()
}


fn main() {
    let input = get_input();
    // println!("{}", input.len());
    // println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
    // 1591977075756 + 4368
    //1591977072978 +
}
