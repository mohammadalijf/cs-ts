use cs_ts::contentstack::ContentstackError;
use cs_ts::error::Error;
use std::error::Error as StdError;

#[test]
fn test_contentstack_error_from_json_error() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let contentstack_error = ContentstackError::from(json_error);

    match contentstack_error {
        ContentstackError::Json(_) => {}
        _ => panic!("Expected Json error"),
    }
}

#[test]
fn test_contentstack_error_from_io_error() {
    let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
    let contentstack_error = ContentstackError::from(io_error);

    match contentstack_error {
        ContentstackError::Io(_) => {}
        _ => panic!("Expected Io error"),
    }
}

#[test]
fn test_contentstack_error_display_messages() {
    let error = ContentstackError::InvalidCredentials {
        field: "api_key".to_string(),
    };
    assert!(error.to_string().contains("Invalid credentials"));
    assert!(error.to_string().contains("api_key"));

    let error = ContentstackError::Api {
        status: 401,
        message: "Unauthorized".to_string(),
    };
    assert!(error.to_string().contains("API error"));
    assert!(error.to_string().contains("401"));
    assert!(error.to_string().contains("Unauthorized"));
}

#[test]
fn test_error_from_contentstack_error() {
    let contentstack_error = ContentstackError::InvalidCredentials {
        field: "api_key".to_string(),
    };
    let error = Error::from(contentstack_error);

    match error {
        Error::Contentstack(_) => {}
        _ => panic!("Expected Contentstack error"),
    }
}

#[test]
fn test_error_display() {
    let contentstack_error = ContentstackError::InvalidCredentials {
        field: "api_key".to_string(),
    };
    let error = Error::from(contentstack_error);

    let error_string = error.to_string();
    assert!(error_string.contains("Invalid credentials"));
    assert!(error_string.contains("api_key"));
}

#[test]
fn test_error_source_chain() {
    let json_error = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
    let contentstack_error = ContentstackError::from(json_error);

    // Test that the error source chain is preserved
    assert!(StdError::source(&contentstack_error).is_some());

    let error = Error::from(contentstack_error);
    assert!(StdError::source(&error).is_some());
}
