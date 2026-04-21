#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Toolzard {
    GlassFinder,
    PassportMaker,
}

impl Toolzard {
    // metadata is just hardcoded here - fast and easy!
    pub fn name(&self) -> &'static str {
        match self {
            Self::GlassFinder => "Glass Finder",
            Self::PassportMaker => "Passport Maker",
        }
    }
}