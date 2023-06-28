pub struct ArgParser<T: ToString, I: Iterator<Item = T>> {
    iter: I
}

pub struct ParsedArgs {

}

pub trait ParseArgs<T: ToString>: IntoIterator<Item = T> {
    fn parse_args(self) -> ArgParser<T, Self::IntoIter>;
}

impl<T: ToString, I: IntoIterator<Item = T>> ParseArgs<T> for I {
    fn parse_args(self) -> ArgParser<T, Self::IntoIter> {
        ArgParser {
            iter: self.into_iter()
        }
    }
}