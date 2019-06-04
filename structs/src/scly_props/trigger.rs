use auto_struct_macros::auto_struct;

use reader_writer::CStr;
use reader_writer::typenum::*;
use reader_writer::generic_array::GenericArray;

use crate::scly_props::structs::DamageInfo;
use crate::SclyPropertyData;

#[auto_struct(Readable, Writable)]
#[derive(Debug, Clone)]
pub struct Trigger<'r>
{
    #[auto_struct(expect = 9)]
    prop_count: u32,

    pub name: CStr<'r>,

    pub position: GenericArray<f32, U3>,
    pub scale: GenericArray<f32, U3>,
    pub damage_info: DamageInfo,
    pub unknown0: GenericArray<f32, U3>,
    pub unknown1: u32,
    pub active: u8,
    pub unknown2: u8,
    pub unknown3: u8,
}

impl<'r> SclyPropertyData for Trigger<'r>
{
    const OBJECT_TYPE: u8 = 0x04;
}
