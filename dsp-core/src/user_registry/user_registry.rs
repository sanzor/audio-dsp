use async_trait::async_trait;
use dsp_domain::{
    domain_user::DomainUser,
    track::{Track, TrackInfo, TrackRef, TrackRefMut},
};

#[async_trait]
pub trait UserRegistry: Send + Sync {
    async fn upsert_user(&self, user: DomainUser) -> Result<(), String>;
    async fn get_user_by_id(&self, id: &str) -> Option<DomainUser>;
    async fn get_all_users(&self) -> Vec<DomainUser>;
    async fn remove_user(&self, id: &str) -> Option<DomainUser>;

    async fn add_track(&self, user_id: &str, track: Track) -> Result<(), String>;
    async fn get_track_copy(&self, user_id: &str, track_name: &str) -> Result<Track, String>;
    async fn delete_track(&self, user_id: &str, track_id: &str) -> Result<(), String>;
    async fn get_tracks_for_user(&self, user_id: &str) -> Vec<TrackInfo>;
    async fn get_user_track_info(
        &self,
        user_id: &str,
        track_name: &str,
    ) -> Result<TrackInfo, String>;

    async fn get_track_ref(&self, user_id: &str, track_name: &str) -> Result<TrackRef, String>;
    async fn get_track_ref_mut(
        &self,
        user_id: &str,
        track_name: &str,
    ) -> Result<TrackRefMut, String>;
}
