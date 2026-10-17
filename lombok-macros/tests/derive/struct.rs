use super::*;
#[derive(Clone, Data, Debug, New)]
pub struct Skipped {
    pub name: String,
    #[new(skip)]
    #[get(type(copy))]
    pub schema: u8,
}

#[derive(Clone, Data, Debug, New)]
pub struct NoSkip {
    pub name: String,
    #[get(type(copy))]
    pub count: u8,
}
