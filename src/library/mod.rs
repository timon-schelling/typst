//! The standard library for the _Typst_ language.

use crate::func::Scope;

mod align;
mod styles;

/// Useful imports for creating your own functions.
pub mod prelude {
    pub use crate::syntax::{SyntaxTree, FuncHeader, Expression};
    pub use crate::parsing::{parse, ParseContext, ParseResult, ParseError};
    pub use crate::layout::{layout_tree, layout_text, MultiLayout, Layout, LayoutContext};
    pub use crate::layout::{LayoutResult, LayoutError};
    pub use crate::func::{Function, Command, FuncCommands};

    pub fn err<S: Into<String>, T>(message: S) -> ParseResult<T> {
        Err(ParseError::new(message))
    }
}

pub use align::AlignFunc;
pub use styles::{ItalicFunc, BoldFunc, MonospaceFunc};


/// Create a scope with all standard functions.
pub fn std() -> Scope {
    let mut std = Scope::new();
    std.add::<BoldFunc>("bold");
    std.add::<ItalicFunc>("italic");
    std.add::<MonospaceFunc>("mono");
    std.add::<AlignFunc>("align");
    std
}
