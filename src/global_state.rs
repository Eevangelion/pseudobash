mod environment;
mod settings;
mod utils;

use crate::global_state::{environment::Environment, settings::Settings, utils::Utils};

#[derive(Default)]
pub struct GlobalState {
    pub(crate) environment: Environment,
    pub(crate) settings: Settings,
    pub(crate) utils: Utils,
}
