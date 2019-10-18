use crate::models::common::ObjectiveCost;
use crate::models::problem::Job;
use crate::models::{Problem, Solution};
use crate::utils::Random;
use std::collections::HashSet;
use std::sync::Arc;

/// Contains information needed to perform refinement.
pub struct RefinementContext {
    /// Original problem.
    pub problem: Arc<Problem>,

    /// Specifies sorted collection discovered and accepted solutions with their cost.
    pub population: Vec<(Solution, ObjectiveCost)>,

    /// Specifies refinement generation (or iteration).
    pub generation: usize,
}

impl RefinementContext {
    pub fn new(problem: Arc<Problem>) -> Self {
        Self { problem, population: vec![], generation: 0 }
    }
}

pub mod acceptance;
pub mod objectives;
pub mod recreate;
pub mod ruin;
pub mod termination;
