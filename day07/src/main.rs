use std::collections::HashMap;

fn get_input() -> Dir {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let mut dir: Dir = Default::default();
    dir.extend_from_iter(&mut file.lines().skip(1));
    dir
}

#[derive(Default)]
struct Dir {
    dirs: HashMap<String, Dir>,
    files: HashMap<String, i64>,
}

impl Dir {
    fn extend_from_iter<'a, T: Iterator<Item = &'a str>>(&mut self, it: &mut T) {
        while let Some(s) = it.next() {
            match s.split_once(' ').unwrap() {
                ("$", "cd ..") => return,
                ("$", "ls") => (),
                ("$", cd_name) => {
                    self.dirs
                        .entry(cd_name.split_once(' ').unwrap().1.to_owned())
                        .or_default()
                        .extend_from_iter(it);
                }
                ("dir", _) => (),
                (size, name) => {
                    self.files.insert(name.to_owned(), size.parse().unwrap());
                }
            }
        }
    }

    fn value(&self) -> i64 {
        self.dirs.values().map(|d| d.value()).sum::<i64>() + self.files.values().sum::<i64>()
    }

    fn capped_value_sum(&self, total: &mut i64) -> i64 {
        let value = self.dirs.values().map(|d| d.capped_value_sum(total)).sum::<i64>() + self.files.values().sum::<i64>();
        if value < 100000 {
            *total += value;
        }
        value
    }

    fn largest_with_max(&self, max: i64, largest: &mut i64) -> i64 {
        let value = self.dirs.values().map(|d| d.largest_with_max(max, largest)).sum::<i64>() + self.files.values().sum::<i64>();
        if value > max {
            *largest = value.min(*largest);
        }
        value
    }
}

fn main() {
    let input = get_input();
    // println!("{input:#?}");
    let mut total = 0;
    input.capped_value_sum(&mut total);
    println!("part1: {}", total);

    let max = 70000000 - input.value();
    let mut largest = 70000000;
    input.largest_with_max(30000000 - max, &mut largest);
    println!("part2: {}", largest);

}
