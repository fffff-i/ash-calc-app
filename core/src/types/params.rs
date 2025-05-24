use crate::types::value::{CritRate, Percent};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DamageCalcParams {
    pub attacker: AttackerStats,
    pub target: TargetStats,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttackerStats {
    pub atk: u32,
    pub def_penetration_rate: Percent,
    pub intimidation: f64,
    pub damage_ratio: Percent,
    pub crit_rate: CritRate,
    pub crit_damage_ratio: Percent,
    pub bad_condition_bonus_rate: Percent,
    pub hp_high_bonus_rate: Percent,
    pub hp_low_bonus_rate: Percent,
    pub skill_bonus_rate: Percent,
    pub vs_boss_ratio: Percent,
    pub vs_monster_ratio: Percent,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TargetStats {
    pub def: u32,
    pub def_ratio: Percent,
    pub fortitude: f64,
    pub crit_resist_rate: Percent,
    pub crit_damage_resist_rate: Percent,
    pub hp_bonus_resist_rate: Percent,
    pub is_boss: bool,
    pub is_high_hp: bool,
    pub has_bad_condition: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedSkill {
    pub id: String,           // スキルID
    pub additional_level: u8, // 追加スキルレベル
}
