use crate::unity::*;
use memflow::prelude::v1::*;

#[repr(C)]
#[derive(Clone, Debug, Pod)]
pub struct LocalGameWorld {
    pad_0x0: [u8; 0x18],
    pub exfil_controller: Pointer64<ExfilController>,
    pad_0x20: [u8; 0x18],
    pub profile_id: UnityString,
}

#[repr(C)]
#[derive(Clone, Debug, Pod)]
pub struct ExfilController {
    pad_0x0: [u8; 0x20],
    pub exfil_list: Pointer64<UnityList<ExfilPoint>>,
}

#[repr(C)]
#[derive(Clone, Debug, Pod)]
pub struct ExfilPoint {
    pad_0x0: [u8; 0x20],
    profile_id: UnityString,
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct ExitTriggerSettings {
    pad_0x0: [u8; 0x10],
    name: UnityString,
    entry_points: UnityString,
    exfil_type: i32,
    time: i32,
    player_count: i32,
    chance: i32,
    min_time: i32,
    max_time: i32,
    start_time: i32,
}

unsafe impl Pod for ExitTriggerSettings {}
