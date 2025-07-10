use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContentTypeResponse {
    pub content_types: Vec<ContentType>,
}

#[derive(Debug, Deserialize)]
pub struct GlobalFieldResponse {
    pub global_fields: Vec<GlobalField>,
}

#[derive(Debug, Deserialize)]
pub struct GlobalField {
    pub uid: String,
    pub description: Option<String>,
    pub schema: Vec<Schema>,
}

#[derive(Debug, Deserialize)]
pub struct ContentType {
    pub uid: String,
    pub description: Option<String>,
    pub schema: Vec<Schema>,
}

#[derive(Debug, Deserialize)]
pub enum DataType {
    #[serde(rename = "text")]
    Text,
    #[serde(rename = "number")]
    Number,
    #[serde(rename = "file")]
    File,
    #[serde(rename = "reference")]
    Reference,
    #[serde(rename = "isodate")]
    Date,
    #[serde(rename = "boolean")]
    Boolean,
    #[serde(rename = "blocks")]
    Blocks,
    #[serde(rename = "group")]
    Group,
    #[serde(rename = "global_field")]
    GlobalField,
    #[serde(rename = "link")]
    Link,
    #[serde(rename = "json")]
    Json,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ReferenceTo {
    String(String),
    Vec(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct Schema {
    pub uid: String,
    pub data_type: DataType,
    pub schema: Option<Vec<Schema>>,
    pub multiple: bool,
    pub mandatory: bool,
    pub field_metadata: Option<FieldMetaData>,
    pub reference_to: Option<ReferenceTo>,
    pub blocks: Option<Vec<Block>>,

    #[serde(rename = "enum")]
    pub enumuration: Option<Enumeration>,
}

#[derive(Debug, Deserialize)]
pub struct Enumeration {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub value: ChoiceValue,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ChoiceValue {
    String(String),
    Number(i32),
}

#[derive(Debug, Deserialize)]
pub struct FieldMetaData {
    pub description: Option<String>,
    pub ref_multiple: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct Block {
    pub uid: String,
    pub schema: Vec<Schema>,
}
