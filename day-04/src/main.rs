use std::{collections::HashMap, fs};

fn main() {
    let result1 = solve_part1();
    println!("Part 1: {}", result1);
    assert!(21105 == result1);

    let result2 = solve_part2();
    println!("Part 2: {}", result2);
    assert!(5329815 == result2);
}

fn parse_lottery_table() -> Vec<(i32, Vec<i32>, Vec<i32>)> {
    fs::read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|x| {
            let mut v = x.split(':');
            (v.next().unwrap(), v.next().unwrap())
        })
        .map(|x| {
            let values = x.1.split('|');
            let parsed: Vec<_> = values
                .map(|y| {
                    y.split(' ')
                        .filter(|z| !z.trim().is_empty())
                        .map(|z| z.trim().parse::<i32>().unwrap())
                        .collect::<Vec<i32>>()
                })
                .collect();
            (
                x.0[5..].trim().parse::<i32>().unwrap(),
                parsed.first().unwrap().clone(),
                parsed.last().unwrap().clone(),
            )
        })
        .collect::<Vec<(i32, Vec<i32>, Vec<i32>)>>()
}

fn solve_part1() -> i32 {
    parse_lottery_table()
        .iter()
        .map(|x| {
            let c =
                x.2.iter()
                    .map(|y| if x.1.contains(y) { 1 } else { 0 })
                    .sum::<u32>();
            if c > 0 {
                2_i32.pow(c - 1)
            } else {
                0
            }
        })
        .sum::<i32>()
}

fn solve_part2() -> i32 {
    let mut i = 0;
    let mut hash: HashMap<i32, i32> = HashMap::new();
    let table = parse_lottery_table();

    while i < table.len() {
        let (id, winning, numbers) = &table[i];
        let won = numbers
            .iter()
            .map(|x| if winning.contains(x) { 1 } else { 0 })
            .sum::<i32>();

        if won > 0 {
            let mut times = 1;
            if let Some(v) = hash.get(&id) {
                times *= *v + 1;
            }

            for j in 1..won + 1 {
                if let Some(v) = hash.get_mut(&(j + id)) {
                    *v += times;
                } else {
                    hash.insert(j + id, times);
                }
            }
        }
        i += 1;
    }
    table.len() as i32 + hash.values().sum::<i32>()
}
