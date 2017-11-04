pub mod generator;
pub mod syntax;

#[cfg(test)]
mod tests {
    use generator::Generator;
    use std::collections::HashMap;

    #[test]
    fn map_internals_accessible() {
        let mut map: HashMap<&str, &[&str]> = HashMap::new();
        map.insert("V", &["a", "e", "i", "o", "u"]);

        let jp = Generator::new("CVN", map);
        let expected = &["a", "e", "i", "o", "u"];

        assert_eq!(jp.components["V"], expected);
    }

    #[test]
    #[should_panic]
    fn components_panic() {
        Generator::new("CVN", HashMap::new());
    }

    #[test]
    #[should_panic]
    fn rules_panic() {
        let mut comps: HashMap<&str, &[&str]> = HashMap::new();
        comps.insert("V", &["a", "e", "i", "o", "u"]);
        Generator::new("", comps);
    }
}
