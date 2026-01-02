use nalgebra::{DVector, DMatrix, Matrix2, Vector2, Matrix2x1};
use std::f64::consts::PI;
use crate::helpers::nonlinear_system::NonlinearQSRSystem;

pub struct Pendulum {
    pub gravitation: f64,
    pub friction_coefficient: f64,
    pub j_matrix: Matrix2<f64>,
    pub r_matrix: Matrix2<f64>,
    pub b_matrix: Matrix2x1<f64>,
    pub initial_state_vec: Vector2<f64>,
}

impl Pendulum {
    pub fn new(
        initial_state: Option<Vector2<f64>>,
        friction_coefficient: f64,
        gravitation: f64,
    ) -> Self {
        let j_matrix = Matrix2::new(0.0, 1.0, -1.0, 0.0);
        let r_matrix = Matrix2::new(0.0, 0.0, 0.0, friction_coefficient);
        let b_matrix = Matrix2x1::new(0.0, 1.0);
        let initial_state_vec = initial_state.unwrap_or_else(|| Vector2::new(PI / 4.0, -1.0));

        Self {
            gravitation,
            friction_coefficient,
            j_matrix,
            r_matrix,
            b_matrix,
            initial_state_vec,
        }
    }
}

impl NonlinearQSRSystem for Pendulum {
    fn state_dim(&self) -> usize {
        2
    }

    fn control_dim(&self) -> usize {
        1
    }

    fn hamiltonian(&self, z: &DVector<f64>) -> f64 {
        self.gravitation * (1.0 - z[0].cos()) + 0.5 * z[1] * z[1]
    }

    fn eta(&self, z: &DVector<f64>) -> DVector<f64> {
        DVector::from_vec(vec![self.gravitation * z[0].sin(), z[1]])
    }

    fn ham_eta(&self, z: &DVector<f64>) -> (f64, DVector<f64>) {
        (self.hamiltonian(z), self.eta(z))
    }

    fn f(&self, z: &DVector<f64>) -> DVector<f64> {
        let j_minus_r = self.j_matrix - self.r_matrix;
        let eta_vec = self.eta(z);
        let eta_v2 = Vector2::new(eta_vec[0], eta_vec[1]);
        let result = j_minus_r * eta_v2;
        DVector::from_vec(vec![result[0], result[1]])
    }

    fn g(&self, _z: &DVector<f64>) -> DMatrix<f64> {
        DMatrix::from_iterator(2, 1, self.b_matrix.iter().copied())
    }

    fn dynamics(&self, z: &DVector<f64>, u: &DVector<f64>) -> DVector<f64> {
        &self.f(z) + &self.g(z) * u
    }

    fn k(&self, _z: &DVector<f64>) -> DMatrix<f64> {
        DMatrix::zeros(1, 1)
    }

    fn qsr(&self) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>) {
        let q = DMatrix::from_element(1, 1, -self.friction_coefficient);
        let s = DMatrix::from_element(1, 1, 0.5);
        let r = DMatrix::zeros(1, 1);
        (q, s, r)
    }

    fn ell(&self, _z: &DVector<f64>) -> DVector<f64> {
        DVector::from_element(1, 0.0)
    }

    fn w(&self, _z: &DVector<f64>) -> DMatrix<f64> {
        DMatrix::zeros(1, 1)
    }

    fn initial_state(&self) -> DVector<f64> {
        DVector::from_vec(vec![self.initial_state_vec[0], self.initial_state_vec[1]])
    }
}
