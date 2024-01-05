macro_rules! lexer {
    (
        $lexer_name:ident -> $token_name:ident {
            $tokenkind_name:ident { $($name:ident{ $($argv:ident, )* },)* },
            $(rule $regex:tt => $body:stmt, )+
        }
    ) => {

    };
}

lexer!{
    Lexer -> Token {
        TokenKind {
            Name{ String, },
            NewLine{},

        },
        rule r#"[a-zA-Z]+"# => { TokenKind::Name(text.to_string()) },
    }
}