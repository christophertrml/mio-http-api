#[derive(Copy, Clone, Debug)]
pub struct Chat {
    pub id: u64,
    pub participant_ids: [u64; 2]
}