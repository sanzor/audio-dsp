

use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::track::{Track, TrackInfo};

pub type SharedState=Arc<RwLock<State>>;
pub struct State{
    tracks:HashMap<String,Track>
}

impl State{

    pub fn get_track_info<'a>(& 'a self,name:&str)->Option<& 'a TrackInfo>{
        self.tracks.get(name).map(|t|&t.info)
    }

    pub fn get_track<'a>(&'a self,name:&str)->Option<& 'a Track>{
        self.tracks.get(name)
    }
    pub fn tracks(&self)->Vec<TrackInfo>{
        self.tracks.values().map(|t|t.info.clone()).collect()
    }


    pub fn upsert_track (&mut self,track:Track)->Result<(),String>{
        let track_name=track.info.name.clone();
        let upsert=self.tracks.insert(track_name.clone(),track);
        let result= match upsert{
            None=>Err(format!("Could not upsert track with name {}",track_name)),
            Some(_)=>Ok(())
        };
        result
    }

}
