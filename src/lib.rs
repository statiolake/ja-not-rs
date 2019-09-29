use typed_igo::conjugation::ConjugationForm as F;
use typed_igo::{Conjugation as C, Morpheme as M, Parser, WordClass as W};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum Form {
    Affirmative,
    Negative,
}

pub fn ja_not_for_polite(parser: &Parser, from: &str) -> String {
    let morphs = parser.parse(from);
    if morphs.is_empty() {
        return "".to_string();
    }

    let current = judge_form(&morphs);
    match current {
        Form::Affirmative => change_affirmative_to_negative(morphs),
        Form::Negative => change_negative_to_affirmative(morphs),
    }
}

fn change_negative_to_affirmative(mut morphs: Vec<M>) -> String {
    match morphs
        .pop()
        .expect("no morphemes: it should be returned earlier")
    {
        M {
            wordclass: W::AuxiliaryVerb,
            basic: "ん",
            ..
        } => match morphs.pop() {
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "ます",
                ..
            }) => match morphs.pop() {
                Some(M {
                    wordclass: W::Verb(_),
                    basic: "ある",
                    ..
                })
                | Some(M {
                    wordclass: W::AuxiliaryVerb,
                    basic: "ある",
                    ..
                }) => match morphs.pop() {
                    Some(M {
                        wordclass: W::Postpositional(_),
                        basic: "は",
                        ..
                    }) => match morphs.pop() {
                        Some(M {
                            wordclass: W::Postpositional(_),
                            basic: "で",
                            ..
                        })
                        | Some(M {
                            wordclass: W::AuxiliaryVerb,
                            basic: "だ",
                            ..
                        }) => morphs_to_string(&morphs) + "です",
                        Some(M { surface, .. }) => {
                            morphs_to_string(&morphs) + surface + "はあります"
                        }
                        None => "はあります".to_string(),
                    },
                    Some(M {
                        wordclass: W::Postpositional(_),
                        basic: "じゃ",
                        ..
                    }) => morphs_to_string(&morphs) + "です",
                    Some(M {
                        wordclass: W::Adjective(_),
                        surface,
                        conjugation: C { kind, form },
                        ..
                    }) => {
                        conjugation::convert(surface, kind, form, F::Basic)
                            .expect("failed to convert to basic form")
                            + "です"
                    }
                    Some(M { surface, .. }) => morphs_to_string(&morphs) + surface + "あります",
                    None => "あります".to_string(),
                },
                Some(M { surface, .. }) => morphs_to_string(&morphs) + surface + "ます",
                None => "ます".to_string(),
            },
            Some(M { surface, .. }) => morphs_to_string(&morphs) + surface,
            None => "".to_string(),
        },
        M {
            wordclass: W::AuxiliaryVerb,
            basic: "です",
            ..
        } => match morphs.pop() {
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "ない",
                ..
            }) => match morphs.pop() {
                Some(M {
                    wordclass: W::Postpositional(_),
                    basic: "は",
                    ..
                }) => match morphs.pop() {
                    Some(M {
                        wordclass: W::Postpositional(_),
                        basic: "で",
                        ..
                    }) => morphs_to_string(&morphs) + "です",
                    Some(M { surface, .. }) => {
                        morphs_to_string(&morphs) + surface + "はあります"
                    }
                    None => "はあります".to_string(),
                },
                Some(M {
                    wordclass: W::Adjective(_),
                    surface,
                    conjugation: C { kind, form },
                    ..
                }) => {
                    conjugation::convert(surface, kind, form, F::Basic)
                        .expect("failed to convert to basic form")
                        + "です"
                }
                Some(M { surface, .. }) => morphs_to_string(&morphs) + surface + "あります",
                None => "あります".to_string(),
            },
            _ => unreachable!("This is not a negative sentence."),
        },
        M {
            wordclass: W::AuxiliaryVerb,
            basic: "た",
            ..
        } => match morphs.pop() {
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "です",
                ..
            }) => match morphs.pop() {
                Some(M {
                    wordclass: W::AuxiliaryVerb,
                    basic: "ん",
                    ..
                }) => match morphs.pop() {
                    Some(M {
                        wordclass: W::AuxiliaryVerb,
                        basic: "ます",
                        ..
                    }) => morphs_to_string(&morphs) + "ました",
                    _ => unreachable!("This is not a negative sentence."),
                },
                _ => unreachable!("This is not a negative sentence."),
            },
            _ => unreachable!("This is not a negative sentence."),
        },
        _ => panic!("The last word is not 「ん」 for negative sentence.  THIS IS A BUG."),
    }
}

fn change_affirmative_to_negative(mut morphs: Vec<M>) -> String {
    match morphs.pop() {
        Some(M {
            wordclass: W::AuxiliaryVerb,
            basic: "です",
            ..
        }) => match morphs.pop() {
            Some(M {
                wordclass: W::Adjective(_),
                surface,
                conjugation: C { kind, form },
                ..
            }) => {
                morphs_to_string(&morphs)
                    + &conjugation::convert(surface, kind, form, F::Continuous)
                        .expect("failed to convert to continuous")
                    + "ありません"
            }
            Some(M { surface, .. }) => {
                morphs_to_string(&morphs) + surface + "ではありません"
            }
            None => "ではありません".to_string(),
        },
        Some(M {
            wordclass: W::AuxiliaryVerb,
            basic: "ます",
            ..
        }) => match morphs.pop() {
            Some(M {
                surface,
                conjugation: C { kind, form },
                ..
            }) => {
                morphs_to_string(&morphs)
                    + &conjugation::convert(surface, kind, form, F::Continuous)
                        .expect("failed to convert to continuous")
                    + "ません"
            }
            None => "ません".to_string(),
        },
        Some(M {
            wordclass: W::AuxiliaryVerb,
            basic: "た",
            ..
        }) => match morphs.pop() {
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "です",
                ..
            }) => morphs_to_string(&morphs) + "ではありませんでした",
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "ます",
                ..
            }) => match morphs.pop() {
                Some(M { surface, .. }) => {
                    morphs_to_string(&morphs) + surface + "ませんでした"
                }
                None => "ませんでした".to_string(),
            },
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn morphs_to_string(morphs: &[M]) -> String {
    morphs.iter().map(|x| x.surface).collect()
}

fn judge_form(morphs: &[M]) -> Form {
    match morphs.last() {
        Some(M {
            wordclass: W::AuxiliaryVerb,
            basic: "ん",
            ..
        }) => Form::Negative,
        Some(M {
            wordclass: W::AuxiliaryVerb,
            basic: "です",
            ..
        }) => match morphs.get(morphs.len() - 2) {
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "ない",
                ..
            }) => Form::Negative,
            _ => Form::Affirmative,
        },
        Some(M {
            wordclass: W::AuxiliaryVerb,
            basic: "た",
            ..
        }) => match morphs.get(morphs.len() - 2) {
            Some(M {
                wordclass: W::AuxiliaryVerb,
                basic: "です",
                ..
            }) => match morphs.get(morphs.len() - 3) {
                Some(M {
                    wordclass: W::AuxiliaryVerb,
                    basic: "ん",
                    ..
                }) => match morphs.get(morphs.len() - 4) {
                    Some(M {
                        wordclass: W::AuxiliaryVerb,
                        basic: "ます",
                        ..
                    }) => Form::Negative,
                    _ => Form::Affirmative,
                },
                _ => Form::Affirmative,
            },
            _ => Form::Affirmative,
        },
        _ => Form::Affirmative,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static::lazy_static! {
        static ref PARSER: Parser = Parser::new();
    }

    macro_rules! check {
        ([neg -> aff] $($naname:ident >> $naa:literal => $nab:literal,)* [aff -> neg] $($anname:ident >> $ana:literal => $anb:literal,)*) => {
            $(
                #[test]
                fn $naname() {
                    assert_eq!(ja_not_for_polite(&*PARSER, $naa), $nab);
                }
            )*
            $(
                #[test]
                fn $anname() {
                    assert_eq!(ja_not_for_polite(&*PARSER, $anb), $ana);
                }
            )*
        }
    }

    check! {
        [neg -> aff]
            na_basic1 >> "寒くないです" => "寒いです",
            na_basic2 >> "寒くありません" => "寒いです",
            na_basic3 >> "寒くはありません" => "寒くはあります",
            na_adverb1 >> "静かではありません" => "静かです",
            na_adverb2 >> "静かじゃありません" => "静かです",
            na_verb1 >> "読みません" => "読みます",
            na_verb2 >> "読んでいません" => "読んでいます",
            na_verb3 >> "読んでいませんでした" => "読んでいました",

        [aff -> neg]
            an_basic2 >> "寒いです" =>  "寒くありません",
            an_adverb1 >> "静かです" =>  "静かではありません",
            an_verb1 >> "読みます" =>  "読みません",
            an_verb2 >> "読んでいます" =>  "読んでいません",
            an_verb3 >> "読んでいました" =>  "読んでいませんでした",
    }
}
