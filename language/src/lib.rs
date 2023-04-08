use std::fmt::{Display, Formatter};
use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest_derive::Parser;
use crate::SyntaxError::UnexpectedGrammarRule;

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
pub enum Language {
    Variable(String),
    Number(u64),
    Compound {
        function: String,
        children: Vec<Box<Language>>
    }
}

#[derive(thiserror::Error, Debug)]
pub enum SyntaxError {
    #[error("{0}")]
    PestError(#[from] Error<Rule>),
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("input ends unexpectedly")]
    UnexpectedEOI,
    #[error("encountered unexpected grammar rule")]
    UnexpectedGrammarRule
}


impl Display for Language {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Variable(x) => write!(f, "{x}"),
            Language::Number(x) => write!(f, "{x}"),
            Language::Compound { function, children } => {
                children.iter().fold(write!(f, "({function}"), |acc, x| {
                    acc.and_then(|_| write!(f, " {x}"))
                })
                    .and_then(|_| write!(f, ")"))
            }
        }
    }
}

#[derive(Parser)]
#[grammar = "language.pest"]
struct LanguageParser;

impl Language {
    pub fn parse<S : AsRef<str>>(input : S) -> Result<Box<Self>, SyntaxError> {
        use pest::Parser;
        let parse_tree : Pairs<Rule> = LanguageParser::parse(Rule::program, input.as_ref())?;
        Self::from_pairs(parse_tree)
    }
    fn from_pair(next: Pair<Rule>) -> Result<Box<Self>, SyntaxError> {
        match next.as_rule() {
            Rule::program => {
                Self::from_pairs(next.into_inner())
            }
            Rule::variable => {
                Ok(Box::from(Self::Variable(next.as_str().to_string())))
            }
            Rule::number => {
                Ok(Box::from(Self::Number(next.as_str().parse()?)))
            }
            Rule::compound => {
                let mut inner = next.into_inner();
                let function = inner.next().ok_or(SyntaxError::UnexpectedEOI)?;
                let function = match function.as_rule() {
                    Rule::function => {
                        function.as_str().to_string()
                    }
                    _ => return Err(UnexpectedGrammarRule)
                };
                let children = inner.fold(Ok(Vec::new()), |acc : Result<Vec<Box<Self>>, SyntaxError>, x| {
                    acc.and_then(|mut acc| {acc.push(Self::from_pair(x)?); Ok(acc)} )
                })?;
                Ok(Box::from(Self::Compound {function, children}))
            }
            _ => Err(UnexpectedGrammarRule)
        }
    }
    fn from_pairs(mut tree: Pairs<Rule>) -> Result<Box<Self>, SyntaxError> {
        let next = tree.next().ok_or(SyntaxError::UnexpectedEOI)?;
        Self::from_pair(next)
    }
}


#[cfg(test)]
mod test {
    use crate::Language;

    #[test]
    fn test_parsing() {
        let test = "(% (+ 12 (* 3 4)) 5261)";
        let tree = Language::parse(test).unwrap();
        println!("{}", tree);
    }
}

