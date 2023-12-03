static LINES: [&str; 5] = [
    "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
    "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
    "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
    "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
    "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
];

struct Bag {
    red: u32,
    green: u32,
    blue: u32,
}

fn cube_set_power(bag: &Bag) -> u32 {
    let mut product = 1;
    product *= 1.max(bag.red);
    product *= 1.max(bag.green);
    product *= 1.max(bag.blue);
    product
}

pub fn main() {
    let mut sum = 0;

    for line in LINES.iter() {
        let subsets = line.split(":").nth(1);
        let mut bag: Bag = Bag {
            red: 0,
            green: 0,
            blue: 0,
        };
        for subset in subsets.unwrap().split(";") {
            for cubes in subset.trim().split(", ") {
                let parts = cubes.split(" ").collect::<Vec<&str>>();
                if parts.len() == 2 {
                    let (count, color) = (parts[0], parts[1]);
                    match color {
                        "red" => bag.red = bag.red.max(count.parse().unwrap()),
                        "green" => bag.green = bag.green.max(count.parse().unwrap()),
                        "blue" => bag.blue = bag.blue.max(count.parse().unwrap()),
                        _ => (),
                    }
                }
            }
        }
        sum += cube_set_power(&bag);
    }
    println!("{sum}")
}
