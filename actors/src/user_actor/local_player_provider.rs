type Players = HashMap<String, ActorRef<AudioPlayerActor>>;
pub struct LocalPlayerProvider{
    pub players:Players
}
impl PlayerOperations for LocalPlayerProvider{
    async fn play(&mut self,track_id:String)->Result<PlayResult,String>{
        todo!()
    }
    async fn pause(&mut self,track_id:String)->Result<PlayResult,String>{
        todo!()
    }
    async fn stop(&mut self,track_id:String)->Result<PlayResult,String>{
        todo!()
    }
    async fn seek(&mut self,track_id:String,position:u32)->Result<PlayResult,String>{
        todo!()
    }
    async fn get_player_state(&self,player_id:String)->Result<GetPlayerStateResult,String>{
        todo!()
    }

    async fn create(&mut self,params:CreatePlayerParams)->Result<CreatePlayerResult,String>{

    }
    async fn remove(&mut self,player_id:String)->Result<(),String>{

    }
}

impl LocalPlayerProvider{
    
}