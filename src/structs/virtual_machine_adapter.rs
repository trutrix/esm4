use nom::multi::count;

use crate::dev::*;


#[derive(Debug)]
pub struct VirtualMachineAdapter {
    pub version: i16,
    pub format: i16,
    pub script_count: u16,
    pub scripts: Vec<Script>,
    pub fragments: Vec<Fragment>
}


impl Parse<&[u8]> for VirtualMachineAdapter {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, version) = le_i16(i)?;
        let (i, format) = le_i16(i)?;
        let (i, script_count) = le_u16(i)?;
        let (i, scripts) = count(Script::parse, script_count.into())(i)?;
        let (i, fragments) = if i.len() > 0 {
            many0(complete(Fragment::parse))(i)?
        } else {
            (i, Vec::new())
        };
        Ok((i, Self {version, format, script_count, scripts, fragments}))
    }
}

#[derive(Debug, NomLE)]
pub struct Script {
    pub name: SizedString16,
    pub status: Status,
    #[nom(LengthCount="le_u16")]
    pub properties: Vec<Property>,
}


#[derive(Debug, NomLE)]
#[repr(u8)]
pub enum Status {
    Local = 0,
    Inherited = 1,
    InheritedRemoved = 3
}

#[derive(Debug, NomLE)]
pub struct Property {
    pub name: SizedString16,
    pub type_: PropertyType,
    pub status: Status,
    pub data: PropertyData
}

#[derive(Debug, NomLE)]
#[repr(u8)]
pub enum PropertyType {
    Object = 1,
    SizedString16 = 2,
    Int32 = 3,
    Float32 = 4,
    Bool = 5,
    ObjectArray = 11,
    SizedString16Array = 12,
    Int32Array = 13,
    Float32Array = 14,
    BoolArray = 15
}

#[derive(Debug)]
pub enum PropertyData {
    Object([u8;8]),
    SizedString16(SizedString16),
    Int32(i32),
    Float(f32),
    Bool(bool),
    ObjectArray(Vec<[u8;8]>),
    SizedString16Array(Vec<SizedString16>),
    Int32Array(Vec<i32>),
    Float32Array(Vec<f32>),
    BoolArray(Vec<bool>)
}

#[derive(Debug, NomLE)]
pub struct PropertyObjectV1 {
    pub form_id: FormId,
    pub alias: u16,
    padding: u16
}

#[derive(Debug, NomLE)]
pub struct PropertyObjectV2 {    
    padding: u16,
    pub alias: u16,
    pub form_id: FormId
}

#[derive(Debug, NomLE)]
#[repr(u8)]
pub enum PropertyStatus {
    Edited = 1,
    Removed = 3
}

#[derive(Debug, NomLE)]
pub struct Fragment {

}