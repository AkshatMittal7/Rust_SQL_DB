#![warn(missing_docs)]
//Found a steeldb parser crate that I will be utilizing here 

use lalrpop_util::lalrpop_mod;

// lalrpop_mod!(select); // synthesized by LALRPOP

// /// Enum used for propagating the parse error.
// /// At the moment it only contains one generic Error.
// /// Internally, this library just forwards the lalrpop error as a formatted string:
// /// ```rust
// /// Err(error) => {
// ///     let error = format!("{:?}", error);
// ///     return Err(ParseError::Error(format!(
// ///         "Failed to parse, error: {}",
// ///         error
// ///     )));
// /// }
// /// ```
// #[derive(Debug)]
// pub enum ParseError {
//     /// Generic parser error case.
//     Error(String),
// }

// /// Parses a select clause in the format 'select col1, col2;'.
// ///
// /// Example:
// /// ```rust
// /// let result = parse_select("select brigadeiro, churros;".to_string()).unwrap();
// /// let v = vec!["brigadeiro".to_string(), "churros".to_string()];
// /// assert_eq!(v, result);
// /// ```
// /// Notice that this function does not yet support the FROM clause.
// pub fn parse_select(input: String) -> Result<Vec<String>, ParseError> {
//     let mut result: Vec<String> = vec![];
//     let parser = select::SelectParser::new();
//     let maybe_error = parser.parse(&mut result, input.as_str());
//     match maybe_error {
//         Ok(_) => {
//             return Ok(result);
//         }
//         Err(error) => {
//             let error = format!("{:?}", error);
//             return Err(ParseError::Error(format!(
//                 "Failed to parse, error: {}",
//                 error
//             )));
//         }
//     };
// }
