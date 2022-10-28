pub struct StreamFrame {
    pub id: u64,
    pub offset: Option<usize>,
    pub length: Option<usize>,
    pub data: Vec<u8>
}