use crate::skill::skill_list;
use crate::types::params::{AttackerStats, DamageCalcParams, SelectedSkill};
use serde::Serialize;

#[derive(Serialize)]
pub struct DamageBreakdown {
    pub non_crit_damage: f64,
    pub crit_damage: f64,
    pub expected_damage: f64,
    pub crit_rate: f64,
    pub crit_damage_ratio: f64,
    pub total_bonus_rate: f64,
    pub effective_def_ratio: f64,
    pub modified_def: u32,
}

pub fn calc_damage_verbose(
    params: &mut DamageCalcParams,
    selected_skills: &[SelectedSkill],
) -> DamageBreakdown {
    enrich_with_skills(&mut params.attacker, selected_skills);

    let effective_def_ratio = (params.target.def_ratio.0 - params.attacker.def_penetration_rate.0).clamp(0.0, 1.0);
    let modified_def = (params.target.def as f64 * effective_def_ratio).floor() as u32;
    let atk_f64 = params.attacker.atk as f64;
    let base = atk_f64 * atk_f64 / (params.attacker.atk + modified_def) as f64;

    let total_bonus_rate = {
        let mut sum = params.attacker.skill_bonus_rate.0;
        if params.target.has_bad_condition {
            sum += params.attacker.bad_condition_bonus_rate.0;
        }
        if params.target.is_high_hp {
            sum += params.attacker.hp_high_bonus_rate.0;
        } else {
            sum += params.attacker.hp_low_bonus_rate.0;
        }
        (sum - params.target.hp_bonus_resist_rate.0).max(0.0)
    };

    let vs_boss_or_monster_ratio = if params.target.is_boss {
        params.attacker.vs_boss_ratio.0
    } else {
        params.attacker.vs_monster_ratio.0
    };

    let non_crit_damage = base
        * params.attacker.damage_ratio.0
        * (1.0 + (params.attacker.intimidation - params.target.fortitude) * 0.1 * 0.01)
        * (1.0 + total_bonus_rate)
        * (1.0 + vs_boss_or_monster_ratio);
    let crit_rate = params
        .attacker
        .crit_rate
        .clamped_for_damage(params.target.crit_resist_rate);
    let crit_damage_ratio =
        (params.attacker.crit_damage_ratio.0 - params.target.crit_damage_resist_rate.0).max(0.0);
    let crit_damage = non_crit_damage * (1.0 + crit_damage_ratio);
    let expected_damage = crit_damage * crit_rate + non_crit_damage * (1.0 - crit_rate);

    DamageBreakdown {
        non_crit_damage,
        crit_damage,
        expected_damage,
        crit_rate,
        crit_damage_ratio,
        total_bonus_rate,
        effective_def_ratio,
        modified_def,
    }
}

fn enrich_with_skills(stats: &mut AttackerStats, selected: &[SelectedSkill]) {
    let list = skill_list();

    for s in selected {
        if let Some(skill) = list.iter().find(|x| x.id == s.id) {
            let m = &skill.modifiers;
            stats.atk += m.atk;
            stats.skill_bonus_rate.0 += m.skill_bonus_rate(s.additional_level);
            stats.crit_damage_ratio.0 += m.crit_damage_ratio;
            stats.vs_boss_ratio.0 += m.vs_boss_ratio;
            stats.vs_monster_ratio.0 += m.vs_monster_ratio;
        }
    }
}
