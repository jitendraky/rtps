use common_types::*;
use super::super::{SubmessageContent,SubmessageId,SubmessageFlags};

pub struct HeartbeatFrag {
    pub is_key: bool,

    pub reader_id: EntityId,
    pub writer_id: EntityId,

    pub writer_sn: SequenceNumber,
    pub last_fragment_num: FragmentNumber,

    pub count: Count
}

impl SubmessageContent for HeartbeatFrag {
    fn submessage_id() -> SubmessageId {
        unimplemented!()
    }

    fn flags(&self) -> SubmessageFlags {
        unimplemented!()
    }

    fn len(&self) -> u16 {
        unimplemented!()
    }

    fn valid(&self) -> bool {
        unimplemented!()
    }
}