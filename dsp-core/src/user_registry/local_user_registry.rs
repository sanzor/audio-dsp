use std::{collections::HashMap, sync::Mutex};

use async_trait::async_trait;
use dsp_domain::{
    domain_user::DomainUser,
    track::{Track, TrackInfo, TrackRef, TrackRefMut},
};

use super::user_registry::UserRegistry;

pub struct LocalUserRegistry {
    users: Mutex<HashMap<String, DomainUser>>,
}

#[async_trait]
impl UserRegistry for LocalUserRegistry {
    async fn upsert(
        &self,
        id: &str,
        player: dsp_domain::domain_user::DomainUser,
    ) -> Result<(), String> {
        todo!()
    }

    async fn get_user_by_id(&self, id: &str) -> Option<dsp_domain::domain_user::DomainUser> {
        todo!()
    }

    async fn get_all_users(&self) -> Vec<DomainUser> {
        todo!()
    }

    async fn remove(&self, id: &str) -> Option<dsp_domain::domain_user::DomainUser> {
        todo!()
    }

    async fn add_track(&self, user_id: &str, track: Track) -> Result<(), String> {
        todo!()
    }
    async fn delete_track(&self, user_id: &str, track_id: &str) -> Result<(), String> {
        todo!()
    }
    async fn get_tracks_for_user(&self, user_id: &str) -> Vec<TrackInfo> {
        todo!()
    }
    async fn get_user_track_info(
        &self,
        user_id: &str,
        track_name: &str,
    ) -> Result<TrackInfo, String> {
        todo!()
    }

    async fn get_track_ref(&self, user_id: &str, track_name: &str) -> Result<TrackRef, String> {
        todo!()
    }

    async fn get_track_ref_mut(
        &self,
        user_id: &str,
        track_name: &str,
    ) -> Result<TrackRefMut, String> {
        todo!()
    }

     async fn get_track_copy(&self, user_id: &str, track_name: &str) -> Result<Track, String> {
        todo!()
    }
}
impl LocalUserRegistry {
    pub fn new() -> LocalUserRegistry {
        LocalUserRegistry {
            users: Mutex::new(HashMap::new()),
        }
    }
}
