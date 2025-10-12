use domain::{
    actors::messages::regions::{
        add_region::{AddRegion, AddRegionResult, EndTimePolicy},
        copy_region::{CopyRegion, CopyRegionResult},
        delete_region::{DeleteRegion, DeleteRegionResult},
        edit_region::{EditRegion, EditRegionResult},
    },
    regions::{
        add_region_params::AddRegionParams, copy_region_params::CopyRegionParams,
        delete_region_params::DeleteRegionParams, edit_region_params::EditRegionParams,
        region_set::RegionSet,
    },
};
use kameo::prelude::{Context, Message};

use crate::user_actor::user_actor::UserActor;

impl Message<AddRegion> for UserActor {
    type Reply = Result<AddRegionResult, String>;

    async fn handle(
        &mut self,
        msg: AddRegion,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let set: RegionSet = self
            .region_set_provider
            .get_region_set(&msg.region_set_id)
            .await?;
        let resolved_time = UserActor::resolve_end_time(&set, msg.start_time, msg.end_time_policy)?;

        let rez: RegionSet = self
            .region_set_provider
            .add_region(AddRegionParams {
                name: msg.name,
                end_time: resolved_time,
                start_time: msg.start_time,
                region_set_id: msg.region_set_id,
            })
            .await?;
        Ok(AddRegionResult { region_set: rez })
    }
}
impl UserActor {
    pub fn resolve_end_time(
        set: &RegionSet,
        new_region_start_time: f32,
        end_time_policy: EndTimePolicy,
    ) -> Result<f32, String> {
        let end_time = match end_time_policy {
            EndTimePolicy::Explicit(val) => {
                if val <= new_region_start_time {
                    Err("End time must be greater than start time".to_string())
                } else {
                    Ok(val)
                }
            }
            EndTimePolicy::NextRegionOrEnd => {
                let next_start = set
                    .regions
                    .iter()
                    .filter(|r| r.start_time > new_region_start_time)
                    .map(|r| r.start_time)
                    .min_by(|a, b| a.partial_cmp(b).unwrap());

                Ok(next_start.unwrap_or(set.track_length))
            }
            EndTimePolicy::FixedLength(len) => Ok(new_region_start_time + len),
        };
        end_time
    }
}

impl Message<EditRegion> for UserActor {
    type Reply = Result<EditRegionResult, String>;

    async fn handle(
        &mut self,
        msg: EditRegion,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let result: RegionSet = self
            .region_set_provider
            .edit_region(EditRegionParams {
                name: msg.name,
                region_id: msg.region_id,
                region_set_id: msg.region_set_id,
                end_time: msg.end_time,
                start_time: msg.start_time,
            })
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
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let region_set = self
            .region_set_provider
            .copy_region(CopyRegionParams {
                region_set_id: msg.region_set_id,
                region_id: msg.region_id,
                copy_name: msg.copy_name,
            })
            .await?;
        Ok(CopyRegionResult { region_set })
    }
}
