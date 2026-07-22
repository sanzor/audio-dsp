use crate::ticket_worker::{
    consumer::consumer::Consumer, events::ticket_created_event::TicketCreatedEvent,
    processor::processor::Processor,
};

pub struct WorkerParams {
    pub consumer: Box<dyn Consumer<TicketCreatedEvent>>,
    pub processor: Processor,
}
