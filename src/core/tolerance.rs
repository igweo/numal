use crate::NumalError;

/// Predefined tolerance levels
pub enum Tolerance {
    /// Custom absolute and relative tolerances
    Custom { eps_abs: f64, eps_rel: f64 },
    /// A 'stricter' tolerance (e.g. 1e-12, 1e-10)
    Strict,
    /// The default tolerance (e.g. 1e-8, 1e-6)
    Default,
    /// A loose tolerance (e.g. 1e-6, 1e-4)
    Loose,
}

impl Tolerance {
    pub fn eps_abs(&self) -> f64 {
        match *self {
            Tolerance::Custom {
                eps_abs,
                eps_rel: _,
            } => eps_abs,
            Tolerance::Strict => 1e-12,
            Tolerance::Default => 1e-8,
            Tolerance::Loose => 1e-6,
        }
    }
    pub fn eps_rel(&self) -> f64 {
        match *self {
            Tolerance::Custom {
                eps_abs: _,
                eps_rel,
            } => eps_rel,
            Tolerance::Strict => 1e-10,
            Tolerance::Default => 1e-6,
            Tolerance::Loose => 1e-4,
        }
    }
}

// Compare two values with absolute+relative criteria.
/// Returns Ok(true) if |a - b| <= eps_abs + eps_rel * |b|,
/// Ok(false) otherwise.
/// Return Err(NumalError::DidNotConverge) if values
/// are not within tolerance after being used in an iterative context.
pub fn is_close(a: f64, b: f64, tol: Tolerance) -> Result<bool, NumalError> {
    if (a - b).abs() <= tol.eps_abs() + tol.eps_rel() * b.abs() {
        Ok(true)
    } else {
        Err(NumalError::DidNotConverge)
    }
}
