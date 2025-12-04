pub fn forklift(input: &str) -> usize {
    let bytes = input.as_bytes();
    let mut res = 0;

    let w = bytes
        .iter()
        .position(|&c| c == b'\n')
        .unwrap_or(bytes.len());
    let stride = if w < bytes.len() { w + 1 } else { w };

    for i in 0..bytes.len() {
        if bytes[i] != b'@' {
            continue;
        }
        let col = if stride > 0 { i % stride } else { 0 };
        let mut n = 0;

        if i > stride && col > 0 && bytes[i - stride - 1] == b'@' {
            n += 1;
        }
        if i >= stride && bytes[i - stride] == b'@' {
            n += 1;
        }
        if i >= stride && col + 1 < w && bytes[i - stride + 1] == b'@' {
            n += 1;
        }
        if col > 0 && bytes[i - 1] == b'@' {
            n += 1;
        }
        if col + 1 < w && i + 1 < bytes.len() && bytes[i + 1] == b'@' {
            n += 1;
        }
        if i + stride < bytes.len() && col > 0 && bytes[i + stride - 1] == b'@' {
            n += 1;
        }
        if i + stride < bytes.len() && bytes[i + stride] == b'@' {
            n += 1;
        }
        if i + stride + 1 < bytes.len() && col + 1 < w && bytes[i + stride + 1] == b'@' {
            n += 1;
        }

        if n < 4 {
            res += 1;
        }
    }

    res
}

pub fn forklift2(input: &str) -> usize {
    let mut input = input.to_owned();
    let bytes = unsafe { input.as_bytes_mut() };
    let mut res = 0;

    let w = bytes
        .iter()
        .position(|&c| c == b'\n')
        .unwrap_or(bytes.len());
    let stride = if w < bytes.len() { w + 1 } else { w };

    loop {
        let mut pos = Vec::new();

        for i in 0..bytes.len() {
            if bytes[i] == b'\n' {
                continue;
            }
            if bytes[i] != b'@' {
                continue;
            }
            let col = if stride > 0 { i % stride } else { 0 };
            let mut n = 0;

            if i > stride && col > 0 && bytes[i - stride - 1] == b'@' {
                n += 1;
            }
            if i >= stride && bytes[i - stride] == b'@' {
                n += 1;
            }
            if i >= stride && col + 1 < w && bytes[i - stride + 1] == b'@' {
                n += 1;
            }
            if col > 0 && bytes[i - 1] == b'@' {
                n += 1;
            }
            if col + 1 < w && i + 1 < bytes.len() && bytes[i + 1] == b'@' {
                n += 1;
            }
            if i + stride < bytes.len() && col > 0 && bytes[i + stride - 1] == b'@' {
                n += 1;
            }
            if i + stride < bytes.len() && bytes[i + stride] == b'@' {
                n += 1;
            }
            if i + stride + 1 < bytes.len() && col + 1 < w && bytes[i + stride + 1] == b'@' {
                n += 1;
            }

            if n < 4 {
                pos.push(i);
                res += 1;
            }
        }

        if pos.is_empty() {
            break;
        }

        for &i in pos.iter() {
            bytes[i] = b'.';
        }
    }

    // println!("{}", input);

    res
}
