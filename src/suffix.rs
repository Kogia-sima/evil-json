pub trait Suffix {
    const SUFFIX: &'static str;
}

pub struct RootSuffix;
pub struct SeqSuffix;
pub struct MapSuffix;

impl Suffix for RootSuffix {
    const SUFFIX: &'static str = "";
}

impl Suffix for SeqSuffix {
    const SUFFIX: &'static str = ",";
}

impl Suffix for MapSuffix {
    const SUFFIX: &'static str = ",\"";
}
