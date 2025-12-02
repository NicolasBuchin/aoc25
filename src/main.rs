use std::fs::read_to_string;

mod day1;

fn main() {
    let input = read_to_string("inputs/day1/input.txt").expect("file not found!");
    assert_eq!(day1::secret_entrance(&input), 969);
    assert_eq!(day1::secret_entrance2(&input), 5887);
    // println!("day1: {}", day1::secret_entrance2(&input));
}
