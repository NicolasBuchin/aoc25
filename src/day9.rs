pub fn red_rectangle2(input: &str) -> usize {
    let positions = parse(input);

    let mut max = 0;

    let len = positions.len();
    for i in 0..len - 1 {
        for j in i + 2..len {
            let a = area(&positions[i], &positions[j]);

            if a > max {
                let x = &positions[i];
                let y = &positions[j];

                let (minx, maxx) = if x.0 < y.0 { (x.0, y.0) } else { (y.0, x.0) };
                let (miny, maxy) = if x.1 < y.1 { (x.1, y.1) } else { (y.1, x.1) };

                let rect = [(minx, miny), (maxx, miny), (maxx, maxy), (minx, maxy)];

                if is_polygon_inside(&positions, &rect) {
                    max = a;
                }
            }
        }
    }

    max
}

#[inline(always)]
fn is_polygon_inside(poly: &[(usize, usize)], rect: &[(usize, usize); 4]) -> bool {
    if !rect.iter().all(|&p| is_point_in_polygon(p, poly)) {
        return false;
    }

    for i in 0..4 {
        let j = if i == 3 { 0 } else { i + 1 };
        let p2_edge = (rect[i], rect[j]);

        for k in 0..poly.len() {
            let l = if k == poly.len() - 1 { 0 } else { k + 1 };
            let p1_edge = (poly[k], poly[l]);

            if intersect(p2_edge, p1_edge) {
                return false;
            }
        }
    }

    true
}

#[inline(always)]
fn is_point_in_polygon(point: (usize, usize), polygon: &[(usize, usize)]) -> bool {
    let (px, py) = (point.0 as isize, point.1 as isize);

    let n = polygon.len();

    let mut inside = false;
    let mut j = n - 1;

    for i in 0..n {
        let (xi, yi) = (polygon[i].0 as isize, polygon[i].1 as isize);
        let (xj, yj) = (polygon[j].0 as isize, polygon[j].1 as isize);

        if is_on_segment(px, py, xi, yi, xj, yj) {
            return true;
        }

        if ((yi > py) != (yj > py)) && (px < (xj - xi) * (py - yi) / (yj - yi) + xi) {
            inside = !inside;
        }

        j = i;
    }

    inside
}

#[inline(always)]
fn is_on_segment(px: isize, py: isize, x1: isize, y1: isize, x2: isize, y2: isize) -> bool {
    let cross = (py - y1) * (x2 - x1) - (px - x1) * (y2 - y1);

    if cross != 0 {
        return false;
    }

    if px < x1.min(x2) || px > x1.max(x2) {
        return false;
    }
    if py < y1.min(y2) || py > y1.max(y2) {
        return false;
    }

    true
}

#[inline(always)]
fn intersect(
    seg1: ((usize, usize), (usize, usize)),
    seg2: ((usize, usize), (usize, usize)),
) -> bool {
    let (p1, p2) = seg1;
    let (p3, p4) = seg2;

    let (x1, y1) = (p1.0 as isize, p1.1 as isize);
    let (x2, y2) = (p2.0 as isize, p2.1 as isize);
    let (x3, y3) = (p3.0 as isize, p3.1 as isize);
    let (x4, y4) = (p4.0 as isize, p4.1 as isize);

    let d1 = direction(x3, y3, x4, y4, x1, y1);
    let d2 = direction(x3, y3, x4, y4, x2, y2);
    let d3 = direction(x1, y1, x2, y2, x3, y3);
    let d4 = direction(x1, y1, x2, y2, x4, y4);

    ((d1 > 0 && d2 < 0) || (d1 < 0 && d2 > 0)) && ((d3 > 0 && d4 < 0) || (d3 < 0 && d4 > 0))
}

#[inline(always)]
fn direction(x1: isize, y1: isize, x2: isize, y2: isize, x3: isize, y3: isize) -> isize {
    (x3 - x1) * (y2 - y1) - (y3 - y1) * (x2 - x1)
}

pub fn red_rectangle(input: &str) -> usize {
    let positions = parse(input);

    let mut max = 0;

    let len = positions.len();
    for i in 0..len - 1 {
        for j in i + 2..len {
            let a = area(&positions[i], &positions[j]);
            if a > max {
                max = a;
            }
        }
    }

    max
}

fn parse_vec(input: &str) -> (usize, usize) {
    let bytes = input.as_bytes();

    let mut i = 0;

    let mut x = 0;
    while bytes[i] != b',' {
        x = x * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }
    i += 1;

    let mut y = 0;
    while i < bytes.len() {
        y = y * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }

    (x, y)
}

fn parse(input: &str) -> Vec<(usize, usize)> {
    input.lines().map(|l| parse_vec(l)).collect()
}

#[inline(always)]
fn area(x: &(usize, usize), y: &(usize, usize)) -> usize {
    (x.0.abs_diff(y.0) + 1) * (x.1.abs_diff(y.1) + 1)
}
