use std::io::{self, Write};

// All terminology should match J terminology:
// Glossary: https://code.jsoftware.com/wiki/Vocabulary/Glossary
#[derive(Debug, PartialEq)]
pub enum Token {
    LP,
    RP,
    Primitive(String),
    Name(String),
    LitNumArray(String),
    LitString(String),
}

#[rustfmt::skip]
fn primitives() -> Vec<String> {
    // https://code.jsoftware.com/wiki/NuVoc
    vec![
    "=","=.","=:",
    "<","<.","<:",
    ">",">.",">:",
    "_","_.","_:",

    "+","+.","+:",
    "*","*.","*:",
    "-","-.","-:",
    "%","%.","%:",

    "^","^.","^:",
    "^!.",
    "$","$.","$:",
    "~","~.","~:",
    "|","|.","|:",

    ".","..",".:",
    ":",":.","::",
    ",.",",",",:",
    ";",";.",";:",

    "#","#.","#:",
    "!","!.","!:",
    "/","/.","/:",
    "\\","\\.","\\:",

    "[","[.","[:",
    "]","].","]:",
    "{","{.","{:","{::",
    "}","}.","}:",
    "{{","}}",

    "\"","\".","\":",
    "`","`:",
    "@","@.","@:",
    "&","&.","&:","&.:",
    "?","?.",

    "a.","a:","A.",
    "b.","C.","C.!.2","d.",
    "D.","D:","e.",
    "E.","f:",
    "F.","F..","F.:",
    "F:","F:.","F::",

    "H.","i.","i:",
    "I.","j.","L.",
    "L:","M.","NB.",
    "o.","p.","p..",

    "p:","q:","r.",
    "s:","S:","t.",
    "T.","u:","x:",
    "Z:",
    "_9:","_8:","_7:","_6:","_5:","_4:","_3:","_2:","_1:","0:","1:","2:","3:","4:","5:","6:","7:","8:","9",
    "u.","v.",
    "assert.", "break.", "continue.",
    "else.", "elseif.", "for.",
    "for_ijk.",   // TODO handle ijk label properly
    "goto_lbl.",  // TODO handle lbl properly
    "label_lbl.", // TODO handle lbl properly
    "if.", "return.", "select.", "case.", "fcase.",
    "throw.", "try.", "catch.", "catchd.", "catcht.",
    "while.", "whilst.",
    ].into_iter().map(String::from).collect()
}

#[derive(Debug)]
pub struct ParseError {
    message: String,
}

pub fn scan(sentence: &str) -> Result<Vec<Token>, ParseError> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut skip: usize = 0;

    //TODO recursive descent instead of a dumb loop.
    //TODO support multiline definitions.
    for (i, c) in sentence.chars().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        match c {
            '(' => {
                tokens.push(Token::LP);
            }
            ')' => {
                tokens.push(Token::RP);
            }
            c if c.is_whitespace() => (),
            '0'..='9' | '_' => {
                let (l, t) = scan_litnumarray(&sentence[i..])?;
                tokens.push(t);
                skip = l;
                continue;
            }
            '\'' => {
                let (l, t) = scan_litstring(&sentence[i..])?;
                tokens.push(t);
                skip = l;
                continue;
            }
            'a'..='z' | 'A'..='Z' => {
                let (l, t) = scan_name(&sentence[i..])?;
                tokens.push(t);
                skip = l;
                continue;
            }
            _ => {
                let (l, t) = scan_primitive(&sentence[i..])?;
                tokens.push(t);
                skip = l;
                continue;
            }
        }
    }
    Ok(tokens)
}

fn scan_litnumarray(sentence: &str) -> Result<(usize, Token), ParseError> {
    let mut l: usize = usize::MAX;
    for (i, c) in sentence.chars().enumerate() {
        l = i;
        match c {
            '0'..='9' | '.' | '_' | 'e' | 'j' | 'r' | ' ' | '\t' => {
                () //still valid keep iterating
            }
            _ => {
                break;
            }
        }
    }
    //Err(ParseError {message: String::from("Empty number literal")})
    Ok((l, Token::LitNumArray(String::from(&sentence[0..l]))))
}

fn scan_litstring(sentence: &str) -> Result<(usize, Token), ParseError> {
    if sentence.len() < 2 {
        return Err(ParseError {
            message: String::from("Empty literal string"),
        });
    }

    let mut l: usize = usize::MAX;
    let mut prev_c_is_quote: bool = false;
    // strings in j are single quoted: 'foobar'.
    // literal ' chars are included in a string by doubling: 'foo ''lol'' bar'.
    for (i, c) in sentence.chars().enumerate().skip(1) {
        l = i;
        match c {
            '\'' => match prev_c_is_quote {
                true =>
                // double quote in string, literal quote char
                {
                    prev_c_is_quote = false
                }
                false => prev_c_is_quote = true,
            },
            '\n' => {
                if prev_c_is_quote {
                    l -= 1;
                    break;
                } else {
                    return Err(ParseError {
                        message: String::from("open quote"),
                    });
                }
            }
            _ => match prev_c_is_quote {
                true => {
                    //string closed previous char
                    l -= 1;
                    break;
                }
                false => {
                    () //still valid keep iterating
                }
            },
        }
    }
    Ok((
        l,
        Token::LitString(String::from(&sentence[1..l]).replace("''", "'")),
    ))
}

fn scan_name(sentence: &str) -> Result<(usize, Token), ParseError> {
    // user defined adverbs/verbs/nouns
    let mut l: usize = usize::MAX;
    let mut p: Option<Token> = None;
    for (i, c) in sentence.chars().enumerate() {
        l = i;
        //if "()`.:; \t\n".contains(c) {
        //break;
        //}
        // Name is a word that begins with a letter and contains letters, numerals, and
        // underscores. (See Glossary).
        match c {
            'a'..='z' | 'A'..='Z' | '_' => {
                match p {
                    None => (),
                    Some(_) => {
                        // Primitive was found on previous char, backtrack and break
                        l -= 1;
                        break;
                    }
                }
            }
            '.' | ':' => {
                match p {
                    None => {
                        if primitives().contains(&String::from(&sentence[0..=l])) {
                            // Found a primitive. eg: F.
                            p = Some(Token::Primitive(String::from(&sentence[0..=l])));
                        }
                    }
                    Some(_) => {
                        if primitives().contains(&String::from(&sentence[0..=l])) {
                            // Found a longer primitive. eg: F.:
                            p = Some(Token::Primitive(String::from(&sentence[0..=l])));
                        } else {
                            // Primitive was found on previous char, backtrack and break
                            l -= 1;
                            break;
                        }
                    }
                }
            }
            _ => {
                l -= 1;
                break;
            }
        }
    }

    match p {
        Some(p) => Ok((l, p)),
        None => {
            if 0 != l {
                Ok((l, Token::Name(String::from(&sentence[0..=l]))))
            } else {
                Err(ParseError {
                    message: String::from("Empty name"),
                })
            }
        }
    }
}

fn scan_primitive(sentence: &str) -> Result<(usize, Token), ParseError> {
    // built in adverbs/verbs
    let mut l: usize = 0;
    let mut p: Option<char> = None;
    //Primitives are 1 to 3 symbols:
    //  - one symbol
    //  - zero or more trailing . or : or both.
    //  - OR {{ }} for definitions
    for (i, c) in sentence.chars().enumerate() {
        l = i;
        match p {
            None => p = Some(c),
            Some(p) => {
                match p {
                    '{' => {
                        if !"{.:".contains(c) {
                            l -= 1;
                            break;
                        }
                    }
                    '}' => {
                        if !"}.:".contains(c) {
                            l -= 1;
                            break;
                        }
                    }
                    //if !"!\"#$%&*+,-./:;<=>?@[\\]^_`{|}~".contains(c) {
                    _ => {
                        if !".:".contains(c) {
                            l -= 1;
                            break;
                        }
                    }
                }
            }
        }
    }
    if 0 == l {
        return Err(ParseError {
            message: String::from("Empty primitive"),
        });
    }
    Ok((l, Token::Primitive(String::from(&sentence[0..=l]))))
}

#[test]
fn test_scan_num() {
    let tokens = scan("1 2 _3\n").unwrap();
    println!("{:?}", tokens);
    assert_eq!(tokens, [Token::LitNumArray(String::from("1 2 _3"))]);
}

#[test]
fn test_scan_string() {
    let tokens = scan("'abc'").unwrap();
    println!("{:?}", tokens);
    assert_eq!(tokens, [Token::LitString(String::from("abc"))]);
}

#[test]
fn test_scan_name() {
    let tokens = scan("abc\n").unwrap();
    println!("{:?}", tokens);
    assert_eq!(tokens, [Token::Name(String::from("abc"))]);
}

#[test]
fn test_scan_name_verb_name() {
    let tokens = scan("foo + bar\n").unwrap();
    println!("{:?}", tokens);
    assert_eq!(
        tokens,
        [
            Token::Name(String::from("foo")),
            Token::Primitive(String::from("+")),
            Token::Name(String::from("bar")),
        ]
    );
}

#[test]
fn only_whitespace() {
    scan("\r").unwrap();
}

#[test]
fn test_scan_string_verb_string() {
    let tokens = scan("'abc','def'").unwrap();
    println!("{:?}", tokens);
    assert_eq!(
        tokens,
        [
            Token::LitString(String::from("abc")),
            Token::Primitive(String::from(",")),
            Token::LitString(String::from("def")),
        ]
    );
}

#[test]
fn test_scan_name_verb_name_not_spaced() {
    let tokens = scan("foo+bar\n").unwrap();
    println!("{:?}", tokens);
    assert_eq!(
        tokens,
        [
            Token::Name(String::from("foo")),
            Token::Primitive(String::from("+")),
            Token::Name(String::from("bar")),
        ]
    );
}

#[test]
fn test_scan_primitives() {
    let tokens = scan("a. I. 'A' \n").unwrap();
    println!("{:?}", tokens);
    assert_eq!(
        tokens,
        [
            Token::Primitive(String::from("a.")),
            Token::Primitive(String::from("I.")),
            Token::LitString(String::from("A")),
        ]
    );
}

#[test]
fn test_scan_primitives_not_spaced() {
    let tokens = scan("a.I.'A' \n").unwrap();
    println!("{:?}", tokens);
    assert_eq!(
        tokens,
        [
            Token::Primitive(String::from("a.")),
            Token::Primitive(String::from("I.")),
            Token::LitString(String::from("A")),
        ]
    );
}
