struct StrSplitter<'input, 'delimiter> {
    remaining: Option<&'input str>,
    delimiter: &'delimiter str,
}

impl<'input, 'delimiter> StrSplitter<'input, 'delimiter> {
    fn new(input: &'input str, delimiter: &'delimiter str) -> Self {
        Self {
            remaining: Some(input),
            delimiter,
        }
    }
}

impl<'input> Iterator for StrSplitter<'input, '_> {
    type Item = &'input str;

    fn next(&mut self) -> Option<Self::Item> {
        let remaining = self.remaining.as_mut()?;
        if let Some(next_delimiter_index) = remaining.find(self.delimiter) {
            let result = &remaining[..next_delimiter_index];
            *remaining = &remaining[(next_delimiter_index + self.delimiter.len())..];
            Some(result)
        } else {
            self.remaining.take()
        }
    }
}

trait Splitable<'input, 'delimiter> {
    fn my_split(&'input self, delimiter: &'delimiter str) -> StrSplitter<'input, 'delimiter>;
}

impl<'input, 'delimiter> Splitable<'input, 'delimiter> for &str {
    fn my_split(&'input self, delimiter: &'delimiter str) -> StrSplitter<'input, 'delimiter> {
        StrSplitter::new(self, delimiter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = "a b c d";
        let delimiter = " ";

        let res: Vec<_> = input.my_split(delimiter).collect();
        let exp = vec!["a", "b", "c", "d"];

        assert_eq!(res, exp)
    }

    #[test]
    fn empty() {
        let input = "";
        let delimiter = " ";

        let res: Vec<_> = input.my_split(delimiter).collect();
        let exp = vec![""];

        assert_eq!(res, exp)
    }

    #[test]
    fn empty_items() {
        let input = " a  b c d ";
        let delimiter = " ";

        let res: Vec<_> = input.my_split(delimiter).collect();
        let exp = vec!["", "a", "", "b", "c", "d", ""];

        assert_eq!(res, exp)
    }
}
