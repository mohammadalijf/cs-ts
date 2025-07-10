pub mod response;

use crate::region::Region;
pub use response::{ContentTypeResponse, GlobalFieldResponse};
use std::time::Duration;

#[derive(Debug)]
pub enum ContentstackError {
    Http(Box<ureq::Error>),
    Json(serde_json::Error),
    Io(std::io::Error),
    InvalidCredentials { field: String },
    Api { status: u16, message: String },
}

impl std::fmt::Display for ContentstackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContentstackError::Http(e) => write!(f, "HTTP request failed: {e}"),
            ContentstackError::Json(e) => write!(f, "JSON parsing failed: {e}"),
            ContentstackError::Io(e) => write!(f, "IO error: {e}"),
            ContentstackError::InvalidCredentials { field } => {
                write!(f, "Invalid credentials: {field}",)
            }
            ContentstackError::Api { status, message } => {
                write!(f, "API error: {status} - {message}")
            }
        }
    }
}

impl std::error::Error for ContentstackError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ContentstackError::Http(e) => Some(e.as_ref()),
            ContentstackError::Json(e) => Some(e),
            ContentstackError::Io(e) => Some(e),
            _ => None,
        }
    }
}

impl From<ureq::Error> for ContentstackError {
    fn from(error: ureq::Error) -> Self {
        ContentstackError::Http(Box::new(error))
    }
}

impl From<serde_json::Error> for ContentstackError {
    fn from(error: serde_json::Error) -> Self {
        ContentstackError::Json(error)
    }
}

impl From<std::io::Error> for ContentstackError {
    fn from(error: std::io::Error) -> Self {
        ContentstackError::Io(error)
    }
}

type Result<T> = std::result::Result<T, ContentstackError>;

pub struct Client {
    agent: ureq::Agent,
    api_key: String,
    access_token: String,
    base_url: String,
}

impl Client {
    /// Creates a new Contentstack client
    ///
    /// # Arguments
    ///
    /// * `region` - The Contentstack region to connect to
    /// * `api_key` - Your Contentstack API key
    /// * `access_token` - Your Contentstack access token
    ///
    /// # Example
    ///
    /// ```
    /// use cs_ts::contentstack::{Client};
    /// use cs_ts::region::Region;
    ///
    /// let client = Client::new(Region::NorthAmerica, "your_api_key", "your_access_token");
    /// ```
    pub fn new(region: Region, api_key: &str, access_token: &str) -> Result<Self> {
        if api_key.trim().is_empty() {
            return Err(ContentstackError::InvalidCredentials {
                field: "api_key".to_string(),
            });
        }

        if access_token.trim().is_empty() {
            return Err(ContentstackError::InvalidCredentials {
                field: "access_token".to_string(),
            });
        }

        let agent = ureq::AgentBuilder::new()
            .timeout(Duration::from_secs(30))
            .build();

        Ok(Self {
            agent,
            api_key: api_key.trim().to_string(),
            access_token: access_token.trim().to_string(),
            base_url: match region {
                Region::Europe => String::from("https://eu-cdn.contentstack.com"),
                Region::NorthAmerica => String::from("https://cdn.contentstack.io"),
                Region::Australia => String::from("https://au-cdn.contentstack.com"),
            },
        })
    }

    /// Fetches all content types from Contentstack
    ///
    /// # Returns
    ///
    /// A `Result` containing the content types response or an error
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cs_ts::contentstack::Client;
    /// # use cs_ts::region::Region;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new(Region::NorthAmerica, "key", "token")?;
    /// let content_types = client.fetch_content_types()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fetch_content_types(&self) -> Result<ContentTypeResponse> {
        let url = format!("{}/v3/content_types", &self.base_url);
        let response = self
            .agent
            .get(&url)
            .set("api_key", &self.api_key)
            .set("access_token", &self.access_token)
            .call();

        match response {
            Ok(resp) => {
                let text = resp.into_string()?;
                let data: ContentTypeResponse = serde_json::from_str(&text)?;
                Ok(data)
            }
            Err(ureq::Error::Status(status, resp)) => {
                let message = resp
                    .into_string()
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(ContentstackError::Api { status, message })
            }
            Err(e) => Err(e.into()),
        }
    }

    /// Fetches all global fields from Contentstack
    ///
    /// # Returns
    ///
    /// A `Result` containing the global fields response or an error
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use cs_ts::contentstack::Client;
    /// # use cs_ts::region::Region;
    /// # fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = Client::new(Region::NorthAmerica, "key", "token")?;
    /// let global_fields = client.fetch_global_fields()?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn fetch_global_fields(&self) -> Result<GlobalFieldResponse> {
        let url = format!(
            "{}/v3/global_fields?include_global_field_schema=true",
            &self.base_url
        );
        let response = self
            .agent
            .get(&url)
            .set("api_key", &self.api_key)
            .set("access_token", &self.access_token)
            .call();

        match response {
            Ok(resp) => {
                let text = resp.into_string()?;
                let data: GlobalFieldResponse = serde_json::from_str(&text)?;
                Ok(data)
            }
            Err(ureq::Error::Status(status, resp)) => {
                let message = resp
                    .into_string()
                    .unwrap_or_else(|_| "Unknown error".to_string());
                Err(ContentstackError::Api { status, message })
            }
            Err(e) => Err(e.into()),
        }
    }
}
