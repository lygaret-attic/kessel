pub type Parser<T> = proc (State) -> Status<T>;

pub struct State {
    file: ~str, text: ~[char], index: uint, line: uint
}

pub enum Status<T> {
    Success { next: State, value: T },
    Failure { next: State, msg: ~str }
}