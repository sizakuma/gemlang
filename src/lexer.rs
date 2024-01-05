use glg::lexer;

lexer!{
    pub kind TokenKind {
        Name { String },
        Newline,
        Whitespace
    },
    pub token Token {
        kind: TokenKind,
        pos: TokenPos,
    },
    pub lexer Lexer -> Token {
        rule(n = r#"[a-zA-Z_][a-zA-Z_0-9]*"#) => { TokenKind::Name(n.to_string) }

        each(kind, pos) -> Token {
            kind
        }
    }
}