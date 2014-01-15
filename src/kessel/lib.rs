#[crate_id   = "kessel"];
#[comment    = "Parsec inspired parser combinator library"];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(struct_variant)];

mod types;
mod predicates;
mod parsers {
    mod base;
    mod characters;
}