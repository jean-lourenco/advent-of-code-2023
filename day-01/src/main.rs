use std::fs;

static SPELLED: &[(&str, char)] = &[
    ("one", '1'),
    ("two", '2'),
    ("three", '3'),
    ("four", '4'),
    ("five", '5'),
    ("six", '6'),
    ("seven", '7'),
    ("eight", '8'),
    ("nine", '9'),
];

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let digits1 = content
        .split('\n')
        .map(parse_first_last_digits)
        .sum::<i32>();

    assert!(54968 == digits1);
    println!("Part 01: {}", digits1);

    let digits2 = content
        .split('\n')
        .map(parse_spelled_out_digits)
        .sum::<i32>();

    assert!(54094 == digits2);
    println!("Part 02: {}", digits2);
}

fn parse_first_last_digits(s: &str) -> i32 {
    let mut first: char = ' ';
    let mut last: char = ' ';

    for c in s.chars() {
        if !c.is_ascii_digit() {
            continue;
        }

        if first == ' ' {
            first = c;
        }
        last = c;
    }

    format!("{}{}", first, last).parse::<i32>().unwrap()
}

fn parse_spelled_out_digits(s: &str) -> i32 {
    let mut first: char = ' ';
    let mut last: char = ' ';

    for (i, c) in s.chars().enumerate() {
        let tail = &s[i..];

        if c.is_ascii_digit() {
            if first == ' ' {
                first = c;
            }
            last = c;
        } else {
            for (k, v) in SPELLED.iter() {
                if tail.starts_with(k) {
                    if first == ' ' {
                        first = *v;
                    }
                    last = *v;
                    break;
                }
            }
        }
    }

    format!("{}{}", first, last).parse::<i32>().unwrap()
}
