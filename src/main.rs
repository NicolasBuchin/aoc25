use std::fs::read_to_string;

// mod day1;
// mod day2;
// mod day3;
mod day4;

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
    let input = read_to_string("inputs/day4/input.txt").expect("file not found!");
    assert_eq!(day4::forklift(&input), 1435);
    assert_eq!(day4::forklift2(&input), 8623);
    // println!("{}", day4::forklift2(&input));
}
