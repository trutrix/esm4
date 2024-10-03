use crate::dev::*;

use crate::prelude::*;
use proc::*;
use common::*;

pub const CELL_WIDTH_UNIT: f32 = 4096.0;
pub const CELL_WIDTH_M: f32 = 58.5;
pub const CELL_WIDTH_CM: f32 = CELL_WIDTH_M * 100.0;
pub const CELL_WIDTH_MM: f32 = CELL_WIDTH_M * 1000.0;
pub const UE_TO_CE_SCALE: f32 = 1.42822265625; // Unvalidated


pub type SkippedField = u8; // Assuming the fields using this are not zero sized

// These field types were pulled from all the top groups
// There may be some missing still

define_allv2! {
    // Game
    Fallout4;

    // Overrides
    [
        WRLD, WorldTopGroup;
        CELL, CellTopGroup;
    ];

    // Common fields
    [
        EDID, EditorId, EditorId;
        CNAM, Color, Color4;
        FULL, Name, u32;
        XOWN, Owner, SkippedField;
        OBND, ObjectBounds, [i16; 6];
        MODL, Model, RawString; 
    ];

    // Records
    [
        TES4, FileHeader, [
            HEDR, Metadata, { pub version: f32, pub num_records: u32, pub next_object_id: u32 };
            CNAM, Author, RawString;
            INTV, TagifyCount, u32;
            INCC, UnknownCounter, u32;
            TNAM, TransientTypes, Vec<u32>;
        ];

        GMST, GameSetting, [
            EDID;
            DATA, Value, Vec<u8>;
        ];

        KYWD, Keyword, [
            EDID;
            CNAM;
            FULL;
            DNAM, Notes, RawString;
            TNAM, Type, u32;
            DATA, AttractionRule, u32;
            NNAM, DisplayName, RawString;
        ];

        WRLD, Worldspace, [
            EDID;
            CNAM, Color, u32;
            ZNAM, Music, FormId;

            CLSZ, CellSizeData, SkippedField;
            CNAM, Climate, FormId;
            DATA, Flags, u8;
            DNAM, DefaultHeight, Vec<f32>;

            ICON, MapImage, RawString;

            MNAM, MapData, MapData;

            NAM2, Water, FormId;
            NAM3, LODWaterType, FormId;
            NAM4, LODWaterHeight, FormId;
            NAMA, DistantLODMultiplier, f32;
    
            NAM0, SizeMin, Vec2<f32>;
            NAM9, SizeMax, Vec2<f32>;

            OFST, AbsoluteData, SkippedField;
            ONAM, WorldOffsetData, WorldOffsetData;

            PNAM, UseFlags, u16;
            WCTR, CenterCell, Vec2<u16>;
            RNAM, LocIdRef, WorldRNAM;
            FULL, Name, u32;
            MHDT, MaxHeightData, MaxHeightDataWorld;
            WNAM, ParentWorldspace, FormId;

            XLCN, Location, FormId;
            XWEM, WaterEnvironmentMap, RawString;
            WLEV, WaterLevelData, SkippedField;
        ];

        CELL, Cell, [
            EDID;
            DATA, Flags, u16;
            XCLC, GridLocation, GridLocation;
            MHDT, MaximumHeightData, u8;
            XCRI, CombinedReferenceIndex, CombinedReferenceIndex;
            LTMP, LightingTemplate, FormId;
            VISI, PreVisTimestamp, u16;
            RVIS, PreVisFileOf, FormId;
            PCMB, PreCombinedFilesTimestamp, u16;
            XCLW, LocalWaterLevel, CellLocalWaterLevel;
            XCWT, Water, FormId;
            XCLR, Regions, FormId;
            XLCN, Location, FormId;
            XPRI, PreVisRefIndex, FormId;
        ];

        REFR, Reference, [
            EDID;
            XOWN;
            NAME, BaseObject, FormId;
            XSCL, Scale, f32;
            DATA, LocationRotation, LocationRotation;
            XTEL, DoorTeleport, DoorTeleport;
            XNDP, DoorPivot, DoorPivot;
            XLRT, LocationReferenceType, FormId;
            XLKR, LinkedReference, LinkedReference;
            VMAD, VirtualMachineAdapter, SkippedField; // TODO: Extremely complex, not sure if needed
            XPRM, Primitive, Primitive;
            XRFG, ReferenceGroup, FormId;
            XLYR, Layer, FormId;
            XBSD, Spline, Spline;
            XPLK, SplineConnection, FormId;
            XRDS, Radius, f32;
            XPRD, IdleTime, f32;
            XPPA, PatrolMarker, SkippedField; // Unknown
            INAM, Idle, FormId;
            PDTO, Unknown1, SkippedField; // Unknown - size 8 bytes
            XPDD, ProjectedDecal, {  pub width_scale: f32, pub height_scale: f32 };
            XATP, ActivationPoint, SkippedField; // Unknown
            XATR, AttachmentReference, FormId;
            XESP, EnableParent, SkippedField; // Shared field
            XRGD, Ragdoll, SkippedField; // Shared field
            XEMI, Emitter, FormId;
            XLOC, Lock, SkippedField;
            XACT, Action, SkippedField; // Unknown if this is actually an action
            XMSP, MaterialSwap, FormId;
            ONAM, Open, SkippedField;
            XIS2, IgnoredSandbox, SkippedField;
            XLIG, LightData, SkippedField;
            XEZN, EncounterZone, FormId;
            XRDO, Radio, SkippedField;
            XTRI, CollisionLayer, FormId;
            XAPD, ActivateParents, SkippedField;
            XAPR, ActivateParentsReference, SkippedField;
            XLIB, LevelItemBaseObject, FormId;
            XWCN, Unknown2, SkippedField;
            XWCU, WaterVelocity, SkippedField;
            XRNK, OwnerFactionRank, i32;
            XLKT, LinkedRefTrans, SkippedField;
            XTNM, TeleportLocationName, SkippedField;
            XLCN, PersistantLocation, FormId;
            XWPG, PowerGrid1, SkippedField;
            XWPN, PowerGrid2, SkippedField;
            XLCM, LevelModifier, i32;
            XHTW, HeadTrackingWeight, f32;
            XFVC, FavorCost, f32;
            XALP, Alpha, Vec2<u8>;
            XAMC, AmmoCount, u32;
            XCNT, ItemCount, i32;
            XCVL, Unknown3, SkippedField;
            MNAM, PowerComments, RawString;
            XHLT, HealthPercentage, u32;
            XMRK, MapMarkerData, SkippedField; // TODO: Compounded
            FNAM, MM1, SkippedField;
            FULL, MM2, SkippedField;
            TNAM, MM3, SkippedField;
            XSPC, SpawnContainer, FormId;
            XOCP, OcculsionPlane, SkippedField; // Size, Loc, Rot
            XCZC, CurrentZoneCell, FormId;
            XCZA, Unknown4, SkippedField;
            XASP, AcousticRestriction, FormId;
        ];

        LAND, Landscape, [
            VHGT, VertexHeight, VertexHeightData;
            VNML, VertexNormals, SkippedField;
            VCLR, VertexColors, SkippedField;
            DATA, Unknown, SkippedField;
            BTXT, BaseTexture, SkippedField;
            ATXT, AdditionalTexture, SkippedField;
            VTXT, Unknown2, SkippedField;
            MPCD, HiResHeightmap, SkippedField;
        ];


        // TODO: Add and fix more record fields

        AACT, Action, [
            EDID;
            CNAM;
            TNAM, Type, u32;
            DNAM, Notes, RawString;
        ];


        ACTI, Activator, [
            EDID;
            OBND;

            VMAD, VirtualMachineAdapter, SkippedField;
        
            
            MODL; // TODO - Common compound field
            MODT, ModelTexture, SkippedField; 
            MODS, AlternateTextures, SkippedField;

            KSIZ, KeywordCount, u32;
            KWDA, Keywords, Vec<FormId>;
            FULL;
            PTRN, Pattern, SkippedField; // TODO
            PNAM, Color, Color4;
            ATTX, Attachments, SkippedField; // TODO
            FNAM, Flags, u16; // TODO
            CITC, Unknown, SkippedField; // TODO
            CTDA, Unknown2, SkippedField; // TODO
            PRPS, Unknown3, SkippedField; // TODO
            DEST, DestructionData, SkippedField; // TODO - Common compound field
            DSTD, DestructionStage, SkippedField; // TODO
            DMDL, DestructionModel, SkippedField; // TODO
            DMDT, DestructionTexture, SkippedField; // TODO
            DSTF, DestructionEnd, SkippedField; // TODO
            RADR, RadioData, SkippedField; // TODO

            NTRM, Unknown4, SkippedField; // TODO
            WNAM, Water, FormId;
            VNAM, Unknown5, SkippedField; // TODO
            STCP, Unknown6, SkippedField; // TODO
            FTYP, Unknown7, SkippedField; // TODO
            CIS1, Unknown8, SkippedField; // TODO
            CIS2, Unknown9, SkippedField; // TODO
            DAMC, Unknown10, SkippedField; // TODO
            DSTA, Unknown11, SkippedField; // TODO
            SNAM, Unknown12, SkippedField; // TODO
        ];


        SOPM, SoundOutputModel, [
            NAM1, NAM1, SkippedField;
            EDID;
            ENAM, ENAM, SkippedField;
            MNAM, MNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            VNAM, VNAM, SkippedField;
            ATTN, ATTN, SkippedField;
        ];

        INGR, Ingredient, [
            DATA, DATA, SkippedField;
            EDID;
            EFID, EFID, SkippedField;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            YNAM, YNAM, SkippedField;
            MODT, MODT, SkippedField;
            EFIT, EFIT, SkippedField;
            ENIT, ENIT, SkippedField;
        ];

        RACE, Race, [
            NAM0, NAM0, SkippedField;
            MSM0, MSM0, SkippedField;
            NAM1, NAM1, SkippedField;
            MSM1, MSM1, SkippedField;
            BOD2, BOD2, SkippedField;
            NAM2, NAM2, SkippedField;
            NAM3, NAM3, SkippedField;
            NAM4, NAM4, SkippedField;
            CTDA, CTDA, SkippedField;
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            TTEB, TTEB, SkippedField;
            BSMB, BSMB, SkippedField;
            SRAC, SRAC, SkippedField;
            TTEC, TTEC, SkippedField;
            RBPC, RBPC, SkippedField;
            MPPC, MPPC, SkippedField;
            DESC, DESC, SkippedField;
            HEAD, HEAD, SkippedField;
            TTED, TTED, SkippedField;
            EDID;
            MSID, MSID, SkippedField;
            SAKD, SAKD, SkippedField;
            ATKD, ATKD, SkippedField;
            STKD, STKD, SkippedField;
            TTGE, TTGE, SkippedField;
            ATKE, ATKE, SkippedField;
            NAME, NAME, SkippedField;
            SRAF, SRAF, SkippedField;
            AHCF, AHCF, SkippedField;
            TTEF, TTEF, SkippedField;
            HCLF, HCLF, SkippedField;
            MPPF, MPPF, SkippedField;
            RPRF, RPRF, SkippedField;
            FTSF, FTSF, SkippedField;
            DFTF, DFTF, SkippedField;
            MPPI, MPPI, SkippedField;
            FMRI, FMRI, SkippedField;
            MLSI, MLSI, SkippedField;
            TETI, TETI, SkippedField;
            QSTI, QSTI, SkippedField;
            VTCK, VTCK, SkippedField;
            MPPK, MPPK, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            TINL, TINL, SkippedField;
            ANAM, ANAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            GNAM, GNAM, SkippedField;
            HNAM, HNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            NNAM, NNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            PNAM, PNAM, SkippedField;
            QNAM, QNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            UNAM, UNAM, SkippedField;
            VNAM, VNAM, SkippedField;
            WNAM, WNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            AHCM, AHCM, SkippedField;
            SGNM, SGNM, SkippedField;
            MTNM, MTNM, SkippedField;
            MPPM, MPPM, SkippedField;
            RPRM, RPRM, SkippedField;
            FTSM, FTSM, SkippedField;
            DFTM, DFTM, SkippedField;
            MPGN, MPGN, SkippedField;
            MPPN, MPPN, SkippedField;
            FMRN, FMRN, SkippedField;
            PHTN, PHTN, SkippedField;
            SPLO, SPLO, SkippedField;
            WMAP, WMAP, SkippedField;
            STCP, STCP, SkippedField;
            TTGP, TTGP, SkippedField;
            BMMP, BMMP, SkippedField;
            BSMP, BSMP, SkippedField;
            NTOP, NTOP, SkippedField;
            PTOP, PTOP, SkippedField;
            UNWP, UNWP, SkippedField;
            APPR, APPR, SkippedField;
            MPGS, MPGS, SkippedField;
            ATKS, ATKS, SkippedField;
            BSMS, BSMS, SkippedField;
            PRPS, PRPS, SkippedField;
            SPCT, SPCT, SkippedField;
            MODT, MODT, SkippedField;
            TTET, TTET, SkippedField;
            ATKT, ATKT, SkippedField;
            SAPT, SAPT, SkippedField;
            MPPT, MPPT, SkippedField;
            PHWT, PHWT, SkippedField;
            WKMV, WKMV, SkippedField;
            FLMV, FLMV, SkippedField;
            SWMV, SWMV, SkippedField;
            ATKW, ATKW, SkippedField;
            INDX, INDX, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        ZOOM, Zoom, [
            EDID;
            GNAM, GNAM, SkippedField;
        ];

        QUST, Quest, [

        ];

        RELA, Relationship, [
            DATA, DATA, SkippedField;
            EDID;
        ];

        MOVT, MovementType, [
            SPED, SPED, SkippedField;
            EDID;
            INAM, INAM, SkippedField;
            JNAM, JNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            MNAM, MNAM, SkippedField;
        ];

        AMMO, Ammo, [
            NAM1, NAM1, SkippedField;
            NAM2, NAM2, SkippedField;
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODT, MODT, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        WTHR, Weather, [
            NAM0, NAM0, SkippedField;
            NAM1, NAM1, SkippedField;
            NAM4, NAM4, SkippedField;
            DATA, DATA, SkippedField;
            DALC, DALC, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            FNAM, FNAM, SkippedField;
            GNAM, GNAM, SkippedField;
            JNAM, JNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            NNAM, NNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            QNAM, QNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            UNAM, UNAM, SkippedField;
            VNAM, VNAM, SkippedField;
            WNAM, WNAM, SkippedField;
            IMSP, IMSP, SkippedField;
            WGDR, WGDR, SkippedField;
            MODT, MODT, SkippedField;
            b"00TX", Tx0, SkippedField;
            b"10TX", Tx1, SkippedField;
            b"20TX", Tx2, SkippedField;
            b"30TX", Tx3, SkippedField;
            b"40TX", Tx4, SkippedField;
            b"50TX", Tx5, SkippedField;
            b"60TX", Tx6, SkippedField;
            b"70TX", Tx7, SkippedField;
            b"80TX", Tx8, SkippedField;
            b"90TX", Tx9, SkippedField;
            b":0TX", Tx10, SkippedField;
            b";0TX", Tx11, SkippedField;
            b"<0TX", Tx12, SkippedField;
            b"=0TX", Tx13, SkippedField;
            b">0TX", Tx14, SkippedField;
            b"?0TX", Tx15, SkippedField;
            b"@0TX", Tx16, SkippedField;
            A0TX, A0TX, SkippedField;
            B0TX, B0TX, SkippedField;
            C0TX, C0TX, SkippedField;
            D0TX, D0TX, SkippedField;
            E0TX, E0TX, SkippedField;
            F0TX, F0TX, SkippedField;
            K0TX, K0TX, SkippedField;
            L0TX, L0TX, SkippedField;
        ];

        GLOB, GlobalVariable, [
            EDID;
            FNAM, FNAM, SkippedField;
            FLTV, FLTV, SkippedField;
        ];

        CONT, Container, [
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            DSTD, DSTD, SkippedField;
            DSTF, DSTF, SkippedField;
            DMDL, DMDL, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            ONAM, ONAM, SkippedField;
            QNAM, QNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            NTRM, NTRM, SkippedField;
            PTRN, PTRN, SkippedField;
            CNTO, CNTO, SkippedField;
            FTYP, FTYP, SkippedField;
            MODS, MODS, SkippedField;
            PRPS, PRPS, SkippedField;
            COCT, COCT, SkippedField;
            DMDT, DMDT, SkippedField;
            MODT, MODT, SkippedField;
            DEST, DEST, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        EXPL, Explosion, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            MNAM, MNAM, SkippedField;
            EITM, EITM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        DEBR, Debris, [
            DATA, DATA, SkippedField;
            EDID;
            MODT, MODT, SkippedField;
        ];

        LENS, LensFlare, [
            EDID;
            LFSD, LFSD, SkippedField;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            LFSP, LFSP, SkippedField;
        ];

        ARMA, ArmorAddon, [
            NAM0, NAM0, SkippedField;
            NAM1, NAM1, SkippedField;
            BOD2, BOD2, SkippedField;
            MOD2, MOD2, SkippedField;
            NAM2, NAM2, SkippedField;
            MOD3, MOD3, SkippedField;
            MOD4, MOD4, SkippedField;
            MOD5, MOD5, SkippedField;
            BSMB, BSMB, SkippedField;
            MO3C, MO3C, SkippedField;
            SNDD, SNDD, SkippedField;
            EDID;
            MO2F, MO2F, SkippedField;
            MO3F, MO3F, SkippedField;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            RNAM, RNAM, SkippedField;
            BSMP, BSMP, SkippedField;
            MO2S, MO2S, SkippedField;
            MO3S, MO3S, SkippedField;
            MO4S, MO4S, SkippedField;
            MO5S, MO5S, SkippedField;
            BSMS, BSMS, SkippedField;
            MO2T, MO2T, SkippedField;
            MO3T, MO3T, SkippedField;
            MO4T, MO4T, SkippedField;
            MO5T, MO5T, SkippedField;
        ];

        LIGH, Light, [
            NAM0, NAM0, SkippedField;
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            FNAM, FNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            WGDR, WGDR, SkippedField;
            PRPS, PRPS, SkippedField;
            MODT, MODT, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        ASTP, AssociationType, [
            DATA, DATA, SkippedField;
            EDID;
            FCHT, FCHT, SkippedField;
            MCHT, MCHT, SkippedField;
            FPRT, FPRT, SkippedField;
            MPRT, MPRT, SkippedField;
        ];

        IDLE, IdleAnimation, [
            CIS1, CIS1, SkippedField;
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            GNAM, GNAM, SkippedField;
        ];

        BNDS, Bounds, [
            EDID;
            OBND, OBND, SkippedField;
            DNAM, DNAM, SkippedField;
            TNAM, TNAM, SkippedField;
        ];

        RFCT, VisualEffect, [
            DATA, DATA, SkippedField;
            EDID;
        ];

        REGN, Region, [
            RDSA, RDSA, SkippedField;
            EDID;
            RPLD, RPLD, SkippedField;
            RPLI, RPLI, SkippedField;
            ANAM, ANAM, SkippedField;
            WNAM, WNAM, SkippedField;
            RDMO, RDMO, SkippedField;
            RDMP, RDMP, SkippedField;
            RCLR, RCLR, SkippedField;
            RDGS, RDGS, SkippedField;
            RDAT, RDAT, SkippedField;
            RDOT, RDOT, SkippedField;
            RDWT, RDWT, SkippedField;
        ];

        MATO, MaterialObject, [
            DATA, DATA, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
        ];

        TREE, Tree, [
            PFPC, PFPC, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            CNAM, CNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        MISC, MiscObject, [
            KWDA, KWDA, SkippedField;
            CVPA, CVPA, SkippedField;
            DATA, DATA, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            FIMD, FIMD, SkippedField;
            OBND, OBND, SkippedField;
            DSTD, DSTD, SkippedField;
            DSTF, DSTF, SkippedField;
            DMDL, DMDL, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODS, MODS, SkippedField;
            DMDT, DMDT, SkippedField;
            MODT, MODT, SkippedField;
            DEST, DEST, SkippedField;
            CDIX, CDIX, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        NAVI, Navigation, [
            NVMI, NVMI, SkippedField;
            NVPP, NVPP, SkippedField;
            NVER, NVER, SkippedField;
        ];

        CMPO, Component, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            CUSD, CUSD, SkippedField;
            FULL, FULL, SkippedField;
            GNAM, GNAM, SkippedField;
            MNAM, MNAM, SkippedField;
        ];


        LVLI, LeveledItem, [
            LLKC, LLKC, SkippedField;
            COED, COED, SkippedField;
            EDID;
            LVLD, LVLD, SkippedField;
            OBND, OBND, SkippedField;
            LVLF, LVLF, SkippedField;
            LVLG, LVLG, SkippedField;
            LVSG, LVSG, SkippedField;
            ONAM, ONAM, SkippedField;
            LVLM, LVLM, SkippedField;
            LVLO, LVLO, SkippedField;
            LLCT, LLCT, SkippedField;
        ];

        ENCH, Enchantment, [
            CTDA, CTDA, SkippedField;
            EDID;
            EFID, EFID, SkippedField;
            OBND, OBND, SkippedField;
            FULL, FULL, SkippedField;
            EFIT, EFIT, SkippedField;
            ENIT, ENIT, SkippedField;
        ];

        LCTN, Location, [
            NAM1, NAM1, SkippedField;
            KWDA, KWDA, SkippedField;
            LCEC, LCEC, SkippedField;
            LCID, LCID, SkippedField;
            EDID;
            FULL, FULL, SkippedField;
            ANAM, ANAM, SkippedField;
            CNAM, CNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            LCUN, LCUN, SkippedField;
            LCEP, LCEP, SkippedField;
            LCPR, LCPR, SkippedField;
            LCSR, LCSR, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        NPC_, NonPlayerCharacter, [
            NAM4, NAM4, SkippedField;
            NAM5, NAM5, SkippedField;
            NAM6, NAM6, SkippedField;
            NAM8, NAM8, SkippedField;
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            TPTA, TPTA, SkippedField;
            LTPC, LTPC, SkippedField;
            CS2D, CS2D, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            PKID, PKID, SkippedField;
            OBND, OBND, SkippedField;
            TEND, TEND, SkippedField;
            DSTD, DSTD, SkippedField;
            CS2E, CS2E, SkippedField;
            OBTE, OBTE, SkippedField;
            CS2F, CS2F, SkippedField;
            CRIF, CRIF, SkippedField;
            HCLF, HCLF, SkippedField;
            OBTF, OBTF, SkippedField;
            DSTF, DSTF, SkippedField;
            CS2H, CS2H, SkippedField;
            FMRI, FMRI, SkippedField;
            TETI, TETI, SkippedField;
            CS2K, CS2K, SkippedField;
            VTCK, VTCK, SkippedField;
            MSDK, MSDK, SkippedField;
            FULL, FULL, SkippedField;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            INAM, INAM, SkippedField;
            PNAM, PNAM, SkippedField;
            QNAM, QNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            WNAM, WNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            FMIN, FMIN, SkippedField;
            PFRN, PFRN, SkippedField;
            SPLO, SPLO, SkippedField;
            CNTO, CNTO, SkippedField;
            STCP, STCP, SkippedField;
            STOP, STOP, SkippedField;
            FTYP, FTYP, SkippedField;
            CSCR, CSCR, SkippedField;
            PRKR, PRKR, SkippedField;
            ATKR, ATKR, SkippedField;
            ECOR, ECOR, SkippedField;
            APPR, APPR, SkippedField;
            ACBS, ACBS, SkippedField;
            PRPS, PRPS, SkippedField;
            FMRS, FMRS, SkippedField;
            OBTS, OBTS, SkippedField;
            COCT, COCT, SkippedField;
            SPCT, SPCT, SkippedField;
            AIDT, AIDT, SkippedField;
            DOFT, DOFT, SkippedField;
            MWGT, MWGT, SkippedField;
            DPLT, DPLT, SkippedField;
            TPLT, TPLT, SkippedField;
            LTPT, LTPT, SkippedField;
            SHRT, SHRT, SkippedField;
            DEST, DEST, SkippedField;
            FTST, FTST, SkippedField;
            MSDV, MSDV, SkippedField;
            MRSV, MRSV, SkippedField;
            KSIZ, KSIZ, SkippedField;
            PRKZ, PRKZ, SkippedField;
        ];

        SPEL, Spell, [
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            EFID, EFID, SkippedField;
            OBND, OBND, SkippedField;
            FULL, FULL, SkippedField;
            ETYP, ETYP, SkippedField;
            EFIT, EFIT, SkippedField;
            SPIT, SPIT, SkippedField;
        ];

        MGEF, MagicEffect, [
            CTDA, CTDA, SkippedField;
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            VMAD, VMAD, SkippedField;
            SNDD, SNDD, SkippedField;
            EDID;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        FSTP, Footstep, [
            DATA, DATA, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
        ];

        OVIS, ObjectVisibilityManager, [
            DATA, DATA, SkippedField;
            INDX, INDX, SkippedField;
        ];

        SCSN, AudioCategorySnapshot, [
            EDID;
            CNAM, CNAM, SkippedField;
            PNAM, PNAM, SkippedField;
        ];

        DMGT, DamageType, [
            EDID;
            DNAM, DNAM, SkippedField;
        ];

        BPTD, BodyPartData, [
            NAM1, NAM1, SkippedField;
            NAM4, NAM4, SkippedField;
            NAM5, NAM5, SkippedField;
            EDID;
            BPND, BPND, SkippedField;
            MODL, MODL, SkippedField;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            FNAM, FNAM, SkippedField;
            INAM, INAM, SkippedField;
            JNAM, JNAM, SkippedField;
            BPNN, BPNN, SkippedField;
            BPTN, BPTN, SkippedField;
            MODT, MODT, SkippedField;
            BPNT, BPNT, SkippedField;
        ];

        GDRY, Godray, [
            DATA, DATA, SkippedField;
            EDID;
        ];

        FACT, Faction, [
            CTDA, CTDA, SkippedField;
            DATA, DATA, SkippedField;
            CRVA, CRVA, SkippedField;
            VENC, VENC, SkippedField;
            CITC, CITC, SkippedField;
            EDID;
            VEND, VEND, SkippedField;
            PLVD, PLVD, SkippedField;
            FULL, FULL, SkippedField;
            RNAM, RNAM, SkippedField;
            XNAM, XNAM, SkippedField;
            CRGR, CRGR, SkippedField;
            VENV, VENV, SkippedField;
        ];

        HAZD, Hazard, [
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        FLOR, Flora, [
            KWDA, KWDA, SkippedField;
            PFPC, PFPC, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            DSTD, DSTD, SkippedField;
            DSTF, DSTF, SkippedField;
            PFIG, PFIG, SkippedField;
            DMDL, DMDL, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            FNAM, FNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODS, MODS, SkippedField;
            PRPS, PRPS, SkippedField;
            DMDT, DMDT, SkippedField;
            MODT, MODT, SkippedField;
            DEST, DEST, SkippedField;
            ATTX, ATTX, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        IPCT, Impact, [
            NAM1, NAM1, SkippedField;
            NAM2, NAM2, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            FNAM, FNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            DODT, DODT, SkippedField;
            MODT, MODT, SkippedField;
        ];

        IPDS, ImpactDataSet, [
            EDID;
            PNAM, PNAM, SkippedField;
        ];

        ARMO, Armor, [
            BOD2, BOD2, SkippedField;
            MOD2, MOD2, SkippedField;
            MOD4, MOD4, SkippedField;
            KWDA, KWDA, SkippedField;
            DAMA, DAMA, SkippedField;
            DATA, DATA, SkippedField;
            DESC, DESC, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            INRD, INRD, SkippedField;
            OBTE, OBTE, SkippedField;
            OBTF, OBTF, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            FNAM, FNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            EITM, EITM, SkippedField;
            PTRN, PTRN, SkippedField;
            STOP, STOP, SkippedField;
            ETYP, ETYP, SkippedField;
            APPR, APPR, SkippedField;
            MO2S, MO2S, SkippedField;
            MO4S, MO4S, SkippedField;
            OBTS, OBTS, SkippedField;
            MO2T, MO2T, SkippedField;
            MO4T, MO4T, SkippedField;
            BAMT, BAMT, SkippedField;
            INDX, INDX, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        DFOB, DefaultObject, [
            DATA, ObjectId, FormId;
            EDID;
        ];

        GRAS, Grass, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            MODT, MODT, SkippedField;
        ];

        
        SMEN, StoryManagerEventNode, [
            CTDA, CTDA, SkippedField;
            CITC, CITC, SkippedField;
            EDID;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            PNAM, PNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            XNAM, XNAM, SkippedField;
        ];

        // Unknown
        KSSM, SoundKeywordMapping, [
            EDID;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            KNAM, KNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            VNAM, VNAM, SkippedField;
        ];

        
        SMQN, StoryManagerQuestNode, [
            CIS1, CIS1, SkippedField;
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            CITC, CITC, SkippedField;
            EDID;
            DNAM, DNAM, SkippedField;
            HNAM, HNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            NNAM, NNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            QNAM, QNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            XNAM, XNAM, SkippedField;
        ];

        SOUN, Sound, [
            SDSC, SDSC, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            REPT, REPT, SkippedField;
        ];

        ECZN, EncounterZone, [
            DATA, DATA, SkippedField;
            EDID;
        ];

        CPTH, CameraPath, [
            CTDA, CTDA, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
            SNAM, SNAM, SkippedField;
        ];

        INNR, InstanceNamingRules, [
            KWDA, KWDA, SkippedField;
            EDID;
            UNAM, UNAM, SkippedField;
            VNAM, VNAM, SkippedField;
            WNAM, WNAM, SkippedField;
            YNAM, YNAM, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        
        IMAD, ImageSpaceModifier, [
            NAM1, NAM1, SkippedField;
            NAM2, NAM2, SkippedField;
            NAM3, NAM3, SkippedField;
            NAM4, NAM4, SkippedField;
            NAM5, NAM5, SkippedField;
            NAM6, NAM6, SkippedField;
        ];

        SCCO, SceneCollection, [
            EDID;
            QNAM, Quest, FormId;
            SNAM, Scene, FormId;
            VNAM, Unknown, SkippedField;
            XNAM, Coordinates, Vec2<i32>;
        ];

        
        LAYR, Layer, [
            EDID;
            PNAM, Parent, FormId;
        ];

        CAMS, CameraShot, [
            CTDA, CTDA, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            MNAM, MNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        TACT, TalkingActivator, [
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            FNAM, FNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            VNAM, VNAM, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
        ];

        STAG, AnimationSoundTagSet, [
            EDID;
            TNAM, Sounds, { pub sound: FormId, pub action: RawString };
        ];

        DOOR, Door, [
            KWDA, KWDA, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            DSTD, DSTD, SkippedField;
            DSTF, DSTF, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            ANAM, ANAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            SNAM, SNAM, SkippedField;
            NTRM, NTRM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
            DEST, DEST, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        VTYP, VoiceType, [
            EDID;
            DNAM, DNAM, SkippedField;
        ];


        ANIO, AnimationObject, [
            MODC, MODC, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            BNAM, BNAM, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
        ];

        KEYM, Key, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            PTRN, PTRN, SkippedField;
            MODT, MODT, SkippedField;
        ];

        SNCT, SoundCategory, [
            EDID;
            FULL, FULL, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            PNAM, PNAM, SkippedField;
            UNAM, UNAM, SkippedField;
            VNAM, VNAM, SkippedField;
        ];

        MATT, MaterialType, [
            EDID;
            ANAM, ANAM, SkippedField;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            HNAM, HNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        LTEX, LandscapeTexture, [
            EDID;
            GNAM, GNAM, SkippedField;
            HNAM, HNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            TNAM, TNAM, SkippedField;
        ];

        IDLM, IdleMarker, [
            IDLA, IDLA, SkippedField;
            IDLC, IDLC, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            IDLF, IDLF, SkippedField;
            IDLT, IDLT, SkippedField;
        ];

        COLL, CollisionLayer, [
            DESC, DESC, SkippedField;
            EDID;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            GNAM, GNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            INTV, INTV, SkippedField;
        ];

        TERM, Terminal, [
            NAM0, NAM0, SkippedField;
            CIS1, CIS1, SkippedField;
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            KWDA, KWDA, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            ITID, ITID, SkippedField;
            OBND, OBND, SkippedField;
            XMRK, XMRK, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            ANAM, ANAM, SkippedField;
            FNAM, FNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            UNAM, UNAM, SkippedField;
            WNAM, WNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            CNTO, CNTO, SkippedField;
            PRPS, PRPS, SkippedField;
            COCT, COCT, SkippedField;
            WBDT, WBDT, SkippedField;
            MODT, MODT, SkippedField;
            BTXT, BTXT, SkippedField;
            ITXT, ITXT, SkippedField;
            BSIZ, BSIZ, SkippedField;
            ISIZ, ISIZ, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        CSTY, CombatStyle, [
            CSRA, CSRA, SkippedField;
            DATA, DATA, SkippedField;
            CSGD, CSGD, SkippedField;
            EDID;
            CSME, CSME, SkippedField;
            CSFL, CSFL, SkippedField;
            CSCR, CSCR, SkippedField;
            CSLR, CSLR, SkippedField;
            CSCV, CSCV, SkippedField;
        ];

        CLMT, Climate, [
            EDID;
            MODL, MODL, SkippedField;
            FNAM, FNAM, SkippedField;
            GNAM, GNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            MODT, MODT, SkippedField;
            WLST, WLST, SkippedField;
        ];

        MUSC, MusicType, [
            EDID;
            FNAM, FNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            WNAM, WNAM, SkippedField;
        ];

        FSTS, FootstepSet, [
            DATA, DATA, SkippedField;
            EDID;
            XCNT, XCNT, SkippedField;
        ];

        HDPT, HeadPart, [
            NAM0, NAM0, SkippedField;
            NAM1, NAM1, SkippedField;
            CTDA, CTDA, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            HNAM, HNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            RNAM, RNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        ALCH, Ingestible, [
            CTDA, CTDA, SkippedField;
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            EFID, EFID, SkippedField;
            OBND, OBND, SkippedField;
            CUSD, CUSD, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
            EFIT, EFIT, SkippedField;
            ENIT, ENIT, SkippedField;
            DEST, DEST, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        LGTM, LightingTemplate, [
            DATA, DATA, SkippedField;
            DALC, DALC, SkippedField;
            EDID;
        ];

        DLVW, DialogView, [
            EDID;
            BNAM, BNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            QNAM, QNAM, SkippedField;
        ];

        RFGP, ReferenceGroup, [
            NNAM, NNAM, SkippedField;
            RNAM, RNAM, SkippedField;
        ];

        SNDR, SoundDescriptor, [
            CTDA, CTDA, SkippedField;
            ITMC, ITMC, SkippedField;
            EDID;
            ITME, ITME, SkippedField;
            ANAM, ANAM, SkippedField;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            GNAM, GNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            SNAM, SNAM, SkippedField;
            ITMS, ITMS, SkippedField;
            INTV, INTV, SkippedField;
        ];

        REVB, ReverbParameters, [
            DATA, DATA, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
        ];

        MSWP, MaterialSwap, [
            EDID;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            SNAM, SNAM, SkippedField;
        ];

        MSTT, MoveableStatic, [
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            DSTA, DSTA, SkippedField;
            MODC, MODC, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            DSTD, DSTD, SkippedField;
            DSTF, DSTF, SkippedField;
            DMDL, DMDL, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            SNAM, SNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            DMDS, DMDS, SkippedField;
            MODS, MODS, SkippedField;
            PRPS, PRPS, SkippedField;
            DMDT, DMDT, SkippedField;
            MODT, MODT, SkippedField;
            DEST, DEST, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        ASPC, AcousticSpace, [
            EDID;
            OBND, OBND, SkippedField;
            XTRI, XTRI, SkippedField;
            BNAM, BNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            WNAM, WNAM, SkippedField;
            RDAT, RDAT, SkippedField;
        ];

        EFSH, EffectShader, [
            ICO2, ICO2, SkippedField;
            NAM7, NAM7, SkippedField;
            NAM8, NAM8, SkippedField;
            NAM9, NAM9, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
            ICON, ICON, SkippedField;
            MODT, MODT, SkippedField;
        ];

        EQUP, EquipType, [
            DATA, DATA, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
            PNAM, PNAM, SkippedField;
        ];

        MESG, Message, [
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            INAM, INAM, SkippedField;
            NNAM, NNAM, SkippedField;
            QNAM, QNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            ITXT, ITXT, SkippedField;
        ];

        PROJ, Projectile, [
            NAM1, NAM1, SkippedField;
            NAM2, NAM2, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            VNAM, VNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        WATR, Water, [
            NAM0, NAM0, SkippedField;
            NAM1, NAM1, SkippedField;
            NAM2, NAM2, SkippedField;
            NAM3, NAM3, SkippedField;
            NAM4, NAM4, SkippedField;
            DATA, DATA, SkippedField;
            EDID;
            FULL, FULL, SkippedField;
            ANAM, ANAM, SkippedField;
            DNAM, DNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            GNAM, GNAM, SkippedField;
            INAM, INAM, SkippedField;
            SNAM, SNAM, SkippedField;
            TNAM, TNAM, SkippedField;
            XNAM, XNAM, SkippedField;
            YNAM, YNAM, SkippedField;
        ];

        AVIF, ActorValueInformation, [
            NAM0, NAM0, SkippedField;
            NAM1, NAM1, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            AVFL, AVFL, SkippedField;
            FULL, FULL, SkippedField;
            ANAM, ANAM, SkippedField;
        ];

        ARTO, ArtObject, [
            KWDA, KWDA, SkippedField;
            MODC, MODC, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        WEAP, Weapon, [
            MOD4, MOD4, SkippedField;
            KWDA, KWDA, SkippedField;
            DAMA, DAMA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            WAMD, WAMD, SkippedField;
            WZMD, WZMD, SkippedField;
            OBND, OBND, SkippedField;
            INRD, INRD, SkippedField;
            OBTE, OBTE, SkippedField;
            OBTF, OBTF, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            INAM, INAM, SkippedField;
            LNAM, LNAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            EITM, EITM, SkippedField;
            PTRN, PTRN, SkippedField;
            STOP, STOP, SkippedField;
            ETYP, ETYP, SkippedField;
            APPR, APPR, SkippedField;
            BIDS, BIDS, SkippedField;
            OBTS, OBTS, SkippedField;
            MO4T, MO4T, SkippedField;
            MODT, MODT, SkippedField;
            CRDT, CRDT, SkippedField;
            BAMT, BAMT, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        CLFM, Color, [
            CTDA, CTDA, SkippedField;
            EDID;
            FULL, FULL, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
        ];

        PKIN, PackIn, [
            EDID;
            OBND, OBND, SkippedField;
            CNAM, CNAM, SkippedField;
            VNAM, VNAM, SkippedField;
            FLTR, FLTR, SkippedField;
        ];

        PACK, Package, [
            CIS1, CIS1, SkippedField;
            PKC2, PKC2, SkippedField;
            PFO2, PFO2, SkippedField;
            CIS2, CIS2, SkippedField;
            POBA, POBA, SkippedField;
            POCA, POCA, SkippedField;
            CTDA, CTDA, SkippedField;
            PTDA, PTDA, SkippedField;
            POEA, POEA, SkippedField;
            IDLA, IDLA, SkippedField;
            PRCB, PRCB, SkippedField;
            IDLC, IDLC, SkippedField;
            CITC, CITC, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            IDLF, IDLF, SkippedField;
            ANAM, ANAM, SkippedField;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            INAM, INAM, SkippedField;
            PNAM, PNAM, SkippedField;
            QNAM, QNAM, SkippedField;
            UNAM, UNAM, SkippedField;
            XNAM, XNAM, SkippedField;
            PDTO, PDTO, SkippedField;
            PKDT, PKDT, SkippedField;
            PLDT, PLDT, SkippedField;
            PSDT, PSDT, SkippedField;
            IDLT, IDLT, SkippedField;
            PKCU, PKCU, SkippedField;
        ];

        FURN, Furniture, [
            NAM0, NAM0, SkippedField;
            KWDA, KWDA, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            DSTD, DSTD, SkippedField;
            OBTE, OBTE, SkippedField;
            DSTF, DSTF, SkippedField;
            XMRK, XMRK, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            ENAM, ENAM, SkippedField;
            FNAM, FNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            CNTO, CNTO, SkippedField;
            STOP, STOP, SkippedField;
            FNPR, FNPR, SkippedField;
            APPR, APPR, SkippedField;
            PRPS, PRPS, SkippedField;
            OBTS, OBTS, SkippedField;
            COCT, COCT, SkippedField;
            WBDT, WBDT, SkippedField;
            MODT, MODT, SkippedField;
            DEST, DEST, SkippedField;
            ATTX, ATTX, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        NOTE, Note, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODT, MODT, SkippedField;
        ];

        IMGS, ImageSpace, [
            TX00, TX00, SkippedField;
            EDID;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            ENAM, ENAM, SkippedField;
            HNAM, HNAM, SkippedField;
            TNAM, TNAM, SkippedField;
        ];

        OTFT, Outfit, [
            EDID;
            INAM, INAM, SkippedField;
        ];

        COBJ, ConstructibleObject, [
            NAM1, NAM1, SkippedField;
            NAM2, NAM2, SkippedField;
            NAM3, NAM3, SkippedField;
            CTDA, CTDA, SkippedField;
            FVPA, FVPA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            INTV, INTV, SkippedField;
        ];

        LVLN, LeveledNPC, [
            EDID;
            LVLD, LVLD, SkippedField;
            OBND, OBND, SkippedField;
            LVLF, LVLF, SkippedField;
            LVLM, LVLM, SkippedField;
            LVLO, LVLO, SkippedField;
            LLCT, LLCT, SkippedField;
        ];

        AMDL, AimModel, [
            EDID;
            DNAM, DNAM, SkippedField;
        ];

        PERK, Perk, [
            CIS1, CIS1, SkippedField;
            EPF2, EPF2, SkippedField;
            EPF3, EPF3, SkippedField;
            CTDA, CTDA, SkippedField;
            DATA, DATA, SkippedField;
            EPFB, EPFB, SkippedField;
            PRKC, PRKC, SkippedField;
            DESC, DESC, SkippedField;
            VMAD, VMAD, SkippedField;
            EPFD, EPFD, SkippedField;
            EDID;
            PRKE, PRKE, SkippedField;
            PRKF, PRKF, SkippedField;
            FULL, FULL, SkippedField;
            FNAM, FNAM, SkippedField;
            NNAM, NNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            EPFT, EPFT, SkippedField;
        ];

        AECH, AudioEffectChain, [
            EDID;
            DNAM, DNAM, SkippedField;
            KNAM, KNAM, SkippedField;
        ];

        BOOK, Book, [
            KWDA, KWDA, SkippedField;
            DATA, DATA, SkippedField;
            DESC, DESC, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            FIMD, FIMD, SkippedField;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            INAM, INAM, SkippedField;
            YNAM, YNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
            PTRN, PTRN, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
            KSIZ, KSIZ, SkippedField;
        ];

        FLST, FormIdList, [
            EDID;
            FULL, FULL, SkippedField;
            LNAM, LNAM, SkippedField;
        ];

        NOCM, NavigationMeshObstacleManager, [
            NAM1, NAM1, SkippedField;
            DATA, DATA, SkippedField;
            INTV, INTV, SkippedField;
            INDX, INDX, SkippedField;
        ];

        MUST, MusicTrack, [
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            CITC, CITC, SkippedField;
            EDID;
            ANAM, ANAM, SkippedField;
            BNAM, BNAM, SkippedField;
            CNAM, CNAM, SkippedField;
            DNAM, DNAM, SkippedField;
            FNAM, FNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            FLTV, FLTV, SkippedField;
        ];

        LSCR, LoadScreen, [
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            NNAM, NNAM, SkippedField;
            ONAM, ONAM, SkippedField;
            TNAM, TNAM, SkippedField;
            ZNAM, ZNAM, SkippedField;
        ];

        ADDN, AddonNode, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            DNAM, DNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            MODT, MODT, SkippedField;
        ];

        OMOD, ObjectModification, [
            NAM1, NAM1, SkippedField;
            DATA, DATA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            FNAM, FNAM, SkippedField;
            LNAM, LNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            FLTR, FLTR, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
        ];

        TXST, TextureSet, [
            TX00, TX00, SkippedField;
            TX01, TX01, SkippedField;
            TX02, TX02, SkippedField;
            TX03, TX03, SkippedField;
            TX04, TX04, SkippedField;
            TX05, TX05, SkippedField;
            TX07, TX07, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            DNAM, DNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            DODT, DODT, SkippedField;
        ];

        CLAS, Class, [
            DATA, DATA, SkippedField;
            DESC, DESC, SkippedField;
            EDID;
            FULL, FULL, SkippedField;
            PRPS, PRPS, SkippedField;
        ];

        LCRT, LocationReferenceType, [
            EDID;
            CNAM, CNAM, SkippedField;
            TNAM, TNAM, SkippedField;
        ];

        DOBJ, DefaultObjectManager, [
            DNAM, DNAM, SkippedField;
        ];

        SPGD, ShaderParticleGeometry, [
            DATA, DATA, SkippedField;
            EDID;
            MNAM, MNAM, SkippedField;
        ];

        TRNS, Transform, [
            DATA, DATA, SkippedField;
            EDID;
        ];

        SCOL, StaticCollection, [
            DATA, DATA, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            ONAM, ONAM, SkippedField;
            PTRN, PTRN, SkippedField;
            FLTR, FLTR, SkippedField;
            MODS, MODS, SkippedField;
            MODT, MODT, SkippedField;
        ];

        STAT, Static, [
            MODC, MODC, SkippedField;
            VMAD, VMAD, SkippedField;
            EDID;
            OBND, OBND, SkippedField;
            MODL, MODL, SkippedField;
            FULL, FULL, SkippedField;
            DNAM, DNAM, SkippedField;
            MNAM, MNAM, SkippedField;
            NVNM, NVNM, SkippedField;
            PTRN, PTRN, SkippedField;
            FTYP, FTYP, SkippedField;
            MODS, MODS, SkippedField;
            PRPS, PRPS, SkippedField;
            MODT, MODT, SkippedField;
        ];

        AORU, AttractionRule, [
            AOR2, AOR2, SkippedField;
            EDID;
        ];

        SMBN, StoryManagerBranchNode, [
            CIS2, CIS2, SkippedField;
            CTDA, CTDA, SkippedField;
            CITC, CITC, SkippedField;
            EDID;
            DNAM, DNAM, SkippedField;
            PNAM, PNAM, SkippedField;
            SNAM, SNAM, SkippedField;
            XNAM, XNAM, SkippedField;
        ];

    

    
    ]
}