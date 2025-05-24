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
    skill_bonus_rate_base: f64,
    skill_bonus_rate_per_level: f64,
    pub atk: u32,
    pub crit_damage_ratio: f64,
    pub vs_boss_ratio: f64,
    pub vs_monster_ratio: f64,
}

impl SkillModifiers {
    pub fn skill_bonus_rate(&self, level: u8) -> f64 {
        self.skill_bonus_rate_base + self.skill_bonus_rate_per_level * level as f64
    }
}

pub fn skill_list() -> Vec<Skill> {
    vec![
        Skill {
            id: "fate_spell".to_string(),
            name: "フェイトスペル".to_string(),
            max_additional_level: 10,
            modifiers: SkillModifiers {
                skill_bonus_rate_base: 0.5,
                skill_bonus_rate_per_level: 0.02,
                atk: 0,
                crit_damage_ratio: 0.0,
                vs_boss_ratio: 0.0,
                vs_monster_ratio: 0.0,
            },
        },
        Skill {
            id: "resonance_caster".to_string(),
            name: "レゾナンストーン(自)".to_string(),
            max_additional_level: 6,
            modifiers: SkillModifiers {
                skill_bonus_rate_base: 0.25,
                skill_bonus_rate_per_level: 0.01,
                atk: 0,
                crit_damage_ratio: 0.0,
                vs_boss_ratio: 0.0,
                vs_monster_ratio: 0.0,
            },
        },
        Skill {
            id: "resonance_receiver".to_string(),
            name: "レゾナンストーン(被)".to_string(),
            max_additional_level: 6,
            modifiers: SkillModifiers {
                skill_bonus_rate_base: 0.1,
                skill_bonus_rate_per_level: 0.01,
                atk: 0,
                crit_damage_ratio: 0.0,
                vs_boss_ratio: 0.0,
                vs_monster_ratio: 0.0,
            },
        },
        Skill {
            id: "life_pot".to_string(),
            name: "生命ノ壺".to_string(),
            max_additional_level: 0,
            modifiers: SkillModifiers {
                skill_bonus_rate_base: 0.0,
                skill_bonus_rate_per_level: 0.0,
                atk: 2000,
                crit_damage_ratio: 0.30,
                vs_boss_ratio: 0.362,
                vs_monster_ratio: 0.362,
            },
        },
        Skill {
            id: "meteor_arrow".to_string(),
            name: "流星ノ裂矢".to_string(),
            max_additional_level: 0,
            modifiers: SkillModifiers {
                skill_bonus_rate_base: 0.0,
                skill_bonus_rate_per_level: 0.0,
                atk: 18000,
                crit_damage_ratio: 0.0,
                vs_boss_ratio: 0.5,
                vs_monster_ratio: 0.5,
            },
        },
    ]
}
