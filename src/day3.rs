pub fn batteries(input: &str) -> usize {
    let mut res = 0;

    for line in input.lines() {
        res += find_pair(line.as_bytes())
    }

    res
}

fn find_pair(bat: &[u8]) -> usize {
    let (a, i) = first_max(&bat[0..bat.len() - 1]);
    let (b, _) = first_max(&bat[i..]);
    a * 10 + b
}

#[inline(always)]
fn first_max(bat: &[u8]) -> (usize, usize) {
    let mut max = 0;
    let mut max_i = 0;

    for (i, &v) in bat.iter().enumerate() {
        let b = (v - b'0') as usize;
        if b > max {
            max = b;
            max_i = i + 1;
            if b == 9 {
                break;
            }
        }
    }

    (max, max_i)
}

pub fn batteries2(input: &str) -> usize {
    let mut res = 0;

    for line in input.lines() {
        res += find_n(line.as_bytes(), 12)
    }

    res
}

fn find_n(bat: &[u8], n: usize) -> usize {
    let mut v = 0;
    let mut start = 0;

    let len = bat.len();

    for i in (1..n).rev() {
        let end = len - i;
        let (a, l) = first_max(&bat[start..end]);
        start += l;
        v = v * 10 + a;
    }

    let (b, _) = first_max(&bat[start..]);

    v * 10 + b
}
