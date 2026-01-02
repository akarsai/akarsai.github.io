pub mod core;
pub mod examples;
pub mod helpers;

use wasm_bindgen::prelude::*;
use nalgebra::{DVector, DMatrix};

use crate::examples::toda::Toda;
use crate::examples::pendulum::Pendulum;
use crate::core::time_discretization::{implicit_midpoint, qsr_discrete_gradient};
use crate::helpers::nonlinear_system::NonlinearQSRSystem;

#[wasm_bindgen]
pub fn compute_energy(
    kind: &str,
    t_final: f64,
    delta_t: f64,
    integrator: &str,
) -> Vec<f64> {
    // setup system
    let system: Box<dyn NonlinearQSRSystem> = if kind == "toda" {
        Box::new(Toda::new(5, 0.1))
    } else if kind == "pendulum" {
        Box::new(Pendulum::new(None, 0.2, 9.81))
    } else {
        panic!("unknown kind!");
    };

    // setup time grid and control input
    let nt = (t_final / delta_t) as usize;
    let tt = DVector::from_iterator(nt, (0..nt).map(|i| i as f64 * delta_t));
    let uu = DMatrix::zeros(nt, 1);
    
    // solve
    let zz = if integrator == "implicit midpoint" {
        implicit_midpoint(
            |z, u| system.dynamics(z, u),
            &tt,
            &system.initial_state(),
            &uu,
        )
    } else if integrator == "discrete gradient" {
        qsr_discrete_gradient(
            |z| system.f(z),
            |z| system.g(z),
            |z| system.k(z),
            |z| system.ham_eta(z),
            |z| system.ell(z),
            |z| system.w(z),
            &system.qsr(),
            &tt,
            &system.initial_state(),
            &uu,
        )
    } else {
        panic!("unknown integrator!");
    };
    
    // compute energy
    let hh: Vec<f64> = (0..nt)
        .map(|i| system.hamiltonian(&zz.row(i).transpose()))
        .collect();
    
    hh
}