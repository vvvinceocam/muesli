use winnow::{
    binary::length_take,
    combinator::{opt, terminated},
    token::take_while,
    Parser, Result,
};

pub(crate) fn unsigned_integer<'s>(input: &mut &'s [u8]) -> Result<&'s [u8]> {
    take_while(1.., b'0'..=b'9').parse_next(input)
}

pub(crate) fn signed_integer<'s>(input: &mut &'s [u8]) -> Result<&'s [u8]> {
    (opt(b'-'), unsigned_integer).take().parse_next(input)
}

pub(crate) fn float<'s>(input: &mut &'s [u8]) -> Result<&'s [u8]> {
    (signed_integer, opt((b'.', unsigned_integer)))
        .take()
        .parse_next(input)
}

pub(crate) fn size(input: &mut &[u8]) -> Result<usize> {
    unsigned_integer.parse_to().parse_next(input)
}

pub(crate) fn sized_string<'s>(input: &mut &'s [u8]) -> Result<&'s [u8]> {
    terminated(length_take(terminated(size, b":\"")), b'"').parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::super::tests::run_cases;
    use super::*;

    #[test]
    fn parse_unsigned_integer() {
        let cases = [
            (b"0".as_slice(), Some((b"".as_slice(), b"0".as_slice()))),
            (b"123".as_slice(), Some((b"".as_slice(), b"123".as_slice()))),
            (
                b"745a".as_slice(),
                Some((b"a".as_slice(), b"745".as_slice())),
            ),
            (
                b"592.6".as_slice(),
                Some((b".6".as_slice(), b"592".as_slice())),
            ),
            (b"".as_slice(), None),
            (b"a".as_slice(), None),
            (b"-123".as_slice(), None),
        ];

        run_cases(unsigned_integer, &cases);
    }

    #[test]
    fn parse_signed_integer() {
        let cases = [
            (b"-0".as_slice(), Some((b"".as_slice(), b"-0".as_slice()))),
            (b"123".as_slice(), Some((b"".as_slice(), b"123".as_slice()))),
            (
                b"745a".as_slice(),
                Some((b"a".as_slice(), b"745".as_slice())),
            ),
            (
                b"592.6".as_slice(),
                Some((b".6".as_slice(), b"592".as_slice())),
            ),
            (
                b"-592.6".as_slice(),
                Some((b".6".as_slice(), b"-592".as_slice())),
            ),
            (b"".as_slice(), None),
            (b"a".as_slice(), None),
        ];

        run_cases(signed_integer, &cases);
    }

    #[test]
    fn parse_float() {
        let cases = [
            (b"-0".as_slice(), Some((b"".as_slice(), b"-0".as_slice()))),
            (b"123".as_slice(), Some((b"".as_slice(), b"123".as_slice()))),
            (
                b"745a".as_slice(),
                Some((b"a".as_slice(), b"745".as_slice())),
            ),
            (
                b"592.6".as_slice(),
                Some((b"".as_slice(), b"592.6".as_slice())),
            ),
            (
                b"-592.6".as_slice(),
                Some((b"".as_slice(), b"-592.6".as_slice())),
            ),
            (b"".as_slice(), None),
            (b"a".as_slice(), None),
        ];

        run_cases(float, &cases);
    }

    #[test]
    fn parse_size() {
        let cases = [
            (b"1".as_slice(), Some((b"".as_slice(), 1))),
            (
                b"1234567890".as_slice(),
                Some((b"".as_slice(), 1_234_567_890)),
            ),
            (b"323;".as_slice(), Some((b";".as_slice(), 323))),
            (b"".as_slice(), None),
            (b"a".as_slice(), None),
            (b"a:\"asd\"".as_slice(), None),
            (b"-2:\"asd\"".as_slice(), None),
        ];

        run_cases(size, &cases);
    }

    #[test]
    fn parse_sized_string() {
        let cases = [
            (
                b"1:\"a\"".as_slice(),
                Some((b"".as_slice(), b"a".as_slice())),
            ),
            (
                b"10:\"1234567890\"".as_slice(),
                Some((b"".as_slice(), b"1234567890".as_slice())),
            ),
            (b"".as_slice(), None),
            (b"a".as_slice(), None),
            (b"a:\"asd\"".as_slice(), None),
            (b"-2:\"asd\"".as_slice(), None),
        ];

        run_cases(sized_string, &cases);
    }
}
