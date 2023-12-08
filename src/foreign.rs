use std::collections::HashMap;

// "Foreign" structs, cannot be annotated

pub struct Foo {
    pub mapping: HashMap<String, Vec<Bar>>,
}

pub struct Bar {
    pub num: i32,
    pub word: String,
}

impl Foo {
    pub fn new() -> Foo {
        let mut foo = Foo {
            mapping: HashMap::new(),
        };

        foo.mapping.insert(
            "greeting".to_string(),
            vec![
                Bar::new(2, "hello".to_string()),
                Bar::new(2, "there".to_string()),
            ],
        );
        foo.mapping.insert(
            "subject".to_string(),
            vec![Bar::new(3, "world".to_string())],
        );

        foo
    }
}

impl Bar {
    pub fn new(num: i32, word: String) -> Bar {
        Bar { num, word }
    }
}
