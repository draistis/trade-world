pub fn cn(classes: impl IntoIterator<Item = impl AsRef<str>>) -> String {
    let mut result = String::new();
    for c in classes {
        let c = c.as_ref().trim();
        if !c.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(c);
        }
    }
    // Simple dedupe of identical consecutive classes
    let mut deduped = String::new();
    for part in result.split_whitespace() {
        if !deduped.ends_with(&format!(" {part}")) {
            if !deduped.is_empty() {
                deduped.push(' ');
            }
            deduped.push_str(part);
        }
    }
    deduped
}
