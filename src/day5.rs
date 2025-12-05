pub fn spoiled2(input: &str) -> usize {
    let mut ranges = Vec::new();
    let mut iter = input.lines();

    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once('-').expect("no '-' found on line!");
        insert_sorted(
            &mut ranges,
            (parse_num(a.as_bytes()), parse_num(b.as_bytes())),
        );
    }

    ranges.iter().map(|(a, b)| *b - *a + 1).sum::<u64>() as usize
}

pub fn spoiled(input: &str) -> usize {
    let mut ranges = Vec::new();
    let mut iter = input.lines();

    loop {
        let line = iter.next().unwrap();
        if line.is_empty() {
            break;
        }
        let (a, b) = line.split_once('-').expect("no '-' found on line!");
        insert_sorted(
            &mut ranges,
            (parse_num(a.as_bytes()), parse_num(b.as_bytes())),
        );
    }

    iter.filter(|&line| find(&ranges, parse_num(line.as_bytes())))
        .count()
}

#[inline(always)]
fn find(ranges: &[(u64, u64)], x: u64) -> bool {
    let i = match ranges.binary_search_by(|&(s, _)| s.cmp(&x)) {
        Ok(_) => return true,
        Err(i) => i,
    };
    if i > 0 && ranges[i - 1].1 >= x {
        return true;
    }
    false
}

#[inline(always)]
fn parse_num(bytes: &[u8]) -> u64 {
    let mut n = 0;
    for c in bytes.iter() {
        n = n * 10 + (c - b'0') as u64;
    }
    n
}

#[inline(always)]
fn insert_sorted(ranges: &mut Vec<(u64, u64)>, pair: (u64, u64)) {
    if ranges.is_empty() {
        ranges.push(pair);
        return;
    }

    let (mut a, mut b) = pair;

    let mut i = match ranges.binary_search_by(|&(s, _)| s.cmp(&a)) {
        Ok(i) => i,
        Err(i) => i,
    };

    if i > 0 && ranges[i - 1].1 >= a {
        i -= 1;
        a = a.min(ranges[i].0);
        b = b.max(ranges[i].1);
    }

    let mut j = i;
    while j < ranges.len() && ranges[j].0 <= b {
        b = b.max(ranges[j].1);
        j += 1;
    }

    ranges.splice(i..j, std::iter::once((a, b)));
}
