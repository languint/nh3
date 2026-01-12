use chumsky::prelude::*;

pub fn parser<'src>(src: &'src str) -> impl Parser<'src, &'src str, ()> {
    end()
}
