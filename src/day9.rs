pub fn red_rectangle3(input: &str) -> usize {
    let (positions, xs, ys) = parse2(input);

    let mut grid = [[false; 512]; 512];
    fill_polygon(&mut grid, &positions);

    let mut max = 0;

    let len = positions.len();
    for i in 0..len - 1 {
        for j in i + 2..len {
            let (x1, y1) = positions[i];
            let (x2, y2) = positions[j];

            let x1u = xs[x1];
            let x2u = xs[x2];
            let y1u = ys[y1];
            let y2u = ys[y2];

            let a = area(&(x1u, y1u), &(x2u, y2u));

            if a > max {
                let (minx, maxx) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
                let (miny, maxy) = if y1 < y2 { (y1, y2) } else { (y2, y1) };

                let mut ok = true;

                for x in minx + 1..maxx {
                    for y in miny + 1..maxy {
                        if !grid[y][x] {
                            ok = false;
                            break;
                        }
                    }
                    if !ok {
                        break;
                    }
                }

                if ok {
                    max = a;
                }
            }
        }
    }

    max
}

pub fn fill_polygon(grid: &mut [[bool; 512]; 512], poly: &[(usize, usize)]) {
    let len = poly.len();

    for y in 0..len {
        let mut intersections = Vec::with_capacity(len);

        let scan_y = y as f32 + 0.5;

        for i in 0..len {
            let j = if i == len - 1 { 0 } else { i + 1 };
            let (x1, y1) = (poly[i].0 as f32, poly[i].1 as f32);
            let (x2, y2) = (poly[j].0 as f32, poly[j].1 as f32);

            if (y1 <= scan_y && y2 > scan_y) || (y2 <= scan_y && y1 > scan_y) {
                let x_intersect = x1 + (scan_y - y1) * (x2 - x1) / (y2 - y1);
                intersections.push(x_intersect);
            }
        }

        intersections.sort_by(|a, b| a.partial_cmp(b).unwrap());

        for chunk in intersections.chunks(2) {
            if chunk.len() == 2 {
                let x_start = chunk[0].floor() as usize;
                let x_end = chunk[1].ceil() as usize;
                for x in x_start..=x_end {
                    grid[y][x] = true;
                }
            }
        }
    }

    for i in 0..len {
        let j = (i + 1) % len;
        draw_line(grid, poly[i], poly[j]);
    }
}

fn draw_line(grid: &mut [[bool; 512]; 512], p0: (usize, usize), p1: (usize, usize)) {
    let (mut x0, mut y0) = (p0.0 as i32, p0.1 as i32);
    let (x1, y1) = (p1.0 as i32, p1.1 as i32);

    let dx = (x1 - x0).abs();
    let dy = -(y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;

    loop {
        if x0 >= 0 && x0 < 512 && y0 >= 0 && y0 < 512 {
            grid[y0 as usize][x0 as usize] = true;
        }
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
}

fn parse2(input: &str) -> (Vec<(usize, usize)>, Vec<usize>, Vec<usize>) {
    let mut positions = Vec::new();
    let mut xs = Vec::new();
    let mut ys = Vec::new();

    for line in input.lines() {
        let (x, y) = parse_vec(line);
        positions.push((x, y));
        xs.push(x);
        ys.push(y);
    }

    xs.sort_unstable();
    ys.sort_unstable();

    for (x, y) in positions.iter_mut() {
        let i = xs.binary_search(x).unwrap();
        let j = ys.binary_search(y).unwrap();
        *x = i;
        *y = j;
    }

    (positions, xs, ys)
}

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
