use crate::int_array;
use crate::JArray;
use crate::JError;
use crate::Word;

use JArray::*;
use Word::*;

#[derive(Clone, Debug, PartialEq)]
pub enum VerbImpl {
    Plus,
    Minus,
    Times,
    Number,
    Dollar,
    NotImplemented,

    DerivedVerb { u: Word, m: Word, a: Word }, //Adverb modified Verb eg. +/
}

impl VerbImpl {
    pub fn exec(&self, x: Option<&Word>, y: &Word) -> Result<Word, JError> {
        match self {
            VerbImpl::Plus => v_plus(x, y),
            VerbImpl::Minus => v_minus(x, y),
            VerbImpl::Times => v_times(x, y),
            VerbImpl::Number => v_number(x, y),
            VerbImpl::Dollar => v_dollar(x, y),
            VerbImpl::NotImplemented => v_not_implemented(x, y),
            VerbImpl::DerivedVerb { u, m, a } => match (u, m, a) {
                (Verb(_, _), Nothing, Adverb(_, a)) => a.exec(x, &u, y),
                (Nothing, Noun(_), Adverb(_, a)) => a.exec(x, &m, y),
                _ => panic!("invalid DerivedVerb {:?}", self),
            },
        }
    }
}

fn promotion(x: &JArray, y: &JArray) -> Result<(JArray, JArray), JError> {
    //https://code.jsoftware.com/wiki/Vocabulary/NumericPrecisions#Automatic_Promotion_of_Argument_Precision
    match (x, y) {
        (BoolArray { a: x }, BoolArray { a: y }) => Ok((
            IntArray {
                a: x.map(|i| *i as i64),
            },
            IntArray {
                a: y.map(|i| *i as i64),
            },
        )),
        (BoolArray { a: x }, IntArray { a: y }) => Ok((
            IntArray {
                a: x.map(|i| *i as i64),
            },
            IntArray { a: y.clone() },
        )),
        (IntArray { a: x }, BoolArray { a: y }) => Ok((
            IntArray { a: x.clone() },
            IntArray {
                a: y.map(|i| *i as i64),
            },
        )),
        (BoolArray { a: x }, FloatArray { a: y }) => Ok((
            FloatArray {
                a: x.map(|i| *i as f64),
            },
            FloatArray { a: y.clone() },
        )),
        (FloatArray { a: x }, BoolArray { a: y }) => Ok((
            FloatArray { a: x.clone() },
            FloatArray {
                a: y.map(|i| *i as f64),
            },
        )),

        (IntArray { a: x }, FloatArray { a: y }) => Ok((
            FloatArray {
                a: x.map(|i| *i as f64),
            },
            FloatArray { a: y.clone() },
        )),
        (FloatArray { a: x }, IntArray { a: y }) => Ok((
            FloatArray { a: x.clone() },
            FloatArray {
                a: y.map(|i| *i as f64),
            },
        )),
        _ => Ok((x.clone(), y.clone())),
    }
}

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
        Some(x) => match (x, y) {
            (Word::Noun(x), Word::Noun(y)) => match promotion(x, y) {
                Ok((IntArray { a: x }, IntArray { a: y })) => Ok(Word::Noun(IntArray { a: x + y })),
                Ok((ExtIntArray { a: x }, ExtIntArray { a: y })) => {
                    Ok(Word::Noun(ExtIntArray { a: x + y }))
                }
                Ok((FloatArray { a: x }, FloatArray { a: y })) => {
                    Ok(Word::Noun(FloatArray { a: x + y }))
                }
                Err(e) => Err(e),
                _ => Err(JError {
                    message: "domain error".to_string(),
                }),
            },
            _ => Err(JError {
                message: "plus not supported for these types yet".to_string(),
            }),
        },
    }
}

pub fn v_minus(x: Option<&Word>, y: &Word) -> Result<Word, JError> {
    //TODO use promotion()
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
    //TODO use promotion()
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
        Some(_x) => Err(JError {
            message: "dyadic # not implemented yet".to_string(),
        }), // Copy
    }
}

pub fn v_dollar(x: Option<&Word>, y: &Word) -> Result<Word, JError> {
    match x {
        None => {
            // Shape-of
            match y {
                Word::Noun(ja) => match ja {
                    IntArray { a } => {
                        Ok(int_array(a.shape().iter().map(|i| *i as i64).collect()).unwrap())
                    }
                    ExtIntArray { a } => {
                        Ok(int_array(a.shape().iter().map(|i| *i as i64).collect()).unwrap())
                    }
                    FloatArray { a } => {
                        Ok(int_array(a.shape().iter().map(|i| *i as i64).collect()).unwrap())
                    }
                    BoolArray { a } => {
                        Ok(int_array(a.shape().iter().map(|i| *i as i64).collect()).unwrap())
                    }
                    CharArray { a } => {
                        Ok(int_array(a.shape().iter().map(|i| *i as i64).collect()).unwrap())
                    }
                },
                _ => Err(JError {
                    message: "domain error".to_string(),
                }),
            }
        }
        Some(x) => {
            // Reshape
            // TODO: ndarray's ArrayBase.into_shape() is incorrect behaviour for j
            // Need to implement full reshape behaviour.
            match x {
                Word::Noun(ja) => match ja {
                    IntArray { a: x } => match y {
                        Word::Noun(ja) => match ja {
                            IntArray { a: y } => Ok(Word::Noun(IntArray {
                                a: y.clone()
                                    .into_shape(
                                        x.clone()
                                            .into_raw_vec()
                                            .iter()
                                            .map(|i| *i as usize)
                                            .collect::<Vec<ndarray::Ix>>(),
                                    )
                                    .unwrap(),
                            })),
                            _ => {
                                todo!("reshape not implemented for the rest of the array types yet")
                            }
                        },
                        _ => Err(JError {
                            message: "domain error".to_string(),
                        }),
                    },
                    _ => Err(JError {
                        message: "domain error".to_string(),
                    }),
                },
                _ => Err(JError {
                    message: "domain error".to_string(),
                }),
            }
        }
    }
}
