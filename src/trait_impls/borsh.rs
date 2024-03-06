#[cfg(test)]
mod test {
    use borsh::{to_vec, BorshDeserialize};

    use crate::UncGas;

    #[test]
    fn borsh() {
        fn test_borsh_ser(val: u64, expected_serialized_value: [u8; 8]) {
            let gas = UncGas::from_gas(val);
            let ser = to_vec(&gas).unwrap();
            // println!("{:?}", ser);
            assert_eq!(expected_serialized_value, ser.as_slice());
            let de: UncGas = UncGas::try_from_slice(&ser).unwrap();
            assert_eq!(de.as_gas(), val);
        }

        test_borsh_ser(u64::MAX, [255, 255, 255, 255, 255, 255, 255, 255]);
        test_borsh_ser(8, [8, 0, 0, 0, 0, 0, 0, 0]);
        test_borsh_ser(0, [0, 0, 0, 0, 0, 0, 0, 0]);
    }
}
