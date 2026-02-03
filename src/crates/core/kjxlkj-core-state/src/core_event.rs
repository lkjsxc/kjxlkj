use kjxlkj_core_types::Key;

use crate::ServiceResult;

#[derive(Clone, Debug)]
pub enum CoreEvent {
    Key(Key),
    Resize { cols: u16, rows: u16 },
    Service(ServiceResult),
}

