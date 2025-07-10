use cs_ts::app::App;
use cs_ts::args::Args;
use cs_ts::contentstack::response::*;
use cs_ts::generator::Field;
use cs_ts::region::Region;

fn create_test_app() -> App {
    let args = Args {
        region: Region::NorthAmerica,
        api_key: "test_key".to_string(),
        access_token: "test_token".to_string(),
        output: None,
        prefix: None,
        postfix: None,
    };
    App::new(args).unwrap()
}

#[test]
fn test_convert_text_field() {
    let app = create_test_app();
    let schema = Schema {
        uid: "title".to_string(),
        data_type: DataType::Text,
        schema: None,
        multiple: false,
        mandatory: true,
        field_metadata: None,
        reference_to: None,
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Text(config) => {
            assert_eq!(config.name, "title");
            assert!(!config.optional);
            assert!(!config.multiple);
        }
        _ => panic!("Expected Text field"),
    }
}

#[test]
fn test_convert_number_field() {
    let app = create_test_app();
    let schema = Schema {
        uid: "price".to_string(),
        data_type: DataType::Number,
        schema: None,
        multiple: false,
        mandatory: false,
        field_metadata: None,
        reference_to: None,
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Number(config) => {
            assert_eq!(config.name, "price");
            assert!(config.optional);
            assert!(!config.multiple);
        }
        _ => panic!("Expected Number field"),
    }
}

#[test]
fn test_convert_boolean_field() {
    let app = create_test_app();
    let schema = Schema {
        uid: "published".to_string(),
        data_type: DataType::Boolean,
        schema: None,
        multiple: false,
        mandatory: true,
        field_metadata: None,
        reference_to: None,
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Bool(config) => {
            assert_eq!(config.name, "published");
            assert!(!config.optional);
            assert!(!config.multiple);
        }
        _ => panic!("Expected Bool field"),
    }
}

#[test]
fn test_convert_text_field_with_enumeration() {
    let app = create_test_app();
    let schema = Schema {
        uid: "status".to_string(),
        data_type: DataType::Text,
        schema: None,
        multiple: false,
        mandatory: true,
        field_metadata: None,
        reference_to: None,
        blocks: None,
        enumuration: Some(Enumeration {
            choices: vec![
                Choice {
                    value: ChoiceValue::String("published".to_string()),
                },
                Choice {
                    value: ChoiceValue::String("draft".to_string()),
                },
            ],
        }),
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Text(config) => {
            assert_eq!(config.name, "status");
            assert!(!config.optional);
            assert!(!config.multiple);
            assert!(config.enumeration.is_some());
            let enum_values = config.enumeration.unwrap();
            assert_eq!(enum_values.len(), 2);
            assert!(enum_values.contains(&"published".to_string()));
            assert!(enum_values.contains(&"draft".to_string()));
        }
        _ => panic!("Expected Text field"),
    }
}

#[test]
fn test_convert_reference_field() {
    let app = create_test_app();
    let schema = Schema {
        uid: "author".to_string(),
        data_type: DataType::Reference,
        schema: None,
        multiple: false,
        mandatory: true,
        field_metadata: None,
        reference_to: Some(ReferenceTo::String("person".to_string())),
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Reference(config) => {
            assert_eq!(config.name, "author");
            assert!(!config.optional);
            assert!(!config.multiple);
            assert_eq!(config.reference_to, vec!["person".to_string()]);
        }
        _ => panic!("Expected Reference field"),
    }
}

#[test]
fn test_convert_reference_field_with_multiple_references() {
    let app = create_test_app();
    let schema = Schema {
        uid: "tags".to_string(),
        data_type: DataType::Reference,
        schema: None,
        multiple: true,
        mandatory: false,
        field_metadata: None,
        reference_to: Some(ReferenceTo::Vec(vec![
            "tag".to_string(),
            "category".to_string(),
        ])),
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Reference(config) => {
            assert_eq!(config.name, "tags");
            assert!(config.optional);
            assert!(config.multiple);
            assert_eq!(
                config.reference_to,
                vec!["tag".to_string(), "category".to_string()]
            );
        }
        _ => panic!("Expected Reference field"),
    }
}

#[test]
fn test_convert_group_field() {
    let app = create_test_app();
    let schema = Schema {
        uid: "address".to_string(),
        data_type: DataType::Group,
        schema: Some(vec![
            Schema {
                uid: "street".to_string(),
                data_type: DataType::Text,
                schema: None,
                multiple: false,
                mandatory: true,
                field_metadata: None,
                reference_to: None,
                blocks: None,
                enumuration: None,
            },
            Schema {
                uid: "city".to_string(),
                data_type: DataType::Text,
                schema: None,
                multiple: false,
                mandatory: true,
                field_metadata: None,
                reference_to: None,
                blocks: None,
                enumuration: None,
            },
        ]),
        multiple: false,
        mandatory: true,
        field_metadata: None,
        reference_to: None,
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Group(config) => {
            assert_eq!(config.name, "address");
            assert!(!config.optional);
            assert!(!config.multiple);
            assert_eq!(config.fields.len(), 2);
        }
        _ => panic!("Expected Group field"),
    }
}

#[test]
fn test_convert_field_with_description() {
    let app = create_test_app();
    let schema = Schema {
        uid: "title".to_string(),
        data_type: DataType::Text,
        schema: None,
        multiple: false,
        mandatory: true,
        field_metadata: Some(FieldMetaData {
            description: Some("The title of the content".to_string()),
            ref_multiple: None,
        }),
        reference_to: None,
        blocks: None,
        enumuration: None,
    };

    let field = app.convert_field(&schema).unwrap();
    match field {
        Field::Text(config) => {
            assert_eq!(config.name, "title");
            assert!(!config.optional);
            assert!(!config.multiple);
            assert_eq!(
                config.description,
                Some("The title of the content".to_string())
            );
        }
        _ => panic!("Expected Text field"),
    }
}
