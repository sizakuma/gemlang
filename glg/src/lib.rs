use proc_macro::{Delimiter, Ident, TokenStream, TokenTree};

enum ParseState {
    Start,
    KindName,
    KindBody,

}

#[proc_macro]
pub fn lexer(_ts: TokenStream) -> TokenStream {
    let mut res = String::new();
    let mut state = ParseState::Start;
    let mut pos = 0;
    for tt in _ts.clone() {
        match state {
            ParseState::Start => {
                match tt {
                    TokenTree::Group(_) => {}
                    TokenTree::Ident(ident) => {
                        res += match ident.to_string().as_str() {
                            "kind" => {  state = ParseState::KindName; "enum " }
                            "pub" => { "pub " }
                            _ => { panic!("Unexpected token!"); }
                        };
                    }
                    TokenTree::Punct(_) => {}
                    TokenTree::Literal(_) => {}
                }
            }
            ParseState::KindName => {
                match tt {
                    TokenTree::Ident(ident) => {
                        res += ident.to_string().as_str();
                    },
                    _ => { panic!("Unexpected token!"); }
                }
            },
            ParseState::KindBody => {
                match tt {
                    TokenTree::Group(group) => {
                        if group.delimiter() != Delimiter::Brace {
                            panic!("Unexpected token!");
                        }

                        res += " {\n";

                        let mut state = 0;
                        for tt in group.stream() {
                            match tt {
                                TokenTree::Ident(ident) => {

                                    state = 1;
                                }
                                TokenTree::Group(_) => {}
                                TokenTree::Punct(_) => {}
                                TokenTree::Literal(_) => {}
                            }
                        }

                        res += "\n}\n";
                    }
                    _ => { panic!("Unexpected token!"); }
                }
            }
        };
        pos += 1;
    }
    res.parse().unwrap()
}