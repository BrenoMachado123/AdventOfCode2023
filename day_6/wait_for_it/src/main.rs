// I am kinda tired today so I'm going to do hardcoded in a very lazy way.
fn part_1_and_2() {
    let race_data: Vec<(i64, i64, i64)> = vec![
    (48, 296, 1),
    (93, 1928, 1),
    (85, 1236, 1),
    (95, 1391, 1),
    (48938595, 296192812361391, 2) /* part 2 */];

    let mut res: i64 = 1;
    for race in race_data {
        if race.2 == 2
        {
            println!("... PART 2 ... ");
            res = 1;
        }
        let mut n = 0;
        for i in 0..race.0 + 1 {
            if i * (race.0 - i) >= race.1 { n += 1; }
        }
        res *= n;
        dbg!(res);
    }
}

fn main() {
    part_1_and_2();
}
