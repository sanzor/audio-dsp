use std::collections::HashMap;

use audiolib::audio_buffer::AudioBuffer;

pub struct State{
    tracks:HashMap<String,Track>
}

pub struct Track{
    pub info:TrackInfo,
    pub data:AudioBuffer
}
#[derive(Clone)]
pub struct TrackInfo{
    pub name:String
}

pub trait TrackStore{
    fn get_track_info<'a>(& 'a self,name:&str)->Option<& 'a TrackInfo>;
    fn get_track<'a>(&'a self,name:&str)->Option<& 'a Track>;
    fn upsert_track (&mut self,track:Track)->Result<(),String>;
}

impl TrackStore for State{

    fn get_track_info<'a>(& 'a self,name:&str)->Option<& 'a TrackInfo>{
        self.tracks.get(name).map(|t|&t.info)
    }

    fn get_track<'a>(&'a self,name:&str)->Option<& 'a Track>{
        self.tracks.get(name)
    }

    fn upsert_track (&mut self,track:Track)->Result<(),String>{
        let track_name=track.info.name.clone();
        let upsert=self.tracks.insert(track_name.clone(),track);
        let result= match upsert{
            None=>Err(format!("Could not upsert track with name {}",track_name)),
            Some(_)=>Ok(())
        };
        result
    }

}
