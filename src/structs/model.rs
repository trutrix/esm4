use crate::dev::*;

#[derive(Debug)]
pub struct ModelData {
    pub path: RawString,
    pub texture_data: ModelTextureData,
    pub alternate_textures: [u8;1]
}


#[derive(Debug)]
pub struct ModelTextureData {

}