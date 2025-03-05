use std::collections::HashMap;

/// fills metadata according to the HTTP status code, the description and metadata of the request.
///
/// # arguments
///
/// * `code` - the HTTP status code.
/// * `Description` - The description associated with the status code.
/// * `Request_Metadata` - Optional metadata of the request in the form of a hashmap`.
///
/// # returns
///
/// a hashmap 'containing the metadata, including the description, if it is a mistake, and the family of status.
///
/// # Example
///
/// ```rust
/// use std::collections::HashMap;
/// use simbld_http::utils::populate_metadata::populate_metadata;
///
/// let code = 404;
/// let description = "Not Found";
/// let request_metadata = Some(HashMap::from([("method", "GET"), ("url", "https://example.com")]));
/// let metadata = populate_metadata(code, description, request_metadata);
/// println!("{:?}", metadata);
/// ```
pub fn populate_metadata(
    code: u16,
    description: &str,
    request_metadata: Option<HashMap<&str, &str>>,
) -> HashMap<String, String> {
    let mut metadata = HashMap::new();

    metadata.insert("description".to_string(), description.to_string());
    metadata.insert("is_error".to_string(), (code >= 400).to_string());
    metadata.insert(
        "status_family".to_string(),
        match code {
            100..=199 => "Informational",
            200..=299 => "Success",
            300..=399 => "Redirection",
            400..=499 => "Client Error",
            500..=599 => "Server Error",
            600..=699 => "Service Error",
            700..=799 => "Crawler Error",
            900..=999 => "Local API Error",
            _ => "Unknown",
        }
        .to_string(),
    );

    if let Some(req_meta) = request_metadata {
        for (key, value) in req_meta {
            metadata.insert((*key).to_string(), (*value).to_string());
        }
    }

    metadata
}
