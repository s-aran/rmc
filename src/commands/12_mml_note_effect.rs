use crate::part_command::{PartCommandParseState, WrappedPartCommand, to_some_i8};
use crate::{
    errors::Pass2Error,
    models::DivisorClock,
    part_command::{PartCommand, PartCommandStruct, PartTokenStack, count_dots, make_some_length},
};

// ===============================================================================
// §12-4	分散和音指定
// 	{{ }}
// -------------------------------------------------------------------------------
// [書式]	{{ 音程1[音程2..音程16] }} [音長1][.][,音長2[,数値1[,数値2[,±数値3]]]]
// -------------------------------------------------------------------------------
// [範囲]	[音長1]	1～255のうち、全音符長の約数となる値(->§2-11)
// 		または % をつけたclock値
// 	[音長2]	上に加え、音長1よりも短い値
// 	[数値1] 0～1
// 	[数値2] 0～255のうち、音長1よりも短いclock値
// 	[数値3] -128～+127
// -------------------------------------------------------------------------------
// [音源]	FM / SSG / PCM(AD,PPZ)
// -------------------------------------------------------------------------------
// 	{{ }} 内に列記した音程を使い、音長1の長さでの分散和音を
// 	コンパイラ上で展開します。
//
// 	音長1 = 全体の長さ
// 	音長2 = 音符一つの長さ (デフォルトは %1)
// 	数値1 = タイフラグ (0=off, 1=on(デフォルト))
// 	数値2 = ゲートタイム (デフォルトは 0)
// 	数値3 = 一周した際の音量調整 (Vコマンドレベル、デフォルトは ±0)
//
// 	音長・符点については、c d e f g a b rコマンドと同等です。
// 	255stepを越える音長は指定出来ません。
// 	{{ }}の中には、c d e f g a b o > < コマンドのみ指定して下さい。
//
// 	タイフラグは、一つ一つの音符の後にタイ(&)を付けるか否かです。
// 	ゲートタイムは、指定したclock値分、最後の部分を休符とします。
// 	音量調整は、音程が一周した時に変化させる音量です。
//
// 	タイをoffにして音長2をそれなりの長さにすれば、
// 	アルペジオ演奏のような事も可能です。
// 	ただし、音長1[-数値2] の長さで音が切れることにご注意ください。
//
// [注意1]	音量調整を行った場合、変化した音量はこのコマンド後もそのままです。
// 	次の音を発音する際、必要に応じて音量を再定義する必要があります。
//
// [注意2]	このコマンドには、W S コマンドの効果は反映されません。
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Alpeggio {
    pub command_begin: String,
    pub notes: Vec<WrappedPartCommand>,
    pub command_end: String,
    pub length1: Option<DivisorClock<u8>>,
    pub dots: u8,
    pub length2: Option<DivisorClock<u8>>,
    pub value1: bool,
    pub value2: Option<u8>,
    pub value3: Option<i8>,
}

impl TryFrom<PartTokenStack> for Alpeggio {
    type Error = Pass2Error;

    fn try_from(mut value: PartTokenStack) -> Result<Self, Self::Error> {
        println!("{:?}", value);

        let command_begin = try_from_get_value!(value.pop_and_cast(1), command);
        let notes = value.part_command_stack_mut().pop_vec().unwrap_or_default();
        let command_end = try_from_get_value!(value.pop_and_cast(3), command);

        let length1 = make_some_length(value.pop_by_state_all(4));
        let dots = count_dots(try_from_get_some_value!(value.pop_and_cast(5), dots));
        let length2 = make_some_length(value.pop_by_state_all(6));
        let value1 = if let Some(v) = try_from_get_some_value!(value.pop_and_cast::<u8>(7), value1)
        {
            v == 1
        } else {
            true
        };
        let value2 = try_from_get_some_value!(value.pop_and_cast::<u8>(8), value2);
        let value3_sign = try_from_get_some_value!(value.pop_and_cast(9), value3_sign);
        let value3 = to_some_i8(
            value3_sign,
            try_from_get_some_value!(value.pop_and_cast(10), value3),
        );

        Ok(Alpeggio {
            command_begin,
            notes,
            command_end,
            length1,
            dots,
            length2,
            value1,
            value2,
            value3,
        })
    }
}

impl PartCommandStruct for Alpeggio {
    fn to_variant(self) -> PartCommand {
        PartCommand::Alpeggio(self)
    }

    fn is_block() -> bool {
        true
    }

    fn is_match(command: &str) -> bool {
        command.len() == 1 && "cdefgab".contains(command)
    }

    fn parse(working: &mut crate::meta_models::Pass2Working, c: char) -> PartCommandParseState {
        match c {
            '%' => {
                if !vec![4, 6].contains(&working.state) {
                    panic!("Alpeggio: unexpected {c}");
                }

                working.eat(c);
                working.jump(3);
            }
            '0'..='9' => {
                // length, required
                if vec![3, 5, 6, 7, 9].contains(&working.state) {
                    working.next();
                }

                if !vec![4, 6, 7, 8, 10].contains(&working.state) {
                    panic!("Alpeggio: unexpected {c}");
                }

                working.eat(c);
            }
            '.' => {
                // dots, optional
                if working.state > 5 {
                    panic!("Alpeggio: unexpected {c}");
                }

                if working.state == 4 {
                    working.push();
                }

                working.eat(c);
                working.jump(5);
            }
            '+' | '-' => {
                // semitone, optional
                if working.state != 9 {
                    panic!("Alpeggio: unexptected {c}");
                }

                working.eat(c);
                working.jump(10);
                working.push();
            }
            ',' => {
                working.push();
                working.next();
            }
            _ => {
                // other command
                working.push();

                return PartCommandParseState::Parsed;
            }
        };

        PartCommandParseState::Parsing
    }
}
