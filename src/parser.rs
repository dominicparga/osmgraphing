//------------------------------------------------------------------------------------------------//
// filtering

trait Filter {}

//------------------------------------------------------------------------------------------------//
// parsing

trait Parser {
    // fn apply(self, filter: &Filter) -> Self;
    fn apply(self) -> Self;
}

pub struct XmlParser {
    pub xml: String,
}

impl Default for XmlParser {
    fn default() -> XmlParser {
        return XmlParser {
            xml: String::from(r#"
                <tag1 att1 = "test">
                    <tag2><!--Test comment-->TEST</tag2>
                    <tag2>
                        Test 2
                    </tag2>
                </tag1>
            "#)
        };
    }
}

impl Parser for XmlParser {
    // fn apply(self, filter: &Filter) -> Self {
    fn apply(self) -> Self {
        println!("{}", self.xml);
        return self;
    }
}
