impl Message<UserPlayerCommand> for UserActor {
    type Reply = Result<UserPlayerCommandResult, String>;
    async fn handle(
        &mut self,
        msg: UserPlayerCommand,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c = match msg {
            UserPlayerCommand::Play { track_id } => self.handle_play(track_id).await,
            UserPlayerCommand::Pause { track_id } => self.handle_pause(track_id).await,
            UserPlayerCommand::Stop { track_id } => self.handle_stop(track_id).await,
            UserPlayerCommand::Seek { track_id, position } => {
                self.handle_seek(track_id, position).await
            }
        };
        c
    }
}

impl Message<UserPlayerStateQuery> for UserActor {
    type Reply = Result<UserPlayerStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: UserPlayerStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c = self.handle_get_player_state(msg.track_id).await?;
        Ok(UserPlayerStateQueryResult {
            cursor: c.cursor,
            state: c.state,
            written: c.written,
        })
    }
}
impl UserActor{
     pub async fn handle_play(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        let player = self.players.get(&track_id);
        match player.cloned() {
            None => self.handle_play_new_player(&track_id).await,
            Some(p) => self.handle_play_existing_player(&p).await,
        }
    }
    async fn handle_pause(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players.get(&track_id) {
            player.tell(PlayerCommand::Pause {}).await.unwrap();
            Ok(UserPlayerCommandResult {
                should_exit: false,
                output: "Paused player".into(),
            })
        } else {
            Err("Could not find player".into())
        }
    }
    async fn handle_stop(
        &mut self,
        track_id: Option<String>,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players.get(&track_id) {
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

    async fn handle_seek(
        &mut self,
        track_id: Option<String>,
        position: u32,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        if let Some(player) = self.players.get(&track_id) {
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

    pub async fn handle_get_player_state(
        &self,
        track_id: Option<String>,
    ) -> Result<PlayerStateQueryResult, String> {
        let track_id = track_id.ok_or_else(|| "invalid id".to_string())?;
        dbg!("got here");
        let player = self.players.get(&track_id);
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

    async fn handle_play_new_player(
        &mut self,
        track_id: &str,
    ) -> Result<UserPlayerCommandResult, String> {
        let track_ref = self.track_state.get_track_ref(track_id).await?;
        let sink = Box::new(CpalSink::new()?);
        let params = AudioPlayerActorParams {
            track: track_ref.inner.clone(),
            cursor: 0,
            sink: sink,
        };
        let player_actor = spawn(AudioPlayerActor::new(params));
        let play_result = player_actor.tell(PlayerCommand::Play {}).await.unwrap();
        if let Some(x) = self.players.insert(track_id.to_string(), player_actor) {
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