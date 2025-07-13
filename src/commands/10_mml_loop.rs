use crate::{
    errors::Pass2Error,
    part_command::{PartCommand, PartCommandStruct, PartTokenStack},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoopBegin {
    pub command: String,
}

impl PartCommandStruct for LocalLoopBegin {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoopBegin(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoopBegin {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);

        Ok(Self { command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoopFinalBreak {
    pub command: String,
}

impl PartCommandStruct for LocalLoopFinalBreak {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoopFinalBreak(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoopFinalBreak {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);

        Ok(Self { command })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoopEnd {
    pub command: String,
    pub count: Option<u8>,
}

impl PartCommandStruct for LocalLoopEnd {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoopEnd(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoopEnd {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let count = try_from_get_some_value!(value.pop_and_cast(1), count);

        Ok(Self { command, count })
    }
}
