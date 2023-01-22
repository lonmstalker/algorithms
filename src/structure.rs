
pub trait Algorithm {
    fn encode(value: &[u16]) -> Box<[u16]>;
    fn decode(value: &[u16]) -> Box<[u16]>;
}