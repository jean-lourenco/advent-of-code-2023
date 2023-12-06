use std::{collections::HashMap, fs, panic::Location};

fn main() {
    let file = fs::read_to_string("input.txt").unwrap();
    let content: Vec<&str> = file.lines().collect();

    let seeds: Vec<i64> = content[0][7..]
        .split(' ')
        .map(|x| x.parse::<i64>().unwrap())
        .collect();

    let mut seed_to_soil: HashMap<i64, i64> = HashMap::new();
    let mut soil_to_fertilizer: HashMap<i64, i64> = HashMap::new();
    let mut fertilizer_to_water: HashMap<i64, i64> = HashMap::new();
    let mut water_to_light: HashMap<i64, i64> = HashMap::new();
    let mut light_to_temperature: HashMap<i64, i64> = HashMap::new();
    let mut temperature_to_humidity: HashMap<i64, i64> = HashMap::new();
    let mut humidity_to_localtion: HashMap<i64, i64> = HashMap::new();

    parse_hashes(
        content,
        &mut seed_to_soil,
        &mut soil_to_fertilizer,
        &mut fertilizer_to_water,
        &mut water_to_light,
        &mut light_to_temperature,
        &mut temperature_to_humidity,
        &mut humidity_to_localtion,
    );

    let result = seeds
        .iter()
        .map(|x| {
            if let Some(soil) = seed_to_soil.get(x) {
                return soil;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(fetilizer) = soil_to_fertilizer.get(x) {
                return fetilizer;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(water) = fertilizer_to_water.get(x) {
                return water;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(light) = water_to_light.get(x) {
                return light;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(temperature) = light_to_temperature.get(x) {
                return temperature;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(humidity) = temperature_to_humidity.get(x) {
                return humidity;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(location) = humidity_to_localtion.get(x) {
                return location;
            } else {
                return x;
            }
        })
        .min()
        .unwrap();

    let result1 = humidity_to_localtion
        .values()
        .
        .iter()
        .map(|x| {
            if let Some(location) = humidity_to_localtion.get(x) {
                return location;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(humidity) = temperature_to_humidity.get(x) {
                return humidity;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(temperature) = light_to_temperature.get(x) {
                return temperature;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(light) = water_to_light.get(x) {
                return light;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(water) = fertilizer_to_water.get(x) {
                return water;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(fetilizer) = soil_to_fertilizer.get(x) {
                return fetilizer;
            } else {
                return x;
            }
        })
        .map(|x| {
            if let Some(soil) = seed_to_soil.get(x) {
                return soil;
            } else {
                return x;
            }
        });

    println!("Result 1: {}", result);
}

fn parse_hashes(
    content: Vec<&str>,
    seed_to_soil: &mut HashMap<i64, i64>,
    soil_to_fertilizer: &mut HashMap<i64, i64>,
    fertilizer_to_water: &mut HashMap<i64, i64>,
    water_to_light: &mut HashMap<i64, i64>,
    light_to_temperature: &mut HashMap<i64, i64>,
    temperature_to_humidity: &mut HashMap<i64, i64>,
    humidity_to_localtion: &mut HashMap<i64, i64>,
) {
    let mut current: &mut HashMap<i64, i64> = seed_to_soil;
    for x in &content[2..] {
        if x.is_empty() {
            continue;
        }

        if !x.chars().next().unwrap().is_ascii_digit() {
            current = match x {
                &"seed-to-soil map:" => seed_to_soil,
                &"soil-to-fertilizer map:" => soil_to_fertilizer,
                &"fertilizer-to-water map:" => fertilizer_to_water,
                &"water-to-light map:" => water_to_light,
                &"light-to-temperature map:" => light_to_temperature,
                &"temperature-to-humidity map:" => temperature_to_humidity,
                _ => humidity_to_localtion,
            };
            continue;
        }

        let parsed: Vec<_> = x.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let (dest, source, range) = (parsed[0], parsed[1], parsed[2]);
        for i in 0..range {
            current.insert(source + i, dest + i);
        }
    }
}
