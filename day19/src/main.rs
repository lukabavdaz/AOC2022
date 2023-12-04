fn get_input() -> Vec<Vec<i64>> {
    std::fs::read_to_string("input/input.txt")
        .unwrap()
        .lines()
        .map(|l| {
            l.split(|c: char| !c.is_numeric())
                .filter_map(|s| s.parse().ok())
                .collect()
        })
        .collect()
}

fn get_geodes(t: i64, res: &[i64], rob: &[i64], buy: &[bool], cost: &[i64]) -> i64 {
    if t == 0 {
        return res[3];
    }

    let mut res_vec = [res.to_vec(), res.to_vec(), res.to_vec(), res.to_vec()];
    res_vec[0][0] -= cost[1];
    res_vec[1][0] -= cost[2];
    res_vec[2][0] -= cost[3];
    res_vec[2][1] -= cost[4];
    res_vec[3][0] -= cost[5];
    res_vec[3][2] -= cost[6];

    let max_rob = [*[cost[1],cost[2],cost[3],cost[5]].iter().max().unwrap(), cost[4], cost[6]];

    let max_buy = res_vec
        .iter()
        .enumerate()
        .filter(|&(i, r)| buy[i] && r.iter().all(|&x| x >= 0) && (i == 3 || rob[i] < max_rob[i]) )
        .map(|(i, r)| {
            let new_res = r.iter().zip(rob).map(|(a, b)| a + b).collect::<Vec<_>>();
            let mut new_rob = rob.to_vec();
            new_rob[i] += 1;
            get_geodes(t - 1, &new_res, &new_rob, &[true; 4], cost)
        })
        .max();

    let new_buy = res_vec.iter().map(|r| !r.iter().all(|&x| x >= 0)).collect::<Vec<_>>();
    let new_res = res.iter().zip(rob).map(|(a, b)| a + b).collect::<Vec<_>>();

    max_buy.max(Some(get_geodes(t - 1, &new_res, rob, &new_buy, cost))).unwrap()
}

fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|v| get_geodes(24, &[0, 0, 0, 0], &[1, 0, 0, 0], &[true; 4], v))
        .enumerate()
        .map(|(i, x)| (i as i64 + 1) * x)
        .sum()
}

fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .take(3)
        .map(|v| get_geodes(32, &[0, 0, 0, 0], &[1, 0, 0, 0], &[true; 4], v))
        .inspect(|x| println!("{x}"))
        .product()
}

fn main() {
    let input = get_input();
    // println!("part1: {}", part1(&input));
    println!("part2: {}", part2(&input));
}
