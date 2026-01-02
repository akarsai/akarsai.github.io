use nalgebra::{DMatrix, DVector};

/// Trait for nonlinear affine systems in port-Hamiltonian form
/// 
/// z' = f(z) + g(z)u,  z(0) = z_0
/// y  = h(z) + k(z)u
///
/// with QSR-dissipativity structure
pub trait NonlinearQSRSystem {
    /// State dimension
    fn state_dim(&self) -> usize;
    
    /// Control input dimension
    fn control_dim(&self) -> usize;

    /// Hamiltonian (energy function)
    fn hamiltonian(&self, z: &DVector<f64>) -> f64;

    /// Gradient of Hamiltonian
    fn eta(&self, z: &DVector<f64>) -> DVector<f64>;

    /// Combined Hamiltonian and gradient
    fn ham_eta(&self, z: &DVector<f64>) -> (f64, DVector<f64>);

    /// Drift dynamics f(z) = (J - R) * ∇H(z)
    fn f(&self, z: &DVector<f64>) -> DVector<f64>;

    /// Input matrix g(z)
    fn g(&self, z: &DVector<f64>) -> DMatrix<f64>;

    /// Full dynamics: z' = f(z) + g(z)*u
    fn dynamics(&self, z: &DVector<f64>, u: &DVector<f64>) -> DVector<f64>;

    /// Output function h(z) (could be eta or other)
    fn h(&self, z: &DVector<f64>) -> DVector<f64> {
        self.eta(z)
    }

    /// Feedthrough term k(z)
    fn k(&self, z: &DVector<f64>) -> DMatrix<f64>;

    /// Output: y = h(z) + k(z)*u
    fn output(&self, z: &DVector<f64>, u: &DVector<f64>) -> DVector<f64> {
        &self.h(z) + &self.k(z) * u
    }

    /// QSR matrices for supply rate s(y, u) = y^T Q y + 2 y^T S u + u^T R u
    fn qsr(&self) -> (DMatrix<f64>, DMatrix<f64>, DMatrix<f64>);

    /// Output dissipation term ℓ(z)
    fn ell(&self, z: &DVector<f64>) -> DVector<f64>;

    /// W matrix in Hill-Moylan conditions
    fn w(&self, z: &DVector<f64>) -> DMatrix<f64>;

    /// Initial state
    fn initial_state(&self) -> DVector<f64>;
}
