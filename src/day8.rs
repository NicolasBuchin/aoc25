pub fn join_circuits2(input: &str) -> usize {
    let positions = parse(input);
    let distances = get_distances(&positions);

    let mut uf = UnionFind::new(positions.len());

    for (_, x, y) in distances {
        uf.union(x, y);
        if uf.get_count() == 1 {
            return positions[x].x * positions[y].x;
        }
    }

    unreachable!()
}

pub fn join_circuits(input: &str, n: usize) -> usize {
    let positions = parse(input);
    let distances = get_distances(&positions);

    let mut uf = UnionFind::new(positions.len());

    for &(_, x, y) in &distances[..n] {
        uf.union(x, y);
    }
    let mut sizes = uf.sizes();

    sizes.sort();
    sizes.iter().rev().take(3).product()
}

#[derive(Clone, Copy)]
struct Vec3 {
    x: usize,
    y: usize,
    z: usize,
}

fn get_distances(positions: &[Vec3]) -> Vec<(usize, usize, usize)> {
    let len = positions.len();
    let mut distances = Vec::with_capacity(len * len);

    for i in 0..len - 1 {
        for j in i + 1..len {
            let d = distance(&positions[i], &positions[j]);
            distances.push((d, i, j));
        }
    }

    distances.sort_by_key(|&(d, _, _)| d);
    distances
}

fn distance(a: &Vec3, b: &Vec3) -> usize {
    a.x.abs_diff(b.x).pow(2) + a.y.abs_diff(b.y).pow(2) + a.z.abs_diff(b.z).pow(2)
}

fn parse_vec(input: &str) -> Vec3 {
    let bytes = input.as_bytes();

    let mut i = 0;

    let mut x = 0;
    while bytes[i] != b',' {
        x = x * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }
    i += 1;

    let mut y = 0;
    while bytes[i] != b',' {
        y = y * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }
    i += 1;

    let mut z = 0;
    while i < bytes.len() {
        z = z * 10 + (bytes[i] - b'0') as usize;
        i += 1;
    }

    Vec3 { x, y, z }
}

fn parse(input: &str) -> Vec<Vec3> {
    input.lines().map(|l| parse_vec(l)).collect()
}

pub struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    pub fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
            count: n,
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
            return self.parent[x];
        }

        x
    }

    pub fn union(&mut self, a: usize, b: usize) {
        let mut ra = self.find(a);
        let mut rb = self.find(b);

        if ra == rb {
            return;
        }

        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }

        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
        self.count -= 1;
    }

    pub fn sizes(&mut self) -> Vec<usize> {
        let mut res = Vec::new();

        for (i, &p) in self.parent.iter().enumerate() {
            if p == i {
                res.push(self.size[i]);
            }
        }

        res
    }

    pub fn get_count(&self) -> usize {
        self.count
    }
}
