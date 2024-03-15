pub mod utils {
    pub mod input;
    pub mod terminal;
    pub mod timestamp;
    pub mod tui;
}

pub mod prelude {
    pub use crate::utils::{input::*, terminal::*, timestamp::*, tui::*};
}
