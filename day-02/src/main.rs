use std::{cmp, fs};

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let result1 = content
        .split('\n')
        .map(transform_to_game_info)
        .filter(|x| x.red <= 12 && x.green <= 13 && x.blue <= 14)
        .map(|x| x.id)
        .sum::<i32>();

    println!("Part 01: {}", result1);
    assert!(2283 == result1);

    let result2 = content
        .split('\n')
        .map(transform_to_game_info)
        .map(|x| x.power())
        .sum::<i32>();

    println!("Part 02: {}", result2);
    assert!(78669 == result2);
}

fn transform_to_game_info(line: &str) -> GameInfo {
    let index = line.find(':').unwrap();
    let game_id = &line[5..index];
    let mut game_info = GameInfo::new(game_id.parse::<i32>().unwrap());

    let reveals = line[index + 1..]
        .split([';', ','])
        .map(|x| x.trim())
        .map(|x| x.split(' '));

    for mut reveal in reveals {
        let number = reveal.next().unwrap().parse::<i32>().unwrap();
        match reveal.next().unwrap() {
            "red" => game_info.red = cmp::max(number, game_info.red),
            "green" => game_info.green = cmp::max(number, game_info.green),
            _ => game_info.blue = cmp::max(number, game_info.blue),
        }
    }

    game_info
}

struct GameInfo {
    id: i32,
    red: i32,
    green: i32,
    blue: i32,
}

impl GameInfo {
    fn new(id: i32) -> GameInfo {
        GameInfo {
            id,
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    fn power(&self) -> i32 {
        self.red * self.green * self.blue
    }
}
