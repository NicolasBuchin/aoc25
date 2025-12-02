pub fn valid_ids(input: &str) -> usize {
    let ranges = parse_double(input);
    let mut res = 0;

    for (min, max, dmin, dmax) in ranges {
        if dmax < dmin {
            continue;
        }

        let mut i: u64 = 1 * 10u64.pow(dmin as u32 - 1);
        let mut s = doubled(i);

        while s <= max {
            if s >= min {
                res += s as usize;
            }
            i += 1;
            s = doubled(i);
        }
    }

    res
}

pub fn doubled(i: u64) -> u64 {
    i * 10u64.pow(i.to_string().len() as u32) + i
}

pub fn parse_num(bytes: &[u8]) -> u64 {
    let mut n = 0;
    for c in bytes.iter() {
        n = n * 10 + (c - b'0') as u64;
    }
    n
}

fn parse_double(input: &str) -> Vec<(u64, u64, usize, usize)> {
    let bytes = input.as_bytes();

    let mut ranges = Vec::new(); // (min, max, min digits to repeat, max digits to repeat)

    let mut s = 0;
    let mut i = 0;

    loop {
        while bytes[i] != b'-' {
            i += 1;
        }
        let x = parse_num(&bytes[s..i]);
        let dx = (i - s).div_ceil(2);

        i += 1;
        s = i;
        while bytes[i] != b',' && bytes[i] != b'\n' {
            i += 1;
        }
        let y = parse_num(&bytes[s..i]);
        let dy = (i - s - 1).div_ceil(2);

        ranges.push((x, y, dx, dy));

        if bytes[i] == b'\n' {
            break;
        }
        i += 1;
        s = i;
    }

    ranges
}

pub fn valid_ids2(input: &str) -> usize {
    let ranges = parse(input);
    let mut res = 0;

    for (min, max) in ranges {
        for i in min..=max {
            if is_repeat(i) {
                res += i;
            }
        }
    }

    res as usize
}

fn is_repeat(i: u64) -> bool {
    let str = i.to_string();
    let len = str.len();
    for n in 2..=len {
        if len.rem_euclid(n) == 0 {
            let s = len.div_euclid(n);
            //
            let x = &str[0..s];
            let mut same = true;
            for i in 1..n {
                let y = &str[i * s..(i + 1) * s];
                same &= x == y;
            }
            if same {
                return true;
            }
        }
    }
    false
}

fn parse(input: &str) -> Vec<(u64, u64)> {
    let bytes = input.as_bytes();

    let mut ranges = Vec::new(); // (min, max)

    let mut s = 0;
    let mut i = 0;

    loop {
        while bytes[i] != b'-' {
            i += 1;
        }
        let x = parse_num(&bytes[s..i]);

        i += 1;
        s = i;
        while bytes[i] != b',' && bytes[i] != b'\n' {
            i += 1;
        }
        let y = parse_num(&bytes[s..i]);

        ranges.push((x, y));

        if bytes[i] == b'\n' {
            break;
        }
        i += 1;
        s = i;
    }

    ranges
}
