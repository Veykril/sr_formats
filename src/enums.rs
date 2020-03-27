use nom::combinator::map_res;
use nom::error::ParseError;
use nom::number::complete::le_u32;
use nom::IResult;

#[cfg(feature = "serde")]
use serde::Serialize;

use std::convert::TryFrom;

#[repr(u32)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum TileSound {
    Dirt = 0,
    Sand = 1,
    Ashfield = 2,
    Stone = 3,
    Metal = 4,
    Wood = 5,
    Mud = 6,
    Water = 7,
    DeepWater = 8,
    Snow = 9,
    Grass = 10,
    LongGrass = 11,
    Forest = 12,
    Cloud = 13,
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownTileSound(u32);

impl TryFrom<u32> for TileSound {
    type Error = UnknownTileSound;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(TileSound::Dirt),
            1 => Ok(TileSound::Sand),
            2 => Ok(TileSound::Ashfield),
            3 => Ok(TileSound::Stone),
            4 => Ok(TileSound::Metal),
            5 => Ok(TileSound::Wood),
            6 => Ok(TileSound::Mud),
            7 => Ok(TileSound::Water),
            8 => Ok(TileSound::DeepWater),
            9 => Ok(TileSound::Snow),
            10 => Ok(TileSound::Grass),
            11 => Ok(TileSound::LongGrass),
            12 => Ok(TileSound::Forest),
            13 => Ok(TileSound::Cloud),
            val => Err(UnknownTileSound(val)),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum NewInterfaceType {
    CNIFMainFrame = 0,
    CNIFrame = 1,
    CNIFNormaltile = 2,
    CNIFStretch = 3,
    CNIFButton = 4,
    CNIFStatic = 5,
    CNIFEdit = 6,
    CNIFTextBox = 7,
    CNIFSlot = 8,
    CNIFLattice = 9,
    CNIFGauge = 10,
    CNIFCheckBox = 11,
    CNIFComboBox = 12,
    CNIFVirticalScroll = 13,
    CNIFPageManager = 14,
    CNIFBarWnd = 15,
    CNIFTabButton = 16,
    CNIFBothSidesGauge = 17,
    CNIFWnd = 18,
    CNIFSlideCtrl = 19,
    CNIFSpinButtonCtrl = 20,
}

impl NewInterfaceType {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map_res(le_u32, TryFrom::try_from)(i)
    }
}

impl TryFrom<u32> for NewInterfaceType {
    type Error = UnknownNewInterfaceType;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0 => Ok(NewInterfaceType::CNIFMainFrame),
            1 => Ok(NewInterfaceType::CNIFrame),
            2 => Ok(NewInterfaceType::CNIFNormaltile),
            3 => Ok(NewInterfaceType::CNIFStretch),
            4 => Ok(NewInterfaceType::CNIFButton),
            5 => Ok(NewInterfaceType::CNIFStatic),
            6 => Ok(NewInterfaceType::CNIFEdit),
            7 => Ok(NewInterfaceType::CNIFTextBox),
            8 => Ok(NewInterfaceType::CNIFSlot),
            9 => Ok(NewInterfaceType::CNIFLattice),
            10 => Ok(NewInterfaceType::CNIFGauge),
            11 => Ok(NewInterfaceType::CNIFCheckBox),
            12 => Ok(NewInterfaceType::CNIFComboBox),
            13 => Ok(NewInterfaceType::CNIFVirticalScroll),
            14 => Ok(NewInterfaceType::CNIFPageManager),
            15 => Ok(NewInterfaceType::CNIFBarWnd),
            16 => Ok(NewInterfaceType::CNIFTabButton),
            17 => Ok(NewInterfaceType::CNIFBothSidesGauge),
            18 => Ok(NewInterfaceType::CNIFWnd),
            19 => Ok(NewInterfaceType::CNIFSlideCtrl),
            20 => Ok(NewInterfaceType::CNIFSpinButtonCtrl),
            val => Err(UnknownNewInterfaceType(val)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownNewInterfaceType(u32);

impl std::error::Error for UnknownNewInterfaceType {}
impl std::fmt::Display for UnknownNewInterfaceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "encountered unknown UnknownNewInterfaceType with value {:X}",
            self.0
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownResourceType(u32);

impl std::error::Error for UnknownResourceType {}
impl std::fmt::Display for UnknownResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "encountered unknown ResourceType with value {:X}",
            self.0
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UnknownResourceAnimationType(u32);

impl std::error::Error for UnknownResourceAnimationType {}
impl std::fmt::Display for UnknownResourceAnimationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "encountered unknown ResourceAnimationType with value {:X}",
            self.0
        )
    }
}

#[repr(u32)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ResourceType {
    /// Characters of all races (EU, CH)
    Character = 0x20000,
    /// NPCs, Monsters, COS
    Npc = 0x20001,
    /// Walls, Houses, Fences
    Building = 0x20002,
    /// Static Map-Objects that are not buildings (carriage, bones, etc.)
    Artifact = 0x20003,
    /// Trees, Plants, Flowers, Bushes
    Nature = 0x20004,
    /// All Items-Props
    Item = 0x20005,
    /// Drops, Marks
    Other = 0x20006,
    /// Compound of character and items he is wearing.
    CompoundCharacter = 0x30000,
    /// Compound of multiple buildings, artifacts or nature objects.
    CompoundObject = 0x30002,
}

impl ResourceType {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        map_res(le_u32, TryFrom::try_from)(i)
    }
}

impl TryFrom<u32> for ResourceType {
    type Error = UnknownResourceType;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        match val {
            0x20000 => Ok(ResourceType::Character),
            0x20001 => Ok(ResourceType::Npc),
            0x20002 => Ok(ResourceType::Building),
            0x20003 => Ok(ResourceType::Artifact),
            0x20004 => Ok(ResourceType::Nature),
            0x20005 => Ok(ResourceType::Item),
            0x20006 => Ok(ResourceType::Other),
            0x30000 => Ok(ResourceType::CompoundCharacter),
            0x30002 => Ok(ResourceType::CompoundObject),
            val => Err(UnknownResourceType(val)),
        }
    }
}

#[repr(u32)]
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ResourceAnimationType {
    Pose = 0x3C,

    Stand1 = 0x00,
    //Stand2 = 0x08,
    Stand2 = 0x7A,
    Stand3 = 0x3D,
    Stand4 = 0x51,

    AttReady = 0x06,

    TurnL = 0x18,
    TurnR = 0x19,

    SitDown = 0x0D,
    Sit = 0x0E,
    StandUp = 0x0F,

    Defence = 0x16,

    Walk = 0x01,
    WalkBack = 0x17,
    Run = 0x07,

    Attack1 = 0x02,
    Attack2 = 0x05,
    Attack3 = 0x10,
    Attack4 = 0x11,
    Attack5 = 0xB7,
    Attack6 = 0xB8,
    Attack7 = 0xB9,
    Attack8 = 0xBA,
    Attack9 = 0xBE,

    Revolution = 0x27,

    Skill1 = 0x1A,
    Skill2 = 0x1B,
    Skill3 = 0x1C,
    Skill4 = 0x1D,
    Skill5 = 0x1E,
    Skill6 = 0x1F,
    Skill7 = 0x20,
    Skill8 = 0x21,
    Skill9 = 0x22,
    Skill10 = 0x23,

    Skill11 = 0x44,
    Skill12 = 0x45,
    Skill13 = 0x46,
    Skill14 = 0x47,
    Skill15 = 0x48,
    Skill16 = 0x49,
    Skill17 = 0x4A,
    Skill18 = 0x4B,
    Skill19 = 0x4C,
    Skill20 = 0x4D,

    Skill21 = 0x65,
    Skill22 = 0x66,
    Skill23 = 0x67,
    Skill24 = 0x68,
    Skill25 = 0x69,
    Skill26 = 0x6A,
    Skill27 = 0x6B,
    Skill28 = 0x6C,
    Skill29 = 0x6D,
    Skill30 = 0x6E,
    Skill31 = 0x6F,
    Skill32 = 0x70,
    Skill33 = 0x71,
    Skill34 = 0x72,
    Skill35 = 0x73,
    Skill36 = 0x74,
    Skill37 = 0x75,
    Skill38 = 0x76,
    Skill39 = 0x77,
    Skill40 = 0x78,

    Skill41 = 0x7B,
    Skill42 = 0x7C,
    Skill43 = 0x7D,
    Skill44 = 0x7E,
    Skill45 = 0x7F,
    Skill46 = 0x80,
    Skill47 = 0x81,
    Skill48 = 0x82,
    Skill49 = 0x83,
    Skill50 = 0x84,
    Skill51 = 0x85,
    Skill52 = 0x86,
    Skill53 = 0x87,
    Skill54 = 0x88,
    Skill55 = 0x89,
    Skill56 = 0x8A,
    Skill57 = 0x8B,
    Skill58 = 0x8C,
    Skill59 = 0x8D,
    Skill60 = 0x8E,
    Skill61 = 0x8F,
    Skill62 = 0x90,
    Skill63 = 0x91,
    Skill64 = 0x92,
    Skill65 = 0x93,
    Skill66 = 0x94,
    Skill67 = 0x95,
    Skill68 = 0x96,
    Skill69 = 0x97,
    Skill70 = 0x98,
    Skill71 = 0x99,
    Skill72 = 0x9A,
    Skill73 = 0x9B,
    Skill74 = 0x9C,
    Skill75 = 0x9D,
    Skill76 = 0x9E,
    Skill77 = 0x9F,
    Skill78 = 0xA0,
    Skill79 = 0xA1,
    Skill80 = 0xA2,
    Skill81 = 0xA3,
    Skill82 = 0xA4,
    Skill83 = 0xA5,
    Skill84 = 0xA6,
    Skill85 = 0xA7,
    Skill86 = 0xA8,
    Skill87 = 0xA9,
    Skill88 = 0xAA,
    Skill89 = 0xAB,
    Skill90 = 0xAC,
    Skill91 = 0xAD,
    Skill92 = 0xAE,
    Skill93 = 0xAF,
    Skill94 = 0xB0,
    Skill95 = 0xB1,
    Skill96 = 0xB2,
    Skill97 = 0xB3,
    Skill98 = 0xB4,
    Skill99 = 0xB5,
    Skill100 = 0xB6,

    Ready1 = 0x28,
    Ready2 = 0x29,
    Ready3 = 0x2A,
    Ready4 = 0x2B,
    Ready5 = 0x2C,

    Wait1 = 0x5B,
    Wait2 = 0x5C,
    Wait3 = 0x5D,
    Wait4 = 0x5E,
    Wait5 = 0x5F,

    Hammer = 0xBB,
    HandLoof = 0xBC,
    Throw = 0xBD,
    MgSSelf = 0x13,
    MgSOther = 0x14,
    Damage1 = 0x03,
    Damage2 = 0x09,
    Help = 0x43,
    Find = 0x4E,
    Stun = 0x4F,

    Die1 = 0x04,
    Die1Rm = 0x24,
    Die2 = 0x12,
    Die2Rm = 0x25,

    Revival = 0x79,

    Down = 0x3E,
    DownRm = 0x3F,
    DownDamage = 0x40,
    DownUp = 0x41,
    DownDie = 0x42,

    Pick = 0x26,
    Click = 0x0A,

    CbYeonhwan = 0x0B,
    Cb2 = 0x0C,

    EtBye = 0x15,

    Emotion01 = 0x32,
    Emotion02 = 0x33,
    Emotion03 = 0x34,
    Emotion04 = 0x35,
    Emotion05 = 0x36,
    Emotion06 = 0x37,
    Emotion07 = 0x38,
    Emotion08 = 0x39,
    Emotion09 = 0x3A,
    Emotion10 = 0x3B,

    Vendor01 = 0x50,

    Shot = 0xBF,
}

impl ResourceAnimationType {
    pub fn parse<'a, E: ParseError<&'a [u8]>>(i: &'a [u8]) -> IResult<&'a [u8], Self, E> {
        nom::error::context(
            "unknown ResourceAnimationType",
            map_res(le_u32, TryFrom::try_from),
        )(i)
    }
}

impl TryFrom<u32> for ResourceAnimationType {
    type Error = UnknownResourceAnimationType;

    fn try_from(val: u32) -> Result<Self, Self::Error> {
        Ok(match val {
            0x3C => ResourceAnimationType::Pose,
            0x00 => ResourceAnimationType::Stand1,
            /* 0x8 is used as Stand2 in older files? */
            0x7A | 0x8 => ResourceAnimationType::Stand2,
            0x3D => ResourceAnimationType::Stand3,
            0x51 => ResourceAnimationType::Stand4,
            0x06 => ResourceAnimationType::AttReady,
            0x18 => ResourceAnimationType::TurnL,
            0x19 => ResourceAnimationType::TurnR,
            0x0D => ResourceAnimationType::SitDown,
            0x0E => ResourceAnimationType::Sit,
            0x0F => ResourceAnimationType::StandUp,
            0x16 => ResourceAnimationType::Defence,
            0x01 => ResourceAnimationType::Walk,
            0x17 => ResourceAnimationType::WalkBack,
            0x07 => ResourceAnimationType::Run,
            0x02 => ResourceAnimationType::Attack1,
            0x05 => ResourceAnimationType::Attack2,
            0x10 => ResourceAnimationType::Attack3,
            0x11 => ResourceAnimationType::Attack4,
            0xB7 => ResourceAnimationType::Attack5,
            0xB8 => ResourceAnimationType::Attack6,
            0xB9 => ResourceAnimationType::Attack7,
            0xBA => ResourceAnimationType::Attack8,
            0xBE => ResourceAnimationType::Attack9,
            0x27 => ResourceAnimationType::Revolution,
            0x1A => ResourceAnimationType::Skill1,
            0x1B => ResourceAnimationType::Skill2,
            0x1C => ResourceAnimationType::Skill3,
            0x1D => ResourceAnimationType::Skill4,
            0x1E => ResourceAnimationType::Skill5,
            0x1F => ResourceAnimationType::Skill6,
            0x20 => ResourceAnimationType::Skill7,
            0x21 => ResourceAnimationType::Skill8,
            0x22 => ResourceAnimationType::Skill9,
            0x23 => ResourceAnimationType::Skill10,
            0x44 => ResourceAnimationType::Skill11,
            0x45 => ResourceAnimationType::Skill12,
            0x46 => ResourceAnimationType::Skill13,
            0x47 => ResourceAnimationType::Skill14,
            0x48 => ResourceAnimationType::Skill15,
            0x49 => ResourceAnimationType::Skill16,
            0x4A => ResourceAnimationType::Skill17,
            0x4B => ResourceAnimationType::Skill18,
            0x4C => ResourceAnimationType::Skill19,
            0x4D => ResourceAnimationType::Skill20,
            0x65 => ResourceAnimationType::Skill21,
            0x66 => ResourceAnimationType::Skill22,
            0x67 => ResourceAnimationType::Skill23,
            0x68 => ResourceAnimationType::Skill24,
            0x69 => ResourceAnimationType::Skill25,
            0x6A => ResourceAnimationType::Skill26,
            0x6B => ResourceAnimationType::Skill27,
            0x6C => ResourceAnimationType::Skill28,
            0x6D => ResourceAnimationType::Skill29,
            0x6E => ResourceAnimationType::Skill30,
            0x6F => ResourceAnimationType::Skill31,
            0x70 => ResourceAnimationType::Skill32,
            0x71 => ResourceAnimationType::Skill33,
            0x72 => ResourceAnimationType::Skill34,
            0x73 => ResourceAnimationType::Skill35,
            0x74 => ResourceAnimationType::Skill36,
            0x75 => ResourceAnimationType::Skill37,
            0x76 => ResourceAnimationType::Skill38,
            0x77 => ResourceAnimationType::Skill39,
            0x78 => ResourceAnimationType::Skill40,
            0x7B => ResourceAnimationType::Skill41,
            0x7C => ResourceAnimationType::Skill42,
            0x7D => ResourceAnimationType::Skill43,
            0x7E => ResourceAnimationType::Skill44,
            0x7F => ResourceAnimationType::Skill45,
            0x80 => ResourceAnimationType::Skill46,
            0x81 => ResourceAnimationType::Skill47,
            0x82 => ResourceAnimationType::Skill48,
            0x83 => ResourceAnimationType::Skill49,
            0x84 => ResourceAnimationType::Skill50,
            0x85 => ResourceAnimationType::Skill51,
            0x86 => ResourceAnimationType::Skill52,
            0x87 => ResourceAnimationType::Skill53,
            0x88 => ResourceAnimationType::Skill54,
            0x89 => ResourceAnimationType::Skill55,
            0x8A => ResourceAnimationType::Skill56,
            0x8B => ResourceAnimationType::Skill57,
            0x8C => ResourceAnimationType::Skill58,
            0x8D => ResourceAnimationType::Skill59,
            0x8E => ResourceAnimationType::Skill60,
            0x8F => ResourceAnimationType::Skill61,
            0x90 => ResourceAnimationType::Skill62,
            0x91 => ResourceAnimationType::Skill63,
            0x92 => ResourceAnimationType::Skill64,
            0x93 => ResourceAnimationType::Skill65,
            0x94 => ResourceAnimationType::Skill66,
            0x95 => ResourceAnimationType::Skill67,
            0x96 => ResourceAnimationType::Skill68,
            0x97 => ResourceAnimationType::Skill69,
            0x98 => ResourceAnimationType::Skill70,
            0x99 => ResourceAnimationType::Skill71,
            0x9A => ResourceAnimationType::Skill72,
            0x9B => ResourceAnimationType::Skill73,
            0x9C => ResourceAnimationType::Skill74,
            0x9D => ResourceAnimationType::Skill75,
            0x9E => ResourceAnimationType::Skill76,
            0x9F => ResourceAnimationType::Skill77,
            0xA0 => ResourceAnimationType::Skill78,
            0xA1 => ResourceAnimationType::Skill79,
            0xA2 => ResourceAnimationType::Skill80,
            0xA3 => ResourceAnimationType::Skill81,
            0xA4 => ResourceAnimationType::Skill82,
            0xA5 => ResourceAnimationType::Skill83,
            0xA6 => ResourceAnimationType::Skill84,
            0xA7 => ResourceAnimationType::Skill85,
            0xA8 => ResourceAnimationType::Skill86,
            0xA9 => ResourceAnimationType::Skill87,
            0xAA => ResourceAnimationType::Skill88,
            0xAB => ResourceAnimationType::Skill89,
            0xAC => ResourceAnimationType::Skill90,
            0xAD => ResourceAnimationType::Skill91,
            0xAE => ResourceAnimationType::Skill92,
            0xAF => ResourceAnimationType::Skill93,
            0xB0 => ResourceAnimationType::Skill94,
            0xB1 => ResourceAnimationType::Skill95,
            0xB2 => ResourceAnimationType::Skill96,
            0xB3 => ResourceAnimationType::Skill97,
            0xB4 => ResourceAnimationType::Skill98,
            0xB5 => ResourceAnimationType::Skill99,
            0xB6 => ResourceAnimationType::Skill100,
            0x28 => ResourceAnimationType::Ready1,
            0x29 => ResourceAnimationType::Ready2,
            0x2A => ResourceAnimationType::Ready3,
            0x2B => ResourceAnimationType::Ready4,
            0x2C => ResourceAnimationType::Ready5,
            0x5B => ResourceAnimationType::Wait1,
            0x5C => ResourceAnimationType::Wait2,
            0x5D => ResourceAnimationType::Wait3,
            0x5E => ResourceAnimationType::Wait4,
            0x5F => ResourceAnimationType::Wait5,
            0xBB => ResourceAnimationType::Hammer,
            0xBC => ResourceAnimationType::HandLoof,
            0xBD => ResourceAnimationType::Throw,
            0x13 => ResourceAnimationType::MgSSelf,
            0x14 => ResourceAnimationType::MgSOther,
            0x03 => ResourceAnimationType::Damage1,
            0x09 => ResourceAnimationType::Damage2,
            0x43 => ResourceAnimationType::Help,
            0x4E => ResourceAnimationType::Find,
            0x4F => ResourceAnimationType::Stun,
            0x04 => ResourceAnimationType::Die1,
            0x24 => ResourceAnimationType::Die1Rm,
            0x12 => ResourceAnimationType::Die2,
            0x25 => ResourceAnimationType::Die2Rm,
            0x79 => ResourceAnimationType::Revival,
            0x3E => ResourceAnimationType::Down,
            0x3F => ResourceAnimationType::DownRm,
            0x40 => ResourceAnimationType::DownDamage,
            0x41 => ResourceAnimationType::DownUp,
            0x42 => ResourceAnimationType::DownDie,
            0x26 => ResourceAnimationType::Pick,
            0x0A => ResourceAnimationType::Click,
            0x0B => ResourceAnimationType::CbYeonhwan,
            0x0C => ResourceAnimationType::Cb2,
            0x15 => ResourceAnimationType::EtBye,
            0x32 => ResourceAnimationType::Emotion01,
            0x33 => ResourceAnimationType::Emotion02,
            0x34 => ResourceAnimationType::Emotion03,
            0x35 => ResourceAnimationType::Emotion04,
            0x36 => ResourceAnimationType::Emotion05,
            0x37 => ResourceAnimationType::Emotion06,
            0x38 => ResourceAnimationType::Emotion07,
            0x39 => ResourceAnimationType::Emotion08,
            0x3A => ResourceAnimationType::Emotion09,
            0x3B => ResourceAnimationType::Emotion10,
            0x50 => ResourceAnimationType::Vendor01,
            0xBF => ResourceAnimationType::Shot,
            val => return Err(UnknownResourceAnimationType(val)),
        })
    }
}
