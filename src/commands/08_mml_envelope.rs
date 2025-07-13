use crate::{
    errors::Pass2Error,
    part_command::{PartCommand, PartCommandStruct, PartTokenStack, to_some_i8},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SsgPcmSoftwareEnvelope {
    pub command: String,
    pub value1: u8,         // 0..255 || 0..31
    pub value2: i8,         // -15..+15 || 0..31
    pub value3: u8,         // 0..255 || 0..31
    pub value4: u8,         // 0.255 || 0..15
    pub value5: Option<u8>, // None || 0..15
    pub value6: Option<u8>, // None || 0..15
}

impl PartCommandStruct for SsgPcmSoftwareEnvelope {
    fn to_variant(self) -> PartCommand {
        PartCommand::SsgPcmSoftwareEnvelope(self)
    }
}

impl TryFrom<PartTokenStack> for SsgPcmSoftwareEnvelope {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        let command = try_from_get_value!(value.pop_and_cast(0), command);
        let value1 = try_from_get_value!(value.pop_and_cast(1), value1);

        let value2_sign = try_from_get_some_value!(value.pop_and_cast(2), value2_sign);
        let value2_value = try_from_get_value!(value.pop_and_cast(3), value2);
        let value2 = to_some_i8(value2_sign, Some(value2_value)).unwrap();

        let value3 = try_from_get_value!(value.pop_and_cast(4), value3);
        let value4 = try_from_get_value!(value.pop_and_cast(5), value4);
        let value5 = try_from_get_some_value!(value.pop_and_cast(6), value5);
        let value6 = try_from_get_some_value!(value.pop_and_cast(7), value6);

        Ok(Self {
            command,
            value1,
            value2,
            value3,
            value4,
            value5,
            value6,
        })
    }
}
