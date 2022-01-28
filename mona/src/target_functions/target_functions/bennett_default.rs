use crate::artifacts::{Artifact, ArtifactSetName};
use crate::artifacts::effect_config::ArtifactEffectConfig;
use crate::attribute::{Attribute, AttributeName, SimpleAttributeGraph2};
use crate::character::Character;
use crate::character::characters::Bennett;
use crate::character::skill_config::CharacterSkillConfig;
use crate::character::traits::CharacterTrait;
use crate::common::StatName;
use crate::damage::{DamageContext, SimpleDamageBuilder};
use crate::enemies::Enemy;
use crate::target_functions::target_function_opt_config::TargetFunctionOptConfig;
use crate::target_functions::TargetFunction;
use crate::team::TeamQuantization;
use crate::weapon::Weapon;

pub struct BennettDefaultTargetFunction;

impl TargetFunction for BennettDefaultTargetFunction {
    fn get_target_function_opt_config(&self) -> TargetFunctionOptConfig {
        TargetFunctionOptConfig {
            atk_fixed: 0.0,
            atk_percentage: 0.0,
            hp_fixed: 0.1,
            hp_percentage: 1.0,
            def_fixed: 0.0,
            def_percentage: 0.0,
            recharge: 1.0,
            elemental_mastery: 0.0,
            critical: 0.0,
            critical_damage: 0.0,
            bonus_electro: 0.0,
            bonus_pyro: 0.0,
            bonus_hydro: 0.0,
            bonus_anemo: 0.0,
            bonus_cryo: 0.0,
            bonus_geo: 0.0,
            bonus_dendro: 0.0,
            bonus_physical: 0.0,
            sand_main_stats: vec![
                StatName::HPPercentage,
                StatName::Recharge
            ],
            goblet_main_stats: vec![
                StatName::HPPercentage
            ],
            head_main_stats: vec![
                StatName::HPPercentage,
                StatName::HealingBonus
            ],
            set_names: Some(vec![
                ArtifactSetName::NoblesseOblige
            ]),
        }
    }

    fn get_default_artifact_config(&self, _team_config: &TeamQuantization) -> ArtifactEffectConfig {
        ArtifactEffectConfig {
            config_archaic_petra: Default::default(),
            config_berserker: Default::default(),
            config_blizzard_strayer: Default::default(),
            config_bloodstained_chivalry: Default::default(),
            config_brave_heart: Default::default(),
            config_crimson_witch_of_flames: Default::default(),
            config_heart_of_depth: Default::default(),
            config_husk_of_opulent_dreams: Default::default(),
            config_instructor: Default::default(),
            config_lavawalker: Default::default(),
            config_martial_artist: Default::default(),
            config_noblesse_oblige: Default::default(),
            config_pale_flame: Default::default(),
            config_retracing_bolide: Default::default(),
            config_shimenawas_reminiscence: Default::default(),
            config_tenacity_of_the_millelith: Default::default(),
            config_thundersoother: Default::default()
        }
    }

    fn target(&self, attribute: &SimpleAttributeGraph2, character: &Character<SimpleAttributeGraph2>, _weapon: &Weapon<SimpleAttributeGraph2>, artifacts: &Vec<&Artifact>, enemy: &Enemy) -> f64 {
        let mut noblesse_count = 0;
        for artifact in artifacts.iter() {
            if artifact.set_name == ArtifactSetName::NoblesseOblige {
                noblesse_count += 1;
            }
        }

        let mut atk_bonus = Bennett::atk_bonus(&character.common_data, attribute);
        const VIRTUAL_BASE_ATK: f64 = 700.0;
        if noblesse_count >= 4 {
            atk_bonus += VIRTUAL_BASE_ATK * 0.2;
        }

        let context: DamageContext<'_, SimpleAttributeGraph2> = DamageContext {
            character_common_data: &character.common_data,
            attribute,
            enemy
        };
        let heal = Bennett::damage::<SimpleDamageBuilder>(
            &context,
            <Bennett as CharacterTrait>::DamageEnumType::QHeal,
            &CharacterSkillConfig::NoConfig
        ).normal.expectation;

        let recharge = attribute.get_value(AttributeName::Recharge);
        let recharge_ratio = recharge.min(1.5);

        recharge_ratio * (atk_bonus * 1000.0 + heal)
    }
}