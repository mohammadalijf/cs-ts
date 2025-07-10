use crate::args::Args;
use crate::contentstack::{Client, ContentTypeResponse, GlobalFieldResponse};
use crate::error::Result;
use crate::generator::{Field, GenerateConfig, Generator, TypescriptGenerator};
use rayon::prelude::*;
use std::fs;

pub struct App {
    client: Client,
    args: Args,
    generator: TypescriptGenerator,
}

impl App {
    pub fn new(args: Args) -> Result<Self> {
        let client = Client::new(args.region.clone(), &args.api_key, &args.access_token)?;
        let generator = TypescriptGenerator::new(args.prefix.as_deref(), args.postfix.as_deref());

        Ok(Self {
            client,
            args,
            generator,
        })
    }

    pub fn run(&self) -> Result<()> {
        let content_types = self.client.fetch_content_types()?;
        let global_fields = self.client.fetch_global_fields()?;

        let typescript_code = self.generate_typescript(&content_types, &global_fields)?;

        self.output_result(&typescript_code)?;

        Ok(())
    }

    fn generate_typescript(
        &self,
        content_types: &ContentTypeResponse,
        global_fields: &GlobalFieldResponse,
    ) -> Result<String> {
        let mut output = String::new();

        // Add builtin types at the beginning
        output.push_str(&self.generator.get_builtins());
        output.push('\n');

        // Generate global field types in parallel
        let global_interfaces: std::result::Result<Vec<String>, crate::error::Error> =
            global_fields
                .global_fields
                .par_iter()
                .map(|global_field| {
                    let fields = self.convert_schema_to_fields(&global_field.schema)?;
                    let config = GenerateConfig {
                        name: global_field.uid.clone(),
                        description: global_field.description.clone(),
                        fields,
                    };
                    let interface = self.generator.generate_interface(&config);
                    Ok(interface)
                })
                .collect();

        for interface in global_interfaces? {
            output.push_str(&interface);
            output.push('\n');
        }

        // Generate content type interfaces in parallel
        let content_interfaces: std::result::Result<Vec<String>, crate::error::Error> =
            content_types
                .content_types
                .par_iter()
                .map(|content_type| {
                    let fields = self.convert_schema_to_fields(&content_type.schema)?;
                    let config = GenerateConfig {
                        name: content_type.uid.clone(),
                        description: content_type.description.clone(),
                        fields,
                    };
                    let interface = self.generator.generate_interface(&config);
                    Ok(interface)
                })
                .collect();

        for interface in content_interfaces? {
            output.push_str(&interface);
            output.push('\n');
        }

        Ok(output)
    }

    fn convert_schema_to_fields(
        &self,
        schema: &[crate::contentstack::response::Schema],
    ) -> Result<Vec<Field>> {
        let mut fields = Vec::new();

        for field in schema {
            let converted_field = self.convert_field(field)?;
            fields.push(converted_field);
        }

        Ok(fields)
    }

    pub fn convert_field(&self, field: &crate::contentstack::response::Schema) -> Result<Field> {
        use crate::contentstack::response::DataType;
        use crate::generator::*;

        let name = field.uid.clone();
        let optional = !field.mandatory;
        let multiple = field.multiple
            || field
                .field_metadata
                .as_ref()
                .and_then(|m| m.ref_multiple)
                .unwrap_or(false);
        let description = field
            .field_metadata
            .as_ref()
            .and_then(|m| m.description.clone());

        let converted_field = match field.data_type {
            DataType::Text => {
                let enumeration = field.enumuration.as_ref().map(|e| {
                    e.choices
                        .iter()
                        .map(|c| match &c.value {
                            crate::contentstack::response::ChoiceValue::String(s) => s.clone(),
                            crate::contentstack::response::ChoiceValue::Number(n) => n.to_string(),
                        })
                        .collect()
                });
                Field::Text(FieldConfig {
                    name,
                    optional,
                    description,
                    enumeration,
                    multiple,
                    ..Default::default()
                })
            }
            DataType::Number => {
                let enumeration = field.enumuration.as_ref().map(|e| {
                    e.choices
                        .iter()
                        .map(|c| match &c.value {
                            crate::contentstack::response::ChoiceValue::String(s) => s.clone(),
                            crate::contentstack::response::ChoiceValue::Number(n) => n.to_string(),
                        })
                        .collect()
                });
                Field::Number(FieldConfig {
                    name,
                    optional,
                    description,
                    enumeration,
                    multiple,
                    ..Default::default()
                })
            }
            DataType::Boolean => Field::Bool(FieldConfig {
                name,
                optional,
                description,
                multiple,
                ..Default::default()
            }),
            DataType::Date => Field::Date(FieldConfig {
                name,
                optional,
                description,
                multiple,
                ..Default::default()
            }),
            DataType::File => Field::File(FieldConfig {
                name,
                optional,
                description,
                multiple,
                ..Default::default()
            }),
            DataType::Link => Field::Link(FieldConfig {
                name,
                optional,
                description,
                multiple,
                ..Default::default()
            }),
            DataType::Json => Field::Json(FieldConfig {
                name,
                optional,
                description,
                multiple,
                ..Default::default()
            }),
            DataType::Reference => {
                let reference_to = match &field.reference_to {
                    Some(crate::contentstack::response::ReferenceTo::String(s)) => vec![s.clone()],
                    Some(crate::contentstack::response::ReferenceTo::Vec(v)) => v.clone(),
                    None => vec![],
                };
                Field::Reference(FieldConfig {
                    name,
                    optional,
                    description,
                    reference_to,
                    multiple,
                    ..Default::default()
                })
            }
            DataType::GlobalField => {
                let reference_to = match &field.reference_to {
                    Some(crate::contentstack::response::ReferenceTo::String(s)) => vec![s.clone()],
                    Some(crate::contentstack::response::ReferenceTo::Vec(v)) => v.clone(),
                    None => vec![],
                };
                Field::Global(FieldConfig {
                    name,
                    optional,
                    description,
                    reference_to,
                    multiple,
                    ..Default::default()
                })
            }
            DataType::Group => {
                let fields = if let Some(nested_schema) = &field.schema {
                    self.convert_schema_to_fields(nested_schema)?
                } else {
                    vec![]
                };
                Field::Group(FieldConfig {
                    name,
                    optional,
                    description,
                    fields,
                    multiple,
                    ..Default::default()
                })
            }
            DataType::Blocks => {
                let mut block_types = Vec::new();
                if let Some(blocks) = &field.blocks {
                    for block in blocks {
                        let converted_fields = self.convert_schema_to_fields(&block.schema)?;
                        block_types.push((block.uid.clone(), converted_fields));
                    }
                }
                Field::Blocks(FieldConfig {
                    name,
                    optional,
                    description,
                    fields: vec![], // Will be handled differently in generator
                    multiple,
                    block_types: Some(block_types),
                    ..Default::default()
                })
            }
        };

        Ok(converted_field)
    }

    fn output_result(&self, content: &str) -> Result<()> {
        match &self.args.output {
            Some(path) => {
                fs::write(path, content)?;
                println!("TypeScript types written to: {path}");
            }
            None => {
                println!("{content}");
            }
        }

        Ok(())
    }
}
