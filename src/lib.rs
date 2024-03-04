//! A zero-setup commandline argument parser.
//!
//! Parse arguments with [parse_arguments].

/// A parsed command line argument.
///
/// An argument is an operand, attribute or flag.
#[derive(Clone, PartialEq, Eq)]
pub enum Argument {
    Operand(String),
    Attribute(String, String),
    Flag(String),
}

#[derive(Debug)]
pub enum Error {
    EmptyAttribute,
    EmptyFlag,
}

/// A character iterator with 1 character lookahead.
struct CharIter<I: Iterator<Item=char>> {
    iter: I,
    c: Option<char>,
}

impl <I: Iterator<Item=char>> CharIter<I> {

    fn new(mut iter: I) -> Self {
        let c = iter.next();
        CharIter { iter, c }
    }

    fn next(&mut self) {
        self.c = self.iter.next();
    }

    fn current(&self) -> Option<char> {
        self.c
    }

}

/// Parse command line arguments.
///
/// Note that the first argument is often the command or binary name.
pub fn parse_arguments<T: Iterator<Item=String>>(mut args: T) -> Result<Vec<Argument>, Error> {
    let mut arguments = vec![];
    for arg in &mut args {
        let mut iter = CharIter::new(arg.chars());
        if let Some(c) = iter.current() {
            if c == '-' {
                iter.next();
                if let Some(d) = iter.current() {
                    if d == '-' {
                        iter.next();
                        if let Some(_) = iter.current() {
                            parse_option(&mut arguments, &mut iter)?;
                        } else {
                            break;
                        }
                    } else {
                        parse_short_options(&mut arguments, &mut iter)?;
                    }
                } else {
                    break;
                }
            } else if c == '+' {
                iter.next();
                if let Some(_) = iter.current() {
                    parse_option(&mut arguments, &mut iter)?;
                } else {
                    break;
                }
            } else {
                parse_argument(&mut arguments, &mut iter)?;
            }
        } else {
            arguments.push(Argument::Operand(String::new()));
        }
    }
    for arg in &mut args {
        arguments.push(Argument::Operand(arg));
    }
    Ok(arguments)
}

/// Parse a short option.
fn parse_short_options<I: Iterator<Item=char>>(arguments: &mut Vec<Argument>, iter: &mut CharIter<I>) -> Result<(), Error> {
    loop {
        if let Some(c) = iter.current() {
            iter.next();
            if c == '=' {
                return Err(Error::EmptyAttribute);
            } else {
                if let Some(d) = iter.current() {
                    if d == '=' {
                        iter.next();
                        parse_attribute_value(arguments, iter, String::from(c))?;
                        break;
                    } else {
                        arguments.push(Argument::Flag(String::from(c)));
                    }
                } else {
                    arguments.push(Argument::Flag(String::from(c)));
                    break;
                }
            }
        } else {
            break;
        }
    }
    Ok(())
}

/// Parse an option.
///
/// An option is a flag or an attribute.
fn parse_option<I: Iterator<Item=char>>(arguments: &mut Vec<Argument>, iter: &mut CharIter<I>) -> Result<(), Error> {
    let mut option = String::new();
    if let Some(c) = iter.current() {
        iter.next();
        if c == '+' {
            return Err(Error::EmptyFlag);
        } else if c == '=' {
            return Err(Error::EmptyAttribute);
        } else {
            option.push(c);
        }
    } else {
        return Err(Error::EmptyFlag);
    }
    loop {
        if let Some(c) = iter.current() {
            iter.next();
            if c == '+' {
                arguments.push(Argument::Flag(option));
                parse_option(arguments, iter)?;
                return Ok(());
            } else if c == '=' {
                parse_attribute_value(arguments, iter, option)?;
                return Ok(());
            } else {
                option.push(c);
            }
        } else {
            arguments.push(Argument::Flag(option));
            return Ok(());
        }
    }
}

/// Parse an argument.
///
/// An argument is an operand, attribute or flag.
fn parse_argument<I: Iterator<Item=char>>(arguments: &mut Vec<Argument>, iter: &mut CharIter<I>) -> Result<(), Error> {
    let mut argument = String::new();
    if let Some(c) = iter.current() {
        iter.next();
        if c == '+' {
            return Err(Error::EmptyFlag);
        } else if c == '=' {
            return Err(Error::EmptyAttribute);
        } else {
            argument.push(c);
        }
    } else {
        arguments.push(Argument::Operand(argument));
        return Ok(());
    }
    loop {
        if let Some(c) = iter.current() {
            iter.next();
            if c == '+' {
                arguments.push(Argument::Flag(argument));
                parse_option(arguments, iter)?;
                return Ok(());
            } else if c == '=' {
                parse_attribute_value(arguments, iter, argument)?;
                return Ok(());
            } else {
                argument.push(c);
            }
        } else {
            arguments.push(Argument::Operand(argument));
            return Ok(());
        }
    }
}

/// Parse an attribute value.
fn parse_attribute_value<I: Iterator<Item=char>>(arguments: &mut Vec<Argument>, iter: &mut CharIter<I>, attribute: String) -> Result<(), Error> {
    let mut value = String::new();
    loop {
        if let Some(c) = iter.current() {
            iter.next();
            value.push(c);
        } else {
            break;
        }
    }
    arguments.push(Argument::Attribute(attribute, value));
    Ok(())
}

#[test]
#[ignore]
fn test_main() {
    let mut args = std::env::args();
    args.next(); args.next(); args.next(); args.next(); args.next();
    for a in parse_arguments(args).unwrap() {
        match a {
            Argument::Operand(o) => println!("Operand: `{}`", o),
            Argument::Attribute(a, v) => println!("Attribute `{}`: {}", a, v),
            Argument::Flag(f) => println!("Flag `{}`", f),
        }
    }
}
