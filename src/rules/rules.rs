use super::region::Region;

/// Determine lexing rules for the parser
/// 
/// Rules struct that contains list of symbols as well as region tree
/// 
/// This struct requires two things:
/// 1. List of symbols
/// 2. Region Tree
/// 
/// More on those below in the **Fields** section
/// 
/// # Example
/// ```
/// # use heraclitus::prelude::*;
/// let symbols = vec!['+', '-', '*', '/', '(', ')', '&', '|', '!'];
/// let region = reg![
///     reg!(str as "string literal" => {
///         begin: "'",
///         end: "'"
///     })
/// ];
/// Rules::new(symbols, region);
/// ```

#[derive(Debug, Clone)]
pub struct Rules {
    /// Symbols that should be separated (most commonly: (, ), +, -, ...)
    /// This handles situations like for instance if we want to parse `1+1` as
    /// three tokens `1` `+` `1` and not single `1+1` token.
    pub symbols: Vec<char>,
    /// Region tree that determines what code regions should remain untokenized.
    /// Most common case is a string where we want to preserve all the spaces and words inside.
    pub region_tree: Region,
    /// Escape symbol
    pub escape_symbol: char,
}

impl Rules {
    /// Creates new rules that can be supplied to the compiler
    pub fn new(symbols: Vec<char>, region_tree: Region) -> Rules {
        Rules {
            symbols,
            region_tree,
            escape_symbol: '\\'
        }
    }

    /// Set custom escape symbol for your language
    pub fn set_escape(mut self, symbol: char) -> Self {
        self.escape_symbol = symbol;
        self
    }
}