use crate::generator::{Field, FieldConfig, GenerateConfig, Generator};

pub struct TypescriptGenerator {
    prefix: String,
    postfix: String,
}

impl TypescriptGenerator {
    fn get_type_name(&self, name: &str) -> String {
        self.to_pascal_case(name)
    }

    fn get_mulitple_marker(&self, multiple: bool, content_type: &str) -> String {
        if multiple {
            format!("Array<{content_type}>")
        } else {
            String::from(content_type)
        }
    }

    fn to_pascal_case(&self, s: &str) -> String {
        // Convert any case to PascalCase
        s.split(&['-', '_', ' '])
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => {
                        first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase()
                    }
                }
            })
            .collect::<Vec<String>>()
            .join("")
    }

    fn get_jsdoc(&self, s: Option<&str>) -> String {
        match s {
            Some(docs) => {
                if docs.trim() != "" {
                    format!("/** {docs} */\n")
                } else {
                    String::new()
                }
            }
            None => String::new(),
        }
    }

    pub fn new(prefix: Option<&str>, postfix: Option<&str>) -> Self {
        Self {
            prefix: prefix.unwrap_or("").to_string(),
            postfix: postfix.unwrap_or("").to_string(),
        }
    }

    fn _generate_field_type(&self, config: &FieldConfig) -> String {
        let docs = self.get_jsdoc(config.description.as_deref());
        let optional_marker = if config.optional { "?" } else { "" };
        let field_name = &config.name;

        format!("{docs}{field_name}{optional_marker}: ")
    }
}

impl Generator for TypescriptGenerator {
    fn generate_text(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let text_type = config
            .enumeration
            .as_ref()
            .map_or(String::from("string"), |choices| {
                choices
                    .iter()
                    .map(|choice| format!("'{choice}'"))
                    .collect::<Vec<String>>()
                    .join(" | ")
            });
        let multiple_text_type = self.get_mulitple_marker(config.multiple, &text_type);
        format!("{base}{multiple_text_type};")
    }

    fn generate_number(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let number_type = config
            .enumeration
            .as_ref()
            .map_or(String::from("number"), |choices| choices.join(" | "));
        let multiple_number_type = self.get_mulitple_marker(config.multiple, &number_type);
        format!("{base}{multiple_number_type};",)
    }

    fn generate_date(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let date_type = self.get_mulitple_marker(config.multiple, "string");
        format!("{base}{date_type};",)
    }

    fn generate_bool(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let boolean_type = self.get_mulitple_marker(config.multiple, "boolean");
        format!("{base}{boolean_type};")
    }

    fn generate_file(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let file_type = self.get_mulitple_marker(config.multiple, "ContentstackFile");
        format!("{base}{file_type};")
    }

    fn generate_link(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let link_type = self.get_mulitple_marker(config.multiple, "ContentstackLink");
        format!("{base}{link_type};")
    }

    fn generate_json(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let json_type = self.get_mulitple_marker(config.multiple, "ContentstackJson");
        format!("{base}{json_type};")
    }

    fn generate_reference(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let any_type = self.get_mulitple_marker(config.multiple, "any");
        if config.reference_to.is_empty() {
            format!("{base}{any_type};")
        } else {
            let types = config
                .reference_to
                .iter()
                .map(|ref_type| {
                    format!(
                        "{} | {{ _content_type_uid: '{ref_type}'; uid: string; }}",
                        self.get_type_name(ref_type),
                    )
                })
                .collect::<Vec<String>>()
                .join(" | ");
            let multiple_types = self.get_mulitple_marker(config.multiple, &types);
            format!("{base}{multiple_types};")
        }
    }

    fn generate_global_field(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let any_type = self.get_mulitple_marker(config.multiple, "any");
        if config.reference_to.is_empty() {
            format!("{base}{any_type};")
        } else {
            let types = config
                .reference_to
                .iter()
                .map(|ref_type| self.get_type_name(ref_type))
                .collect::<Vec<String>>()
                .join(" | ");
            let multiple_types = self.get_mulitple_marker(config.multiple, &types);
            format!("{base}{multiple_types};")
        }
    }

    fn generate_blocks(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        if let Some(block_types) = &config.block_types {
            if block_types.is_empty() {
                format!("{base}Array<any>;")
            } else {
                // Generate object type with block uids as keys
                let mut block_definitions = String::new();
                for (block_uid, fields) in block_types {
                    let mut block_fields = String::new();
                    for field in fields {
                        let field_code = match field {
                            Field::Text(config) => self.generate_text(config),
                            Field::Number(config) => self.generate_number(config),
                            Field::Date(config) => self.generate_date(config),
                            Field::Bool(config) => self.generate_bool(config),
                            Field::File(config) => self.generate_file(config),
                            Field::Link(config) => self.generate_link(config),
                            Field::Json(config) => self.generate_json(config),
                            Field::Reference(config) => self.generate_reference(config),
                            Field::Global(config) => self.generate_global_field(config),
                            Field::Blocks(config) => self.generate_blocks(config),
                            Field::Group(config) => self.generate_group(config),
                        };

                        // Add proper indentation for nested fields
                        for line in field_code.lines() {
                            block_fields.push_str(&format!("    {line}\n"));
                        }
                    }
                    block_definitions
                        .push_str(&format!("  {block_uid}: {{\n{block_fields}  }};\n"));
                }
                format!("{base}Array<{{\n{block_definitions}}}>;")
            }
        } else {
            format!("{base}Array<any>;")
        }
    }

    fn generate_group(&self, config: &FieldConfig) -> String {
        let base = self._generate_field_type(config);
        let any_type = self.get_mulitple_marker(config.multiple, "any");
        if config.fields.is_empty() {
            format!("{base}{any_type};")
        } else {
            // Generate inline object type instead of referencing a separate type
            let mut group_fields = String::new();
            for field in &config.fields {
                let field_code = match field {
                    Field::Text(config) => self.generate_text(config),
                    Field::Number(config) => self.generate_number(config),
                    Field::Date(config) => self.generate_date(config),
                    Field::Bool(config) => self.generate_bool(config),
                    Field::File(config) => self.generate_file(config),
                    Field::Link(config) => self.generate_link(config),
                    Field::Json(config) => self.generate_json(config),
                    Field::Reference(config) => self.generate_reference(config),
                    Field::Global(config) => self.generate_global_field(config),
                    Field::Blocks(config) => self.generate_blocks(config),
                    Field::Group(config) => self.generate_group(config),
                };

                // Add proper indentation for nested fields
                for line in field_code.lines() {
                    group_fields.push_str(&format!("  {line}\n"));
                }
            }
            let multiple_types =
                self.get_mulitple_marker(config.multiple, &format!("{{\n{group_fields}}}"));
            format!("{base}{multiple_types}")
        }
    }

    fn generate_interface(&self, config: &GenerateConfig) -> String {
        let docs = self.get_jsdoc(config.description.as_deref());
        let type_name = format!(
            "{}{}{}",
            self.prefix,
            self.get_type_name(&config.name),
            self.postfix
        );

        let mut interface = format!("{docs}export interface {type_name} {{\n");

        interface.push_str(&format!("  _content_type_uid: '{}'\n", &config.name));
        for field in &config.fields {
            let field_code = match field {
                Field::Text(config) => self.generate_text(config),
                Field::Number(config) => self.generate_number(config),
                Field::Date(config) => self.generate_date(config),
                Field::Bool(config) => self.generate_bool(config),
                Field::File(config) => self.generate_file(config),
                Field::Link(config) => self.generate_link(config),
                Field::Json(config) => self.generate_json(config),
                Field::Reference(config) => self.generate_reference(config),
                Field::Global(config) => self.generate_global_field(config),
                Field::Blocks(config) => self.generate_blocks(config),
                Field::Group(config) => self.generate_group(config),
            };

            for line in field_code.lines() {
                interface.push_str(&format!("  {line}\n"));
            }
        }

        interface.push('}');
        interface
    }

    fn get_builtins(&self) -> String {
        "export interface ContentstackFile {
  uid: string;
  created_at: string;
  updated_at: string;
  created_by: string;
  updated_by: string;
  content_type: string;
  file_size: string;
  tags: string[];
  filename: string;
  url: string;
  ACL: any[] | object;
  is_dir: boolean;
  parent_uid: string;
  _version: number;
  title: string;
  _metadata?: object;
  description?: string;
  dimension?: {
    height: number;
    width: number;
  };
  publish_details: PublishDetails;
}

export interface PublishDetails {
  environment: string;
  locale: string;
  time: string;
  user: string;
}

export interface ContentstackLink {
  title: string;
  href: string;
}

export interface ContentstackJson {
  [key: string]: any;
}
"
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case_snake_case() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("snake_case"), "SnakeCase");
        assert_eq!(generator.to_pascal_case("user_profile"), "UserProfile");
        assert_eq!(generator.to_pascal_case("api_key_config"), "ApiKeyConfig");
    }

    #[test]
    fn test_to_pascal_case_kebab_case() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("kebab-case"), "KebabCase");
        assert_eq!(generator.to_pascal_case("user-profile"), "UserProfile");
        assert_eq!(generator.to_pascal_case("api-key-config"), "ApiKeyConfig");
    }

    #[test]
    fn test_to_pascal_case_space_separated() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("space case"), "SpaceCase");
        assert_eq!(generator.to_pascal_case("user profile"), "UserProfile");
        assert_eq!(generator.to_pascal_case("api key config"), "ApiKeyConfig");
    }

    #[test]
    fn test_to_pascal_case_mixed_separators() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("mixed-case_test"), "MixedCaseTest");
        assert_eq!(
            generator.to_pascal_case("api-key_config test"),
            "ApiKeyConfigTest"
        );
    }

    #[test]
    fn test_to_pascal_case_single_word() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("user"), "User");
        assert_eq!(generator.to_pascal_case("test"), "Test");
        assert_eq!(generator.to_pascal_case("API"), "Api");
    }

    #[test]
    fn test_to_pascal_case_empty_string() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case(""), "");
    }

    #[test]
    fn test_to_pascal_case_multiple_separators() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("test__case"), "TestCase");
        assert_eq!(generator.to_pascal_case("test--case"), "TestCase");
        assert_eq!(generator.to_pascal_case("test  case"), "TestCase");
    }

    #[test]
    fn test_to_pascal_case_leading_trailing_separators() {
        let generator = TypescriptGenerator::new(None, None);
        assert_eq!(generator.to_pascal_case("_test_case_"), "TestCase");
        assert_eq!(generator.to_pascal_case("-test-case-"), "TestCase");
        assert_eq!(generator.to_pascal_case(" test case "), "TestCase");
    }
}
