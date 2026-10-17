#[derive(Clone, lombok_macros::Data, Debug, lombok_macros::New)]
pub struct Skipped {
    pub name: String,
    #[new(skip)]
    #[get(type(copy))]
    pub schema: u8,
}

#[derive(Clone, lombok_macros::Data, Debug, lombok_macros::New)]
pub struct NoSkip {
    pub name: String,
    #[get(type(copy))]
    pub count: u8,
}
