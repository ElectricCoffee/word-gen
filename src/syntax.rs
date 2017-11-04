/// The Syntax trait is to enable one to convert their data into a syntax rule
pub trait Syntax<'a> {
    fn to_syntax(&'a self) -> SyntaxRule<'a>;
}

pub enum SyntaxRule<'a> {
    Id(&'a str), // anything that isn't any of the following
    Opt(Box<SyntaxRule<'a>>), // the ? operator
    Plus(Box<SyntaxRule<'a>>), // the + operator
    Star(Box<SyntaxRule<'a>>), // the * operator
    Group(Vec<SyntaxRule<'a>>), // the (...) construct
    Choice(Vec<SyntaxRule<'a>>), // the [...] construct
}

impl<'a> SyntaxRule<'a> {
    pub fn from(input: &'a str) -> SyntaxRule<'a> {
        panic!("Not yet implemented!")
    }
}


impl<'a> Syntax<'a> for str {
    /// to_syntax takes a str and returns a Syntax rule
    ///
    /// This function is meant as the dual of SyntaxRule::from("foo").
    /// Much like how you can write both String::from("foo") and "foo".to_string(),
    /// This allows you to write both SyntaxRule::from("foo") and "foo".to_syntax()
    fn to_syntax(&'a self) -> SyntaxRule<'a> {
        SyntaxRule::from(self)
    }
}
