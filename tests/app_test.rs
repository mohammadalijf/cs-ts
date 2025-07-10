use cs_ts::app::App;
use cs_ts::args::Args;
use cs_ts::region::Region;

#[test]
fn test_app_initialization_with_valid_credentials() {
    let args = Args {
        region: Region::NorthAmerica,
        api_key: "test_key".to_string(),
        access_token: "test_token".to_string(),
        output: None,
        prefix: None,
        postfix: None,
    };

    let app = App::new(args);
    assert!(app.is_ok());
}

#[test]
fn test_app_initialization_with_empty_api_key() {
    let args = Args {
        region: Region::NorthAmerica,
        api_key: "".to_string(),
        access_token: "test_token".to_string(),
        output: None,
        prefix: None,
        postfix: None,
    };

    let app = App::new(args);
    assert!(app.is_err());
}

#[test]
fn test_app_initialization_with_empty_access_token() {
    let args = Args {
        region: Region::NorthAmerica,
        api_key: "test_key".to_string(),
        access_token: "".to_string(),
        output: None,
        prefix: None,
        postfix: None,
    };

    let app = App::new(args);
    assert!(app.is_err());
}

#[test]
fn test_app_initialization_with_whitespace_credentials() {
    let args = Args {
        region: Region::NorthAmerica,
        api_key: "   ".to_string(),
        access_token: "   ".to_string(),
        output: None,
        prefix: None,
        postfix: None,
    };

    let app = App::new(args);
    assert!(app.is_err());
}
