use crate::{
    errors::Pass2Error,
    part_command::{PartCommand, PartCommandStruct, PartTokenStack},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoop {
    pub begin_command: String,
    pub body_pre: Vec<PartCommand>,
    pub separator: String,
    pub body_post: Vec<PartCommand>,
    pub break_commands: Vec<PartCommand>,
    pub count: Option<u8>,
}

impl PartCommandStruct for LocalLoop {
    fn to_variant(self) -> PartCommand {
        PartCommand::LocalLoop(self)
    }
}

impl TryFrom<PartTokenStack> for LocalLoop {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let begin = try_from_get_value!(value.pop_and_cast::<String>(0), begin);
        let count = try_from_get_some_value!(value.pop_and_cast::<u8>(1), count);

        Ok(LocalLoop {
            begin_command: begin,
            body_pre: Vec::new(),
            separator: "".to_string(),
            body_post: Vec::new(),
            break_commands: Vec::new(),
            count,
        })
    }
}
