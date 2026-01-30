//! Tests for document creation, parsing, navigation, and manipulation defined generically

use xrust::item::Node;
use xrust::xdmerror::Error;

pub fn document_create<N: Node, G>(make_empty_doc: G) -> Result<(), Error>
where
    G: Fn() -> N,
{
    // Setup a source document
    let _ = make_empty_doc();
    Ok(())
}
pub fn descope_1<N: Node, H>(parse_doc: H) -> Result<(), Error>
where
    H: Fn(&str) -> Result<N, Error>,
{
    let result = parse_doc(
        r#"<?xml version="1.1"?>
    <x xmlns:n1="http://www.w3.org">
        <n1:ok>namespace n1 is in scope</n1:ok>
        <x xmlns:n1="">
            <n1:error>namespace n1 has been descoped</n1:error>
            <x xmlns:n1="http://www.w3.org">
                <n1:ok>namespace n1 has been redeclared</n1:ok>
            </x>
        </x>
    </x>"#,
    );

    assert!(result.is_err());
    Ok(())
}
pub fn descope_2<N: Node, H>(parse_doc: H) -> Result<(), Error>
where
    H: Fn(&str) -> Result<N, Error>,
{
    let result = parse_doc(
        r#"<?xml version="1.1"?>
    <x xmlns:n1="http://www.w3.org">
        <n1:a/>
        <x xmlns:n1="">
            <a/>
            <x xmlns:n1="http://www.w3.org">
                <n1:a/>
            </x>
        </x>
    </x>"#,
    );

    assert!(result.is_ok());
    Ok(())
}
