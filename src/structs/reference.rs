use crate::dev::*;


#[derive(Debug, NomLE)]
pub struct DoorTeleport {
    pub door: FormId,
    pub location_rotation: LocationRotation,
    pub flags: u32
}

#[derive(Debug, NomLE)]
pub struct DoorPivot {
    pub nav_mesh: FormId,
    pub triangle_index: u16,
    pub padding: u16
}

#[derive(Debug, NomLE)]
pub struct LinkedReference {
    pub refr: FormId,
    // TODO: it has been reported that this field is sometimes only 4 bytes
}


#[derive(Debug, NomLE)]
pub struct Primitive {
    pub bounds: [f32;3], // Divide by 2
    pub color: [f32;3], // Divide by 255
    pub unknown: f32, // Alpha? 
    pub unknown2: u32 // Visibility flags?
}


#[derive(Debug, NomLE)]
pub struct Spline {
    pub slack: f32,
    pub thickness: f32,
    pub extent_x: f32,
    pub extent_y: f32,
    pub extent_z: f32,
    pub wind: u8
}