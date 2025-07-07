use crate::{
    meta_models::{Code, MetaData, Token, TokenStack, TokenStackTrait, TokenTrait},
    models::{DivisorClock, NegativePositive, NoteCommand},
};

pub(crate) type State = u8;

#[derive(Debug, Clone, Default)]
pub(crate) struct PartToken {
    state: State,
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
}

#[derive(Debug, Clone)]
pub struct Note {
    pub command: String,
    pub natural: Option<bool>,
    pub semitone: Option<NegativePositive>,
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NoteX {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct NoteR {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct PortamentoBegin {
    pub pitch1: Option<NoteCommand>,
    pub pitch2: Option<NoteCommand>,
    pub length1: Option<u8>,
    pub dots: Option<String>,
    pub length2: Option<u8>,
}

#[derive(Debug, Clone)]
pub struct Octave {
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct OctaveUp;

#[derive(Debug, Clone)]
pub struct OctaveDown;

#[derive(Debug, Clone)]
pub struct OctaveReverse;

#[derive(Debug, Clone)]
pub struct PartOctaveChangePositive;

#[derive(Debug, Clone)]
pub struct PartOctaveChangeNegative;

#[derive(Debug, Clone)]
pub struct DefaultLength {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthUpdate {
    pub value_type: Option<DivisorClock>,
    pub value: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthAdd {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthSubtract {
    pub value_type: Option<DivisorClock>,
    pub value: u8,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ProcessLastLengthMultiply {
    pub value: u8,
}

#[derive(Debug, Clone)]
pub struct Tie {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Slur {
    pub length: Option<u8>,
    pub dots: Option<String>,
}

#[derive(Debug, Clone)]
pub enum PartCommand {
    Nop,

    NoteC(Code, Note),
    NoteD(Code, Note),
    NoteE(Code, Note),
    NoteF(Code, Note),
    NoteG(Code, Note),
    NoteA(Code, Note),
    NoteB(Code, Note),
    NoteX(Code, NoteX),
    NoteR(Code, NoteR),

    Portamento(Code, PortamentoBegin),

    Octave(Code, Octave),
    OctaveUp(Code),
    OctaveDown(Code),
    OctaveReverse(Code),
    PartOctaveChangePositive(Code),
    PartOctaveChangeNegative(Code),

    DefaultLength(Code, DefaultLength),

    ProcessLastLengthUpdate(Code, ProcessLastLengthUpdate),
    ProcessLastLengthAdd(Code, ProcessLastLengthAdd),
    ProcessLastLengthSubtract(Code, ProcessLastLengthSubtract),
    ProcessLastLengthMultiply(Code, ProcessLastLengthMultiply),

    Tie(Code, Tie),
    Slur(Code, Slur),
}

pub type WrappedPartCommand = MetaData<PartCommand>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_note_1() {}
}
