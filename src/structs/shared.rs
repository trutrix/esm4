use crate::dev::*;

#[derive(Debug, NomLE, PartialEq, Eq, Hash, Clone, PartialOrd, Ord)]
pub struct FormId(u32);

impl FormId {
    pub fn new(value: u32) -> Self {
        FormId(value)
    }
}

#[derive(Debug)]
pub struct EditorId(pub String);

impl Parse<&[u8]> for EditorId {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let i = &i[0..(i.len()-1)];
        Ok((&[], EditorId(String::from_utf8_lossy(i).to_string())))
    }
}


/// Location in world grid for cell
#[derive(Debug, Clone, NomLE)]
pub struct CellLoc {
    pub y: i16,
    pub x: i16
}


impl From<u32> for CellLoc {
    fn from(value: u32) -> Self {
        let b = value.to_le_bytes();

        let y = [b[0], b[1]];
        let x = [b[2], b[3]];

        let y = i16::from_le_bytes(y);
        let x = i16::from_le_bytes(x);
        CellLoc { y, x }
    }
}


#[derive(Debug, NomLE)]
pub struct LocationRotation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rx: f32,
    pub ry: f32,
    pub rz: f32,
}