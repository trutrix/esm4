use crate::dev::*;


#[derive(Debug, NomLE, PartialEq, Eq, Clone, Hash)]
pub struct FieldHeader {
    pub iden: FourCC,
    pub size: u16
}

#[derive(Debug)]
pub struct Field<T> {
    pub header: FieldHeader,
    pub value: T
}

impl<'nom, T> Parse<&'nom[u8]> for Field<T> where T: Parse<&'nom[u8]> {
    fn parse(i: &'nom[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_field(i)?;
        let (_, value) = T::parse(raw)?;
        Ok((i, Self { header, value }))
    }
}

#[derive(Debug)]
pub struct RawField<'esm> {
    pub iden: FourCC,
    pub data: &'esm[u8]
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawField<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_field(i)?;
        Ok((i, Self { iden: header.iden, data }))
    }
}


pub fn alloc_field(i: &[u8]) -> IResult<&[u8], (FieldHeader, &[u8])> {
    let (i, header) = FieldHeader::parse(i)?;

    if header.iden.0 == cc4!(XXXX) {

        let (i, real_size) = le_u32(i)?;
        let (i, real_header) = FieldHeader::parse(i)?;
        let (i, real_data) = take(real_size)(i)?;

        Ok((i, (real_header, real_data)))

    } else {
        let (i, raw) = take(header.size)(i)?;
        Ok((i, (header, raw)))
    }
}