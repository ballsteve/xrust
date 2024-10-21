use std::fs;
use xrust::parser::xml;
use xrust::trees::smite::RNode;
use xrust::Node;

#[test]
fn serializer_issue_98() {
    /*
        Github issue number 98
        We wish to have XML documents output attributes in some stable order for test purposes.
        IMPORTANT NOTE: We will be stable for a particular version, but XML itself does not care
        about attribute order. We may switch the ordering between versions if we find a technical
        reason to do so.
    */

    let data = fs::read_to_string("tests/xml/issue-98.xml").unwrap();
    let mut prev_xml_output = None;

    for iteration in 0..100 {
        let doc = xml::parse(RNode::new_document(), data.clone().as_str(), None).unwrap();
        let xml_output = doc.to_xml();
        if let Some(prev_xml_output) = &prev_xml_output {
            assert_eq!(&xml_output, prev_xml_output, "Failed on run {}", iteration);
        }
        prev_xml_output = Some(xml_output);
    }
}

#[test]
fn serializer_1() {
    /*
        Testing the XML output, simple document.
    */

    let data = "<doc><child/></doc>";

    let doc = xml::parse(RNode::new_document(), data, None).unwrap();
    let xml_output = doc.to_xml();

    /*
       Note, xRust currently does not output self closing tags, if it does you'll need to update
       this test with

       assert_eq!(xml_output, "<doc><child/></doc>");
    */
    assert_eq!(xml_output, "<doc><child></child></doc>");
}

#[test]
fn serializer_2() {
    /*
        Testing the XML output, with some namespaces.
    */

    let data = "<doc xmlns='ns1'><child xmlns='ns2'/></doc>";

    let doc = xml::parse(RNode::new_document(), data, None).unwrap();
    let xml_output = doc.to_xml();

    /*
       Note, xRust currently does not output self closing tags, if it does you'll need to update
       this test with

       assert_eq!(xml_output, "<doc xmlns='ns1'><child xmlns='ns2'/></doc>");
    */
    assert_eq!(
        xml_output,
        "<doc xmlns='ns1'><child xmlns='ns2'></child></doc>"
    );
}

#[test]
fn serializer_3() {
    /*
        Testing the XML output, with some namespace aliases.
    */

    let data = "<a:doc xmlns:a='ns1'><a:child xmlns:a='ns2'/></a:doc>";

    let doc = xml::parse(RNode::new_document(), data, None).unwrap();
    let xml_output = doc.to_xml();

    /*
       Note, xRust currently does not output self closing tags, if it does you'll need to update
       this test with

       assert_eq!(xml_output, "<a:doc xmlns:a='ns1'><a:child xmlns:a='ns2'/></a:doc>");
    */
    assert_eq!(
        xml_output,
        "<a:doc xmlns:a='ns1'><a:child xmlns:a='ns2'></a:child></a:doc>"
    );
}

#[test]
fn serializer_4() {
    /*
        Testing the XML output, mixed content
    */

    let data = r#"<content att1='val1' xmlns:a='someothernamespace' att2='val2' xmlns='somenamespace' a:att4='val4' someatt='val5' other='valx'>
    <content2>text</content2>
    <content3/>
    <content4 xmlns='thirdnamespace' a:something='test'>text3</content4>
    <content05 xmlns:a='fourthnamespace' a:somethingelse='test2'>text4</content05>
</content>"#;

    let doc = xml::parse(RNode::new_document(), data, None).unwrap();
    let xml_output = doc.to_xml();

    /*
            Note, xRust currently does not output self closing tags, if it does you'll need to update
            this test with

        assert_eq!(xml_output, "<content xmlns='somenamespace' xmlns:a='someothernamespace' att1='val1' att2='val2' other='valx' someatt='val5' a:att4='val4'>
        <content2>text</content2>
        <content3/>
        <content4 xmlns='thirdnamespace' a:something='test'>text3</content4>
        <content05 xmlns:a='fourthnamespace' a:somethingelse='test2'>text4</content05>
    </content>");

         */
    assert_eq!(xml_output, "<content xmlns='somenamespace' xmlns:a='someothernamespace' att1='val1' att2='val2' other='valx' someatt='val5' a:att4='val4'>
    <content2>text</content2>
    <content3></content3>
    <content4 xmlns='thirdnamespace' a:something='test'>text3</content4>
    <content05 xmlns:a='fourthnamespace' a:somethingelse='test2'>text4</content05>
</content>");
}

#[test]
#[ignore]
fn serializer_5() {
    /*
        Testing the XML output, characters to be escaped
    */

    let data = "<doc attr='&apos;'>XML escape test: &lt; &gt; &amp; &apos; &quot;</doc>";

    let doc = xml::parse(RNode::new_document(), data, None).unwrap();
    let xml_output = doc.to_xml();

    assert_eq!(
        xml_output,
        "<doc attr='&apos;'>XML escape test: &lt; &gt; &amp; &apos; &quot;</doc>"
    );
}
