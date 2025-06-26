use crate::{
    errors::Pass1Error,
    meta_models::{Code, Command, Pass1Result, Token, TokenStack, VariantValue},
    models::{FmToneDefine, Macro, Variable},
    utils::{is_n, is_sep, split},
};

pub struct Pass1 {
    code: Code,
    mml: String,
}

impl Pass1 {
    pub fn new(code: Code, mml: String) -> Self {
        Self { code, mml }
    }

    pub fn pass1(&mut self) -> Result<Pass1Result, Pass1Error> {
        let mut result = Pass1Result::default();

        let mut skipping = false;
        let mut tokens = TokenStack::new();
        let mut token = Token::new();
        let mut command = Command::Nop;

        for c in self.mml.chars() {
            if skipping {
                if c == '`' {
                    skipping = false;
                }

                self.code.next_char();

                // skip
                continue;
            }

            match &command {
                Command::Nop => 'nop: {
                    command = self.parse_command(c);
                    break 'nop;
                }
                Command::Comment1(_) | Command::Comment2(_) => 'comment_command: {
                    break 'comment_command;
                }
                Command::FmToneDefine(code) => 'fm_tone_command: {
                    if !is_sep(c) {
                        token.eat(c);
                        break 'fm_tone_command;
                    }

                    tokens.push(&token);
                    token.clear();

                    if !is_n(c) {
                        break 'fm_tone_command;
                    }

                    let tone = match self.parse_fm_tone(&mut tokens) {
                        Ok(t) => t,
                        Err(e) => panic!("@ command"),
                    };

                    result.fm_tones.push(tone);

                    tokens.clear();
                    token.clear();
                }
                Command::Macro(code) => 'macro_command: {
                    println!(
                        "{}:{}=>{c} / {:?} / {:?} / {:?} (skip={})",
                        self.code.lines, self.code.chars, command, tokens, token, skipping,
                    );

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
                        Err(e) => panic!("# command"),
                    };

                    result.macros.push(macro_command);

                    tokens.clear();
                    token.clear();
                }
                Command::Variable(code) => 'variable_command: {
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
                        Err(e) => panic!("! command"),
                    };

                    result.variables.push(variable_command);

                    tokens.clear();
                    token.clear();
                }
                _ => {}
            }

            self.code.next_char();

            if is_n(c) {
                command = Command::Nop;
                self.code.next_line();
                continue;
            }
        }

        Ok(result)
    }

    fn parse_command(&self, c: char) -> Command {
        match c {
            ';' | ' ' | '\t' => {
                if self.code.chars == 0 {
                    return Command::Comment1(self.code.clone());
                }
            }
            '`' => {
                return Command::Comment2(self.code.clone());
            }
            '@' => {
                if self.code.chars == 0 {
                    return Command::FmToneDefine(self.code.clone());
                }
            }
            '#' => {
                if self.code.chars == 0 {
                    return Command::Macro(self.code.clone());
                }
            }
            '!' => {
                return Command::Variable(self.code.clone());
            }
            _ => {}
        };

        Command::Nop
    }

    fn parse_fm_tone(&self, tokens: &mut TokenStack) -> Result<FmToneDefine, Pass1Error> {
        // 	@ 記号は必ず行頭に表記し、数値と数値の間には、１つ以上の
        // SPACE、TAB、カンマ、改行のいずれかが必要です。
        // ただし、音色名の区切りはTABと改行のみです。

        let len = tokens.len();
        match len {
            4 | 3 => {
                let name = if len == 4 {
                    let r = &mut tokens.pop().unwrap();
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
