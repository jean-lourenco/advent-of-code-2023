use std::{
    collections::{HashMap, HashSet},
    fs,
};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let matrix = content
        .lines()
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let (result1, result2) = solve_part_number_and_gear_ratio(&matrix);
    println!("Part 1: {}", result1);
    assert!(512794 == result1);

    println!("Part 2: {}", result2);
    assert!(67779080 == result2);
}

// This is a very ugly solution
// It'd probably be best to parse the values instead of processing everything in one pass
fn solve_part_number_and_gear_ratio(matrix: &Vec<Vec<char>>) -> (i32, i32) {
    let mut y = 0;
    let mut x = 0;
    let mut sum = 0;
    let mut gears: HashMap<(usize, usize), Vec<i32>> = HashMap::new();

    while y < matrix.len() {
        let line = &matrix[y];
        while x < line.len() {
            let mut current = &line[x];
            if current.is_ascii_digit() {
                let mut number = String::new();
                let mut has_symbol = false;
                let mut stars: HashSet<(usize, usize)> = HashSet::new();

                while x < line.len() && current.is_ascii_digit() {
                    number.push(*current);
                    has_symbol = has_symbol
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, -1, x, -1))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, -1, x, 0))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, -1, x, 1))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, 0, x, -1))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, 0, x, 1))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, 1, x, -1))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, 1, x, 0))
                        || is_symbol(get_safe_index(&mut stars, &matrix, y, 1, x, 1));

                    x += 1;
                    if x < line.len() {
                        current = &line[x];
                    }
                }
                if has_symbol {
                    let parsed = number.parse::<i32>().unwrap();
                    sum += parsed;

                    for star in stars {
                        if gears.contains_key(&(star.0, star.1)) {
                            gears.get_mut(&(star.0, star.1)).unwrap().push(parsed);
                        } else {
                            gears.insert((star.0, star.1), vec![parsed]);
                        }
                    }
                }
            } else {
                x += 1;
            }
        }
        x = 0;
        y += 1;
    }
    let gear_ratio = gears
        .values()
        .filter(|x| x.len() == 2)
        .map(|x| x[0] * x[1])
        .sum::<i32>();
    (sum, gear_ratio)
}

fn get_safe_index(
    stars: &mut HashSet<(usize, usize)>,
    matrix: &Vec<Vec<char>>,
    y: usize,
    mod_y: i32,
    x: usize,
    mod_x: i32,
) -> char {
    let y = y as i32 + mod_y;
    let x = x as i32 + mod_x;
    if y >= matrix.len() as i32 || x >= matrix[0].len() as i32 || y < 0 || x < 0 {
        return '.';
    }
    let c = matrix[y as usize][x as usize];
    if c == '*' {
        stars.insert((y as usize, x as usize));
    }
    c
}

fn is_symbol(c: char) -> bool {
    !c.is_ascii_digit() && c != '.'
}
