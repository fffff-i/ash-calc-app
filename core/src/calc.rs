use crate::skill::skill_list;
use crate::types::params::{AttackerStats, DamageCalcParams, SelectedSkill};
use serde::Serialize;

#[derive(Serialize)]
pub struct DamageCalcResult {
    pub breakdown: DamageBreakdown,
    pub sensitivity: DamageSensitivity
}

#[derive(Serialize)]
pub struct DamageSensitivity {
    pub atk_ratio_per_percent: f64,
    pub intimidation_ratio_per_percent: f64,
    pub damage_ratio_per_percent: f64,   
    pub crit_rate_per_percent: f64,
    pub crit_damage_ratio_per_percent: f64,
    pub total_bonus_rate_per_percent: f64,
    pub vs_boss_or_monster_ratio_per_percent: f64
}

#[derive(Serialize)]
pub struct DamageBreakdown {
    pub non_crit_damage: f64,
    pub crit_damage: f64,
    pub expected_damage: f64,
    pub crit_rate: f64,
    pub crit_damage_ratio: f64,
    pub total_bonus_rate: f64,
    pub effective_def_ratio: f64,
    pub modified_def: u32
}

pub(crate) fn calc(params: &mut DamageCalcParams, selected_skills: &[SelectedSkill]) -> DamageCalcResult {
    enrich_with_skills(&mut params.attacker, selected_skills);
    let damage_breakdown = calc_internal(params);
    let base_damage = damage_breakdown.expected_damage;
    
    let mut params_atk_ratio = params.clone();
    params_atk_ratio.attacker.atk_ratio.0 += 0.01;
    let atk_ratio_damage = calc_internal(&mut params_atk_ratio).expected_damage;
    let atk_ratio_per_percent = (atk_ratio_damage - base_damage) / base_damage * 100.0;

    let mut params_intimidation_ratio = params.clone();
    params_intimidation_ratio.attacker.intimidation_ratio.0 += 0.01;
    let intimidation_ratio_damage = calc_internal(&mut params_intimidation_ratio).expected_damage;
    let intimidation_ratio_per_percent = (intimidation_ratio_damage - base_damage) / base_damage * 100.0;

    let mut params_damage_ratio = params.clone();
    params_damage_ratio.attacker.damage_ratio.0 += 0.01;
    let damage_ratio_damage = calc_internal(&mut params_damage_ratio).expected_damage;
    let damage_ratio_per_percent = (damage_ratio_damage - base_damage) / base_damage * 100.0;

    let mut params_crit_rate = params.clone();
    params_crit_rate.attacker.crit_rate.0 += 0.01;
    let crit_rate_damage = calc_internal(&mut params_crit_rate).expected_damage;
    let crit_rate_per_percent = (crit_rate_damage - base_damage) / base_damage * 100.0;
    
    let mut params_crit_damage = params.clone();
    params_crit_damage.attacker.crit_damage_ratio.0 += 0.01;
    let crit_damage_damage = calc_internal(&mut params_crit_damage).expected_damage;
    let crit_damage_ratio_per_percent = (crit_damage_damage - base_damage) / base_damage * 100.0;
    
    let mut params_bonus = params.clone();
    params_bonus.attacker.skill_bonus_rate.0 += 0.01;
    let bonus_damage = calc_internal(&mut params_bonus).expected_damage;
    let total_bonus_rate_per_percent = (bonus_damage - base_damage) / base_damage * 100.0;
    
    let mut params_vs = params.clone();
    if params.target.is_boss {
        params_vs.attacker.vs_boss_ratio.0 += 0.01;
    } else {
        params_vs.attacker.vs_monster_ratio.0 += 0.01;
    }
    let vs_damage = calc_internal(&mut params_vs).expected_damage;
    let vs_boss_or_monster_ratio_per_percent = (vs_damage - base_damage) / base_damage * 100.0;
    
    DamageCalcResult {
        breakdown: damage_breakdown,
        sensitivity: DamageSensitivity {
            atk_ratio_per_percent,
            intimidation_ratio_per_percent,
            damage_ratio_per_percent,
            crit_rate_per_percent,
            crit_damage_ratio_per_percent,
            total_bonus_rate_per_percent,
            vs_boss_or_monster_ratio_per_percent
        }
    }
}

fn calc_internal(
    params: &mut DamageCalcParams
) -> DamageBreakdown {
    let effective_def_ratio = (params.target.def_ratio.0 - params.attacker.def_penetration_rate.0).clamp(0.0, 1.0);
    let modified_def = (params.target.def as f64 * effective_def_ratio).floor() as u32;
    let modified_atk = (params.attacker.base_atk as f64 * (1.0 + params.attacker.atk_ratio.0)).floor() as u32;
    let modified_atk_f64 = modified_atk as f64;
    let base = modified_atk_f64 * modified_atk_f64 / (modified_atk + modified_def) as f64;

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
    
    let modified_intimidation = (params.attacker.base_intimidation * (1.0 + params.attacker.intimidation_ratio.0)).floor();

    let non_crit_damage = base
        * params.attacker.damage_ratio.0
        * (1.0 + (modified_intimidation - params.target.fortitude) * 0.1 * 0.01)
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
        modified_def
    }
}

fn enrich_with_skills(stats: &mut AttackerStats, selected: &[SelectedSkill]) {
    let list = skill_list();

    for s in selected {
        if let Some(skill) = list.iter().find(|x| x.id == s.id) {
            let m = &skill.modifiers;
            stats.base_atk += m.atk;
            stats.skill_bonus_rate.0 += m.skill_bonus_rate(s.additional_level);
            stats.crit_damage_ratio.0 += m.crit_damage_ratio;
            stats.vs_boss_ratio.0 += m.vs_boss_ratio;
            stats.vs_monster_ratio.0 += m.vs_monster_ratio;
        }
    }
}
