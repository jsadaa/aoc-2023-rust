use std::collections::HashMap;

pub fn day_6_part1() {
    let mut races: HashMap<i32, i32> = HashMap::with_capacity(3);
    races.insert(46, 358);
    races.insert(68, 1054);
    races.insert(98, 1807);
    races.insert(66, 1080);

    let mut wins_list: Vec<i32> = vec![];

    for (duration, best_dist) in races {
        let mut speed: i32 = 0;
        let mut wins: i32 = 0;
        for i in 1..=duration {
            speed += 1;
            if speed * (duration - i) > best_dist {
                wins += 1;
            }
        }

        wins_list.push(wins);
    }

    println!("{}", wins_list.iter().product::<i32>());
}

pub fn day_6_part2() {
    let duration: i64 = 46689866;
    let best: i64 = 358105418071080;
    let mut speed: i64 = 0;
    let mut wins: i64 = 0;

    for i in 1..=duration {
        speed += 1;
        if speed * (duration - i) > best {
            wins += 1;
        }
    }

    println!("{}", wins);
}