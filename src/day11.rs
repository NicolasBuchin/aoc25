use cached::proc_macro::cached;
use std::sync::Arc;

pub fn reactor2(input: &str) -> usize {
    let map = Arc::new(parse(input));

    // println!("{}", encode("svr".as_bytes())); // 12056
    // println!("{}", encode("dac".as_bytes())); // 1355
    // println!("{}", encode("fft".as_bytes())); // 12979
    // println!("{}", encode("out".as_bytes())); // 13378

    let svr = 12056;
    let dac = 1355;
    let fft = 12979;
    let out = 13378;

    let svr_dac = find_cached(svr, dac, map.clone());
    let dac_ftt = find_cached(dac, fft, map.clone());
    let fft_out = find_cached(fft, out, map.clone());

    let svr_fft = find_cached(svr, fft, map.clone());
    let fft_dac = find_cached(fft, dac, map.clone());
    let dac_out = find_cached(dac, out, map.clone());

    svr_dac * dac_ftt * fft_out + svr_fft * fft_dac * dac_out
}

#[cached]
fn find_cached(s: usize, e: usize, map: Arc<Vec<Vec<usize>>>) -> usize {
    if s == e {
        return 1;
    }

    map[s].iter().map(|&c| find_cached(c, e, map.clone())).sum()
}

pub fn reactor(input: &str) -> usize {
    let map = parse(input);

    // println!("{}", encode("aaa".as_bytes())); // 0
    // println!("{}", encode("zzz".as_bytes())); // 17575
    // println!("{}", encode("you".as_bytes())); // 13908
    // println!("{}", encode("out".as_bytes())); // 13378

    find(13908, &map)
}

#[inline(never)]
fn find(s: usize, map: &[Vec<usize>]) -> usize {
    if s == 13378 {
        return 1;
    }

    map[s].iter().map(|&c| find(c, map)).sum()
}

fn parse(input: &str) -> Vec<Vec<usize>> {
    let bytes = input.as_bytes();

    let mut map = vec![vec![]; 17575];

    let mut i = 0;
    while i < bytes.len() {
        let p = encode(&bytes[i..i + 3]);
        i += 5;
        let cs = &mut map[p];
        while bytes[i - 1] != b'\n' {
            let c = encode(&bytes[i..i + 3]);
            i += 4;
            cs.push(c);
        }
    }

    map
}

#[inline(always)]
fn encode(b: &[u8]) -> usize {
    let x = (b[2] - b'a') as usize;
    let y = (b[1] - b'a') as usize;
    let z = (b[0] - b'a') as usize;
    x * 26 * 26 + y * 26 + z
}
