use std::collections::HashMap;

use bevy_reflect::{FromReflect, Reflect};
use mlua::LuaSerdeExt;
use serde::{Deserialize, Serialize};

use crate::{
    editable::{Editable, FieldType, HeaderField, ValidationResult},
    serde_utils::LuaFileBased,
};

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct ConfMod {
    #[serde(rename = "SelectLoadout")]
    pub select_loadout: String,
    #[serde(rename = "PruneScriptConf")]
    pub prune_script: PruneScriptConf,

    #[serde(rename = "AIemergencyLaunch")]
    pub ai_emergency_launch: bool,

    pub parking_hotstart: bool,
    pub intercept_hotstart: u32,
    pub startup_time_player: u32,

    pub failure: bool,
    #[serde(rename = "failureProbMax")]
    pub failure_prob_max: u32,
    #[serde(rename = "failureNbMax")]
    pub failure_number_max: u32,

    #[serde(rename = "Keep_USNdeckCrew")]
    pub keep_usn_deck_crew: bool,
    #[serde(rename = "OnlyDayMission")]
    pub only_day_mission: bool,
    #[serde(rename = "HourlyTolerance")]
    pub hourly_tolerance: u32,
    #[serde(rename = "MovedBullseye")]
    pub move_bullseye: bool,
    #[serde(rename = "TriggerStart")]
    pub trigger_start: bool,

    #[serde(rename = "CVN_CleanDeck")]
    pub cvn_clean_deck: bool,
    #[serde(rename = "CVN_TimeBtwPlane")]
    pub cvn_time_between_plane: u32,
    #[serde(rename = "CVN_Vmax")]
    pub cvn_vmax: u32,
    #[serde(rename = "CVN_windDeck")]
    pub cvn_wind_deck: u32,
    #[serde(rename = "CVN_despawnAfterLanding")]
    pub cvn_despawn_after_landing: bool,
    #[serde(rename = "SC_SpawnOn")]
    pub sc_spawn_on: HashMap<String, String>,
    #[serde(rename = "SC_CarrierIntoWind")]
    pub sc_carrier_into_wind: String,
    #[serde(rename = "MP_PlaneRecovery")]
    pub mp_plane_recovery: u32,
    #[serde(rename = "WrittenOnScratchpadMod")]
    pub written_scratchpad_mod: bool,
    #[serde(rename = "backupAllMissionFiles")]
    pub backup_mission_files: bool,
    #[serde(rename = "Slider_CampaignDuration")]
    pub slider_campaign_duration: bool,
    #[serde(rename = "Slider_EnemyLevel")]
    pub slider_enemy_level: bool,
    #[serde(rename = "RandomizeSkills")]
    pub randomize_skills: bool,
    #[serde(rename = "Slider_PercentPlane")]
    pub slider_percent_plane: bool,
    #[serde(rename = "SilenceATC")]
    pub silence_atc: String,
    #[serde(rename = "load_CTLD")]
    pub load_ctld: bool,
    #[serde(rename = "load_mist")]
    pub load_mist: bool,
    #[serde(rename = "cheat_Mode_Eye")]
    pub cheat_mode_eye: bool,
    #[serde(rename = "ejectedPilotFrequency")]
    pub ejected_pilot_frequency: EjectedPilotFrequencies,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct PruneScriptConf {
    #[serde(rename = "PruneScript")]
    pub prune_script: bool,
    #[serde(rename = "PruneAggressiveness")]
    pub prune_aggressiveness: u32,
    #[serde(rename = "PruneStatic")]
    pub prune_static: bool,
    #[serde(rename = "ForcedPruneSam")]
    pub force_prune_sam: bool,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct EjectedPilotFrequencies {
    pub blue: EjectedFrequencies,
    pub red: EjectedFrequencies,
    pub neutral: EjectedFrequencies,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Reflect, FromReflect)]
pub struct EjectedFrequencies {
    #[serde(rename = "GuardEjection")]
    pub guard: u32,
    #[serde(rename = "radioBeacon")]
    pub beacon: u32,
}

impl ConfMod {
    pub fn new() -> ConfMod {
        ConfMod {
            select_loadout: "init".into(),
            prune_script: PruneScriptConf {
                prune_script: false,
                prune_aggressiveness: 0,
                prune_static: false,
                force_prune_sam: false,
            },
            ai_emergency_launch: true,
            parking_hotstart: false,
            intercept_hotstart: 2,
            startup_time_player: 600,
            failure: false,
            failure_prob_max: 10,
            failure_number_max: 5,
            keep_usn_deck_crew: false,
            only_day_mission: true,
            hourly_tolerance: 2,
            move_bullseye: true,
            trigger_start: true,
            cvn_clean_deck: false,
            cvn_time_between_plane: 45,
            cvn_vmax: 10,
            cvn_wind_deck: 9,
            cvn_despawn_after_landing: true,
            sc_spawn_on: HashMap::from([
                ("F-14B".into(), "deck".into()),
                ("E-2C".into(), "deck".into()),
                ("S-3B Tanker".into(), "deck".into()),
                ("Pedro".into(), "deck".into()),
            ]),
            sc_carrier_into_wind: "man".into(),
            mp_plane_recovery: 2,
            written_scratchpad_mod: true,
            backup_mission_files: true,
            slider_campaign_duration: false,
            slider_enemy_level: false,
            randomize_skills: false,
            slider_percent_plane: false,
            silence_atc: "auto".into(),
            load_ctld: false,
            load_mist: false,
            cheat_mode_eye: false,
            ejected_pilot_frequency: EjectedPilotFrequencies {
                blue: EjectedFrequencies {
                    guard: 243000000,
                    beacon: 121500000,
                },
                red: EjectedFrequencies {
                    guard: 114115000,
                    beacon: 114585000,
                },
                neutral: EjectedFrequencies {
                    guard: 243000000,
                    beacon: 121500000,
                },
            },
        }
    }
}

impl Editable for ConfMod {
    fn get_name(&self) -> String {
        "conf_mod".into()
    }

    fn validate(&self, _: &crate::DCEInstance) -> crate::editable::ValidationResult {
        ValidationResult::Pass
    }

    fn get_mut_by_name<'a>(instance: &'a mut crate::DCEInstance, _: &str) -> &'a mut Self
    where
        Self: Sized,
    {
        &mut instance.conf_mod
    }

    fn get_header() -> Vec<crate::editable::HeaderField>
    where
        Self: Sized,
    {
        vec![
            HeaderField::new(
                "startup_time_player",
                "Startup time for Player",
                FieldType::IntTime,
                true,
            ),
            HeaderField::new(
                "parking_hotstart",
                "Start Parking Hot",
                FieldType::Bool,
                true,
            ),
            HeaderField::new(
                "only_day_mission",
                "Only Day Missions?",
                FieldType::Bool,
                true,
            ),
            HeaderField::new(
                "hourly_tolerance",
                "Day mission tolerance time",
                FieldType::Int,
                true,
            ),
            HeaderField::new(
                "move_bullseye",
                "Move Bullseye each mission",
                FieldType::Bool,
                true,
            ),
            HeaderField::new(
                "mp_plane_recovery",
                "MP recovery slots per flight",
                FieldType::Int,
                true,
            ),
        ]
    }

    fn delete_by_name(_: &mut crate::DCEInstance, _: &str) -> Result<(), anyhow::Error>
    where
        Self: Sized,
    {
        Err(anyhow::anyhow!(
            "Can't delete the configuration modifications!"
        ))
    }
}

impl Default for ConfMod {
    fn default() -> Self {
        Self::new()
    }
}

impl LuaFileBased<'_> for ConfMod {
    fn to_lua_str(&self, key: &str) -> Result<String, anyhow::Error> {
        let lua = mlua::Lua::new();
        // load utils:
        crate::lua_utils::load_utils(&lua)?;

        lua.globals().set(key.to_string(), lua.to_value(&self)?)?;

        let table = lua
            .load(&format!("TableSerialization({}, 0)", &key))
            .eval::<String>()?;

        let version_str = r#"
if not versionDCE then versionDCE = {} end
versionDCE["conf_mod_check.lua"] = "1.35.86"

-- Force your own options rather than those of base_ini.miz, which correspond to those of PBO-CEF ^^
-- Force vos propres options plutot que ceux de base_ini.miz, qui correspondent � ceux de PBO-CEF ^^
mission_forcedOptions = {
	["wakeTurbulence"]  = true,								-- False / true : turbulence  [MP: recommend: false]
	["labels"]  = 0,										-- etiquette : ( 0 : aucune �tiquette ) || ( 1 : �tiquette PLEINE ) || ( 2 : �tiquette abr�g�e )|| ( 3 : �tiquette Plate )
	["optionsView"]  = "optview_all",						-- Vue de la Map F10: ( "optview_onlymap": ONLY the MAP) || ( "optview_myaircraft": only my plane on map) || ( "optview_allies": fog of war) || ( "optview_onlyallies" : Allied only  ) || ( "optview_all" : every visible targets and planes on map allowed by campaign maker : usefull to program JDAM or JSAW | non target units will stay invisible to player )
	["externalViews"]  = true,								-- False / true : Vue externe
	["permitCrash"]  = true,								-- False / true : R�cup�ration de crash
	["miniHUD"]  = false,									-- False / true : Mini HUD
	["cockpitVisualRM"]  = true,							-- False / true : Mod reconnaissance Visuel dans le cockoit
	["userMarks"]  = true,									-- False / true : autorise les marqueurs sur la vue MAP F10
	["civTraffic"]  = "",									-- Traffic civil routier : ( "" : OFF ) || ( "low" : BAS ) || ( "medium" : MOYEN )|| ( "high" : ELEVE )  [MP: recommend: ""]
	["birds"]  = 0,											-- Collision volatile (probabilit�) ( 0 � 1000 )  [MP: recommend: 0]
	["cockpitStatusBarAllowed"]  = false,					-- False / true : Barre d'�tat cockpit
	["RBDAI"]  = true,										-- False / true : Evaluation des dommages au combat
}

TargetPointF14_BullsToFP = true							-- assigns the BullsEye position to the NavPoint FP of the F-14
			
-- limit the number of F-14s (in the same Flight as the player) on the CVN to avoid taxiing collisions
limiteNbF14CVN = 4										-- advice 3 max is a good value





-- 2 ############################################################################################################################################################
-- 2 ############################################################################################################################################################
--The options in this second part are exclusively reserved for the campaign editor. Players must not modify them.
-- 2 ############################################################################################################################################################	
-- 2 ############################################################################################################################################################


Debug = {
	debug  = false,											--(replaces the variable Init/camp/debug), when the mission was created and creates some files in the /debug folder During the DCE/DCS game, enter information in the DCS log and/or in the /Debug folder of the campaign.								
	debugInGamePopup  = false,								--popup the lua/DCS bug window in game, be careful, it blocks the game

	AfficheFailure  = false,								-- affiche les infos Pannes Al�atoires
	AfficheFlight  = false,									-- affiche les infos des packages cr��s dans ATO_FlightPlan
	AfficheSol  = false,									-- affiche les infos des cibles encore intactes
	KillGround = {
		flag  = false,											-- Active la destruction al�atoires des cibles, via les options plus bas
		sideGround  = "red",									-- le camp o� l'on veut d�truire les unit�s
		sideTarget  = "blue",									-- les targets de notre camp
		pourcent  = 50,										-- pourcentage de chance que l'unit� soit d�truite (juste l'unit�, pas le groupe)
	},
	Generator = {
		affiche  = false,										-- affiche les infos des premiers vols cr��s dans ATO_Generator
		chapter   = "ABC",										-- affiche les infos des 3 parties de ATO_Generator (ABC)
		nb  = 10,												-- nb de vol � afficher
		SpySquad  = "1052 Squadron",							-- affiche le passage de ce squad dans ATO_Generator
		SpyTask  = "Strike",									-- affiche le passage de ce squad ET de son Task dans ATO_Generator
		SpyTarget  = "HawkSiteNorthWest",						-- affiche le passage de ce Target dans ATO_Generator
	},
	checkTargetName  = true,								-- FirsMission Alerte si les noms des targets possede 1 espace en premier ou en dernier
	checkTargetName2Space  = true,							-- FirsMission Alerte si les noms des targets possede 2 espaces cons�cutif			
}

-- soit avec une valeur par caterorie RepairSAM RepairAirbase etc
campMod = {
	RepairMinimumDestroyed  = 25,							-- ne r�pare pas si le target.alive est inf�rieur �
	RepairSAM  = 15,										-- en %, Only CampaignMaker please
	RepairAirbase  = 12,									-- en %, Only CampaignMaker please
	RepairStation  = 8,										-- en %, Only CampaignMaker please
	RepairBridge  = 8,										-- en %, Only CampaignMaker please
	Repair  = 2,											-- en %, Only CampaignMaker please	
	
	KillTargetValue  = 20,									-- en %, si la vie du Target est < 20%, on d�clare les survivants mort, pour �viter d'y retourner.
	
	----attention, name of the map in lower case 
	MovedBullseye = {
		caucasus = {
			pos = {
				x  = -281713,										--Kolkhi']
				y  = 647369,											--Kolkhi']
			},
			rayon  = 200,											-- distance en Km autour de laquelle on peut placer le bullsEye
		},
		persiangulf = {
			pos = {
				x =	282607.962896,
				y = 141685.262108,									-- Jiroft airport
			},
			rayon  = 200,											-- distance en Km autour de laquelle on peut placer le bullsEye
		},
		syria = {
			pos = {
				x  = -22163,											-- Israel Line 974
				y  = -11800,											-- Israel Line 974
			},
			rayon  = 200,											-- distance en Km autour de laquelle on peut placer le bullsEye
		},
		normandy = {
			pos = {
				x  = -26144.085385954,								--26144.085385954,	
				y  = -41381.855008994,								--41381.855008994,		
				},
			rayon  = 200,											-- distance en Km autour de laquelle on peut placer le bullsEye
		},
		TheChannel = {
			pos = {
				y  = -15831.502170023,								--Manston
				x  = 52281.058730041,								--Manston		
				},
			rayon  = 200,											-- distance en Km autour de laquelle on peut placer le bullsEye
		},
		falklands = {
			pos = {
				x   = 72705,											--Falklands	
				y   = -31294,										--Falklands		
				},
			rayon   = 200,										-- distance en Km autour de laquelle on peut placer le bullsEye
		},
	},
	
	-- reglage composition Package
	Setting_Generation = {
		["limit_escort"]  = 8,									-- (default : 99)(recommended : 8), limit escort number to
	},
	
	StrikeOnlyWithEscorte  = false,							-- (default : true) strikes are possible with only one escort
}

-- modif Miguel21 M05.b : ajout picture Briefing + pictures Target

		 -- "FrontlineGulf.png",
		 -- "TF-Infos.png",
		 -- "TF-71.png",
	-- },
		 -- "FrontlineGulf.png",
		 -- "TF-71.png",
	-- },
-- }
pictureBrief = {
	['blue'] = {
	--	[1] = 'Frontline.png',
	},
	['red'] = {
	--	[1] = 'Frontline.png',
	},
}

"#;

        // Ok(key.to_owned() + " = " + &table + "\n" + version_str)
        Ok(version_str.to_owned() + "\n" + key + " = " + &table + "\n")
    }
}
