use player::player_ref::PlayerRef;

    pub trait PlayerRegistry:Send+Sync{
        fn upsert(&self,id:String,player:Box<dyn PlayerRef>)->Result<(),String>;
        fn get_player_by_id(&self,id:String)->Option<Box<dyn PlayerRef>>;
        fn get_all_ids(&self)->Vec<String>;
        fn remove(&self,id:String)->Option<Box<dyn PlayerRef>>;
    }