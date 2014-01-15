use std::fmt;
use types::*;

pub trait CharParsers
{
    fn any(&self) -> Parser<char>;
    fn none(&self) -> Parser<char>;
}

impl<'a> CharParsers for &'a str {

    fn any(&self) -> Parser<char> {
        let this = self.to_owned();

        proc(input: State) {
            let i = input.index;
            let c = input.text[i];

            if this.find(c).is_some() {
                let next = State { index: i + 1, ..input };
                Success { next: next, value: c }
            }

            else {
                let msg = format_args!(fmt::format, "Didn't match any of '{}'!", this);
                Failure { next: input, msg: msg }
            }
        }
    }

    fn none(&self) -> Parser<char> {
        let this = self.to_owned();

        proc(input: State) {
            let i = input.index;
            let c = input.text[i];

            if this.find(c).is_some() {
                let msg = format_args!(fmt::format, "Shouldn't have matched any of '{}'!", this);
                Failure { next: input, msg: msg }
            }

            else {
                let next = State { index: i + 1, ..input };
                Success { next: next, value: c }
            }
        }
    }
}