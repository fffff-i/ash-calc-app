use serde::{Deserialize, Serialize};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Percent(pub f64);

impl Percent {
    /// 乗算に使う形式（例: 0.14514 → 1.14514）
    pub fn as_multiplier(self) -> f64 {
        1.0 + self.0
    }
}

impl Add for Percent {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Percent(self.0 + rhs.0)
    }
}

impl Sub for Percent {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Percent(self.0 - rhs.0)
    }
}

impl Mul<f64> for Percent {
    type Output = f64;
    fn mul(self, rhs: f64) -> f64 {
        self.0 * rhs
    }
}

impl Mul<Percent> for f64 {
    type Output = f64;
    fn mul(self, rhs: Percent) -> f64 {
        self * rhs.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct CritRate(pub f64);

impl CritRate {
    pub fn value(self) -> f64 {
        self.0
    }

    /// 敵の会心率耐性を引いた後の値を0.0〜0.8の間に丸め込む
    pub fn clamped_for_damage(self, resist: Percent) -> f64 {
        (self.0 - resist.0).clamp(0.0, 0.8)
    }
}
