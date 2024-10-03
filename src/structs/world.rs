use std::collections::HashSet;

use fallout4::Fallout4Worldspace;

use crate::{dev::*, prelude::*};



// -- RawWorldGroup

#[derive(Debug)]
pub struct RawWorldGroup<'esm> {
    pub worlds: Vec<RawWorldDefinition<'esm>>
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawWorldGroup<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (_header, data)) = alloc_group(i)?;
        let (_, worlds) = many0(complete(RawWorldDefinition::parse))(&data)?;
        Ok((i, RawWorldGroup { worlds }))
    }
}


// -- WorldGroup

#[derive(Debug)]
pub struct WorldTopGroup {
    pub worlds: Vec<WorldDefinition>
}

impl Parse<&[u8]> for WorldTopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, (_header, raw)) = alloc_group(i)?;
        let (_, worlds) = <Vec<WorldDefinition>>::parse_le(raw)?;
        Ok((i, Self { worlds }))
    }
}



// -- RawWorldDefinition

#[derive(Debug, NomLE)]
pub struct RawWorldDefinition<'esm> {
    pub world: Record<Fallout4Worldspace>,
    pub children: RawWorldChildren<'esm>
}


// -- WorldDefinition

#[derive(Debug, NomLE)]
pub struct WorldDefinition {
    pub world: Record<Fallout4Worldspace>,
    pub world_children: WorldChildren,
}



// -- RawWorldChildren

#[derive(Debug)]
pub struct RawWorldChildren<'esm> {
    pub parent: FormId,
    pub cell: CellDefinition,
    pub exterior_cell_blocks: Vec<RawExteriorCellBlock<'esm>>
}


impl<'esm, 'nom> Parse<&'nom[u8]> for RawWorldChildren<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;

        match header.label {
            GroupLabel::WorldChildren(form_id) => {
                let (leftover, cell) = CellDefinition::parse(data)?;
                let (_leftover, children) = many0(complete(RawExteriorCellBlock::parse))(&leftover)?;
                Ok((i, RawWorldChildren { parent: form_id, cell, exterior_cell_blocks: children }))
            }

            _ => {
                panic!("Invalid group label for WorldChildren: {:?}", header.label);
            }
        }

    }
}


// -- WorldChildren

#[derive(Debug)]
pub struct WorldChildren {
    /// Reference to the World record that this group belongs to.
    pub parent: FormId,
    /// The cell definition for this world.
    pub cell: CellDefinition,
    /// The exterior cell blocks for this world.
    pub exterior_cell_blocks: Vec<ExteriorCellBlock>
}

impl Parse<&[u8]> for WorldChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {

        // Parse the group header and allocate raw data.
        let (i, (header, raw)) = alloc_group(i)?;
        
        // Match the group label to the expected WorldChildren variant.
        match header.label {

            GroupLabel::WorldChildren(form_id) => {
                // Parse the cell definition.
                let (raw, cell) = CellDefinition::parse(raw)?;

                // Parse the exterior cell blocks.
                let (_leftover, exterior_cell_blocks) = many0(complete(ExteriorCellBlock::parse))(raw)?;

                // Return the parsed WorldChildren struct.
                Ok((i, Self { parent: form_id, cell, exterior_cell_blocks }))
            },

            // If the group label is not WorldChildren, panic.
            _ => {
                panic!("Invalid group label for WorldChildren: {:?}", header);
            }
        }
        
    }
}


// -- RawExteriorCellBlock

#[derive(Debug)]
pub struct RawExteriorCellBlock<'esm> {
    pub location: CellLoc,
    pub exterior_cell_sub_blocks: Vec<RawExteriorCellSubBlock<'esm>>,
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawExteriorCellBlock<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellBlock(location) => {
                let (_, blocks) = many0(complete(RawExteriorCellSubBlock::parse))(&data)?;
                Ok((i, RawExteriorCellBlock { location, exterior_cell_sub_blocks: blocks }))
            }

            _ => {
                panic!("Invalid group label for ExteriorCellBlock: {:?}", header.label);
            }
        }

        
    }
}


// -- ExteriorCellBlock

#[derive(Debug)]
pub struct ExteriorCellBlock {
    pub location: CellLoc,
    pub sub_blocks: Vec<ExteriorCellSubBlock>
}

impl Parse<&[u8]> for ExteriorCellBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, (header, raw)) = alloc_group(i)?;
        match header.label {
            GroupLabel::ExteriorCellBlock(location) => {
                
                let (_, sub_blocks) = <Vec<ExteriorCellSubBlock>>::parse_le(raw)?;

                Ok((i, Self { location, sub_blocks }))
            }
            _ => {
                panic!("Invalid GroupHeader for ExteriorCellBlock: {:?}", header);
            }
        }
    }
}



// -- RawExteriorCellSubBlock

#[derive(Debug)]
pub struct RawExteriorCellSubBlock<'esm> {
    pub location: CellLoc,
    pub data: Vec<RawCellDefinition<'esm>>
}


impl<'esm, 'nom> Parse<&'nom[u8]> for RawExteriorCellSubBlock<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;

        match header.label {
            GroupLabel::ExteriorCellSubBlock(location) => {
                let (_, data) = many0(complete(RawCellDefinition::parse))(&raw)?;
                Ok((i, Self { location, data }))
            }

            _ => {
                panic!("Invalid group label for ExteriorCellSubBlock: {:?}", header.label);
            }
        }
    }
}



// -- ExteriorCellSubBlock
// TODO: Check if the location is absolute or relative to the parent cell.

#[derive(Debug)]
pub struct ExteriorCellSubBlock {
    pub location: CellLoc,
    pub cells: Vec<CellDefinition>
}

impl Parse<&[u8]> for ExteriorCellSubBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;
        match header.label {
            
            GroupLabel::ExteriorCellSubBlock(location) => {
                let (_, cells) = <Vec<CellDefinition>>::parse_le(raw)?;
                Ok((i, Self { location, cells }))
            },
            _ => {
                panic!("Invalid GroupHeader for ExteriorCellSubBlock: {:?}", header);
            }
        }
    }
}



// -- Worldspace specific field structs

#[derive(NomLE)]
pub struct WorldRNAM {
    pub loc: CellLoc,
    #[nom(LengthCount = "le_u32")]
    pub references: Vec<WorldIdLoc>
}

impl std::fmt::Debug for WorldRNAM {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Location: {:?}, Verbose reference array", self.loc)
    }
}

#[derive(Debug, NomLE)]
pub struct WorldIdLoc {
    pub form_id: FormId,
    pub loc: CellLoc
}


#[derive(Debug, NomLE)]
pub struct MaxHeightDataWorld {
    pub min: Vec2<i16>,
    pub max: Vec2<i16>,
    pub cell_data: WorldCellData
}

#[derive(NomLE)]
pub struct WorldCellData(pub Vec<WorldQuadHeight>);

impl std::fmt::Debug for WorldCellData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Data too verbose.")
    }
}

#[derive(Debug, NomLE)]
pub struct WorldQuadHeight {
    pub bottom_left: u8,
    pub bottom_right: u8,
    pub top_left: u8,
    pub top_right: u8
}


#[derive(Debug, NomLE)]
pub struct WorldOffsetData {
    pub scale: f32, 
    pub x: f32, 
    pub y: f32, 
    pub z: f32
}

#[derive(Debug, NomLE)]
pub struct MapData {
    pub width: i32, 
    pub height: i32, 
    pub top_left: Vec2<i16>, 
    pub bottom_right: Vec2<i16>
}