impl Message<UserCrudCommand> for UserActor {
    type Reply = Result<UserCrudCommandResult, String>;

    async fn handle(
        &mut self,
        msg: UserCrudCommand,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let c = match msg {
            UserCrudCommand::Remove => self.handle_delete().await,
            UserCrudCommand::Update(params) => self.handle_update(params).await,
            UserCrudCommand::GetTrack { track_id }=>self.hand
        };
        c
    }
}

impl UserActor {
   
    async fn handle_delete(&mut self) -> Result<UserCrudCommandResult, String> {
        Ok(UserCrudCommandResult {
            output: "deleted successfully".to_string(),
        })
    }
    async fn handle_update(
        &mut self,
        update_params: UserUpdateParams,
    ) -> Result<UserCrudCommandResult, String> {
        self.email = update_params.email;
        self.name = update_params.name;
        Ok(UserCrudCommandResult {
            output: "updated succesfully".to_string(),
        })
    }
}

impl Message<UserStateQuery> for UserActor {
    type Reply = Result<UserStateQueryResult, String>;

    async fn handle(
        &mut self,
        msg: UserStateQuery,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let mut player_list: HashMap<String, PlayerStateQueryResult> = HashMap::new();
        let mut track_list: HashMap<String, TrackInfo> = HashMap::new();

        for (key, player_ref) in self.players.iter() {
            let player_state = self.handle_get_player_state(Some(key.into())).await?;
            player_list.insert(key.into(), player_state);
        }
        for (key, track) in self.track_state.tracks.iter() {
            track_list.insert(key.into(), track.info.clone());
        }

        Ok(UserStateQueryResult {
            players: player_list,
            tracks: track_list,
        })
    }
}