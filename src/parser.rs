
pub struct Element<'a> {
    pub hash: &'a str,
    pub element: &'a str
}

pub trait Parser {
    fn depleted(&self) -> bool;
    fn next(&self) -> Element;
    fn notify(&self, element: &Element) -> Result<bool, &str>;
}

impl<'a, T> Parser for &'a T where T: Parser {
    fn depleted(&self) -> bool {
        (*self).depleted()
    }

    fn next(&self) -> Element {
        (*self).next()
    }

    fn notify(&self, element: &Element) -> Result<bool, &str> {
        (*self).notify(element)
    }
}
