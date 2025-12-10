use good_lp::*;

pub fn toggle_ligths2(input: &str) -> usize {
    let machines = parse2(input);

    machines.iter().map(|(sets, jolt)| solve2(sets, jolt)).sum()
}

fn solve2(sets: &[Vec<f64>], jolt: &[f64]) -> usize {
    let mut vars = ProblemVariables::new();

    let x: Vec<_> = (0..sets.len())
        .map(|_| vars.add(variable().min(0).integer()))
        .collect();

    let objective: Expression = x.iter().fold(Expression::from(0.0), |acc, &var| acc + var);

    let mut model = vars.minimise(objective).using(default_solver);
    model.set_parameter("log", "0");

    for i in 0..jolt.len() {
        let constraint_expr: Expression = x
            .iter()
            .enumerate()
            .fold(Expression::from(0.0), |acc, (j, &var)| {
                acc + sets[j][i] * var
            });
        model = model.with(constraint!(constraint_expr == jolt[i]));
    }

    let solution = model.solve().ok().unwrap();

    x.iter().map(|&var| solution.value(var) as usize).sum()
}

fn parse_machine2(input: &str) -> (Vec<Vec<f64>>, Vec<f64>) {
    let bytes = input.as_bytes();

    let mut i = 3;

    while bytes[i] != b']' {
        i += 1;
    }
    let setlen = i - 1;
    i += 2;

    let mut sets = Vec::new();
    let mut jolt = Vec::new();

    while i < bytes.len() {
        if bytes[i] == b'{' {
            let mut n = 0.0;
            i += 1;

            while bytes[i] != b'}' {
                if bytes[i] == b',' {
                    jolt.push(n);
                    n = 0.0;
                } else {
                    let b = (bytes[i] - b'0') as f64;
                    n = n * 10.0 + b;
                }

                i += 1;
            }

            jolt.push(n);
            break;
        }

        if bytes[i] == b'(' {
            let mut n = vec![0.0; setlen];
            i += 1;

            while bytes[i] != b')' {
                if bytes[i] != b',' {
                    let bit = (bytes[i] - b'0') as usize;
                    n[bit] = 1.0;
                }

                i += 1;
            }

            sets.push(n);
        }

        i += 1;
    }

    (sets, jolt)
}

fn parse2(input: &str) -> Vec<(Vec<Vec<f64>>, Vec<f64>)> {
    input.lines().map(|l| parse_machine2(l)).collect()
}

pub fn toggle_ligths(input: &str) -> usize {
    let machines = parse(input);

    machines
        .iter()
        .map(|(target, sets)| solve(*target, sets))
        .sum()
}

fn dfs(i: usize, cur: u16, used: usize, best: &mut usize, target: u16, a: &[u16]) {
    if used >= *best {
        return;
    }
    if cur == target {
        *best = used;
        return;
    }
    if i == a.len() {
        return;
    }

    dfs(i + 1, cur, used, best, target, a);
    dfs(i + 1, cur ^ a[i], used + 1, best, target, a);
}

fn solve(target: u16, sets: &[u16]) -> usize {
    let mut best = usize::MAX;

    dfs(0, 0, 0, &mut best, target, sets);

    best
}

fn parse_machine(input: &str) -> (u16, Vec<u16>) {
    let bytes = input.as_bytes();

    let mut i = 1;

    let mut target = 0;
    while bytes[i] != b']' {
        if bytes[i] == b'#' {
            target |= 1 << (i as u16 - 1);
        }
        i += 1;
    }
    i += 2;

    let mut sets = Vec::new();

    while i < bytes.len() {
        if bytes[i] == b'{' {
            break;
        }

        if bytes[i] == b'(' {
            let mut mask = 0;
            i += 1;

            while bytes[i] != b')' {
                if bytes[i] != b',' {
                    let bit = bytes[i] - b'0';
                    mask |= 1 << bit;
                }

                i += 1;
            }

            sets.push(mask);
        }

        i += 1;
    }

    (target, sets)
}

fn parse(input: &str) -> Vec<(u16, Vec<u16>)> {
    input.lines().map(|l| parse_machine(l)).collect()
}
