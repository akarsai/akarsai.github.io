use nalgebra::{DVector, DMatrix};

/// Broyden's quasi-Newton method for solving f(x) = 0
pub fn broyden<F>(
    f: &F,
    x0: &DVector<f64>,
    max_iter: usize,
    tol: f64,
) -> DVector<f64>
where
    F: Fn(&DVector<f64>) -> DVector<f64>,
{
    let n = x0.len();
    let mut x = x0.clone();
    let mut b = DMatrix::<f64>::identity(n, n);

    for _ in 0..max_iter {
        let fx = f(&x);

        if fx.norm() < tol {
            break;
        }

        let update = match b.clone().lu().solve(&fx) {
            Some(u) => u,
            None => break,
        };

        let x_new = &x - &update;
        let s = &x_new - &x;
        let y = &f(&x_new) - &fx;
        let bs = &b * &s;
        let denom = s.dot(&s);

        if denom > 1e-16 {
            let outer = (&y - &bs) * s.transpose();
            b = b + outer / denom;
        }

        x = x_new;
    }

    x
}
