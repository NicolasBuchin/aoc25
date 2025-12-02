use std::fs::read_to_string;

mod day1;
mod day2;

fn main() {
    // let input = read_to_string("inputs/day1/input.txt").expect("file not found!");
    // assert_eq!(day1::secret_entrance(&input), 969);
    // assert_eq!(day1::secret_entrance2(&input), 5887);
    let input = read_to_string("inputs/day2/input.txt").expect("file not found!");
    assert_eq!(day2::valid_ids(&input), 20223751480);
    assert_eq!(day2::valid_ids2(&input), 30260171216);
}
