use trace_doc::{Artifact, Realization, Realizes, Requirement, Status};

/// The system shall be able to calculate the mathematical constant π.
pub struct ReqCalculatePI;
impl Artifact for ReqCalculatePI {
    const STATUS: Status = Status::Accepted;
}
impl Requirement for ReqCalculatePI {
    const TITLE: &str = "Calculate mathematical constant π";
}

/// This realization implements the calculation of π.
pub struct PICalculation; // implements pub fn calculate_pi() -> f64
impl Artifact for PICalculation {}
impl Realization for PICalculation {}
impl Realizes<ReqCalculatePI> for PICalculation {}
impl PICalculation {
    /// Calculates PI
    pub fn calculate_pi() -> f64 {
        // let's cheat a bit and return the PI constant directly for simplicity
        std::f64::consts::PI
    }
}

#[cfg(test)]
mod tests {
    use super::PICalculation;

    #[test]
    fn test_pi_calculation() {
        let pi = PICalculation::calculate_pi();
        // PI should be approximately 3.14159 to 5 decimal places
        assert!((pi - 3.14159).abs() < 0.00001);
    }

    #[test]
    fn test_pi_bounds() {
        let pi = PICalculation::calculate_pi();
        // PI should be between 3 and 4
        assert!(pi > 3.0);
        assert!(pi < 4.0);
    }
}
