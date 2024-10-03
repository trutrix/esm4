#![allow(dead_code, unused_imports)]
use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet}, fs::File, io::prelude::*, os::windows::fs::MetadataExt};
use log::LevelFilter;
use nom_derive::Parse;
use simplelog::{ColorChoice, CombinedLogger, Config, TermLogger, TerminalMode, WriteLogger};

use crate::{esm::{ParsedESMFile, RawESM}, structs::world};

const ESM_PATH: &str = "C:/Users/trutr/Desktop/Fallout 4/Data/Fallout4.esm";
//const ESM_PATH: &str = "D:\\SteamLibrary\\steamapps\\common\\Fallout 4\\Data\\Fallout4.esm";

#[test]
pub fn test1() {

    CombinedLogger::init(
        vec![
            TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed, ColorChoice::Auto),
            WriteLogger::new(LevelFilter::Debug, Config::default(), std::fs::File::create("debug.log").unwrap()),
        ]
    ).unwrap();

    let mut file = std::fs::File::open(ESM_PATH).unwrap();
    let mut buffer = Vec::with_capacity(file.metadata().unwrap().len() as usize);
    file.read_to_end(&mut buffer).unwrap();

    let start = std::time::Instant::now();
    let (_, esm) = ParsedESMFile::parse(&buffer).unwrap();
    println!("Parse: {:?}", start.elapsed());

    println!("{:?}", esm.header);


    //println!("Records: {:?}", esm.data_map.len());
    //println!("World Groups: {:?}", esm.worlds.unwrap().worlds.len());
    //println!("Cell Groups: {:?}", esm.cells.unwrap().interior_cell_blocks.len());

    /*
    for w in esm.worlds.unwrap().worlds {
        for f in w.world.fields {
            match f {
                crate::prelude::fallout4::Fallout4Worldspace::SizeMin(v) => {
                    println!("SizeMin: {:?}", v);
                },
                crate::prelude::fallout4::Fallout4Worldspace::SizeMax(v) => {
                    println!("SizeMax: {:?}", v);
                },

                _ => {}
            }
        }
        //println!("{:#?}", w.world.fields[1]);
    }
    */
}