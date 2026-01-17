use o2o::o2o;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, o2o)]
#[o2o(map_owned(egui::ThemePreference))]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
}
