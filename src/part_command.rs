use std::str::FromStr;

use crate::{
    commands::{
        commands_envelope::SsgPcmSoftwareEnvelope,
        commands_loop::LocalLoop,
        commands_mml::{
            DefaultLength, MasterTranspose, Note, NoteR, NoteX, Octave, OctaveDown, OctaveReverse,
            OctaveUp, PartTransposeBegin, PartTransposeEnd, Portamento, ProcessLastLengthAddSub,
            ProcessLastLengthMultiply, ProcessLastLengthUpdate, Quantize1, Quantize2, Slur,
            TemporaryTranspose, Tie,
        },
        commands_volume::Volume,
    },
    meta_models::{Code, MetaData, Token, TokenStackTrait, TokenTrait},
    models::{DivisorClock, NegativePositive},
    utils::get_type_name,
};

macro_rules! try_from_get_value {
    ($expr:expr, $field:ident) => {
        match $expr {
            Ok(Some(v)) => v,
            Ok(None) => panic!(
                "TryFrom for {} ({}): None",
                stringify!(Self),
                stringify!($field)
            ),
            Err(e) => panic!(
                "TryFrom for {} ({}): {}",
                stringify!($typ),
                stringify!($field),
                e
            ),
        }
    };
}

macro_rules! try_from_get_some_value {
    ($expr:expr, $field:ident) => {
        match $expr {
            Ok(v) => v,
            Err(e) => panic!(
                "TryFrom for {} ({}): {}",
                stringify!(Self),
                stringify!($field),
                e
            ),
        }
    };
}

// macro_rules! try_from_get_vec {
//     ($expr:expr, $field:ident) => {
//         match $expr {
//             Ok(v) => {
//                 if v.len() > 0 {
//                     v
//                 } else {
//                     panic!(
//                         "TryFrom for {} ({}) is empty",
//                         stringify!(Self),
//                         stringify!($field)
//                     );
//                 }
//             }
//             Err(e) => panic!(
//                 "TryFrom for {} ({}): {}",
//                 stringify!(Self),
//                 stringify!($field),
//                 e
//             ),
//         }
//     };
// }

macro_rules! try_from_get_some_vec {
    ($expr:expr, $field:ident) => {
        match $expr {
            Ok(v) => v,
            Err(e) => panic!(
                "TryFrom for {} ({}): {}",
                stringify!(Self),
                stringify!($field),
                e
            ),
        }
    };
}

pub(crate) type State = u8;

#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub(crate) struct PartToken {
    state: State,
    code: Code,
    token: Token,
}

impl TokenTrait for PartToken {
    fn chars(&self) -> &String {
        self.token.chars()
    }

    fn chars_mut(&mut self) -> &mut String {
        self.token.chars_mut()
    }

    fn begin(&self) -> &usize {
        self.token.begin()
    }

    fn begin_mut(&mut self) -> &mut usize {
        self.token.begin_mut()
    }

    fn end(&self) -> &usize {
        self.token.end()
    }

    fn end_mut(&mut self) -> &mut usize {
        self.token.end_mut()
    }
}

impl PartToken {
    pub fn set_state(&mut self, state: State) {
        self.state = state;
    }

    pub fn get_state(&self) -> State {
        self.state
    }

    pub fn set_code(&mut self, code: &Code) {
        self.code = code.clone();
    }

    pub fn get_code(&self) -> &Code {
        &self.code
    }
}
#[derive(Debug, Clone, Default)]
pub(crate) struct PartTokenStack {
    stack: Vec<PartToken>,
}

impl TokenStackTrait<PartToken> for PartTokenStack {
    fn stack(&self) -> &Vec<PartToken> {
        &self.stack
    }

    fn stack_mut(&mut self) -> &mut Vec<PartToken> {
        &mut self.stack
    }
}

impl PartTokenStack {
    pub fn first(&self) -> Option<&PartToken> {
        self.stack.first()
    }

    pub fn dequeue(&mut self) -> Option<PartToken> {
        if self.stack.len() > 0 {
            return Some(self.stack.remove(0));
        }

        None
    }

    pub fn get(&self, index: usize) -> Option<&PartToken> {
        self.stack.get(index)
    }

    pub fn find_by_state(&self, state: State) -> Vec<&PartToken> {
        self.stack.iter().filter(|&e| e.state == state).collect()
    }

    pub fn get_by_state(&self, state: State) -> Option<&PartToken> {
        let r = self.find_by_state(state);
        if r.len() > 1 {
            panic!("get_by_state({})", state);
        }

        match r.get(0) {
            Some(e) => Some(*e),
            None => None,
        }
    }

    pub fn get_and_cast<T>(&self, state: State) -> Result<Option<T>, <T as FromStr>::Err>
    where
        T: FromStr + Clone,
    {
        let t = self.get_by_state(state);
        match t {
            Some(e) => match T::from_str(e.token.chars.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            None => Ok(None),
        }
    }

    pub fn pop_by_state_all(&mut self, state: State) -> Vec<PartToken> {
        let (removed, kept) = self.stack.drain(..).partition(|e| e.state == state);
        self.stack = kept;

        removed
    }

    pub fn pop_by_state(&mut self, state: State) -> Option<PartToken> {
        let r = self.find_by_state(state);
        if r.len() > 1 {
            panic!("pop_by_state({}): len > 1", state);
        }

        match r.get(0) {
            Some(e) => {
                let t = self
                    .stack
                    .remove(self.stack.iter().position(|x| x == *e).unwrap());
                Some(t)
            }
            None => None,
        }
    }

    pub fn pop_and_cast_vec<T>(&mut self, state: State) -> Result<Vec<T>, <T as FromStr>::Err>
    where
        T: FromStr + Clone,
    {
        let t = self.pop_by_state_all(state);
        Ok(t.iter()
            .map(|e| {
                if let Ok(v) = T::from_str(e.token.chars.as_str()) {
                    v
                } else {
                    panic!("pop_and_cast_vec<{}>", get_type_name::<T>());
                }
            })
            .collect())
    }

    pub fn pop_and_cast<T>(&mut self, state: State) -> Result<Option<T>, <T as FromStr>::Err>
    where
        T: FromStr + Clone,
    {
        let t = self.pop_by_state(state);
        match t {
            Some(e) => match T::from_str(e.token.chars.as_str()) {
                Ok(v) => Ok(Some(v)),
                Err(e) => Err(e),
            },
            None => Ok(None),
        }
    }
}

pub trait PartCommandStruct: std::fmt::Debug {
    fn to_variant(self) -> PartCommand;
}

#[derive(Default, Debug, Clone)]
pub(crate) struct PartCommandStack {
    stack: Vec<Vec<WrappedPartCommand>>,
}

impl PartCommandStack {
    pub fn stack(&self) -> &Vec<Vec<WrappedPartCommand>> {
        &self.stack
    }

    pub fn stack_mut(&mut self) -> &mut Vec<Vec<WrappedPartCommand>> {
        &mut self.stack
    }

    /// Push a set of commands onto the stack (e.g., at a begin of portamento or loop).
    pub fn push(&mut self, commands: Vec<WrappedPartCommand>) {
        self.stack.push(commands);
    }

    /// Pop the most recently pushed set of commands, if any.
    pub fn pop(&mut self) -> Option<Vec<WrappedPartCommand>> {
        self.stack.pop()
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PartCommand {
    Nop,

    Note(Note),
    NoteX(NoteX),
    NoteR(NoteR),

    Portamento(Portamento),

    Octave(Octave),
    OctaveUp(OctaveUp),
    OctaveDown(OctaveDown),
    OctaveReverse(OctaveReverse),
    PartOctaveChangePositive(Octave),
    PartOctaveChangeNegative(Octave),

    DefaultLength(DefaultLength),

    ProcessLastLengthUpdate(ProcessLastLengthUpdate),
    ProcessLastLengthAdd(ProcessLastLengthAddSub),
    ProcessLastLengthSubtract(ProcessLastLengthAddSub),
    ProcessLastLengthMultiply(ProcessLastLengthMultiply),

    Tie(Tie),
    Slur(Slur),

    Quantize1(Quantize1),
    Quantize2(Quantize2),
    AbsoluteTranspose(TemporaryTranspose),
    RelativeTranspose(TemporaryTranspose),
    PartTransposeBegin(PartTransposeBegin),
    PartTransposeEnd(PartTransposeEnd),
    MasterTranspose(MasterTranspose),

    LocalLoop(LocalLoop),

    SsgPcmSoftwareEnvelope(SsgPcmSoftwareEnvelope),

    Volume1(Volume),
    Volume2(Volume),
    GlobalVolume1Positive(Volume),
    GlobalVolume1Negative(Volume),
    GlobalVolume2Positive(Volume),
    GlobalVolume2Negative(Volume),
}

pub trait IsPartCommand {}
impl IsPartCommand for PartCommand {}

pub type WrappedPartCommand = MetaData<PartCommand>;

pub(crate) fn to_some_i8(sign: Option<NegativePositive>, value: Option<u8>) -> Option<i8> {
    if value.is_none() {
        return None;
    }

    Some(match sign {
        Some(NegativePositive::Positive) | None => value.unwrap() as i8,
        Some(NegativePositive::Negative) => -(value.unwrap() as i8),
    })
}

pub(crate) fn count_dots(dots: Option<String>) -> u8 {
    if let Some(dots) = dots {
        dots.chars().filter(|&c| c == '.').count() as u8
    } else {
        0
    }
}

pub(crate) fn make_some_length(length_vec: Vec<PartToken>) -> Option<DivisorClock<u8>> {
    if length_vec.len() > 0 {
        Some(DivisorClock::try_from(length_vec).unwrap())
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_1() {}
}
