use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

mod calc;
mod skill;
pub mod types;

use calc::{calc_damage_verbose, calc_damage_with_skills};
use skill::skill_list;
use types::params::{DamageParams, SelectedSkill};

#[wasm_bindgen]
pub fn calc_damage_with_skills_js(params: JsValue, skills: JsValue) -> f64 {
    let params: DamageParams = from_value(params).unwrap();
    let selected: Vec<SelectedSkill> = from_value(skills).unwrap();
    calc_damage_with_skills(&params, &selected)
}

#[wasm_bindgen]
pub fn calc_damage_verbose_js(params: JsValue, skills: JsValue) -> JsValue {
    calc_damage_verbose(params, skills)
}

#[wasm_bindgen]
pub fn get_skill_list() -> JsValue {
    to_value(&skill_list()).unwrap()
}
