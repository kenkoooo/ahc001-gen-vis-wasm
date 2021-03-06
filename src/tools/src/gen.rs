use rand::prelude::*;
use std::collections::BTreeSet;

pub fn gen(seed: u64) -> String {
    let mut rng = rand_chacha::ChaCha20Rng::seed_from_u64(seed);
    let mut output = String::new();
    let n = (50.0 * 4.0f64.powf(rng.gen::<f64>())).round() as usize;
    let mut ps = vec![];
    let mut used = BTreeSet::new();
    for _ in 0..n {
        loop {
            let x = rng.gen_range(0, 10000);
            let y = rng.gen_range(0, 10000);
            if used.insert((x, y)) {
                ps.push((x, y));
                break;
            }
        }
    }
    let mut q = rand::seq::index::sample(&mut rng, 10000 * 10000 - 1, n - 1)
        .into_iter()
        .map(|a| a + 1)
        .collect::<Vec<_>>();
    q.sort();
    q.insert(0, 0);
    q.push(10000 * 10000);
    let mut r = vec![];
    for i in 0..n {
        r.push(q[i + 1] - q[i]);
    }
    output.push_str(&format!("{}\n", n));
    for i in 0..n {
        output.push_str(&format!("{} {} {}\n", ps[i].0, ps[i].1, r[i]));
    }
    output
}
