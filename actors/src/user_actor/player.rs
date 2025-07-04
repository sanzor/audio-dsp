
use domain::actors::messages::user_to_player::{
    get_player_state::GetPlayerState, 
    pause::Pause,
     play::Play, seek::Seek, stop::Stop};
use kameo::prelude::{Context, Message};
use player::audio_sink::cpal_sink::CpalSink;

use crate::user_actor::user_actor::UserActor;


impl Message<Play> for UserActor{
    type Reply=Result<PlayResult,String>;
    
    async fn handle(
        &mut self,
        msg: Play,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player = self.tracks_provider.play(&track_id);
        match player.cloned() {
            None => self.handle_play_new_player(&track_id).await,
            Some(p) => self.handle_play_existing_player(&p).await,
        }
    }
    
}

impl Message<Pause> for UserActor{
    type Reply=Result<PauseResult,String>;
    
    async fn handle(
        &mut self,
        msg: Pause,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
         let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players_provider.get(&track_id) {
            player.tell(PlayerCommand::Pause {}).await.unwrap();
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Paused player".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }
    
}

impl Message<Stop> for UserActor{
    type Reply=Result<StopResult,String>;
    
    async fn handle(
        &mut self,
        msg: Stop,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players_provider.get(&track_id) {
            player.tell(PlayerCommand::Stop {}).await.unwrap();
            let removed_player = self.players.remove(&track_id);
            if let Some(pl) = removed_player {
                drop(pl);
            }
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Paused stopped".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }  
}

impl Message<Seek> for UserActor{
    type Reply=Result<SeekResult,String>;
    
    async fn handle(
        &mut self,
        msg: Seek,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players_provider.get(&track_id) {
            player
                .tell(PlayerCommand::Seek { position: position })
                .await
                .unwrap();
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Player moved at position".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }  
}

impl Message<GetPlayerState> for UserActor{
    type Reply=Result<GetPlayerStateResult,String>;
    
    async fn handle(
        &mut self,
        msg: GetPlayerState,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        dbg!("got here");
        let player = self.players_provider.get(&track_id);
        match player.cloned() {
            None => Err("Player does not exist".to_string()),
            Some(p) => {
                let x = p
                    .ask(PlayerStateQuery {})
                    .await
                    .map_err(|e| e.to_string())?;
                Ok(x)
            }
        }
    }  
}


impl UserActor{


    async fn handle_play_new_player(
        &mut self,
        track_id: &str,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_ref = self.tracks_provider.get_track_ref(track_id).await?;
        let sink = Box::new(CpalSink::new()?);
        let params = AudioPlayerActorParams {
            track: track_ref.inner.clone(),
            cursor: 0,
            sink: sink,
        };
        let player_actor = spawn(AudioPlayerActor::new(params));
        let play_result = player_actor.tell(PlayerCommand::Play {}).await.unwrap();
        if let Some(x) = self.(track_id.to_string(), player_actor) {
            Err("Could not insert ".into())
        } else {
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Inserted succesfully ".into(),
            })
        }
    }
    async fn handle_play_existing_player(
        &mut self,
        player_ref: &ActorRef<AudioPlayerActor>,
    ) -> Result<UserPlayerCommandResult, String> {
        if player_ref.tell(PlayerCommand::Play {}).await.is_err() {
            Err("Could not start player".into())
        } else {
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Started player".into(),
            })
        }
    }

}