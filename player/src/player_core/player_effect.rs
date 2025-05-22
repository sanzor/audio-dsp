use crate::player_command::PlayerCommand;

pub(crate) enum PlayerEffect {
    ResetCursor,
    Close,
    Seek { pos: usize },
    Schedule { command: PlayerCommand },
}
