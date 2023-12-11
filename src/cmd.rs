use std::convert::TryFrom;
use std::vec;

use crate::error::{self, Error};

// Statements express how commands are related
#[derive(Debug, PartialEq)]
pub enum Statement<'a> {
    Pipe {
        input: Box<Statement<'a>>,
        output: Box<Statement<'a>>,
    },

    // A command consists of a binary and its arguments
    Cmd {
        binary: &'a str,
        args: Vec<&'a str>,
    },
}

#[derive(Debug, PartialEq)]
pub struct Statements<'a>(pub Vec<Statement<'a>>);

// We cannot implement TryFrom for a type from the standard library (vec in
// sthis case). Neither `Vec<T>` nor the `TryFrom` trait is defined by us.
// See https://github.com/rust-lang/rust/issues/24745
// A common pattern is to use a newtype instead and implement `TryFrom` for that
// instead.
// See https://doc.rust-lang.org/book/second-edition/ch19-04-advanced-types.html
// See https://doc.rust-lang.org/rust-by-example/generics/new_types.html
// See https://github.com/rust-unofficial/patterns/blob/master/patterns/newtype.md
impl<'a> TryFrom<&'a str> for Statements<'a> {
    type Error = error::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let commands = line.split(';');

        // `std::iter::FromIterator` is not implemented for our newtype. We could either implement it or
        // specify what type we want to collect into and then wrap the result into an `Ok()`.
        let v: Result<Vec<_>, Self::Error> = commands.map(Statement::try_from).collect();

        Ok(Statements(v?))
    }
}

impl<'a> TryFrom<&'a str> for Statement<'a> {
    type Error = error::Error;

    fn try_from(line: &'a str) -> Result<Self, Self::Error> {
        let commands: Vec<_> = line.split('|').collect();

        match commands.len() {
            1 => {
                let mut parts = commands[0].split_whitespace();
                let binary = parts.nth(0).ok_or_else(|| Error::NoBinary)?;
                let args = parts.collect();
                Ok(Statement::Cmd { binary, args })
            }
            2 => {
                let mut parts = commands[0].split_whitespace();
                let binary = parts.nth(0).ok_or_else(|| Error::NoBinary)?;
                let args = parts.collect();
                let input = Box::new(Statement::Cmd { binary, args });

                let mut parts = commands[1].split_whitespace();
                let binary = parts.nth(0).ok_or_else(|| Error::NoBinary)?;
                let args = parts.collect();
                let output = Box::new(Statement::Cmd { binary, args });
                Ok(Statement::Pipe { input, output })
            }
            _ => unimplemented!(),
        }
    }
}

// Since we defined a newtype for our `Cmds` (see above),
// we also need to define all necessary traits for that.
// In order to iterate over `Cmds`, we implement `IntoIterator`,
// which is used when writing `for cmd in cmds`.
impl<'a> IntoIterator for Statements<'a> {
    type Item = Statement<'a>;
    type IntoIter = vec::IntoIter<Statement<'a>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty_line() {
        match Statements::try_from("") {
            Err(Error::NoBinary) => (),
            _ => panic!("Expected NoBinary error"),
        }
    }

    #[test]
    fn test_single_binary() {
        assert_eq!(
            Statements::try_from("echo").unwrap(),
            Statements(vec![Statement::Cmd {
                binary: "echo",
                args: vec![]
            }])
        );
    }

    #[test]
    fn test_binary_with_arguments() {
        assert_eq!(
            Statements::try_from("echo 1 2 3").unwrap(),
            Statements(vec![Statement::Cmd {
                binary: "echo",
                args: vec!["1", "2", "3"]
            }])
        );
    }

    #[test]
    fn test_multiple_commands() {
        assert_eq!(
            Statements::try_from("cat test.txt; echo hello").unwrap(),
            Statements(vec![
                Statement::Cmd {
                    binary: "cat",
                    args: vec!["test.txt"]
                },
                Statement::Cmd {
                    binary: "echo",
                    args: vec!["hello"]
                }
            ])
        );
    }
}
