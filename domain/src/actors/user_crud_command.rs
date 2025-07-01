use crate::{actors::user_update_params::UserUpdateParams, track::Track};

pub enum UserCrudCommand {
    Remove,
    InsertTrack{track:Track},
    GetTrack{track_id:String},
    GetTrackInfo{track_id:String},
    Update(UserUpdateParams),
}
