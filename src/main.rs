use rand::prelude::*;

fn shamir_secret_sharing(secret: u64, n: u64, k: u64) -> Vec<( u64, u64 )> {
    if k > n {
        panic!("k must be less than or equal to n");
    }

    let coefficients = random_polynomial(secret, k);

    let mut shares = vec![];

    for i in 1..n + 1 {
        let poly_result = evaluate_polynomial(i, &coefficients);

        shares.push((i, poly_result));
    }

    shares
}


fn random_polynomial(secret: u64, k: u64) -> Vec<u64> {
    let mut rng = rand::thread_rng();
    let mut poly = vec![];
    for _ in 0..k-1 {
        poly.push(rng.gen_range(0..10000));
    }
    poly[0] = secret;
    poly
}


fn evaluate_polynomial(x: u64, coefficients: &[u64]) -> u64 {
    let mut result = 0.0 as u64;
    for (i, &c) in coefficients.iter().enumerate() {
        result += (c * x.pow(i.try_into().unwrap())) as u64;
    }
    result
}


fn mod_euc(lhs: f64, rhs: f64) -> f64 {
    let r = lhs % rhs;
    if r < 0.0 {
        return if rhs > 0.0 { r + rhs } else { r - rhs }
    }
    r
}
struct Point {
    x: f64,
    y: f64,
}
// https://gist.github.com/robertDurst/24d286725b70b54b90918238be495451
// p must be larger than the secret
fn interpolate(f: Vec<Point>, p: f64) -> f64 {
    // Since we will loop over all the points in the vector, capture n.
    // Also, convert n to usize because we will be using th iterator
    // associated types as indices.
    let n = f.len() as usize;
    let mut result = 0.0;

    // Each point in the vector of "known" points will be interpolated
    // to calculate the point at f(0).
    for i in 0..n {
        let mut term = f[i].y;
        // A good old nested for loop :)
        for j in 0..n {
            if i != j {
                // X's should be unique
                assert!(f[i].x - f[j].x != 0.0);
                let denominator = f[i].x - f[j].x;
                let numerator = - f[j].x;
                term = term * (numerator/denominator);
            }
        }
        result += term;
        result = mod_euc(result, p);
    }

    result
}

fn main() {
    let secret = 42.0;
    let shares = shamir_secret_sharing(secret as u64, 4.0 as u64, 3.0 as u64);
    let mut points = vec![];
    for (x, y) in shares {
        points.push(Point { x: x as f64, y: y as f64 });
    }

    let recovered_secret = interpolate(points, 100.0);
    println!("recovered secret: {}", recovered_secret);
}
