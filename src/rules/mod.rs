mod parser;

pub enum RuleStrategy {
    ByExt,
    ByModifiedDate,
    ByFileName,
    BySize,
}

pub struct Rule {
    pub src: String,
    pub dir_name: String,
    pub strategy: RuleStrategy,
    pub sub_rules: Vec<Rule>,
}

impl Rule {
    fn from_src(src: String) -> Self {
        Self {
            src,
            dir_name: String::from(""),
            strategy: RuleStrategy::ByExt,
            sub_rules: Vec::new(),
        }
    }
}