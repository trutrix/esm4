use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use fallout4::{Fallout4FileHeader, Fallout4Group};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use crate::{dev::*, prelude::*, structs::record};


#[derive(Debug)]
pub struct RawESM<'esm> {
    pub header: Record<Fallout4FileHeader>,
    //pub data_groups: Vec<RawDataGroup<'esm>>,
    pub data_map: HashMap<FormId, RawRecord<'esm>>,
    pub worlds: Option<RawWorldGroup<'esm>>,
    pub cells: Option<RawCellTopGroup<'esm>>,
}

impl RawESM<'_> {
    pub fn get_data_record(&self, id: FormId) -> Option<&RawRecord> {
        self.data_map.get(&id)
    }

    // pub fn get_and_parse_data_record(&self, id: FormId) -> Result<Fallout4Record, ()> {
    //     if let Some(rr) = self.get_data_record(id) {
    //         let record = Fallout4Record::try_from(rr)?;
    //         Ok(record)
    //     } else {
    //         Err(())
    //     }
    // }
}


impl<'esm, 'nom> Parse<&'nom[u8]> for RawESM<'esm> where 'nom: 'esm {
    fn parse(i: &'nom[u8]) -> IResult<&'nom[u8], Self, nom::error::Error<&'nom[u8]>> {
        let (i, header) = Record::parse(i)?;

        let mut buf = i;

        // TODO: Benchmark performance of BTreeMap vs HashMap
        // BTreeMap is about 10% slower to populate than HashMap but might be faster for lookups
        let mut data_map = HashMap::with_capacity(100000);
        
        let mut worlds = None;
        let mut cells = None;

        let mut idens = BTreeSet::new();
        
        while buf.len() > 0 {

            let (_, raw_header) = GroupHeader::parse(buf)?;
            
            match raw_header.label {
                GroupLabel::Top(iden) => {

                    match iden.0 {
                        cc4!(WRLD) => {
                            let (i, world) = RawWorldGroup::parse(buf)?;
                            buf = i;
                            worlds = Some(world);
                        }
                        cc4!(CELL) => {
                            let (i, cell) = RawCellTopGroup::parse(buf)?;
                            buf = i;
                            cells = Some(cell);
                        }
                        _ => {
                            let (i, group) = RawDataGroup::parse(buf)?;
                            buf = i; 
                            
                            for record in group.records {
                                data_map.insert(record.header.form_id.clone(), record);
                            }

                            idens.insert(iden.to_string());
                        }
                    }

                }

                _ => {}
            }
        }

        Ok((i, Self { header, data_map, worlds, cells }))
    }
}


#[derive(Debug, NomLE)]
pub struct ParsedESMFile {
    pub header: Record<Fallout4FileHeader>,
    pub groups: Vec<Fallout4Group>
}


impl ParsedESMFile {
    pub fn parse_mt(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, header) = Record::<Fallout4FileHeader>::parse(i)?;

        let (i, raw_groups) = many0(complete(RawGroupMT::parse))(i)?;

        let groups = raw_groups.par_iter().map(|g| { Fallout4Group::parse(g.data).unwrap().1 }).collect();

        Ok((i, Self { header, groups }))
    }
}