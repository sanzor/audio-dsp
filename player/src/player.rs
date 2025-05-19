
pub struct Player{
    track:AudioTrack
}


impl Player{
    pub fn new(player_params:PlayerParams)->Player{
        Player{track:player_params.track}
    }
}