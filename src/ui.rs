use crate::config::AddressBookEntry;
use crate::email::escape_html;

const TEMPLATE: &str = include_str!("../templates/app.html");

pub fn render_app_html(
    from_name: &str,
    from_email: &str,
    address_book: &[AddressBookEntry],
) -> String {
    let book = serde_json::to_string(address_book).unwrap_or_else(|_| "[]".into());
    let book = book.replace('<', "\\u003c");
    TEMPLATE
        .replace("___FROM_NAME___", &escape_html(from_name))
        .replace("___FROM_EMAIL___", &escape_html(from_email))
        .replace("___ADDRESS_BOOK___", &book)
}
