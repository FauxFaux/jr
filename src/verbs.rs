use crate::int_array;
use crate::JArray::*;
use crate::JError;
use crate::Word;

pub fn v_not_implemented(_x: Option<&Word>, _y: &Word) -> Result<Word, JError> {
    Err(JError {
        message: "verb not implemented yet".to_string(),
    })
}

pub fn v_plus(x: Option<&Word>, y: &Word) -> Result<Word, JError> {
    match x {
        None => Err(JError {
            message: "monadic + not implemented yet".to_string(),
        }),
        Some(x) => {
            if let (Word::Noun(IntArray { a: x }), Word::Noun(IntArray { a: y })) = (x, y) {
                Ok(Word::Noun(IntArray { a: x + y }))
            } else {
                Err(JError {
                    message: "plus not supported for these types yet".to_string(),
                })
            }
        }
    }
}

pub fn v_minus(x: Option<&Word>, y: &Word) -> Result<Word, JError> {
    match x {
        None => Err(JError {
            message: "monadic - not implemented yet".to_string(),
        }),
        Some(x) => {
            if let (Word::Noun(IntArray { a: x }), Word::Noun(IntArray { a: y })) = (x, y) {
                Ok(Word::Noun(IntArray { a: x - y }))
            } else {
                Err(JError {
                    message: "minus not supported for these types yet".to_string(),
                })
            }
        }
    }
}

pub fn v_times(x: Option<&Word>, y: &Word) -> Result<Word, JError> {
    match x {
        None => Err(JError {
            message: "monadic * not implemented yet".to_string(),
        }),
        Some(x) => {
            if let (Word::Noun(IntArray { a: x }), Word::Noun(IntArray { a: y })) = (x, y) {
                Ok(Word::Noun(IntArray { a: x * y }))
            } else {
                Err(JError {
                    message: "times not supported for these types yet".to_string(),
                })
            }
        }
    }
}

pub fn v_number(x: Option<&Word>, y: &Word) -> Result<Word, JError> {
    match x {
        None => {
            // Tally
            match y {
                Word::Noun(ja) => match ja {
                    IntArray { a } => Ok(int_array(vec![a.len() as i64]).unwrap()),
                    ExtIntArray { a } => Ok(int_array(vec![a.len() as i64]).unwrap()),
                    FloatArray { a } => Ok(int_array(vec![a.len() as i64]).unwrap()),
                    BoolArray { a } => Ok(int_array(vec![a.len() as i64]).unwrap()),
                    CharArray { a } => Ok(int_array(vec![a.len() as i64]).unwrap()),
                },
                _ => Err(JError {
                    message: "domain error".to_string(),
                }),
            }
        }
        Some(x) => Err(JError {
            message: "dyadic # not implemented yet".to_string(),
        }), // Copy
    }
}
