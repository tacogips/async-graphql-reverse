use lazy_static::lazy_static;
use std::collections::HashSet;

/// ref. https://doc.rust-lang.org/reference/keywords.html
lazy_static! {
    pub static ref RUST_KEYWORDS: HashSet<&'static str> = {
        let mut s = HashSet::new();
        s.insert("as");
        s.insert("break");
        s.insert("const");
        s.insert("continue");
        s.insert("crate");
        s.insert("else");
        s.insert("enum");
        s.insert("extern");
        s.insert("false");
        s.insert("fn");
        s.insert("for");
        s.insert("if");
        s.insert("impl");
        s.insert("in");
        s.insert("let");
        s.insert("loop");
        s.insert("match");
        s.insert("mod");
        s.insert("move");
        s.insert("mut");
        s.insert("pub");
        s.insert("ref");
        s.insert("return");
        s.insert("self");
        s.insert("Self");
        s.insert("static");
        s.insert("struct");
        s.insert("super");
        s.insert("trait");
        s.insert("true");
        s.insert("type");
        s.insert("unsafe");
        s.insert("use");
        s.insert("where");
        s.insert("while");
        s.insert("async");
        s.insert("await");
        s.insert("dyn");
        s.insert("abstract");
        s.insert("become");
        s.insert("box");
        s.insert("do");
        s.insert("final");
        s.insert("macro");
        s.insert("override");
        s.insert("priv");
        s.insert("typeof");
        s.insert("unsized");
        s.insert("virtual");
        s.insert("yield");
        s.insert("try");
        s.insert("union");
        s.insert("dyn");
        s
    };
}
