use biome_css_syntax::AnyCssGenericComponentValue;

use crate::CssGenericComponentValueCursor;
use crate::parse::ext::CssNodeExt;

impl CssGenericComponentValueCursor {
    /// Advance over consecutive delimiter nodes.
    pub fn skip_delims(&mut self) {
        while let Some(node) = self.peek() {
            if node.is_delim() {
                self.bump();
            } else {
                break;
            }
        }
    }

    /// Peek next non-delimiter node by reference, without consuming.
    pub fn peek_nd_ref(&self) -> Option<&AnyCssGenericComponentValue> {
        let mut iter = self.clone();
        while let Some(node) = iter.peek() {
            if node.is_delim() {
                iter.bump();
            } else {
                return Some(node);
            }
        }
        None
    }

    /// Save current cursor position for backtracking.
    pub fn checkpoint(&self) -> Self {
        self.clone()
    }

    /// Restore to a previously saved checkpoint.
    pub fn restore(&mut self, save: Self) {
        *self = save;
    }
}
