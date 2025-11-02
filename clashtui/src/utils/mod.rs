mod config;
mod flags;
mod ipc;
mod state;
mod tui;
#[allow(clippy::module_inception)]
mod utils;
mod clashtui_data;

pub type SharedClashTuiUtil = std::rc::Rc<tui::ClashTuiUtil>;
pub type SharedClashTuiState = std::rc::Rc<core::cell::RefCell<State>>;

pub use config::{init_config, CfgError, check_essential_files};
pub use flags::{BitFlags as Flags, Flag};
pub use state::State;
pub use tui::{ClashTuiUtil, ClashSrvOpArg, ProfileType};
pub use utils::*;
pub use clashtui_data::ClashTuiData;

pub use ipc::check_sudo_password_required;
