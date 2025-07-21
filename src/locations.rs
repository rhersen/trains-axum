pub fn name(location_signature: String) -> String {
    if location_signature == "Sk" {
        "Skövde".to_string()
    } else if location_signature == "Tu" {
        "Tumba".to_string()
    } else if location_signature == "Tul" {
        "Tullinge".to_string()
    } else {
        location_signature
    }
}
