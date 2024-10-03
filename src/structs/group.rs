use crate::dev::*;

use super::record::RawRecord;

#[derive(Debug)]
pub struct Group<T> {
    pub header: GroupHeader,
    pub data: T
}

impl<'esm, 'nom, T> nom_derive::Parse<&'nom[u8]> for Group<T> where 'nom: 'esm, T: nom_derive::Parse<&'nom[u8]> {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, data) = T::parse(data)?;
        Ok((i, Group { header, data }))
    }
}



#[derive(Debug)]
pub struct RawGroup<'esm> {
    pub header: GroupHeader,
    pub data: &'esm[u8]
}

impl<'esm, 'nom> nom_derive::Parse<&'nom[u8]> for RawGroup<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        Ok((i, RawGroup { header, data }))
    }
}


#[derive(Debug)]
pub struct GroupHeader {
    pub label: GroupLabel,
    pub size: u32
}

impl Parse<&[u8]> for GroupHeader {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        
        let (i, iden) = FourCC::parse(i)?;
        
        if iden.0 != cc4!(GRUP) {
            panic!("GroupHeader exptected GRUP, got {} instead", iden);
            return Err(nom::Err::Failure(nom::error::Error::new(i, nom::error::ErrorKind::Tag)));
        }
        
        let (i, size) = le_u32(i)?;
        let (i, raw_label) = le_u32(i)?;
        let (i, label_type) = le_u32(i)?;
        let (i, _skip) = take(8usize)(i)?;

        match label_type {
            0 => {
                Ok((i, GroupHeader { label: GroupLabel::Top(FourCC(raw_label)), size }))
            }

            1 => {
                Ok((i, GroupHeader { label: GroupLabel::WorldChildren(FormId::new(raw_label)), size }))
            }

            2 => {
                Ok((i, GroupHeader { label: GroupLabel::InteriorCellBlock(i32::from_le_bytes(raw_label.to_le_bytes())), size }))
            }

            3 => {
                Ok((i, GroupHeader { label: GroupLabel::InteriorCellSubBlock(i32::from_le_bytes(raw_label.to_le_bytes())), size }))
            }

            4 => {
                Ok((i, GroupHeader { label: GroupLabel::ExteriorCellBlock(CellLoc::from(raw_label)), size }))
            }

            5 => {
                Ok((i, GroupHeader { label: GroupLabel::ExteriorCellSubBlock(CellLoc::from(raw_label)), size }))
            }

            6 => {
                Ok((i, GroupHeader { label: GroupLabel::CellChildren(FormId::new(raw_label)), size }))
            }

            7 => {
                Ok((i, GroupHeader { label: GroupLabel::TopicChildren(FormId::new(raw_label)), size }))
            }

            8 => {
                Ok((i, GroupHeader { label: GroupLabel::CellPersistentChildren(FormId::new(raw_label)), size }))
            }

            9 => {
                Ok((i, GroupHeader { label: GroupLabel::CellTemporaryChildren(FormId::new(raw_label)), size }))
            }

            _ => {
                panic!("Unknown group type encountered: [{}]", label_type);
            }
        }

    }
}


#[derive(Debug)]
pub enum GroupLabel {
    Top(FourCC),
    WorldChildren(FormId),
    InteriorCellBlock(i32),
    InteriorCellSubBlock(i32),
    ExteriorCellBlock(CellLoc),
    ExteriorCellSubBlock(CellLoc),
    CellChildren(FormId),
    TopicChildren(FormId),
    CellPersistentChildren(FormId),
    CellTemporaryChildren(FormId)
}

pub fn alloc_group(i: &[u8]) -> IResult<&[u8], (GroupHeader, &[u8])> {
    let (i, header) = GroupHeader::parse(i)?;
    let (i, raw) = take(header.size-24)(i)?;
    Ok((i, (header, raw)))
}


#[derive(Debug)]
pub struct RawDataGroup<'esm> {
    pub header: GroupHeader,
    pub records: Vec<RawRecord<'esm>>
}

impl<'esm, 'nom> nom_derive::Parse<&'nom[u8]> for RawDataGroup<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_group(i)?;
        let (_, records) = many0(complete(RawRecord::parse))(&data)?;
        Ok((i, RawDataGroup { header, records }))
    }
}

/// Takes the normal bytes plus the header's bytes  
/// Useful for multithreading
#[derive(Debug)]
pub struct RawGroupMT<'esm> {
    pub data: &'esm[u8]
}

impl<'esm, 'nom> Parse<&'nom [u8]> for RawGroupMT<'esm> where 'nom: 'esm {
    fn parse(i: &'nom [u8]) -> IResult<&'nom [u8], Self, nom::error::Error<&'nom [u8]>> {
        let (_, header) = GroupHeader::parse(i)?;
        let (i, data) = take(header.size)(i)?;
        Ok((i, Self { data }))
    }
}