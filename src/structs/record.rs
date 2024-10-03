use std::collections::HashSet;

use crate::dev::*;


#[derive(Debug, NomLE, Clone)]
pub struct RecordHeader {
    pub iden: FourCC,
    pub size: u32,
    pub flags: RecordFlags,
    #[nom(SkipAfter(4))]
    pub form_id: FormId,
    //pub timestamp: u16,
    //pub vc_info: u16,
    #[nom(SkipAfter(2))]
    pub version: u16
    // pub padding: u16
}

#[derive(Debug, NomLE, Clone)]
pub struct RecordFlags(u32);

impl RecordFlags {
    pub fn is_compressed(&self) -> bool {
        (self.0 & 0x00040000) != 0
    }
}


#[derive(Debug)]
pub struct RawRecord<'esm> {
    pub header: RecordHeader,
    pub data: &'esm[u8]
}

impl<'esm, 'nom> Parse<&'nom[u8]> for RawRecord<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, (header, data)) = alloc_record(i)?;
        Ok((i, Self { header, data }))
    }
}


#[derive(Debug)]
pub struct Record<T> {
    pub header: RecordHeader,
    pub fields: Vec<T>
}

impl<T> UnhandledFields for Record<T>
where T: UnhandledField 
{
    fn get_unhandled_fields(&self) -> std::collections::HashSet<super::prelude::FieldHeader> {
        let mut fields = HashSet::new();
        for field in &self.fields {
            if let Some(f) = field.get_unhandled_field() {
                fields.insert(f);
            }
        }
        fields
    }
}


impl<T: for<'nom> Parse<&'nom[u8]>> Parse<&[u8]> for Record<T> {
    fn parse(i: &[u8]) -> IResult<&[u8], Self, nom::error::Error<&[u8]>> {
        let (i, (header, raw)) = alloc_record(i)?;

        if header.flags.is_compressed() {
            if let Ok(dec) = decompress_record(raw) {
                
                if let Ok((_, fields)) = many0(complete(T::parse))(&dec) {
                    return Ok((i, Self { header, fields }));
                } else {
                    return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
                }
                
            } else {
                return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
            }
            
        } else {
            if let Ok((_, fields)) = many0(complete(T::parse))(&raw) {
                return Ok((i, Self { header, fields }));
            } else {
                return Err(nom::Err::Error(nom::error::Error::new(i, nom::error::ErrorKind::Complete)));
            }
            
        }       
    }
}

impl<T: for<'nom> Parse<&'nom[u8]>> TryFrom<RawRecord<'_>> for Record<T>  {
    type Error = ParseRecordError;
    
    fn try_from(value: RawRecord<'_>) -> Result<Self, Self::Error> {
        let header = value.header;

        if header.flags.is_compressed() {
            if let Ok(dec) = decompress_record(value.data) {
                
                if let Ok((_, fields)) = Vec::<T>::parse(&dec) {
                    return Ok(Self { header, fields });
                } else {
                    return Err(ParseRecordError::ParseError(nom::error::ErrorKind::Complete));
                }
                
            } else {
                return Err(ParseRecordError::DecompressError(std::io::Error::new(std::io::ErrorKind::InvalidData, "decompress_record(): could not decompress record")));
            }
            
        } else {
            if let Ok((_, fields)) = many0(complete(T::parse))(&value.data) {
                return Ok(Self { header, fields });
            } else {
                return Err(ParseRecordError::ParseError(nom::error::ErrorKind::Complete));
            }
            
        }       
    }
}

pub enum ParseRecordError {
    DecompressError(std::io::Error),
    ParseError(nom::error::ErrorKind)
}

/// Parses RecordHeader and then allocates the correct byte slice for the Record
pub fn alloc_record(i: &[u8]) -> IResult<&[u8], (RecordHeader, &[u8])> {
    let (i, header) = RecordHeader::parse(i)?;
    let (i, raw) = take(header.size)(i)?;
    Ok((i, (header, raw)))
}


/// Parse the u32 for the real size, then decompress the zlib
pub fn decompress_record(i: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    
    if let Ok((i, real_size)) = le_u32::<&[u8], nom::error::Error<&[u8]>>(i) {
        let mut buf = Vec::with_capacity(real_size as usize);
        let mut dec = flate2::bufread::ZlibDecoder::new(i);
        dec.read_to_end(&mut buf)?;

        Ok(buf)
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "decompress_record(): could not get real size"))
    }

}



// impl<'r> TryFrom<&RawRecord<'r>> for Fallout4Record {
//     type Error = ();
//     fn try_from(value: &RawRecord) -> Result<Self, Self::Error> {
//         if let Ok((_, record)) = Fallout4Record::parse_body(value.data, &value.header) {
//             Ok(record)
//         } else {
//             Err(())
//         }
//     }
// }