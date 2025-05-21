use serde::Serialize;

#[derive(Serialize)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub max_additional_level: u8,
    pub modifiers: SkillModifiers,
}

#[derive(Serialize)]
pub struct SkillModifiers {
    pub skill_bonus_base: f64,
    pub skill_bonus_per_level: f64,
    pub atk_add: u32,
    pub crit_damage_add: f64,
    pub boss_bonus: f64,
    pub monster_bonus: f64,
}

pub fn skill_list() -> Vec<Skill> {
    vec![
        Skill {
            id: "fate_spell".to_string(),
            name: "フェイトスペル".to_string(),
            max_additional_level: 10,
            modifiers: SkillModifiers {
                skill_bonus_base: 50.0,
                skill_bonus_per_level: 2.0,
                atk_add: 0,
                crit_damage_add: 0.0,
                boss_bonus: 0.0,
                monster_bonus: 0.0,
            },
        },
        Skill {
            id: "resonance_caster".to_string(),
            name: "レゾナンストーン(自)".to_string(),
            max_additional_level: 6,
            modifiers: SkillModifiers {
                skill_bonus_base: 25.0,
                skill_bonus_per_level: 1.0,
                atk_add: 0,
                crit_damage_add: 0.0,
                boss_bonus: 0.0,
                monster_bonus: 0.0,
            },
        },
        Skill {
            id: "resonance_receiver".to_string(),
            name: "レゾナンストーン(被)".to_string(),
            max_additional_level: 6,
            modifiers: SkillModifiers {
                skill_bonus_base: 10.0,
                skill_bonus_per_level: 1.0,
                atk_add: 0,
                crit_damage_add: 0.0,
                boss_bonus: 0.0,
                monster_bonus: 0.0,
            },
        },
        Skill {
            id: "life_pot".to_string(),
            name: "生命ノ壺".to_string(),
            max_additional_level: 0,
            modifiers: SkillModifiers {
                skill_bonus_base: 0.0,
                skill_bonus_per_level: 0.0,
                atk_add: 2000,
                crit_damage_add: 30.0,
                boss_bonus: 36.2,
                monster_bonus: 36.2,
            },
        },
        Skill {
            id: "meteor_arrow".to_string(),
            name: "流星ノ裂矢".to_string(),
            max_additional_level: 0,
            modifiers: SkillModifiers {
                skill_bonus_base: 0.0,
                skill_bonus_per_level: 0.0,
                atk_add: 18000,
                crit_damage_add: 0.0,
                boss_bonus: 50.0,
                monster_bonus: 50.0,
            },
        },
    ]
}
