pub enum SyntaxRule<'a> {
    Id(&'a str), // anything that isn't any of the following
    Opt(Box<SyntaxRule<'a>>), // the ? operator
    Plus(Box<SyntaxRule<'a>>), // the + operator
    Star(Box<SyntaxRule<'a>>), // the * operator
    Group(Vec<SyntaxRule<'a>>), // the (...) construct
    Choice(Vec<SyntaxRule<'a>>), // the [...] construct
}

impl<'a> SyntaxRule<'a> {

}
