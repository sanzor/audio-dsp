use domain::actors::messages::regions::{add_region::{AddRegion, AddRegionResult}, delete_region::{DeleteRegion, DeleteRegionResult}, edit_region::{EditRegion, EditRegionResult}, get_regions::{GetRegions, GetRegionsResult}};
use kameo::prelude::{Context, Message};

use crate::user_actor::user_actor::UserActor;

impl Message<AddRegion> for UserActor{
    type Reply=Result<AddRegionResult,String>;

    async fn handle(
        &mut self,
        msg: AddRegion,
        ctx: &mut Context<Self, Self::Reply>,
    ) -> Self::Reply {
        let mut track=self.tracks_provider.
    }
}

impl Message<EditRegion> for UserActor{
    type Reply = Result<EditRegionResult,String>;

    async fn handle(
            &mut self,
            msg: EditRegion,
            ctx: &mut Context<Self, Self::Reply>,
        ) -> Self::Reply {
        todo!()
    }
}

impl Message<DeleteRegion> for UserActor{
    type Reply = Result<DeleteRegionResult,String>;

    async fn handle(
            &mut self,
            msg: DeleteRegion,
            ctx: &mut Context<Self, Self::Reply>,
        ) -> Self::Reply {
        todo!()
    }
}

impl Message<GetRegions> for UserActor{
    type Reply = Result<GetRegionsResult,String>;

    async fn handle(
            &mut self,
            msg: GetRegions,
            ctx: &mut Context<Self, Self::Reply>,
        ) -> Self::Reply {
        todo!()
    }
}