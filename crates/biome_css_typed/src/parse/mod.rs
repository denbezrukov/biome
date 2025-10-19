pub mod cursor_ext;
pub mod ext;

use std::iter::Peekable;

use biome_css_syntax::{AnyCssGenericComponentValue, CssGenericComponentValueList, CssLanguage};
use biome_rowan::{AstNodeList, AstNodeListIterator};

#[derive(Clone)]
pub struct CssGenericComponentValueCursor {
    iter: Peekable<AstNodeListIterator<CssLanguage, AnyCssGenericComponentValue>>,
}

impl CssGenericComponentValueCursor {
    pub fn new(list: &CssGenericComponentValueList) -> Self {
        Self {
            iter: list.iter().peekable(),
        }
    }

    pub fn is_eof(&self) -> bool {
        let mut iter = self.iter.clone();
        iter.next().is_none()
    }

    pub fn peek(&mut self) -> Option<&AnyCssGenericComponentValue> {
        self.iter.peek()
    }

    pub fn bump(&mut self) -> Option<AnyCssGenericComponentValue> {
        self.iter.next()
    }

    pub fn bump_non_delim(&mut self) -> Option<AnyCssGenericComponentValue> {
        while let Some(node) = self.bump() {
            if node.as_css_generic_delimiter().is_none() {
                return Some(node);
            }
        }
        None
    }

    pub fn peek_non_delim(&self) -> Option<AnyCssGenericComponentValue> {
        let mut iter = self.iter.clone();
        while let Some(node) = iter.next() {
            if node.as_css_generic_delimiter().is_none() {
                return Some(node);
            }
        }
        None
    }
}
