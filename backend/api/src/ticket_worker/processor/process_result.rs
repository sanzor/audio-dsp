use domain::db::ticket::{db_resource::DbResource, db_ticket::DbTicket};

use crate::transform_drafts::data_provider::transform_drafts_data_provider::CompiledPrimitiveDraft;

pub struct ProcessResult {
    pub data: CompiledPrimitiveDraft,
}
