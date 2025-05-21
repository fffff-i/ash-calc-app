use crate::types::value::{CritRate, Percent};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DamageParams {
    pub atk: u32,                   // 攻撃力（攻撃力 * 攻撃倍率）
    pub enemy_def: u32,             // 敵の防御力
    pub enemy_def_rate: Percent,    // 敵の防御率
    pub ignore_def_rate: Percent,   // 防御無視
    pub intimidation: f64,          // 威圧（威圧倍率適用後の値）
    pub fortitude: f64,             // 不屈（入力値）
    pub damage_amp: Percent,        // ダメージ倍率
    pub abnormal_bonus: Percent,    // 状態異常中の敵への与ダメージ増加
    pub hp_high_bonus: Percent,     // HP50%以上の敵への与ダメージ増加
    pub hp_low_bonus: Percent,      // HP50%未満の敵への与ダメージ増加
    pub skill_bonus: Percent,       // スキル由来の与ダメージ増加（例: フェイトスペル）
    pub boss_bonus: Percent,        // 対ボス与ダメージ増加
    pub monster_bonus: Percent,     // 対魔物（非ボス）与ダメージ増加
    pub crit_rate: CritRate,        // 会心率
    pub enemy_crit_resist: Percent, // 敵の会心率耐性
    pub crit_damage: Percent,       // 会心ダメージ
    pub is_boss: bool,              // 敵がボスかどうか
    pub is_high_hp: bool,           // 敵のHPが50%以上かどうか
    pub is_abnormal: bool,          // 敵が状態異常かどうか
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SelectedSkill {
    pub id: String,     // スキルID
    pub additional_level: u8, // 追加スキルレベル
}
