use super::*;
#[derive(Clone, Data, Debug, New)]
pub struct Skipped {
    pub(crate) name: String,
    #[new(skip)]
    #[get(type(copy))]
    pub(crate) schema: u8,
}

#[derive(Clone, Data, Debug, New)]
pub struct NoSkip {
    pub(crate) name: String,
    #[get(type(copy))]
    pub(crate) count: u8,
}
