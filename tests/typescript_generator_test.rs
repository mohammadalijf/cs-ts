use cs_ts::generator::typescript::TypescriptGenerator;
use cs_ts::generator::{FieldConfig, Generator};

#[test]
fn test_generate_text() {
    let generator = TypescriptGenerator::new(None, None);
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: false,
            description: None,
            enumeration: None,
            ..Default::default()
        }),
        "title: string;"
    );
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: true,
            description: None,
            enumeration: None,
            ..Default::default()
        }),
        "title?: string;"
    );
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: false,
            description: Some(String::from("Document")),
            enumeration: None,
            ..Default::default()
        }),
        "/** Document */\ntitle: string;"
    );
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: true,
            description: Some(String::from("Document")),
            enumeration: None,
            ..Default::default()
        }),
        "/** Document */\ntitle?: string;"
    );

    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: false,
            description: None,
            enumeration: Some(vec![String::from("one"), String::from("two")]),
            ..Default::default()
        }),
        "title: 'one' | 'two';"
    );

    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: false,
            description: None,
            enumeration: Some(vec![String::from("one")]),
            ..Default::default()
        }),
        "title: 'one';"
    );
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: true,
            description: None,
            enumeration: Some(vec![String::from("one"), String::from("two")]),
            ..Default::default()
        }),
        "title?: 'one' | 'two';"
    );
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: false,
            description: Some(String::from("Document")),
            enumeration: Some(vec![String::from("one"), String::from("two")]),
            ..Default::default()
        }),
        "/** Document */\ntitle: 'one' | 'two';"
    );
    assert_eq!(
        generator.generate_text(&FieldConfig {
            name: String::from("title"),
            optional: true,
            description: Some(String::from("Document")),
            enumeration: Some(vec![String::from("one"), String::from("two")]),
            ..Default::default()
        }),
        "/** Document */\ntitle?: 'one' | 'two';"
    );
}
