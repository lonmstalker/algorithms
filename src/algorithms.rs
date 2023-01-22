pub mod md5 {
    use crate::structure::Algorithm;

    /**
    L' = 512 * N + 448,
        where L' - input bytes stream
     */
    const MD5_MOD_EQ: u16 = 448;
    const MD5_MODULE: u16 = 512;

    pub struct MD5 {
        pub(crate) salt: [u16],
    }

    impl Algorithm for MD5 {
        fn encode(value: &[u16]) -> Box<[u16]> {
            let bit_counts = value.len() * u16::BITS as usize;
            let mut bytes: Vec<u8> = Vec::with_capacity(value.len() * 2);

            for two_bytes in value.iter() {
                let split_bytes = two_bytes.to_be_bytes();
                bytes.push(split_bytes[0]);
                bytes.push(split_bytes[1]);
            }

            todo!()
        }

        fn decode(value: &[u16]) -> Box<[u16]> {
            todo!()
        }
    }
}