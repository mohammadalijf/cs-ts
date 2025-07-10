#[derive(Debug, Default)]
pub struct FieldConfig {
    pub name: String,
    pub optional: bool,
    pub multiple: bool,
    pub description: Option<String>,
    pub enumeration: Option<Vec<String>>,
    pub reference_to: Vec<String>,
    pub fields: Vec<Field>,
    pub block_types: Option<Vec<(String, Vec<Field>)>>,
}

#[derive(Debug)]
pub struct GenerateConfig {
    pub name: String,
    pub description: Option<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug)]
pub enum Field {
    Text(FieldConfig),
    Number(FieldConfig),
    Date(FieldConfig),
    Bool(FieldConfig),
    File(FieldConfig),
    Link(FieldConfig),
    Json(FieldConfig),
    Reference(FieldConfig),
    Global(FieldConfig),
    Blocks(FieldConfig),
    Group(FieldConfig),
}

pub trait Generator {
    fn generate_text(&self, config: &FieldConfig) -> String;
    fn generate_number(&self, config: &FieldConfig) -> String;
    fn generate_date(&self, config: &FieldConfig) -> String;
    fn generate_bool(&self, config: &FieldConfig) -> String;
    fn generate_file(&self, config: &FieldConfig) -> String;
    fn generate_link(&self, config: &FieldConfig) -> String;
    fn generate_json(&self, config: &FieldConfig) -> String;
    fn generate_reference(&self, config: &FieldConfig) -> String;
    fn generate_global_field(&self, config: &FieldConfig) -> String;
    fn generate_blocks(&self, config: &FieldConfig) -> String;
    fn generate_group(&self, config: &FieldConfig) -> String;
    fn generate_interface(&self, config: &GenerateConfig) -> String;
    fn get_builtins(&self) -> String;
}

pub mod typescript;
pub use typescript::TypescriptGenerator;
