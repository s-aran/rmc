use crate::{
    errors::Pass2Error,
    part_command::{PartCommand, PartCommandStruct, PartTokenStack, WrappedPartCommand},
    utils::some_vec,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LocalLoop {
    pub begin_command: String,
    pub body_pre: Vec<WrappedPartCommand>,
    pub separator: Option<String>,
    pub body_post: Vec<WrappedPartCommand>,
    pub end_command: String,
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
        println!("TryFrom LocalLoop: {:?}", value);
        let begin_command = try_from_get_value!(value.pop_and_cast::<String>(0), begin_command);
        let body_pre = value.part_command_stack_mut().pop_vec().unwrap_or_default();
        let end_command = try_from_get_value!(value.pop_and_cast::<String>(5), end_comamnd);
        let body_post = value.part_command_stack_mut().pop_vec().unwrap_or_default();
        let count = try_from_get_some_value!(value.pop_and_cast::<u8>(6), count);

        Ok(LocalLoop {
            begin_command,
            body_pre,
            separator: None,
            body_post,
            end_command,
            count,
        })
    }
}
