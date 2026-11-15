//! In Vivo studies: cohorts/cages, animals, randomization, weight-based dosing,
//! timeseries measurements and welfare alerts. Pure Rust, offline, local-first.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Study {
    pub id: u64,
    pub name: String,
    pub species: String,
    pub strain: String,
    pub created: u64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Cohort {
    pub id: u64,
    pub study: u64,
    pub name: String,
    pub cage: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Animal {
    pub id: u64,
    pub cohort: u64,
    pub tag: String,
    pub sex: char,
    pub group: u32,
    pub baseline_weight_g: f32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measurement {
    pub animal: u64,
    pub day: u32,
    pub weight_g: f32,
    pub tumor_mm3: Option<f32>,
}

/// Deterministic (seeded) balanced randomization of animal ids into `groups`.
pub fn randomize(ids: &[u64], groups: u32, seed: u64) -> Vec<(u64, u32)> {
    let mut idx: Vec<usize> = (0..ids.len()).collect();
    // simple LCG shuffle for reproducibility without external deps
    let mut state = seed.wrapping_mul(6364136223846793005).wrapping_add(1);
    for i in (1..idx.len()).rev() {
        state = state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let j = (state >> 33) as usize % (i + 1);
        idx.swap(i, j);
    }
    let g = groups.max(1);
    idx.iter()
        .enumerate()
        .map(|(k, &i)| (ids[i], (k as u32) % g))
        .collect()
}

/// Dosing volume (mL) = dose (mg/kg) * weight (kg) / concentration (mg/mL).
pub fn dose_volume_ml(dose_mg_per_kg: f32, weight_g: f32, conc_mg_per_ml: f32) -> f32 {
    if conc_mg_per_ml <= 0.0 {
        return 0.0;
    }
    dose_mg_per_kg * (weight_g / 1000.0) / conc_mg_per_ml
}

/// Welfare alert when current weight drops >= `threshold` (fraction) below baseline.
pub fn welfare_alert(baseline_g: f32, current_g: f32, threshold: f32) -> bool {
    baseline_g > 0.0 && current_g < baseline_g * (1.0 - threshold)
}

/// Tumor growth inhibition vs a control mean (%).
pub fn tgi_percent(control_mean: f32, treated_mean: f32) -> f32 {
    if control_mean <= 0.0 {
        return 0.0;
    }
    (1.0 - treated_mean / control_mean) * 100.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn randomize_balanced_and_deterministic() {
        let ids: Vec<u64> = (1..=12).collect();
        let a = randomize(&ids, 3, 42);
        let b = randomize(&ids, 3, 42);
        assert_eq!(a, b);
        for g in 0..3 {
            assert_eq!(a.iter().filter(|(_, x)| *x == g).count(), 4);
        }
    }

    #[test]
    fn dosing_volume() {
        // 10 mg/kg, 20 g mouse, 5 mg/mL -> 0.04 mL
        let v = dose_volume_ml(10.0, 20.0, 5.0);
        assert!((v - 0.04).abs() < 1e-6);
        assert_eq!(dose_volume_ml(10.0, 20.0, 0.0), 0.0);
    }

    #[test]
    fn welfare_and_tgi() {
        assert!(welfare_alert(25.0, 19.0, 0.2)); // 24% loss
        assert!(!welfare_alert(25.0, 21.0, 0.2)); // 16% loss
        assert!((tgi_percent(1000.0, 400.0) - 60.0).abs() < 1e-3);
    }
}
