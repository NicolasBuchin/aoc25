use std::fs::read_to_string;

// mod day1;
// mod day2;
// mod day3;
// mod day4;
// mod day5;
// mod day6;
// mod day7;
// mod day8;
mod day9;

fn main() {
    // let input = read_to_string("inputs/day1/input.txt").expect("file not found!");
    // assert_eq!(day1::secret_entrance(&input), 969);
    // assert_eq!(day1::secret_entrance2(&input), 5887);
    // let input = read_to_string("inputs/day2/input.txt").expect("file not found!");
    // assert_eq!(day2::valid_ids(&input), 20223751480);
    // assert_eq!(day2::valid_ids2(&input), 30260171216);
    // let input = read_to_string("inputs/day3/input.txt").expect("file not found!");
    // assert_eq!(day3::batteries(&input), 17207);
    // assert_eq!(day3::batteries2(&input), 170997883706617);
    // let input = read_to_string("inputs/day4/input.txt").expect("file not found!");
    // assert_eq!(day4::forklift(&input), 1435);
    // assert_eq!(day4::forklift2(&input), 8623);
    // let input = read_to_string("inputs/day5/input.txt").expect("file not found!");
    // assert_eq!(day5::spoiled(&input), 615);
    // assert_eq!(day5::spoiled2(&input), 353716783056994);
    // let input = read_to_string("inputs/day6/input.txt").expect("file not found!");
    // assert_eq!(day6::cephalopod_math(&input), 4951502530386);
    // assert_eq!(day6::cephalopod_math2(&input), 8486156119946);
    // let input = read_to_string("inputs/day7/input.txt").expect("file not found!");
    // assert_eq!(day7::tachyon_manifolds(&input), 1543);
    // assert_eq!(day7::tachyon_manifolds2(&input), 3223365367809);

    // let test = read_to_string("inputs/day8/test.txt").expect("file not found!");
    // assert_eq!(day8::join_circuits(&test, 10), 40);
    // assert_eq!(day8::join_circuits2(&test), 25272);
    // let input = read_to_string("inputs/day8/input.txt").expect("file not found!");
    // assert_eq!(day8::join_circuits(&input, 1000), 140008);
    // assert_eq!(day8::join_circuits2(&input), 9253260633);

    let test = read_to_string("inputs/day9/test.txt").expect("file not found!");
    assert_eq!(day9::red_rectangle(&test), 50);
    assert_eq!(day9::red_rectangle2(&test), 24);
    assert_eq!(day9::red_rectangle3(&test), 24);
    let input = read_to_string("inputs/day9/input.txt").expect("file not found!");
    assert_eq!(day9::red_rectangle(&input), 4777967538);
    assert_eq!(day9::red_rectangle2(&input), 1439894345);
    assert_eq!(day9::red_rectangle3(&input), 1439894345);

    // println!("{}", day9::red_rectangle2(&test));
    // println!("{}", day9::red_rectangle2(&input));
}
