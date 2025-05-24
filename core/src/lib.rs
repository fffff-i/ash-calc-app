use serde_wasm_bindgen::{from_value, to_value};
use wasm_bindgen::prelude::*;

mod calc;
mod skill;
pub mod types;

use crate::calc::{calc, DamageCalcResult};
use skill::skill_list;
use types::params::{DamageCalcParams, SelectedSkill};

#[wasm_bindgen]
pub fn calc_js(params: JsValue, selected_skills: JsValue) -> JsValue {
    let mut base_params: DamageCalcParams = from_value(params).unwrap();
    let skills: Vec<SelectedSkill> = from_value(selected_skills).unwrap();

    let result: DamageCalcResult = calc(&mut base_params, &skills);
    to_value(&result).unwrap()
}

#[wasm_bindgen]
pub fn get_skill_list() -> JsValue {
    to_value(&skill_list()).unwrap()
}
