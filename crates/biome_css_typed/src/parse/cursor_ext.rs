use crate::parse::CssGenericComponentValueCursor;
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

    /// Save current cursor position for backtracking.
    pub fn checkpoint(&self) -> Self {
        self.clone()
    }

    /// Restore to a previously saved checkpoint.
    pub fn restore(&mut self, save: Self) {
        *self = save;
    }
}
