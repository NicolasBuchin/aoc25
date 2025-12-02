pub fn parse_num(bytes: &[u8]) -> i32 {
    let mut n = 0;
    for c in bytes.iter() {
        n = n * 10 + (c - b'0') as i32;
    }
    n
}

pub fn secret_entrance(input: &str) -> usize {
    let mut n = 50;
    let mut c = 0;

    for line in input.lines() {
        let x = parse_num(&line.as_bytes()[1..]);

        if line.as_bytes()[0] == b'L' {
            n -= x;
        } else {
            n += x;
        }
        n = n.rem_euclid(100);

        if n == 0 {
            c += 1;
        }
    }
    c
}

pub fn secret_entrance2(input: &str) -> usize {
    let mut n = 50;
    let mut c = 0;

    for line in input.lines() {
        let p = n;
        let x = parse_num(&line.as_bytes()[1..]);

        n = if line.as_bytes()[0] == b'L' {
            n - x
        } else {
            n + x
        };

        if n <= 0 && p != 0 {
            c += 1;
        }
        c += n.abs().div_euclid(100);
        n = n.rem_euclid(100);
    }
    c as usize
}
