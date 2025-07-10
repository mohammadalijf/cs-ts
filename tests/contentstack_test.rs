use cs_ts::contentstack::{Client, ContentstackError};
use cs_ts::region::Region;

#[test]
fn test_client_creation_with_valid_credentials() {
    let client = Client::new(Region::NorthAmerica, "test_key", "test_token");
    assert!(client.is_ok());
}

#[test]
fn test_client_creation_with_empty_api_key() {
    let client = Client::new(Region::NorthAmerica, "", "test_token");
    assert!(client.is_err());

    if let Err(ContentstackError::InvalidCredentials { field }) = client {
        assert_eq!(field, "api_key");
    } else {
        panic!("Expected InvalidCredentials error for api_key");
    }
}

#[test]
fn test_client_creation_with_empty_access_token() {
    let client = Client::new(Region::NorthAmerica, "test_key", "");
    assert!(client.is_err());

    if let Err(ContentstackError::InvalidCredentials { field }) = client {
        assert_eq!(field, "access_token");
    } else {
        panic!("Expected InvalidCredentials error for access_token");
    }
}

#[test]
fn test_client_creation_with_whitespace_api_key() {
    let client = Client::new(Region::NorthAmerica, "   ", "test_token");
    assert!(client.is_err());

    if let Err(ContentstackError::InvalidCredentials { field }) = client {
        assert_eq!(field, "api_key");
    } else {
        panic!("Expected InvalidCredentials error for api_key");
    }
}

#[test]
fn test_client_creation_with_whitespace_access_token() {
    let client = Client::new(Region::NorthAmerica, "test_key", "   ");
    assert!(client.is_err());

    if let Err(ContentstackError::InvalidCredentials { field }) = client {
        assert_eq!(field, "access_token");
    } else {
        panic!("Expected InvalidCredentials error for access_token");
    }
}

#[test]
fn test_client_creation_with_different_regions() {
    let na_client = Client::new(Region::NorthAmerica, "test_key", "test_token");
    let eu_client = Client::new(Region::Europe, "test_key", "test_token");
    let au_client = Client::new(Region::Australia, "test_key", "test_token");

    assert!(na_client.is_ok());
    assert!(eu_client.is_ok());
    assert!(au_client.is_ok());
}

#[test]
fn test_contentstack_error_display() {
    let error = ContentstackError::InvalidCredentials {
        field: "api_key".to_string(),
    };
    assert_eq!(error.to_string(), "Invalid credentials: api_key");

    let error = ContentstackError::Api {
        status: 404,
        message: "Not found".to_string(),
    };
    assert_eq!(error.to_string(), "API error: 404 - Not found");
}
