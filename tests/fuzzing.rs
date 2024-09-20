use muesli::{session_decode, unserialize};
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig::with_cases(10_000))]

    #[test]
    fn unserialize_should_not_panic(data: Vec<u8>) {
        let _ = unserialize(&data);
    }

    #[test]
    fn session_decode_should_not_panic(data: Vec<u8>) {
        let _ = session_decode(&data);
    }
}
