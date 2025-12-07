pub fn tachyon_manifolds(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut pos = Vec::new();

    let mut w = 0;

    for (i, c) in bytes.iter().enumerate() {
        if *c == b'S' {
            pos.push(i);
        }
        if *c == b'\n' {
            w = i + 1;
            break;
        }
    }

    let mut h = 2;
    let mut check;
    let mut res = 0;

    while h * w < bytes.len() {
        let l = h * w;

        check = pos.clone();
        pos.clear();

        for i in check {
            let x = l + i;
            if bytes[x] == b'^' {
                res += 1;
                insert_sorted(&mut pos, i - 1);
                insert_sorted(&mut pos, i + 1);
            } else {
                insert_sorted(&mut pos, i);
            }
        }
        h += 2;
    }

    res
}
#[inline(always)]
fn insert_sorted(vec: &mut Vec<usize>, n: usize) {
    if vec.is_empty() {
        vec.push(n);
        return;
    }

    let i = match vec.binary_search_by(|&p| p.cmp(&n)) {
        Ok(_) => return,
        Err(i) => i,
    };

    vec.insert(i, n);
}

pub fn tachyon_manifolds2(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut pos = Vec::new();
    let mut w = 0;

    for (i, c) in bytes.iter().enumerate() {
        if *c == b'S' {
            pos.push((i, 1));
        }
        if *c == b'\n' {
            w = i + 1;
            break;
        }
    }

    let mut h = 2;
    let mut check;

    while h * w < bytes.len() {
        let l = h * w;

        check = pos.clone();
        pos.clear();

        for (i, v) in check {
            let x = l + i;

            if bytes[x] == b'^' {
                insert_sorted2(&mut pos, i - 1, v);
                insert_sorted2(&mut pos, i + 1, v);
            } else {
                insert_sorted2(&mut pos, i, v);
            }
        }
        h += 2;
    }

    pos.iter().map(|&(_, c)| c).sum()
}

#[inline(always)]
fn insert_sorted2(vec: &mut Vec<(usize, usize)>, n: usize, c: usize) {
    if vec.is_empty() {
        vec.push((n, c));
        return;
    }

    let i = match vec.binary_search_by(|&(p, _)| p.cmp(&n)) {
        Ok(i) => {
            vec[i].1 += c;
            return;
        }
        Err(i) => i,
    };

    vec.insert(i, (n, c));
}
