// errors6.rs

// Using catch-all error types like `Box<dyn error::Error>` isn't recommended
// for library code, where callers might want to make decisions based on the
// error content, instead of printing it out or propagating it further. Here,
// we define a custom error type to make it possible for callers to decide
// what to do next when our function returns an error.

// Make these tests pass! Execute `rustlings hint errors6` for hints :)

use std::num::ParseIntError;

// This is a custom error type that we will be using in `parse_pos_nonzero()`.
#[derive(PartialEq, Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError)
}

impl ParsePosNonzeroError {
    fn from_parse(err: ParseIntError) -> ParsePosNonzeroError
    {
        ParsePosNonzeroError::ParseInt(err)
    }

    fn from_creation(err: CreationError) -> ParsePosNonzeroError
    {
        ParsePosNonzeroError::Creation(err)
    }
}

fn parse_pos_nonzero(s: &str)
    -> Result<PositiveNonzeroInteger, ParsePosNonzeroError>
{
    // Each of the next two lines can create a panic.  The first line from parse,
    // and the second one from a creation error in creating the 
    // PositiveNonzeroInteger.  In both cases we need to allow things to proceed
    // as normal most of the time, and if an error occurs to return the error.
    // The return of the function is a Result, so we have a way to return an error.
    // But the errors thrown can be of two different types.  One way to handle this
    // would be with a Box<dyn error::Error> but this method is opaque to the caller
    // and loses critical information about the error.  The OTHER way, shown here,
    // is to use an custom error enum (represented by the ParsePosNonzeroError enum
    // here).  Then, we need to trap any errors that occur and convert them into 
    // the enum.  Both parse and PositiveNonzeroInteger return a Result with their
    // error information, and .map_err will call the defined function if there is
    // an error reported.  (For no error, .map_err will just return the original
    // Result.) In our case, then, we have .map_err call two functions that each
    // return the correct error type from our enum for the appropriate situation.
    // Each of the enum values embeds the original error, so no information is
    // lost in this approach.  As a final step, we use the '?' operator to return
    // early from the function if an error is reported.
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parse)?;
    PositiveNonzeroInteger::new(x)
        .map_err(ParsePosNonzeroError::from_creation)
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64))
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_error() {
        // We can't construct a ParseIntError, so we have to pattern match.
        assert!(matches!(
            parse_pos_nonzero("not a number"),
            Err(ParsePosNonzeroError::ParseInt(_))
        ));
    }

    #[test]
    fn test_negative() {
        assert_eq!(
            parse_pos_nonzero("-555"),
            Err(ParsePosNonzeroError::Creation(CreationError::Negative))
        );
    }

    #[test]
    fn test_zero() {
        assert_eq!(
            parse_pos_nonzero("0"),
            Err(ParsePosNonzeroError::Creation(CreationError::Zero))
        );
    }

    #[test]
    fn test_positive() {
        let x = PositiveNonzeroInteger::new(42);
        assert!(x.is_ok());
        assert_eq!(parse_pos_nonzero("42"), Ok(x.unwrap()));
    }
}
