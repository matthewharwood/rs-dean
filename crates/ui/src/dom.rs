pub(crate) fn ui_dom_id(prefix: &str, value: &str) -> String {
    let mut id = String::with_capacity(prefix.len() + value.len() + 1);
    id.push_str(prefix);
    id.push('-');
    let mut previous_dash = false;
    for character in value.chars() {
        if character.is_ascii_alphanumeric() {
            id.push(character.to_ascii_lowercase());
            previous_dash = false;
        } else if !previous_dash {
            id.push('-');
            previous_dash = true;
        }
    }
    while id.ends_with('-') {
        id.pop();
    }
    id
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dom_ids_are_stable_and_ascii() {
        assert_eq!(
            ui_dom_id("component-panel", "Billing & Plans"),
            "component-panel-billing-plans"
        );
    }
}
