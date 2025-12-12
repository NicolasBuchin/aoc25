pub fn presents(input: &str) -> usize {
    let bytes = input.as_bytes();

    let pieces = [
        area([
            bytes[3] == b'#',
            bytes[4] == b'#',
            bytes[5] == b'#',
            bytes[7] == b'#',
            bytes[8] == b'#',
            bytes[9] == b'#',
            bytes[11] == b'#',
            bytes[12] == b'#',
            bytes[13] == b'#',
        ]),
        area([
            bytes[19] == b'#',
            bytes[20] == b'#',
            bytes[21] == b'#',
            bytes[23] == b'#',
            bytes[24] == b'#',
            bytes[25] == b'#',
            bytes[27] == b'#',
            bytes[28] == b'#',
            bytes[29] == b'#',
        ]),
        area([
            bytes[35] == b'#',
            bytes[36] == b'#',
            bytes[37] == b'#',
            bytes[39] == b'#',
            bytes[40] == b'#',
            bytes[41] == b'#',
            bytes[43] == b'#',
            bytes[44] == b'#',
            bytes[45] == b'#',
        ]),
        area([
            bytes[51] == b'#',
            bytes[52] == b'#',
            bytes[53] == b'#',
            bytes[55] == b'#',
            bytes[56] == b'#',
            bytes[57] == b'#',
            bytes[59] == b'#',
            bytes[60] == b'#',
            bytes[61] == b'#',
        ]),
        area([
            bytes[67] == b'#',
            bytes[68] == b'#',
            bytes[69] == b'#',
            bytes[71] == b'#',
            bytes[72] == b'#',
            bytes[73] == b'#',
            bytes[75] == b'#',
            bytes[76] == b'#',
            bytes[77] == b'#',
        ]),
        area([
            bytes[83] == b'#',
            bytes[84] == b'#',
            bytes[85] == b'#',
            bytes[87] == b'#',
            bytes[88] == b'#',
            bytes[89] == b'#',
            bytes[91] == b'#',
            bytes[92] == b'#',
            bytes[93] == b'#',
        ]),
    ];

    input[96..]
        .lines()
        .filter(|line| {
            let bytes = line.as_bytes();
            let mut i = 0;

            let mut w = 0;
            while bytes[i] != b'x' {
                w = w * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            i += 1;

            let mut h = 0;
            while bytes[i] != b':' {
                h = h * 10 + (bytes[i] - b'0') as usize;
                i += 1;
            }
            i += 2;

            line[i..]
                .split_whitespace()
                .enumerate()
                .map(|(i, n)| pieces[i] * parse_num(n.as_bytes()))
                .sum::<usize>()
                <= w * h
        })
        .count()
}

#[inline(always)]
fn area(piece: [bool; 9]) -> usize {
    piece.iter().filter(|&&e| e).count()
}

#[inline(always)]
pub fn parse_num(bytes: &[u8]) -> usize {
    let mut n = 0;
    for c in bytes.iter() {
        n = n * 10 + (c - b'0') as usize;
    }
    n
}
