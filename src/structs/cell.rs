use fallout4::Fallout4Cell;

use crate::dev::*;

use super::group::{alloc_group, GroupHeader, RawGroup};
use crate::prelude::*;

#[derive(Debug)]
pub struct RawCellTopGroup<'esm> {
    pub header: GroupHeader,
    pub interior_cell_blocks: Vec<RawInteriorCellBlock<'esm>>
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawCellTopGroup<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, blocks) = many0(complete(RawInteriorCellBlock::parse))(&data)?;
        Ok((i, RawCellTopGroup { header, interior_cell_blocks: blocks }))
    }
}


#[derive(Debug)]
pub struct CellTopGroup {
    pub header: GroupHeader,
    pub interior_cell_blocks: Vec<InteriorCellBlock>
}

impl Parse<&[u8]> for CellTopGroup {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;
        let (_, interior_cell_blocks) = many0(complete(InteriorCellBlock::parse))(raw)?;
        Ok((i, Self { header, interior_cell_blocks }))
    }
}


#[derive(Debug)]
pub struct RawInteriorCellBlock<'esm> {
    pub header: GroupHeader,
    pub interior_cell_sub_blocks: Vec<RawInteriorCellSubBlock<'esm>>
}


impl<'esm, 'nom> Parse<&'nom[u8]> for RawInteriorCellBlock<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, blocks) = many0(complete(RawInteriorCellSubBlock::parse))(&data)?;
        Ok((i, RawInteriorCellBlock { header, interior_cell_sub_blocks: blocks }))
    }
}


#[derive(Debug)]
pub struct RawInteriorCellSubBlock<'esm> {
    pub header: GroupHeader,
    pub data: &'esm[u8]
    //pub interior_cell_sub_blocks: Vec<RawGroup<'cb>>
}


impl<'esm, 'nom> Parse<&'nom[u8]> for RawInteriorCellSubBlock<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        //let (_, blocks) = many0(complete(RawGroup::parse))(&data)?;
        Ok((i, RawInteriorCellSubBlock { header, data }))
    }
}

#[derive(Debug)]
pub struct InteriorCellBlock {
    pub sub_blocks: Vec<InteriorCellSubBlock>
}

impl Parse<&[u8]> for InteriorCellBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (_header, raw)) = alloc_group(i)?;
        let (_, sub_blocks) = many0(complete(InteriorCellSubBlock::parse))(raw)?;
        Ok((i, Self { sub_blocks }))
    }
}

#[derive(Debug)]
pub struct InteriorCellSubBlock {
    pub header: GroupHeader,
    pub cells: Vec<CellDefinition>
}

impl Parse<&[u8]> for InteriorCellSubBlock {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_group(i)?;
        
        let (_, cells) = Vec::<CellDefinition>::parse(raw)?;

        Ok((i, Self { header, cells }))
    }
}

#[derive(Debug)]
pub struct CellDefinition {
    pub cell: Record<Fallout4Cell>,
    pub cell_children: Option<CellChildren>
}

impl Parse<&[u8]> for CellDefinition {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, cell) = <Record<Fallout4Cell>>::parse(i)?;

        let (_, next_id) = FourCC::parse(i)?;

        if next_id.0 == cc4!(GRUP) {
            let (_, next_grup) = GroupHeader::parse(i)?;

            match next_grup.label {
                GroupLabel::CellChildren(_form_id) => {
                    let (i, cell_children) = CellChildren::parse(i)?;
                    Ok((i, Self { cell, cell_children: Some(cell_children)}))
                },
                _ => {
                    Ok((i, Self { cell, cell_children: None }))
                }
            }
        } else {
            Ok((i, Self { cell, cell_children: None }))
        }
        
    }
}

#[derive(Debug)]
pub struct RawCellDefinition<'esm> {
    pub cell: Record<Fallout4Cell>,
    pub cell_children_data: Option<&'esm[u8]>
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawCellDefinition<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&[u8], Self> {
        let (i, cell) = <Record<Fallout4Cell>>::parse(i)?;

        let (_, next_id) = FourCC::parse(i)?;

        if next_id.0 == cc4!(GRUP) {
            let (_, next_grup) = GroupHeader::parse(i)?;

            match next_grup.label {
                GroupLabel::CellChildren(_form_id) => {
                    
                    Ok((i, Self { cell, cell_children_data: Some(&i)}))
                },
                _ => {
                    Ok((i, Self { cell, cell_children_data: None }))
                }
            }
        } else {
            Ok((i, Self { cell, cell_children_data: None }))
        }
        
    }
}


// -- CellChildren
// CellChildren is a group that contains two optional groups: CellTemporaryChildren and CellPersistentChildren
// There is probably a better way to handle this, but I'm not sure what it is yet.

#[derive(Debug)]
pub struct CellChildren {
    pub form_id: FormId,
    pub temporary_children: Option<Group<Vec<Fallout4Subrecord>>>,
    pub persistant_children: Option<Group<Vec<Fallout4Subrecord>>>
}

impl CellChildren {
    pub fn has_children(&self) -> bool {
        self.temporary_children.is_some() || self.persistant_children.is_some()
    }
}

impl Parse<&[u8]> for CellChildren {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {

        let (i, cc) = RawGroup::parse(i)?;

        match cc.header.label {
            GroupLabel::CellChildren(form_id) => {

                let (lo, rg) = RawGroup::parse(cc.data)?;
                let (_, rs) = many0(complete(Fallout4Subrecord::parse))(rg.data)?;
                let g1 = Group { header: rg.header, data: rs };
                
                let mut temporary_children = None;
                let mut persistant_children = None;

                match g1.header.label {
                    
                    GroupLabel::CellPersistentChildren(_) => {
                        persistant_children = Some(g1);
                        
                    },
                    GroupLabel::CellTemporaryChildren(_) => {
                        temporary_children = Some(g1);
                    },
                    _ => { panic!("CellChildren::parse_le(): First parsed group is not CellPersistentChildren or CellTemporaryChildren: {:?}", g1.header) }
                }

                if lo.len() > 0 {
                    let (_lo, rg) = RawGroup::parse(lo)?;
                    let (_, rs) = many0(complete(Fallout4Subrecord::parse))(rg.data)?;
                    let g2 = Group { header: rg.header, data: rs };
                    match g2.header.label {
                    
                        GroupLabel::CellPersistentChildren(_) => {
                            persistant_children = Some(g2);
                        },
                        GroupLabel::CellTemporaryChildren(_) => {
                            temporary_children = Some(g2);
                        },
                        _ => { panic!("CellChildren::parse_le(): Second parsed group is not CellPersistentChildren or CellTemporaryChildren: {:?}", g2.header) }
                    }
                    return Ok((i, Self { form_id, temporary_children, persistant_children }));
                }

                Ok((i, Self { form_id, temporary_children, persistant_children }))
            }
            _ => { panic!("CellChildren::parse_le(): CellChildren expected. Instead got: {:?}", cc.header) }
        }

        
    }
}



#[derive(Debug, NomLE)]
pub struct CombinedReferenceIndex {
    pub mesh_count: u32,
    pub reference_count: u32,
    #[nom(Count = "mesh_count")]
    pub meshes: Vec<u32>,
    #[nom(Count = "reference_count")]
    pub references: Vec<u32>
    // todo
}

#[derive(Debug)]
pub enum CellLocalWaterLevel {
    NoWater,
    WaterHeight(f32)
}

impl Parse<&[u8]> for CellLocalWaterLevel {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, height) = le_f32(i)?;
        if height == f32::MAX {
            return Ok((i, CellLocalWaterLevel::NoWater));
        } else {
            return Ok((i, CellLocalWaterLevel::WaterHeight(height)));
        }
    }
}

#[derive(Debug, NomLE)]
pub struct CombinedReference {
    pub local_id: u32,
    pub ref_id: u32
}


#[derive(Debug, NomLE)]
pub struct GridLocation {
    pub x: u32,
    pub y: u32,
    pub flags: u32
}
#[derive(Debug)]
pub struct RawCellChildren<'esm> {
    pub header: GroupHeader,
    pub data: &'esm[u8]
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawCellChildren<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, RawCellChildren { header, data }))
    }
}


#[derive(Debug)]
pub enum Fallout4Subrecord {
    Unhandled
}

impl Parse<&[u8]> for Fallout4Subrecord {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;
        Ok((i, Self::Unhandled))
    }
}