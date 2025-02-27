use crate::record::RecordInput;

use super::super::model::{JsonStub, request::RequestStub, response::ResponseStub};

pub mod req;
pub mod resp;

impl From<RecordInput<'_>> for JsonStub {
    fn from((ex, cfg): RecordInput) -> Self {
        Self {
            id: None,
            uuid: None,
            priority: None,
            request: RequestStub::from((&mut *ex, cfg)),
            response: ResponseStub::from((&mut *ex, cfg)),
        }
    }
}
