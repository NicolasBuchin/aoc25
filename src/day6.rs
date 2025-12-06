pub fn cephalopod_math(input: &str) -> usize {
    let mut maths = Vec::new();
    let mut lines = input.lines();

    let l = lines.next().unwrap();
    for num in l.split_whitespace() {
        let num: usize = num.parse().unwrap();
        maths.push((num, num));
    }

    let mut res = 0;

    for line in lines {
        if line.as_bytes()[0] == b'*' || line.as_bytes()[0] == b'+' {
            for (i, op) in line.split_whitespace().enumerate() {
                if op.as_bytes()[0] == b'*' {
                    res += maths[i].1;
                } else if op.as_bytes()[0] == b'+' {
                    res += maths[i].0;
                } else {
                    unreachable!();
                };
            }
            return res;
        }
        for (i, num) in line.split_whitespace().enumerate() {
            let (a, b) = maths[i];
            let n: usize = num.parse().unwrap();
            maths[i] = (a + n, b * n);
        }
    }

    0
}

#[derive(Debug)]
enum Op {
    Mul,
    Sum,
}

pub fn cephalopod_math2(input: &str) -> usize {
    let bytes = input.as_bytes();

    let mut ops = Vec::new();

    let mut i = bytes.len() - 1;
    while i > 0 {
        let op = if bytes[i] == b'*' {
            Some(Op::Mul)
        } else if bytes[i] == b'+' {
            Some(Op::Sum)
        } else {
            None
        };
        if let Some(op) = op {
            ops.push((bytes.len() - i, op));
        }
        if bytes[i - 1] == b'\n' {
            break;
        }
        i -= 1;
    }

    let w = bytes.len() - i - 1;
    let h = bytes.len() / w;

    let mut opi = 0;
    let mut nums = Vec::new();
    let mut skip = false;

    let mut res = 0;

    for x in (0..w).rev() {
        if skip {
            skip = false;
            continue;
        }
        let mut n = 0;

        for y in 0..h - 1 {
            let i = x + y * (w + 1);
            if bytes[i].is_ascii_digit() {
                n = n * 10 + (bytes[i] - b'0') as usize
            }
        }

        nums.push(n);

        if ops[opi].0 == w - x + 1 {
            let n: usize = match ops[opi].1 {
                Op::Sum => nums.iter().sum(),
                Op::Mul => nums.iter().product(),
            };
            res += n;
            opi += 1;
            nums = Vec::new();
            skip = true;
        }
    }

    res
}
