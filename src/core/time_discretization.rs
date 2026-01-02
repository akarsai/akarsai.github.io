use nalgebra::{DVector, DMatrix};
use crate::helpers::broyden::broyden;

/// Implicit midpoint integrator for z' = f(z,u), z(0) = z0
pub fn implicit_midpoint<F>(
    f: F,
    tt: &DVector<f64>,
    z0: &DVector<f64>,
    uu: &DMatrix<f64>,
) -> DMatrix<f64>
where
    F: Fn(&DVector<f64>, &DVector<f64>) -> DVector<f64>,
{
    let n = z0.len();
    let nt = tt.len();
    let dt = tt[1] - tt[0];
    let mut z = DMatrix::zeros(nt, n);
    z.row_mut(0).copy_from(&z0.transpose());
    
    // uumid = (uu[1:, :] + uu[:-1, :]) * 0.5
    let uumid = (uu.rows(1, nt - 1).clone_owned() + uu.rows(0, nt - 1).clone_owned()) * 0.5;
    
    for j in 1..nt {
        let zjm1 = z.row(j - 1).transpose();
        let uj12 = uumid.row(j - 1).transpose();
        
        let f_implicit = |zj: &DVector<f64>| -> DVector<f64> {
            let zmid = (&zjm1 + zj) * 0.5;
            zj - &zjm1 - dt * f(&zmid, &uj12)
        };
        
        let zj = broyden(&f_implicit, &zjm1, 1000, 1e-14);
        z.row_mut(j).copy_from(&zj.transpose());
    }
    
    z
}

/// QSR discrete gradient integrator for dissipative port-Hamiltonian systems
pub fn qsr_discrete_gradient<F, G, K, HamEta, Ell, W>(
    f: F,
    g: G,
    k: K,
    ham_eta: HamEta,
    ell: Ell,
    w_fn: W,
    qsr: &(DMatrix<f64>, DMatrix<f64>, DMatrix<f64>),
    tt: &DVector<f64>,
    z0: &DVector<f64>,
    uu: &DMatrix<f64>,
) -> DMatrix<f64>
where
    F: Fn(&DVector<f64>) -> DVector<f64>,
    G: Fn(&DVector<f64>) -> DMatrix<f64>,
    K: Fn(&DVector<f64>) -> DMatrix<f64>,
    HamEta: Fn(&DVector<f64>) -> (f64, DVector<f64>),
    Ell: Fn(&DVector<f64>) -> DVector<f64>,
    W: Fn(&DVector<f64>) -> DMatrix<f64>,
{
    let nt = tt.len();
    let nsys = z0.len();
    let delta_t = tt[1] - tt[0];
    let uumid = (uu.rows(1, nt - 1).clone_owned() + uu.rows(0, nt - 1).clone_owned()) * 0.5;
    let (q, s, _r) = qsr;
    
    let mut z = DMatrix::zeros(nt, nsys);
    z.row_mut(0).copy_from(&z0.transpose());
    let (ham_z0, _) = ham_eta(z0);
    let mut ham = DVector::zeros(nt);
    ham[0] = ham_z0;
    
    for i in 0..(nt - 1) {
        let zi = z.row(i).transpose();
        let ham_zi = ham[i];
        let u_mid = uumid.row(i).transpose();
        
        let solve_fn = |zip1: &DVector<f64>| -> DVector<f64> {
            let zmid = (&zi + zip1) * 0.5;
            let (_, eta_mid) = ham_eta(&zmid);
            let (ham_zhat, _) = ham_eta(zip1);
            
            let diff = zip1 - &zi;
            let alpha1 = ham_zhat - ham_zi - eta_mid.dot(&diff);
            let alpha2 = diff.dot(&diff);
            
            let eta_bar = if alpha2.abs() < 1e-14 {
                eta_mid.clone()
            } else {
                &eta_mid + alpha1 / alpha2 * &diff
            };
            
            let norm_eta_bar = eta_bar.norm();
            let projection = if norm_eta_bar < 1e-14 {
                DMatrix::identity(nsys, nsys)
            } else {
                let eta_norm = &eta_bar / norm_eta_bar;
                let outer = &eta_norm * eta_norm.transpose();

                DMatrix::identity(nsys, nsys) - outer
            };
            
            let f_mid = f(&zmid);
            let g_mid = g(&zmid);
            let k_mid = k(&zmid);
            let ell_mid = ell(&zmid);
            let w_mid = w_fn(&zmid);
            
            let qk_s = q * &k_mid + s;
            let rhs = 0.5 * g_mid.transpose() * &eta_bar + w_mid.transpose() * &ell_mid;
            let h_mid = qk_s.transpose().lu().solve(&rhs).unwrap();
            
            let gamma = if norm_eta_bar < 1e-14 {
                0.0
            } else {
                let qh = q * &h_mid;
                let qh_result = h_mid.dot(&qh);
                let numerator = qh_result - ell_mid.dot(&ell_mid);
                numerator / (norm_eta_bar * norm_eta_bar)
            };
            
            let uncontrolled = gamma * &eta_bar + projection * &f_mid;
            
            zip1 - &zi - delta_t * (uncontrolled + g_mid * &u_mid)
        };
        
        let y = broyden(&solve_fn, &zi, 1000, 1e-14);
        z.row_mut(i + 1).copy_from(&y.transpose());
        let (ham_zkp1, _) = ham_eta(&y);
        ham[i + 1] = ham_zkp1;
    }
    
    z
}
