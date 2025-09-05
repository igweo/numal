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

#[cfg(test)]
mod tests {
    use super::*;
    
    // eps_abs values per variant
    #[test]
    fn eps_abs_values_match_variants() {
        assert_eq!(Tolerance::Strict.eps_abs(), 1e-12);
        assert_eq!(Tolerance::Default.eps_abs(), 1e-8);
        assert_eq!(Tolerance::Loose.eps_abs(), 1e-6);
        assert_eq!(
            Tolerance::Custom { eps_abs: 1e-3, eps_rel: 2e-3 }.eps_abs(),
            1e-3
        );
    }

    // eps_rel values per variant
    #[test]
    fn eps_rel_values_match_variants() {
        assert_eq!(Tolerance::Strict.eps_rel(), 1e-10);
        assert_eq!(Tolerance::Default.eps_rel(), 1e-6);
        assert_eq!(Tolerance::Loose.eps_rel(), 1e-4);
        assert_eq!(
            Tolerance::Custom { eps_abs: 2e-3, eps_rel: 3e-3 }.eps_rel(),
            3e-3
        );
    }

    // is_close success cases
    #[test]
    fn is_close_equal_values() {
        assert!(matches!(is_close(1.0, 1.0, Tolerance::Default), Ok(true)));
    }

    #[test]
    fn is_close_within_absolute_tolerance() {
        // b = 0 => purely absolute tolerance
        assert!(matches!(is_close(5e-9, 0.0, Tolerance::Default), Ok(true)));
    }

    #[test]
    fn is_close_within_relative_tolerance() {
        // Allowed diff (Default) for b=1000: 1e-8 + 1e-6*1000 ≈ 0.00100001
        assert!(matches!(is_close(1000.0005, 1000.0, Tolerance::Default), Ok(true)));
    }

    #[test]
    fn is_close_negative_values_handled() {
        // Uses absolute values internally for relative part
        assert!(matches!(
            is_close(-1.0000005, -1.0, Tolerance::Default),
            Ok(true)
        ));
    }

    #[test]
    fn is_close_strict_tolerance_edges() {
        // For b = 1e6, Strict allows ≈ 1e-12 + 1e-10*1e6 = 1e-4
        assert!(matches!(is_close(1_000_000.00009, 1_000_000.0, Tolerance::Strict), Ok(true)));
        assert!(matches!(
            is_close(1_000_000.00011, 1_000_000.0, Tolerance::Strict),
            Err(NumalError::DidNotConverge)
        ));
    }

    #[test]
    fn is_close_loose_allows_larger_diff() {
        // For b = 1e6, Loose allows ≈ 1e-6 + 1e-4*1e6 ≈ 100
        assert!(matches!(is_close(1_000_050.0, 1_000_000.0, Tolerance::Loose), Ok(true)));
    }

    // is_close failure and edge cases
    #[test]
    fn is_close_outside_absolute_tolerance() {
        assert!(matches!(
            is_close(2e-8, 0.0, Tolerance::Default),
            Err(NumalError::DidNotConverge)
        ));
    }

    #[test]
    fn is_close_outside_relative_tolerance() {
        // Allowed diff for b=1000 (Default) is ≈ 0.00100001
        assert!(matches!(
            is_close(1001.002, 1000.0, Tolerance::Default),
            Err(NumalError::DidNotConverge)
        ));
    }

    #[test]
    fn is_close_with_custom_tolerance() {
        let tol = Tolerance::Custom { eps_abs: 1e-3, eps_rel: 1e-2 };
        // Allowed diff for b=10 is 1e-3 + 1e-2*10 = 0.101
        assert!(matches!(is_close(10.05, 10.0, tol), Ok(true)));
    }

    #[test]
    fn is_close_with_nan_returns_error() {
        assert!(matches!(
            is_close(f64::NAN, 1.0, Tolerance::Default),
            Err(NumalError::DidNotConverge)
        ));
        assert!(matches!(
            is_close(1.0, f64::NAN, Tolerance::Default),
            Err(NumalError::DidNotConverge)
        ));
    }
}
