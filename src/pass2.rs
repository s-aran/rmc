use std::str::FromStr;

use crate::{
    commands::{
        commands_envelope::SsgPcmSoftwareEnvelope,
        commands_loop::{LocalLoopBegin, LocalLoopEnd, LocalLoopFinalBreak},
        commands_mml::{
            MasterTranspose, Note, Octave, PartTransposeBegin, PartTransposeEnd, Quantize1,
            Quantize2, TemporaryTranspose,
        },
        commands_volume::Volume,
    },
    errors::Pass2Error,
    meta_models::{Code, Command, Pass1Result, Pass2Result, Pass2Working, TokenTrait},
    models::PartSymbol,
    part_command::{PartCommand, PartCommandStruct, PartTokenStack, WrappedPartCommand},
    utils::{ParseUtil, get_type_name, is_n, is_sep},
};

#[derive(Debug, Clone)]
pub struct Pass2 {
    code: Code,
    mml: String,
    pass1: Pass1Result,
}

impl ParseUtil for Pass2 {
    fn get_mml(&self) -> &String {
        &self.mml
    }

    fn get_code(&self) -> &Code {
        &self.code
    }

    fn parse_command(&self, c: char) -> Command {
        if is_sep(c) || is_n(c) {
            return Command::Nop;
        }

        match c {
            ';' => {
                return Command::Comment1(self.clone_code());
            }
            '`' => {
                return Command::Comment2(self.clone_code());
            }
            '@' => {
                if self.get_code().chars == 0 {
                    return Command::FmToneDefine(self.clone_code());
                }
            }
            '#' => {
                if self.get_code().chars == 0 {
                    return Command::Macro(self.clone_code());
                }
            }
            '!' => {
                if self.get_code().chars == 0 {
                    return Command::Variable(self.clone_code());
                }
            }
            'A'..'Z' => {
                if self.get_code().chars == 0 {
                    return Command::Part(
                        self.clone_code(),
                        PartSymbol::from_str(&c.to_string().as_str()).unwrap(),
                    );
                }
            }
            'a'..'z' => {
                if self.get_code().chars == 0 {
                    return Command::Part(
                        self.clone_code(),
                        PartSymbol::from_str(&c.to_string().as_str()).unwrap(),
                    );
                }
            }
            _ => {
                // eprintln!("unsupported command: {}", c);
            }
        }

        return Command::Nop;
    }
}

impl Pass2 {
    pub fn new(code: Code, mml: String, pass1_result: Pass1Result) -> Self {
        Self {
            code,
            mml,
            pass1: pass1_result,
        }
    }

    pub fn parse(&mut self) -> Result<Pass2Result, Pass2Error> {
        let mut result = Pass2Result::default();

        let mut working = Pass2Working::default();

        let mut command = Command::Nop;

        let new_lined_mml = format!("{}\n", self.mml);
        let mut chars = new_lined_mml.chars();
        let mut maybe_c = chars.next();
        while maybe_c.is_some() {
            let c = maybe_c.unwrap();

            match command {
                Command::Nop => 'nop: {
                    command = self.parse_command(c);
                    break 'nop;
                }
                Command::Comment1(_) => 'comment1_command: {
                    if !is_n(c) {
                        break 'comment1_command;
                    }

                    command = Command::Nop;
                }
                Command::Comment2(_) => 'comment2_command: {
                    if c != '`' {
                        break 'comment2_command;
                    }

                    command = Command::Nop;
                }
                Command::Part(_, ref part) => 'part_command: {
                    // let w = WrappedPartCommand::new(self.get_code(), part_command.clone());
                    // commands.push(w);
                    working.code = self.code.clone();
                    let res = self.parse_part_command(&mut working, c);
                    if is_n(c) {
                        if !working.token.is_empty() {
                            working.push();
                        }

                        working.clear();

                        result.parts.push((part.clone(), working.commands.clone()));
                    }

                    if let Ok(r) = res {
                        // match r {
                        //     PartCommand::Nop => {
                        //         break 'part_command;
                        //     }
                        //     _ => {
                        //         working.clear();
                        //         commands.push(WrappedPartCommand::new(self.get_code(), r));
                        //         if is_n(c) {
                        //             result.parts.push((part.clone(), commands.clone()));
                        //             commands.clear();
                        //         }

                        //         command = Command::Nop;
                        //     }
                        // };
                    }
                }
                _ => {
                    // nop
                }
            }

            self.code.inc_chars();
            maybe_c = chars.next();
            if is_n(c) {
                self.code.inc_lines();
                command = Command::Nop;
            }
        }

        Ok(result)
    }

    fn parse_part_command(
        &self,
        working: &mut Pass2Working,
        c: char,
    ) -> Result<PartCommand, Pass2Error> {
        // println!(
        //     "begin: {c}: {:?} {:?} / {}",
        //     working.token,
        //     working.tokens,
        //     working.tokens.first().is_some()
        // );

        if working.tokens.first().is_none() {
            if working.token.is_empty() && is_sep(c) {
                return Ok(PartCommand::Nop);
            }

            if working.token.is_empty() {
                working.eat(c);
            }

            let t = working.token.chars().as_str();
            match t {
                "c" | "d" | "e" | "f" | "g" | "a" | "b" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "o" => {
                    if working.state <= 0 {
                        working.jump(1);
                        return Ok(PartCommand::Nop);
                    }

                    match c {
                        '+' | '-' => {
                            working.eat(c);
                            working.push();
                            return Ok(PartCommand::Nop);
                        }
                        _ => {
                            working.push();
                            // fall
                        }
                    }
                }
                "_" => {
                    if working.state <= 0 {
                        working.jump(1);
                        return Ok(PartCommand::Nop);
                    }

                    match c {
                        '_' | '{' | 'M' => {
                            working.eat(c);
                            working.push();
                            return Ok(PartCommand::Nop);
                        }
                        _ => {
                            working.push();
                            // fall
                        }
                    }
                }
                "}" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "[" => {
                    working.loop_nest += 1;
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "]" => {
                    working.loop_nest -= 1;
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                ":" => {
                    // special char in loop
                    if working.loop_nest > 0 {
                        working.eat(c);
                        working.push();
                        working.jump(1);
                        // TODO: define pseudo command
                        return Ok(PartCommand::Nop);
                    }

                    panic!("unexpected : command")
                }
                "E" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "v" => {
                    if working.state <= 0 {
                        working.jump(1);
                        return Ok(PartCommand::Nop);
                    }

                    match c {
                        '+' | '-' | ')' | '(' => {
                            working.eat(c);
                            working.push();
                            return Ok(PartCommand::Nop);
                        }
                        _ => {
                            working.push();
                            // fall
                        }
                    }
                }
                "V" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "q" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                "Q" => {
                    working.push();
                    working.jump(1);
                    return Ok(PartCommand::Nop);
                }
                _ => {
                    panic!("unknwon command: {c}");
                }
            }
        }

        match working.tokens.first().unwrap().chars().as_str() {
            "c" | "d" | "e" | "f" | "g" | "a" | "b" => {
                match c {
                    '=' => {
                        // natural, optional
                        if working.state > 1 {
                            panic!("Note: unexpected {c}");
                        }

                        working.eat(c);
                        working.jump(1);
                        working.push();
                    }
                    '+' | '-' => {
                        // semitone, optional
                        if working.state > 2 {
                            panic!("Note: unexptected {c}");
                        }

                        working.eat(c);
                        working.jump(2);
                        working.push();
                    }
                    '0'..'9' => {
                        // length, optional
                        if working.state > 4 {
                            panic!("Note: unexpected {c}");
                        }

                        working.eat(c);
                        working.jump(3);
                    }
                    '.' => {
                        // dots, optional
                        if working.state > 5 {
                            panic!("Note: unexpected {c}");
                        }

                        if working.state == 3 {
                            working.push();
                        }

                        working.eat(c);
                        working.jump(4);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<Note>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                };
            }
            "o" | "o+" | "o-" => {
                match c {
                    '0'..'9' => {
                        // value
                        if working.state > 2 {
                            panic!("Octave: unexpected {c}");
                        }

                        working.jump(2);

                        working.eat(c);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<Octave>(working);

                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "_" | "__" => {
                match c {
                    '+' | '-' => {
                        // semitone, optional
                        if working.state > 1 {
                            panic!("Absolute Transpose: unexpected {c}");
                        }

                        working.jump(2);

                        working.eat(c);
                        working.push();
                    }
                    '0'..'9' => {
                        // value
                        if working.state > 3 {
                            panic!("Absolute Transpose: unexpected {c}");
                        }

                        working.jump(3);

                        working.eat(c);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<TemporaryTranspose>(working);

                        working.clear();

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "_{" => {
                match c {
                    '+' | '-' | '=' => {
                        // semitone|natural, optional
                        if working.state > 2 {
                            panic!("Part Transpose: unexpected {c}");
                        }

                        working.jump(2);

                        working.eat(c);
                        working.push();
                    }
                    'c' | 'd' | 'e' | 'f' | 'g' | 'a' | 'b' => {
                        if working.state > 3 {
                            panic!("Part Transpose: unexpected {c}");
                        }

                        working.jump(3);
                        working.eat(c);
                        working.push();
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<PartTransposeBegin>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "}" => {
                working.push();

                Self::push_part_command::<PartTransposeEnd>(working);

                // retry
                return self.parse_part_command(working, c);
            }
            "_M" => {
                match c {
                    '+' | '-' => {
                        // semitone, optional
                        if working.state > 1 {
                            panic!("Master Transpose: unexpected {c}");
                        }

                        working.jump(2);

                        working.eat(c);
                        working.push();
                    }
                    '0'..'9' => {
                        // value
                        if working.state > 3 {
                            panic!("Master Transpose: unexpected {c}");
                        }

                        working.eat(c);
                        working.jump(3);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<MasterTranspose>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "[" => {
                Self::push_part_command::<LocalLoopBegin>(working);

                // retry
                return self.parse_part_command(working, c);
            }
            ":" => {
                working.push();

                Self::push_part_command::<LocalLoopFinalBreak>(working);

                // retry
                return self.parse_part_command(working, c);
            }
            "]" => {
                match c {
                    '0'..'9' => {
                        // loop count
                        if working.state > 1 {
                            panic!("Loop: unexpected {c}");
                        }

                        working.jump(1);
                        working.eat(c);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<LocalLoopEnd>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "E" => {
                match c {
                    '0'..'9' => {
                        if working.state == 2 {
                            // sign is not specified
                            working.next();
                        }

                        working.eat(c);
                    }
                    ',' => {
                        working.push();
                        working.next();
                    }
                    '+' | '-' => {
                        working.eat(c);
                        working.push();
                        working.next();
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<SsgPcmSoftwareEnvelope>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "v" | "V" | "v+" | "v-" | "v)" | "v(" => {
                match c {
                    '0'..'9' => {
                        working.eat(c);
                        working.jump(2);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<Volume>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "Q" => {
                match c {
                    '%' => {
                        working.eat(c);
                        working.push();
                        working.jump(2);
                    }
                    '0'..'9' => {
                        working.eat(c);
                        working.jump(3);
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<Quantize1>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            "q" => {
                match c {
                    '-' => {
                        if !(working.state == 1 || working.state == 3) {
                            panic!("q: unexpected -");
                        }

                        working.eat(c);
                        working.push();
                        working.next();
                    }
                    'l' => {
                        if !(working.state == 0 || working.state == 4 || working.state == 7) {
                            panic!("q: unexpected l");
                        }

                        working.eat(c);
                        working.push();
                        working.next();
                    }
                    '0'..'9' => {
                        working.eat(c);
                    }
                    '.' => {
                        // dots, optional
                        if working.state == 2 || working.state == 8 {
                            working.push();
                        }

                        working.eat(c);
                        match working.state {
                            2 => working.jump(4),
                            8 => working.jump(10),
                            _ => panic!("q: unexpected dot"),
                        };
                    }
                    ',' => {
                        working.push();
                        working.next();
                    }
                    _ => {
                        // other command
                        working.push();

                        Self::push_part_command::<Quantize2>(working);

                        // retry
                        return self.parse_part_command(working, c);
                    }
                }
            }
            _ => {
                //
            }
        }

        Ok(PartCommand::Nop)
    }

    fn push_part_command<T>(working: &mut Pass2Working)
    where
        T: TryFrom<PartTokenStack, Error = Pass2Error> + PartCommandStruct,
    {
        let command = match T::try_from(working.tokens.clone()) {
            Ok(v) => v,
            Err(e) => panic!("{}: {} // {:?}", get_type_name::<T>(), e, working.tokens),
        };

        println!("==> {}: {:?}", get_type_name::<T>(), command);

        let w = WrappedPartCommand::new(
            working.tokens.first().unwrap().get_code(),
            command.to_variant(),
        );

        working.commands.push(w);

        working.clear();
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        models::{DivisorClock, NegativePositive, NegativePositiveEqual, NoteCommand, PartSymbol},
        pass1::Pass1,
    };

    use super::*;

    #[test]
    fn test_1() {
        let mml = r#"G	c+4d-12e8f.g=a..b4...._-2[e__+1]8_0_{-eab}_M+120"#;

        let code = Code::default();
        let mut pass1 = Pass1::new(code, mml.to_owned());
        let pass1_result = pass1.parse().unwrap();

        // moved
        let code = Code::default();
        let mut pass2 = Pass2::new(code, mml.to_owned(), pass1_result);
        let result: Pass2Result = pass2.parse().unwrap();

        let part_g_list = result.get_parts(&PartSymbol::G);
        assert_eq!(1, part_g_list.len());

        let g_commands = part_g_list.get(0).unwrap();
        assert_eq!(16, g_commands.len());

        // c+4
        let expected = Note {
            command: "c".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Positive),
            length: Some(DivisorClock::Divisor(4)),
            dots: 0,
        };
        let actual = g_commands.get(0).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // d-12
        let expected = Note {
            command: "d".to_string(),
            natural: false,
            semitone: Some(NegativePositive::Negative),
            length: Some(DivisorClock::Divisor(12)),
            dots: 0,
        };
        let actual = g_commands.get(1).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // e8
        let expected = Note {
            command: "e".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Clock(8)),
            dots: 0,
        };
        let actual = g_commands.get(2).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // f.
        let expected = Note {
            command: "f".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 1,
        };
        let actual = g_commands.get(3).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // g=
        let expected = Note {
            command: "g".to_string(),
            natural: true,
            semitone: None,
            length: None,
            dots: 0,
        };
        let actual = g_commands.get(4).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // a..
        let expected = Note {
            command: "a".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 2,
        };
        let actual = g_commands.get(5).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // b4....
        let expected = Note {
            command: "b".to_string(),
            natural: false,
            semitone: None,
            length: Some(DivisorClock::Clock(4)),
            dots: 4,
        };
        let actual = g_commands.get(6).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _-2
        let expected = TemporaryTranspose {
            command: "_".to_string(),
            semitone: Some(NegativePositive::Negative),
            value: 2,
        };
        let actual = g_commands.get(7).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::AbsoluteTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // [
        let expected = LocalLoopBegin {
            command: "[".to_string(),
        };
        let actual = g_commands.get(8).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::LocalLoopBegin(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // e
        let expected = Note {
            command: "e".to_string(),
            natural: false,
            semitone: None,
            length: None,
            dots: 0,
        };
        let actual = g_commands.get(9).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::Note(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // __+1
        let expected = TemporaryTranspose {
            command: "__".to_string(),
            semitone: Some(NegativePositive::Positive),
            value: 1,
        };
        let actual = g_commands.get(10).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::RelativeTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // ]8
        let expected = LocalLoopEnd {
            command: "]".to_string(),
            count: Some(8),
        };
        let actual = g_commands.get(11).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::LocalLoopEnd(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _0
        let expected = TemporaryTranspose {
            command: "_".to_string(),
            semitone: None,
            value: 0,
        };
        let actual = g_commands.get(12).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::AbsoluteTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _{-eab
        let expected = PartTransposeBegin {
            command: "_{".to_string(),
            sign: Some(NegativePositiveEqual::Negative),
            notes: vec![NoteCommand::e, NoteCommand::a, NoteCommand::b],
        };
        let actual = g_commands.get(13).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::PartTransposeBegin(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // }
        let expected = PartTransposeEnd {
            command: "}".to_string(),
        };
        let actual = g_commands.get(14).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::PartTransposeEnd(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );

        // _M+120
        let expected = MasterTranspose {
            command: "_M".to_string(),
            sign: Some(NegativePositive::Positive),
            value: 120,
        };
        let actual = g_commands.get(15).unwrap();
        assert_eq!(
            expected,
            if let PartCommand::MasterTranspose(ref c) = *actual.data() {
                c.clone()
            } else {
                panic!("unexpected command: {:?}", actual)
            }
        );
    }

    #[test]
    fn test_2() {
        let mml = r#";{{ 音階1 [音階2 …音階16まで] }} [音長] [ ,1音の長さ(def=%1) [ ,タイon(1,def)/off(0)
;				         [ ,gate(def=0) [ ,1loopで変化する音量±(def=0) ]]]]
;音量は変化させたら戻らない手抜き仕様です >_<

G	E1,-2,24,0 v15 q0
G	o4l8[{{eg>c<}}2. {{eg>c<}},,,2 {{fa>c<}}%108
G	o4l8 {{fa->c<}}%60 {{fa->c<}}r {{eg>c<}}%108]2
G	o5l8 {{eg>c<}}2. {{eg>c<}},,,2 {{fa>c<}}%108
G	o5l8 {{fa->c<}}%60 {{fa->c<}}r {{eg>c<}}%108
G	o5l8 {{eg>c<}}2. {{eg>c<}},,,2 {{fa>c<}}%108
G	o5l8[{{fb>d<}},,,2]3rr {{fb>d<}}r {{eg>c<}} r1
H	E2,-2,6,0 v14o2l8 [c>Q6cQ8<]56 <gggrrgr>c r1
!b	E1,-4,1,0v15q99,3P1o3c16q0
!h	E1,-3,1,0v15q99,4P2w0c16q0
!s	E1,-2,1,0v15P1o3a%1P2w10v13c16-%1
!c4	E0,-0,2,0v15P1o3e%1P2w04c4-%1
I	l16MP-128 [!b[!h]3!s[!h]3]15 !b!h!b!b!s!h!b!b
I	!c4!s[!h]3[!b[!h]3!s[!h]3]11 !sr!sr!sr !b!b!br !brr8 !br r1
"#;

        let code = Code::default();
        let mut pass1 = Pass1::new(code, mml.to_owned());
        let pass1_result = pass1.parse().unwrap();

        // moved
        let code = Code::default();
        let mut pass2 = Pass2::new(code, mml.to_owned(), pass1_result);
        let result: Pass2Result = pass2.parse().unwrap();

        assert_eq!(7, result.get_parts(&PartSymbol::G).len());
    }
}
