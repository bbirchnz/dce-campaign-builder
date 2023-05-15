

use dce_lib::{
    mappable::MapPoint,
    oob_air::Squadron,
    target_list::{Strike},
    DCEInstance,
};

#[derive(Clone, PartialEq)]
pub enum Selectable {
    Squadron(Squadron),
    TargetStrike(Strike),
    None,
}

impl Selectable {
    pub fn from_map(map_point: &MapPoint, instance: DCEInstance) -> Selectable {
        match map_point.class.as_str() {
            "TargetCAP" => Selectable::None,
            _ => Selectable::None,
        }
    }
}

pub trait ToSelectable {
    fn to_selectable(&self) -> Selectable;

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self;

    fn from_selectable(sel: &Selectable) -> Option<Self>
    where
        Self: Sized;

    fn get_name(&self) -> String;
}

impl ToSelectable for Squadron {
    fn to_selectable(&self) -> Selectable {
        Selectable::Squadron(self.clone())
    }

    fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Squadron {
        instance
            .oob_air
            .red
            .iter_mut()
            .find(|s| s.name == name)
            .unwrap()
    }

    fn from_selectable(sel: &Selectable) -> Option<Self> {
        if let Selectable::Squadron(squad) = sel {
            return Some(squad.clone());
        }
        None
    }

    fn get_name(&self) -> String {
        self.name.to_string()
    }
}

// impl ToSelectable for Strike {
//     fn to_selectable(&self) -> Selectable {
//         Selectable::TargetStrike(self.clone())
//     }

//     fn get_mut_by_name<'a>(instance: &'a mut DCEInstance, name: &str) -> &'a mut Self {
    
//         let key = instance
//         .target_list
//         .blue
//         .iter()
//         .zip(repeat("blue"))
//             .chain(instance.target_list.red.iter().zip(repeat("red")))
//             .find_map(|((key, tgt), side)| {
//                 if let Target::Strike(strike) = tgt {
//                     if strike.text == name {
//                         return Some(key);
//                     }
//                 }
//                 return None;
//             })
//             .unwrap();
//         let item = instance.target_list.blue.get_mut(key).unwrap();
//         if let Target::Strike(mut strike) = item {

//             return &mut strike;
//         }
//         panic!("shouldn't get here");
//     }

//     fn from_selectable(sel: &Selectable) -> Option<Self>
//     where
//         Self: Sized,
//     {
//         if let Selectable::TargetStrike(t) = sel {
//             return Some(t.clone());
//         }
//         None
//     }

//     fn get_name(&self) -> String {
//         self.text.to_string()
//     }
// }
