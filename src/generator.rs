use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Generator<'a> {
    pub rules: &'a str,
    pub components: HashMap<&'a str, &'a [&'a str]>,
}

impl<'a> Generator<'a> {
    pub fn new(rules: &'a str, components: HashMap<&'a str, &'a [&'a str]>) -> Self {
        if components.is_empty() {
            panic!("Generator must contain components!")
        }

        if rules == "" {
            panic!("Generator must have rules!")
        }

        Generator{rules: rules, components: components}
    }
}
