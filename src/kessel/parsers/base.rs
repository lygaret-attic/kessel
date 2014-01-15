use types::*;

static EOT: char = '\u0003';

pub fn eot() -> Parser<()> {
    proc (input: State) {
        if input.text[input.index] == EOT {
            let next = State { index: input.index + 1, ..input };
            Success { next: next, value: () }
        }
        else {
            Failure { next: input, msg: ~"EOT" }
        }
    }
}

pub fn anycp(pred: proc(char) -> bool) -> Parser<char> {
    proc (input: State) {
        let i = input.index;
        let c = input.text[i];

        // if it passes the pred, consume the input
        if c != EOT && pred(c) {
            let next = State { index: i + 1, ..input };
            Success { next: next, value: c }
        }

        // otherwise, fail
        else {
            Failure { next: input, msg : ~"" }
        }
    }
}
