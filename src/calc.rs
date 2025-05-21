use crate::skill::{SkillModifiers, skill_list};
use crate::types::params::{DamageParams, SelectedSkill};
use crate::types::value::Percent;
use serde::Serialize;
use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
pub struct DamageBreakdown {
    pub non_crit: f64,
    pub crit: f64,
    pub expected_damage: f64,
    pub crit_rate: f64,
    pub crit_damage: f64,
    pub adjust_total: f64,
}

fn enrich_with_skills(base: &DamageParams, selected_skills: &[SelectedSkill]) -> DamageParams {
    let mut atk_add = 0;
    let mut skill_bonus_total = 0.0;
    let mut crit_damage_add = 0.0;
    let mut boss_bonus_add = 0.0;
    let mut monster_bonus_add = 0.0;

    for selected in selected_skills {
        if let Some(skill) = skill_list().iter().find(|s| s.id == selected.id) {
            let lv = selected.additional_level.min(skill.max_additional_level);
            let m: &SkillModifiers = &skill.modifiers;
            atk_add += m.atk_add;
            skill_bonus_total += m.skill_bonus_base + m.skill_bonus_per_level * lv as f64;
            crit_damage_add += m.crit_damage_add;
            boss_bonus_add += m.boss_bonus;
            monster_bonus_add += m.monster_bonus;
        }
    }

    let mut modified = base.clone();

    modified.atk = modified.atk + atk_add;
    modified.skill_bonus = modified.skill_bonus + Percent(skill_bonus_total);
    modified.crit_damage = modified.crit_damage + Percent(crit_damage_add);
    modified.boss_bonus = modified.boss_bonus + Percent(boss_bonus_add);
    modified.monster_bonus = modified.monster_bonus + Percent(monster_bonus_add);

    modified
}

fn calc_damage_base(p: &DamageParams) -> f64 {
    let atk = p.atk as f64;
    let def = p.enemy_def as f64;
    let def_effect = def * (1.0 + (p.enemy_def_rate - p.ignore_def_rate).0);
    let atk_value = atk * atk / def_effect;

    let abnormal_bonus = if p.is_abnormal {
        p.abnormal_bonus
    } else {
        Percent(0.0)
    };
    let hp_bonus = if p.is_high_hp {
        p.hp_high_bonus
    } else {
        p.hp_low_bonus
    };
    let adjust = abnormal_bonus + hp_bonus + p.skill_bonus;

    let effective_intimidation = (p.intimidation - p.fortitude).max(0.0);
    let intimidation_bonus = Percent((effective_intimidation / 10.0).floor() * 0.01);
    let race_bonus = if p.is_boss {
        p.boss_bonus
    } else {
        p.monster_bonus
    };

    atk_value
        * adjust.as_multiplier()
        * p.damage_amp.as_multiplier()
        * race_bonus.as_multiplier()
        * intimidation_bonus.as_multiplier()
}

#[wasm_bindgen]
pub fn calc_damage_verbose(params: JsValue, skills: JsValue) -> JsValue {
    let base: DamageParams = from_value(params).unwrap();
    let selected: Vec<SelectedSkill> = from_value(skills).unwrap();
    let p = enrich_with_skills(&base, &selected);

    let base_dmg = calc_damage_base(&p);
    let crit_rate = p.crit_rate.clamped_for_damage(p.enemy_crit_resist);
    let crit_dmg = p.crit_damage;

    let abnormal_bonus = if p.is_abnormal {
        p.abnormal_bonus
    } else {
        Percent(0.0)
    };
    let hp_bonus = if p.is_high_hp {
        p.hp_high_bonus
    } else {
        p.hp_low_bonus
    };
    let adjust_total = (abnormal_bonus + hp_bonus + p.skill_bonus).0;

    let result = DamageBreakdown {
        non_crit: base_dmg,
        crit: base_dmg * crit_dmg,
        expected_damage: (base_dmg * (1.0 - crit_rate) + base_dmg * crit_dmg * crit_rate),
        crit_rate,
        crit_damage: crit_dmg.0,
        adjust_total,
    };

    to_value(&result).unwrap()
}

pub fn calc_damage(p: &DamageParams) -> f64 {
    let atk = p.atk as f64;
    let def = p.enemy_def as f64;

    let def_effect = def * (1.0 + (p.enemy_def_rate - p.ignore_def_rate).0);
    let atk_value = atk * atk / def_effect;

    let abnormal_bonus = if p.is_abnormal {
        p.abnormal_bonus
    } else {
        Percent(0.0)
    };
    let hp_bonus = if p.is_high_hp {
        p.hp_high_bonus
    } else {
        p.hp_low_bonus
    };
    let adjust = abnormal_bonus + hp_bonus + p.skill_bonus;

    let crit = p.crit_rate.clamped_for_damage(p.enemy_crit_resist);
    let crit_factor = 1.0 + crit * p.crit_damage;

    let effective_intimidation = (p.intimidation - p.fortitude).max(0.0);
    let intimidation_bonus = Percent((effective_intimidation / 10.0).floor() * 0.01);

    let race_bonus = if p.is_boss {
        p.boss_bonus
    } else {
        p.monster_bonus
    };

    atk_value
        * adjust.as_multiplier()
        * p.damage_amp.as_multiplier()
        * crit_factor
        * race_bonus.as_multiplier()
        * intimidation_bonus.as_multiplier()
}

pub fn calc_damage_with_skills(base: &DamageParams, selected_skills: &[SelectedSkill]) -> f64 {
    let enriched = enrich_with_skills(base, selected_skills);
    calc_damage(&enriched)
}
