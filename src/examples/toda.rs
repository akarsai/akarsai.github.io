use nalgebra::{DVector, DMatrix};
use crate::helpers::nonlinear_system::NonlinearQSRSystem;

pub struct Toda {
    n: usize,
    gamma: f64,
    j_matrix: DMatrix<f64>,
    r_matrix: DMatrix<f64>,
    b_matrix: DMatrix<f64>,
}

impl Toda {
    pub fn new(number_of_particles: usize, gamma: f64) -> Self {
        let n = number_of_particles;
        let i = DMatrix::identity(n, n);

        let mut j_matrix = DMatrix::zeros(2 * n, 2 * n);
        j_matrix.view_mut((0, n), (n, n)).copy_from(&i);
        j_matrix.view_mut((n, 0), (n, n)).copy_from(&(&i * -1.0));

        let mut r_matrix = DMatrix::zeros(2 * n, 2 * n);
        r_matrix.view_mut((n, n), (n, n)).copy_from(&(&i * gamma));

        let mut b_matrix = DMatrix::zeros(2 * n, 1);
        b_matrix[(n, 0)] = 1.0;

        Toda { n, gamma, j_matrix, r_matrix, b_matrix }
    }
}

impl NonlinearQSRSystem for Toda {
    fn state_dim(&self) -> usize {
        2 * self.n
    }

    fn control_dim(&self) -> usize {
        1
    }

    fn hamiltonian(&self, z: &DVector<f64>) -> f64 {
        let q = z.rows(0, self.n);
        let p = z.rows(self.n, self.n);
        let kinetic = 0.5 * p.dot(&p);
        let mut potential = 0.0;
        for i in 0..(self.n - 1) {
            potential += (q[i] - q[i + 1]).exp();
        }
        potential += (q[self.n - 1] - q[0]).exp();
        kinetic + potential - self.n as f64
    }

    fn eta(&self, z: &DVector<f64>) -> DVector<f64> {
        let q = z.rows(0, self.n);
        let p = z.rows(self.n, self.n);
        let mut grad_q = DVector::zeros(self.n);
        if self.n > 1 {
            let exp_wrap = (q[self.n - 1] - q[0]).exp();
            grad_q[0] = (q[0] - q[1]).exp() - exp_wrap;
            for i in 1..(self.n - 1) {
                grad_q[i] = (q[i] - q[i + 1]).exp() - (q[i - 1] - q[i]).exp();
            }
            grad_q[self.n - 1] = exp_wrap - (q[self.n - 2] - q[self.n - 1]).exp();
        }
        DVector::from_iterator(2 * self.n, grad_q.iter().chain(p.iter()).copied())
    }

    fn ham_eta(&self, z: &DVector<f64>) -> (f64, DVector<f64>) {
        (self.hamiltonian(z), self.eta(z))
    }

    fn f(&self, z: &DVector<f64>) -> DVector<f64> {
        (&self.j_matrix - &self.r_matrix) * self.eta(z)
    }

    fn g(&self, _z: &DVector<f64>) -> DMatrix<f64> {
        self.b_matrix.clone()
    }

    fn dynamics(&self, z: &DVector<f64>, u: &DVector<f64>) -> DVector<f64> {
        &self.f(z) + &self.g(z) * u
    }

    fn k(&self, _z: &DVector<f64>) -> DMatrix<f64> {
        DMatrix::zeros(1, 1)
    }

    fn qsr(&self) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>) {
        (
            DMatrix::zeros(1, 1),
            DMatrix::identity(1, 1) * 0.5,
            DMatrix::zeros(1, 1),
        )
    }

    fn ell(&self, z: &DVector<f64>) -> DVector<f64> {
        let eta_z = self.eta(z);
        let nabla_ham_p = eta_z.rows(self.n, self.n);
        nabla_ham_p * self.gamma.sqrt()
    }

    fn w(&self, _z: &DVector<f64>) -> DMatrix<f64> {
        DMatrix::zeros(self.n, 1)
    }

    fn initial_state(&self) -> DVector<f64> {
        DVector::from_iterator(
            2 * self.n,
            (0..self.n).map(|i| i as f64).chain((0..self.n).map(|_| 0.0)),
        )
    }
}
