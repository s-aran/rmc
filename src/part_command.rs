use crate::{
    meta_models::MetaData,
    models::{DivisorClock, NegativePositive, NoteCommand},
};

#[derive(Debug, Clone)]
pub struct Note {
    command: char,
    natural: Option<()>,
    semitone: Option<NegativePositive>,
    length: Option<u8>,
    dots: Option<String>,
}

#[derive(Debug, Clone, strum::EnumString)]
pub enum PartCommand {
    Nop,
    #[strum(serialize = "c")]
    NoteC(Code, Note),
    #[strum(serialize = "d")]
    NoteD {
        natural: Option<()>,
        semitone: Option<NegativePositive>,
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "e")]
    NoteE {
        natural: Option<()>,
        semitone: Option<NegativePositive>,
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "f")]
    NoteF {
        natural: Option<()>,
        semitone: Option<NegativePositive>,
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "g")]
    NoteG {
        natural: Option<()>,
        semitone: Option<NegativePositive>,
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "a")]
    NoteA {
        natural: Option<()>,
        semitone: Option<NegativePositive>,
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "b")]
    NoteB {
        natural: Option<()>,
        semitone: Option<NegativePositive>,
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "x")]
    NoteX {
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "r")]
    NoteR {
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "[")]
    PortamentoBegin {
        pitch1: Option<NoteCommand>,
        pitch2: Option<NoteCommand>,
        length1: Option<u8>,
        dots: Option<String>,
        length2: Option<u8>,
    },
    #[strum(serialize = "]")]
    PortamentoEnd,
    #[strum(serialize = "o")]
    Octave {
        value: u8,
    },
    #[strum(serialize = ">")]
    OctaveUp,
    #[strum(serialize = "<")]
    OctaveDown,
    #[strum(serialize = "X")]
    OctaveReverse,
    #[strum(serialize = "o+")]
    PartOctaveChangePositive,
    #[strum(serialize = "o-")]
    PartOctaveChangeNegative,
    #[strum(serialize = "l")]
    DefaultLength {
        value_type: Option<DivisorClock>,
        value: u8,
        dots: Option<String>,
    },
    #[strum(serialize = "l=")]
    ProcessLastLengthUpdate {
        value_type: Option<DivisorClock>,
        value: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "l+")]
    ProcessLastLengthAdd {
        value_type: Option<DivisorClock>,
        value: u8,
        dots: Option<String>,
    },
    #[strum(serialize = "l-")]
    ProcessLastLengthSubtract {
        value_type: Option<DivisorClock>,
        value: u8,
        dots: Option<String>,
    },
    #[strum(serialize = "l^")]
    ProcessLastLengthMultiply {
        value: u8,
    },
    #[strum(serialize = "&")]
    Tie {
        length: Option<u8>,
        dots: Option<String>,
    },
    #[strum(serialize = "&&")]
    Slur {
        length: Option<u8>,
        dots: Option<String>,
    },
}

pub type WrappedPartCommand = MetaData<PartCommand>;

// impl WrappedPartCommand {
//     pub fn new(code: &Code, command: PartCommand) -> Self {
//         MetaData::<PartCommand>::new(code, command)
//     }
// }
