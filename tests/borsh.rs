#![cfg(feature = "borsh")]
use std::fmt;
extern crate arrayvec;
extern crate borsh;

fn assert_ser<T: borsh::BorshSerialize>(v: &T, expected_bytes: &[u8]) {
    let mut actual_bytes = Vec::new();
    v.serialize(&mut actual_bytes).unwrap();
    assert_eq!(actual_bytes, expected_bytes);
}

fn assert_roundtrip<T: borsh::BorshSerialize + borsh::BorshDeserialize + PartialEq + fmt::Debug>(
    v: &T,
) {
    let mut bytes = Vec::new();
    v.serialize(&mut bytes).unwrap();
    let v_de = T::try_from_slice(&bytes).unwrap();
    assert_eq!(*v, v_de);
}

mod array_vec {
    use super::{assert_roundtrip, assert_ser};
    use arrayvec::ArrayVec;

    #[test]
    fn test_empty() {
        let vec = ArrayVec::<u32, 0>::new();
        assert_ser(&vec, b"\0\0\0\0");
        assert_roundtrip(&vec);
    }

    #[test]
    fn test_full() {
        let mut vec = ArrayVec::<u32, 3>::new();
        vec.push(0xdeadbeef);
        vec.push(0x123);
        vec.push(0x456);
        assert_ser(&vec, b"\x03\0\0\0\xef\xbe\xad\xde\x23\x01\0\0\x56\x04\0\0");
        assert_roundtrip(&vec);
    }

    #[test]
    fn test_with_free_capacity() {
        let mut vec = ArrayVec::<u32, 3>::new();
        vec.push(0xdeadbeef);
        assert_ser(&vec, b"\x01\0\0\0\xef\xbe\xad\xde");
        assert_roundtrip(&vec);
    }

    #[test]
    fn test_borsh_one_to_one() {
        use borsh::{BorshDeserialize, BorshSerialize};
        #[derive(BorshSerialize, BorshDeserialize)]
        struct Something {
            a: String,
        }
        let a = Something {
            a: "hello".to_owned(),
        };
        let mut vec = ArrayVec::<u8, 9>::new();
        a.serialize(&mut vec).unwrap();
        let mut test = Vec::new();
        a.serialize(&mut test).unwrap();
        assert_eq!(vec.as_slice(), test.as_slice());
    }

    #[test]
    fn test_borsh_more_than() {
        use borsh::{io::ErrorKind, BorshDeserialize, BorshSerialize};
        #[derive(BorshSerialize, BorshDeserialize)]
        struct Something {
            a: String,
        }
        let mut b = ArrayVec::<u8, 5>::new();
        let err = Something {
            a: "Hello, world!".to_owned(),
        }
        .serialize(&mut b)
        .unwrap_err();
        assert_eq!(ErrorKind::WriteZero, err.kind());
    }
}

mod array_string {
    use super::{assert_roundtrip, assert_ser};
    use arrayvec::ArrayString;

    #[test]
    fn test_empty() {
        let string = ArrayString::<0>::new();
        assert_ser(&string, b"\0\0\0\0");
        assert_roundtrip(&string);
    }

    #[test]
    fn test_full() {
        let string = ArrayString::from_byte_string(b"hello world").unwrap();
        assert_ser(&string, b"\x0b\0\0\0hello world");
        assert_roundtrip(&string);
    }

    #[test]
    fn test_with_free_capacity() {
        let string = ArrayString::<16>::from("hello world").unwrap();
        assert_ser(&string, b"\x0b\0\0\0hello world");
        assert_roundtrip(&string);
    }
}
