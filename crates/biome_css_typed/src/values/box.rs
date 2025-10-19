#[derive(Debug, Clone, PartialEq)]
pub struct Box4<T> {
    pub top: T,
    pub right: Option<T>,
    pub bottom: Option<T>,
    pub left: Option<T>,
}

impl<T> Box4<T> {
    pub fn new(values: Vec<T>) -> Option<Self> {
        match values.len() {
            1 => {
                let mut iter = values.into_iter();
                let top = iter.next().expect("length checked above");
                Some(Self {
                    top,
                    right: None,
                    bottom: None,
                    left: None,
                })
            }
            2 => {
                let mut iter = values.into_iter();
                let top = iter.next().expect("length checked above");
                let horizontal = iter.next().expect("length checked above");
                Some(Self {
                    top,
                    right: Some(horizontal),
                    bottom: None,
                    left: None,
                })
            }
            3 => {
                let mut iter = values.into_iter();
                let top = iter.next().expect("length checked above");
                let horizontal = iter.next().expect("length checked above");
                let bottom = iter.next().expect("length checked above");
                Some(Self {
                    top,
                    right: Some(horizontal),
                    bottom: Some(bottom),
                    left: None,
                })
            }
            4 => {
                let mut iter = values.into_iter();
                let top = iter.next().expect("length checked above");
                let right = iter.next().expect("length checked above");
                let bottom = iter.next().expect("length checked above");
                let left = iter.next().expect("length checked above");
                Some(Self {
                    top,
                    right: Some(right),
                    bottom: Some(bottom),
                    left: Some(left),
                })
            }
            _ => None,
        }
    }

    pub fn top(&self) -> &T {
        &self.top
    }

    pub fn right(&self) -> &T {
        self.right.as_ref().unwrap_or(&self.top)
    }

    pub fn bottom(&self) -> &T {
        self.bottom.as_ref().unwrap_or(&self.top)
    }

    pub fn left(&self) -> &T {
        if let Some(left) = self.left.as_ref() {
            left
        } else if let Some(right) = self.right.as_ref() {
            right
        } else {
            &self.top
        }
    }
}
