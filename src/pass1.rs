use crate::{
    errors::Pass1Error,
    meta_models::{
        Code, Command, Pass1Result, Token, TokenStack, TokenStackTrait, TokenTrait, VariantValue,
    },
    models::{Comment1, Comment2, FmToneDefine, Macro, Variable},
    utils::{ParseUtil, is_n, is_sep},
};

pub struct Pass1 {
    code: Code,
    mml: String,
}

impl ParseUtil for Pass1 {
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
                return Command::Comment1(self.get_code().clone());
            }
            '`' => {
                return Command::Comment2(self.get_code().clone());
            }
            '@' => {
                if self.get_code().chars == 0 {
                    return Command::FmToneDefine(self.get_code().clone());
                }
            }
            '#' => {
                if self.get_code().chars == 0 {
                    return Command::Macro(self.get_code().clone());
                }
            }
            '!' => {
                if self.get_code().chars == 0 {
                    return Command::Variable(self.get_code().clone());
                }
            }
            _ => {}
        };

        Command::Nop
    }
}

impl Pass1 {
    pub fn new(code: Code, mml: String) -> Self {
        Self { code, mml }
    }

    pub fn parse(&mut self) -> Result<Pass1Result, Pass1Error> {
        let mut result = Pass1Result::default();

        let mut tokens = TokenStack::new();
        let mut token = Token::new();
        let mut command = Command::Nop;

        for c in self.mml.chars() {
            match command {
                Command::Nop => 'nop: {
                    command = self.parse_command(c);
                    break 'nop;
                }
                Command::Comment1(_) => 'comment1_command: {
                    // println!(
                    //     "{}:{}=>{c} / {:?} / {:?} / {:?}",
                    //     self.code.lines, self.code.chars, command, tokens, token,
                    // );

                    if !is_n(c) {
                        token.eat(c);
                        break 'comment1_command;
                    }

                    tokens.push(&token);
                    token.clear();

                    if let Ok(comment) = self.parse_comment1(&mut tokens) {
                        result.comment1s.push(comment);
                        tokens.clear();
                        token.clear();
                        command = Command::Nop;
                    } else {
                        panic!("; command");
                    }
                }
                Command::Comment2(_) => 'comment2_command: {
                    if c != '`' {
                        token.eat(c);
                        break 'comment2_command;
                    }

                    tokens.push(&token);
                    token.clear();

                    if let Ok(comment) = self.parse_comment2(&mut tokens) {
                        result.comment2s.push(comment);
                        tokens.clear();
                        token.clear();
                        command = Command::Nop;
                    } else {
                        panic!("` command");
                    }
                }
                Command::FmToneDefine(_) => 'fm_tone_command: {
                    if !is_sep(c) {
                        token.eat(c);
                        break 'fm_tone_command;
                    }

                    tokens.push(&token);
                    token.clear();

                    if !is_n(c) {
                        break 'fm_tone_command;
                    }

                    if c == '=' {
                        tokens.push(&token);
                        token.clear();
                        break 'fm_tone_command;
                    }

                    let tone = match self.parse_fm_tone(&mut tokens) {
                        Ok(t) => t,
                        Err(_) => panic!("@ command"),
                    };

                    result.fm_tones.push(tone);
                    tokens.clear();
                    token.clear();
                    command = Command::Nop;
                }
                Command::Macro(_) => 'macro_command: {
                    if !is_sep(c) {
                        token.eat(c);
                        break 'macro_command;
                    }

                    if tokens.len() <= 0 {
                        tokens.push(&token);
                        token.clear();
                        break 'macro_command;
                    }

                    if !is_n(c) {
                        if !token.is_empty() {
                            token.eat(c);
                        }
                        break 'macro_command;
                    }

                    tokens.push(&token);
                    token.clear();

                    let macro_command = match self.parse_macro(&mut tokens) {
                        Ok(t) => t,
                        Err(_) => panic!("# command"),
                    };

                    result.macros.push(macro_command);
                    tokens.clear();
                    token.clear();
                    command = Command::Nop;
                }
                Command::Variable(_) => 'variable_command: {
                    if !is_sep(c) {
                        token.eat(c);
                        break 'variable_command;
                    }

                    if tokens.len() <= 0 {
                        tokens.push(&token);
                        token.clear();
                        break 'variable_command;
                    }

                    if !is_n(c) {
                        if !token.is_empty() {
                            token.eat(c);
                        }
                        break 'variable_command;
                    }

                    tokens.push(&token);
                    token.clear();

                    let variable_command = match self.parse_variable(&mut tokens) {
                        Ok(t) => t,
                        Err(_) => panic!("! command"),
                    };

                    result.variables.push(variable_command);
                    tokens.clear();
                    token.clear();
                    command = Command::Nop;
                }
                _ => {
                    panic!("unknown comand");
                }
            }

            self.code.inc_chars();
            if is_n(c) {
                self.code.inc_lines();
            }
        }

        Ok(result)
    }

    fn parse_comment1(&self, tokens: &mut TokenStack) -> Result<Comment1, Pass1Error> {
        Ok(Comment1 {
            code: self.code.clone(),
            comment: tokens.pop().unwrap_or_default().chars.to_owned(),
        })
    }

    fn parse_comment2(&self, tokens: &mut TokenStack) -> Result<Comment2, Pass1Error> {
        Ok(Comment2 {
            code: self.code.clone(),
            comment: tokens.pop().unwrap_or_default().chars.to_owned(),
        })
    }

    fn parse_fm_tone(&self, tokens: &mut TokenStack) -> Result<FmToneDefine, Pass1Error> {
        // 	@ 記号は必ず行頭に表記し、数値と数値の間には、１つ以上の
        // SPACE、TAB、カンマ、改行のいずれかが必要です。
        // ただし、音色名の区切りはTABと改行のみです。

        let len = tokens.len();
        match len {
            5 | 3 => {
                let name = if len == 5 {
                    let r = &mut tokens.pop().unwrap();
                    // skip "="" token
                    let _ = &mut tokens.pop();

                    Some(r.chars.to_owned())
                } else {
                    None
                };

                let feedback = if let Some(f) = &tokens.pop() {
                    if let Ok(v) = f.chars.parse::<u8>() {
                        v
                    } else {
                        return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
                    }
                } else {
                    return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
                };

                let algorism = if let Some(a) = &tokens.pop() {
                    if let Ok(v) = a.chars.parse::<u8>() {
                        v
                    } else {
                        return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
                    }
                } else {
                    return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
                };

                let tone_number = if let Some(t) = &tokens.pop() {
                    if let Ok(v) = t.chars.parse::<u8>() {
                        v
                    } else {
                        return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
                    }
                } else {
                    return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
                };

                return Ok(FmToneDefine {
                    code: self.code.clone(),
                    tone_number,
                    algorism,
                    feedback,
                    name,
                });
            }
            _ => return Err(Pass1Error::ParseError(self.code.lines, self.code.chars)),
        }
    }

    fn parse_macro(&self, tokens: &mut TokenStack) -> Result<Macro, Pass1Error> {
        let value = if let Some(t) = tokens.pop() {
            VariantValue::String(t.chars.to_owned())
        } else {
            return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
        };

        let key = if let Some(t) = tokens.pop() {
            t.chars.to_owned()
        } else {
            return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
        };

        Ok(Macro {
            code: self.code.clone(),
            key,
            value,
        })
    }

    fn parse_variable(&self, tokens: &mut TokenStack) -> Result<Variable, Pass1Error> {
        let value = if let Some(t) = tokens.pop() {
            t.chars.to_owned()
        } else {
            return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
        };

        let name = if let Some(t) = tokens.pop() {
            t.chars.to_owned()
        } else {
            return Err(Pass1Error::ParseError(self.code.lines, self.code.chars));
        };

        Ok(Variable {
            code: self.code.clone(),
            name,
            value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        // data from SSEG_S.mml
        let mml = r#"#Title		PMD ver4.8s SSG-EG Sample
#Composer	M.Kajihara
#Memo		emulatorではいろいろと厳しいかもしれません

!h	E1,-2,1,0v12P3w0q4
!o	E1,-1,4,0v13P3w0q0
!s	E2,-1,2,0v13P2w8q0

; nm alg fbl
@000 007 000				=	SSG-EG1
; ar  dr  sr  rr  sl  tl  ks  ml  dt ams   seg
 031 019 014 008 014 000 000 008 001 000 ; 014
 031 020 015 008 014 000 000 008 003 000 ; 014
 031 021 016 008 014 000 000 008 005 000 ; 014
 031 022 017 008 014 000 000 008 007 000 ; 014

; nm alg fbl
@001 002 007				=	SSG-EG2
; ar  dr  sr  rr  sl  tl  ks  ml  dt ams   seg
 031 028 010 015 004 020 000 000 000 000 ; 000
 031 029 000 015 015 030 000 010 000 000 ; 013
 024 018 016 015 002 020 000 000 000 000 ; 000
 028 018 013 015 002 000 000 001 000 000 ; 000

; nm alg fbl
@004 002 007				=	SEGDrm
; ar  dr  sr  rr  sl  tl  ks  ml  dt ams   seg
 031 024 000 007 011 000 000 001 000 000 ; 000
 026 026 031 008 015 000 003 015 000 000 ; 014
 023 028 016 007 004 020 000 003 000 000 ; 000
 031 024 013 008 001 000 000 001 000 000 ; 000
 "#;

        let code = Code::default();
        let mut pass1 = Pass1::new(code, mml.to_owned());
        let result = pass1.parse().unwrap();

        assert_eq!(3, result.fm_tones.len());
        assert_eq!(3, result.variables.len());
        assert_eq!(3, result.macros.len());
        assert_eq!(18, result.comment1s.len());
        assert_eq!(0, result.comment2s.len());

        assert_eq!(0, result.fm_tones.get(0).unwrap().tone_number);
        assert_eq!(7, result.fm_tones.get(0).unwrap().algorism);
        assert_eq!(0, result.fm_tones.get(0).unwrap().feedback);
        assert_eq!(
            &"SSG-EG1".to_owned(),
            result.fm_tones.get(0).unwrap().name.as_ref().unwrap()
        );

        assert_eq!(1, result.fm_tones.get(1).unwrap().tone_number);
        assert_eq!(2, result.fm_tones.get(1).unwrap().algorism);
        assert_eq!(7, result.fm_tones.get(1).unwrap().feedback);
        assert_eq!(
            &"SSG-EG2".to_owned(),
            result.fm_tones.get(1).unwrap().name.as_ref().unwrap()
        );

        assert_eq!(4, result.fm_tones.get(2).unwrap().tone_number);
        assert_eq!(2, result.fm_tones.get(2).unwrap().algorism);
        assert_eq!(7, result.fm_tones.get(2).unwrap().feedback);
        assert_eq!(
            &"SEGDrm".to_owned(),
            result.fm_tones.get(2).unwrap().name.as_ref().unwrap()
        );

        assert_eq!("h", result.variables.get(0).unwrap().name);
        assert_eq!("E1,-2,1,0v12P3w0q4", result.variables.get(0).unwrap().value);
        assert_eq!("o", result.variables.get(1).unwrap().name);
        assert_eq!("E1,-1,4,0v13P3w0q0", result.variables.get(1).unwrap().value);
        assert_eq!("s", result.variables.get(2).unwrap().name);
        assert_eq!("E2,-1,2,0v13P2w8q0", result.variables.get(2).unwrap().value);

        assert_eq!("Title", result.macros.get(0).unwrap().key);
        if let VariantValue::String(title) = &result.macros.get(0).unwrap().value {
            assert_eq!("PMD ver4.8s SSG-EG Sample", title);
        } else {
            assert!(false);
        }

        assert_eq!("Composer", result.macros.get(1).unwrap().key);
        if let VariantValue::String(composer) = &result.macros.get(1).unwrap().value {
            assert_eq!("M.Kajihara", composer);
        } else {
            assert!(false);
        }

        assert_eq!("Memo", result.macros.get(2).unwrap().key);
        if let VariantValue::String(arranger) = &result.macros.get(2).unwrap().value {
            assert_eq!("emulatorではいろいろと厳しいかもしれません", arranger);
        } else {
            assert!(false);
        }

        assert_eq!(" nm alg fbl", result.comment1s.get(0).unwrap().comment);
        assert_eq!(
            " ar  dr  sr  rr  sl  tl  ks  ml  dt ams   seg",
            result.comment1s.get(1).unwrap().comment
        );
        assert_eq!(" 014", result.comment1s.get(2).unwrap().comment);
        assert_eq!(" 014", result.comment1s.get(3).unwrap().comment);
        assert_eq!(" 014", result.comment1s.get(4).unwrap().comment);
        assert_eq!(" 014", result.comment1s.get(5).unwrap().comment);

        assert_eq!(" nm alg fbl", result.comment1s.get(6).unwrap().comment);
        assert_eq!(
            " ar  dr  sr  rr  sl  tl  ks  ml  dt ams   seg",
            result.comment1s.get(7).unwrap().comment
        );
        assert_eq!(" 000", result.comment1s.get(8).unwrap().comment);
        assert_eq!(" 013", result.comment1s.get(9).unwrap().comment);
        assert_eq!(" 000", result.comment1s.get(10).unwrap().comment);
        assert_eq!(" 000", result.comment1s.get(11).unwrap().comment);

        assert_eq!(" nm alg fbl", result.comment1s.get(12).unwrap().comment);
        assert_eq!(
            " ar  dr  sr  rr  sl  tl  ks  ml  dt ams   seg",
            result.comment1s.get(13).unwrap().comment
        );
        assert_eq!(" 000", result.comment1s.get(14).unwrap().comment);
        assert_eq!(" 014", result.comment1s.get(15).unwrap().comment);
        assert_eq!(" 000", result.comment1s.get(16).unwrap().comment);
        assert_eq!(" 000", result.comment1s.get(17).unwrap().comment);
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
        let result = pass1.parse().unwrap();

        assert_eq!(0, result.fm_tones.len());
        assert_eq!(4, result.variables.len());
        assert_eq!(0, result.macros.len());
        assert_eq!(3, result.comment1s.len());
        assert_eq!(0, result.comment2s.len());
    }
}
