use std::{collections::HashMap, sync::Mutex};

use player::player_ref::PlayerRef;

use super::player_registry::PlayerRegistry;

pub struct LocalPlayerRegistry{
    players:Mutex<HashMap<String,Box<dyn PlayerRef>>>
}
impl PlayerRegistry for LocalPlayerRegistry{
    fn upsert(&self,id:String,player:Box<dyn player::player_ref::PlayerRef>)->Result<(),String> {
        todo!()
    }

    fn get_player_by_id(&self,id:String)->Option<Box<dyn player::player_ref::PlayerRef>> {
        todo!()
    }

    fn get_all_ids(&self)->Vec<String> {
        todo!()
    }

    fn remove(&self,id:String)->Option<Box<dyn player::player_ref::PlayerRef>> {
        todo!()
    }
}

impl LocalPlayerRegistry{
    pub fn new()->LocalPlayerRegistry{
        LocalPlayerRegistry { players: Mutex::new(HashMap::new()) }
    }
}