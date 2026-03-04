use domain::{
    actors::messages::regions::{
        add_region::{AddRegion, AddRegionResult, EndTimePolicy},
        copy_region::{CopyRegion, CopyRegionResult},
        delete_region::{DeleteRegion, DeleteRegionResult},
        edit_region::{EditRegion, EditRegionResult},
    },
    db::db_region::DbRegion,
    db::db_region_set::DbRegionSet,
    regions::{
        add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
    },
};
use kameo::prelude::{Context, Message};

use crate::user_actor::actor::UserActor;

impl UserActor {
    pub fn resolve_end_time(
        regions: &[DbRegion],
        track_length: f32,
        new_region_start_time: f32,
        end_time_policy: EndTimePolicy,
    ) -> Result<f32, String> {
        match end_time_policy {
            EndTimePolicy::Explicit(val) => {
                if val <= new_region_start_time {
                    Err("End time must be greater than start time".to_string())
                } else {
                    Ok(val)
                }
            }
            EndTimePolicy::NextRegionOrEnd => {
                let next_start = regions
                    .iter()
                    .filter(|r| r.start_time_seconds > new_region_start_time)
                    .map(|r| r.start_time_seconds)
                    .min_by(|a, b| a.partial_cmp(b).unwrap());

                Ok(next_start.unwrap_or(track_length))
            }
            EndTimePolicy::FixedLength(len) => Ok(new_region_start_time + len),
        }
    }
}

impl Message<AddRegion> for UserActor {
    type Reply = Result<AddRegionResult, String>;

    async fn handle(
        &mut self,
        msg: AddRegion,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let set: DbRegionSet = self
            .region_set_provider
            .get_region_set(&msg.region_set_id)
            .await?;
        let regions: Vec<DbRegion> = self
            .region_set_provider
            .get_regions_for_region_set(&msg.region_set_id)
            .await?;
        let resolved_time = UserActor::resolve_end_time(
            &regions,
            set.track_length_seconds,
            msg.start_time,
            msg.end_time_policy,
        )?;

        let _region: DbRegion = self
            .region_set_provider
            .add_region(AddRegionParams {
                name: msg.name,
                end_time: resolved_time,
                start_time: msg.start_time,
                region_set_id: msg.region_set_id.clone(),
            })
            .await?;
        let result: DbRegionSet = self
            .region_set_provider
            .get_region_set(&msg.region_set_id)
            .await?;
        Ok(AddRegionResult { region_set: result })
    }
}

impl Message<EditRegion> for UserActor {
    type Reply = Result<EditRegionResult, String>;

    async fn handle(
        &mut self,
        msg: EditRegion,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let _region: DbRegion = self
            .region_set_provider
            .edit_region(EditRegionParams {
                name: msg.name,
                region_id: msg.region_id,
                region_set_id: msg.region_set_id.clone(),
                end_time: msg.end_time,
                start_time: msg.start_time,
            })
            .await?;
        let result: DbRegionSet = self
            .region_set_provider
            .get_region_set(&msg.region_set_id)
            .await?;
        Ok(EditRegionResult { region_set: result })
    }
}

impl Message<DeleteRegion> for UserActor {
    type Reply = Result<DeleteRegionResult, String>;

    async fn handle(
        &mut self,
        msg: DeleteRegion,
        _: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        self.region_set_provider
            .delete_region(DeleteRegionParams {
                region_id: msg.region_id,
                region_set_id: msg.region_set_id,
            })
            .await?;
        Ok(DeleteRegionResult {})
    }
}

impl Message<CopyRegion> for UserActor {
    type Reply = Result<CopyRegionResult, String>;

    async fn handle(
        &mut self,
        msg: CopyRegion,
        _ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let _region: DbRegion = self
            .region_set_provider
            .copy_region(CopyRegionParams {
                source_region_set_id: msg.source_region_set_id,
                source_region_id: msg.source_region_id,
                source_track_id: msg.source_track_id,
                destination_region_set_id: msg.destination_region_set_id.clone(),
                destination_track_id: msg.destination_track_id,
                region_copy_name: msg.copy_name,
            })
            .await?;
        let region_set: DbRegionSet = self
            .region_set_provider
            .get_region_set(&msg.destination_region_set_id)
            .await?;
        Ok(CopyRegionResult { region_set })
    }
}
