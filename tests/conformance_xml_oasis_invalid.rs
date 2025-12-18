/*

OASIS/NIST test cases

*/

#[cfg(all(test, feature = "test-conformance-xml"))]
use std::fs;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::{Error, ErrorKind};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::item::Node;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder, xml};
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::trees::smite::RNode;
#[cfg(all(test, feature = "test-conformance-xml"))]
use xrust::validators::Schema;

#[cfg(all(test, feature = "test-conformance-xml"))]
fn test_oasis_invalid(xmldoc: &str) {
    let ss = StaticStateBuilder::new()
        .dtd_resolver(dtdfileresolve())
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let testxml = RNode::new_document();
    let ps = ParserStateBuilder::new()
        .doc(testxml)
        .document_location("tests/conformance/xml/xmlconf/oasis/".to_string())
        .build();
    let parseresult = xml::parse_with_state(xmldoc, ps, ss);

    assert!(parseresult.is_ok());

    let doc = parseresult.unwrap();
    let validation = doc.validate(Schema::DTD);
    assert!(validation.is_err());
}
#[cfg(all(test, feature = "test-conformance-xml"))]
fn dtdfileresolve() -> fn(Option<String>, String) -> Result<String, Error> {
    move |locdir, uri| {
        let u = match locdir {
            None => uri,
            Some(ld) => ld + uri.as_str(),
        };
        match fs::read_to_string(u) {
            Err(_) => Err(Error::new(
                ErrorKind::Unknown,
                "Unable to read external DTD".to_string(),
            )),
            Ok(s) => Ok(s),
        }
    }
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op01pass1() {
    /*
        Test ID:o-p01pass1
        Test URI:p01pass1.xml
        Spec Sections:2.1 [1]
        Description:no prolog
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op01pass3() {
    /*
        Test ID:o-p01pass3
        Test URI:p01pass3.xml
        Spec Sections:2.1 [1]
        Description:Misc items after the document
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op03pass1() {
    /*
        Test ID:o-p03pass1
        Test URI:p03pass1.xml
        Spec Sections:2.3 [3]
        Description:all valid S characters
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op04pass1() {
    /*
        Test ID:o-p04pass1
        Test URI:p04pass1.xml
        Spec Sections:2.3 [4]
        Description:names with all valid ASCII characters, and one from each other class in NameChar

        Note: While this may be well formed in a not namespace-aware XML processor, we don't have any
        interest in supporting those at this time.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p04pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op05pass1() {
    /*
        Test ID:o-p05pass1
        Test URI:p05pass1.xml
        Spec Sections:2.3 [5]
        Description:various valid Name constructions

        Note: While this may be well formed in a not namespace-aware XML processor, we don't have any
        interest in supporting those at this time.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p05pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op06fail1() {
    /*
        Test ID:o-p06fail1
        Test URI:p06fail1.xml
        Spec Sections:2.3 [6]
        Description:Requires at least one name.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p06fail1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op08fail1() {
    /*
        Test ID:o-p08fail1
        Test URI:p08fail1.xml
        Spec Sections:2.3 [8]
        Description:at least one Nmtoken is required.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p08fail1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op08fail2() {
    /*
        Test ID:o-p08fail2
        Test URI:p08fail2.xml
        Spec Sections:2.3 [8]
        Description:an invalid Nmtoken character.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p08fail2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op10pass1() {
    /*
        Test ID:o-p10pass1
        Test URI:p10pass1.xml
        Spec Sections:2.3 [10]
        Description:valid attribute values
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p10pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op14pass1() {
    /*
        Test ID:o-p14pass1
        Test URI:p14pass1.xml
        Spec Sections:2.4 [14]
        Description:valid CharData
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p14pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op15pass1() {
    /*
        Test ID:o-p15pass1
        Test URI:p15pass1.xml
        Spec Sections:2.5 [15]
        Description:valid comments
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p15pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op16pass1() {
    /*
        Test ID:o-p16pass1
        Test URI:p16pass1.xml
        Spec Sections:2.6 [16] [17]
        Description:Valid form of Processing Instruction. Shows that whitespace character data is valid before end of processing instruction.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p16pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op16pass2() {
    /*
        Test ID:o-p16pass2
        Test URI:p16pass2.xml
        Spec Sections:2.6 [16]
        Description:Valid form of Processing Instruction. Shows that whitespace character data is valid before end of processing instruction.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p16pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op16pass3() {
    /*
        Test ID:o-p16pass3
        Test URI:p16pass3.xml
        Spec Sections:2.6 [16]
        Description:Valid form of Processing Instruction. Shows that whitespace character data is valid before end of processing instruction.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p16pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op18pass1() {
    /*
        Test ID:o-p18pass1
        Test URI:p18pass1.xml
        Spec Sections:2.7 [18]
        Description:valid CDSect's. Note that a CDStart in a CDSect is not recognized as such
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p18pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op22pass1() {
    /*
        Test ID:o-p22pass1
        Test URI:p22pass1.xml
        Spec Sections:2.8 [22]
        Description:prolog can be empty
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op22pass2() {
    /*
        Test ID:o-p22pass2
        Test URI:p22pass2.xml
        Spec Sections:2.8 [22]
        Description:XML declaration only
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op22pass3() {
    /*
        Test ID:o-p22pass3
        Test URI:p22pass3.xml
        Spec Sections:2.8 [22]
        Description:XML decl and Misc
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op23pass1() {
    /*
        Test ID:o-p23pass1
        Test URI:p23pass1.xml
        Spec Sections:2.8 [23]
        Description:Test shows a valid XML declaration along with version info.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op23pass2() {
    /*
        Test ID:o-p23pass2
        Test URI:p23pass2.xml
        Spec Sections:2.8 [23]
        Description:Test shows a valid XML declaration along with encoding declaration.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op23pass3() {
    /*
        Test ID:o-p23pass3
        Test URI:p23pass3.xml
        Spec Sections:2.8 [23]
        Description:Test shows a valid XML declaration along with Standalone Document Declaration.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op23pass4() {
    /*
        Test ID:o-p23pass4
        Test URI:p23pass4.xml
        Spec Sections:2.8 [23]
        Description:Test shows a valid XML declaration, encoding declarationand Standalone Document Declaration.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23pass4.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op24pass1() {
    /*
        Test ID:o-p24pass1
        Test URI:p24pass1.xml
        Spec Sections:2.8 [24]
        Description:Test shows a prolog that has the VersionInfo delimited by double quotes.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p24pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op24pass2() {
    /*
        Test ID:o-p24pass2
        Test URI:p24pass2.xml
        Spec Sections:2.8 [24]
        Description:Test shows a prolog that has the VersionInfo delimited by single quotes.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p24pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op24pass3() {
    /*
        Test ID:o-p24pass3
        Test URI:p24pass3.xml
        Spec Sections:2.8 [24]
        Description:Test shows whitespace is allowed in prolog before version info.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p24pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op24pass4() {
    /*
        Test ID:o-p24pass4
        Test URI:p24pass4.xml
        Spec Sections:2.8 [24]
        Description:Test shows whitespace is allowed in prolog on both sides of equal sign.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p24pass4.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op25pass1() {
    /*
        Test ID:o-p25pass1
        Test URI:p25pass1.xml
        Spec Sections:2.8 [25]
        Description:Test shows whitespace is NOT necessary before or after equal sign of versioninfo.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p25pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op25pass2() {
    /*
        Test ID:o-p25pass2
        Test URI:p25pass2.xml
        Spec Sections:2.8 [25]
        Description:Test shows whitespace can be used on both sides of equal sign of versioninfo.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p25pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op26pass1() {
    /*
        Test ID:o-p26pass1
        Test URI:p26pass1.xml
        Spec Sections:2.8 [26]
        Description:The valid version number. We cannot test others because a 1.0 processor is allowed to fail them.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p26pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op27pass1() {
    /*
        Test ID:o-p27pass1
        Test URI:p27pass1.xml
        Spec Sections:2.8 [27]
        Description:Comments are valid as the Misc part of the prolog.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p27pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op27pass2() {
    /*
        Test ID:o-p27pass2
        Test URI:p27pass2.xml
        Spec Sections:2.8 [27]
        Description:Processing Instructions are valid as the Misc part of the prolog.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p27pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op27pass3() {
    /*
        Test ID:o-p27pass3
        Test URI:p27pass3.xml
        Spec Sections:2.8 [27]
        Description:Whitespace is valid as the Misc part of the prolog.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p27pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op27pass4() {
    /*
        Test ID:o-p27pass4
        Test URI:p27pass4.xml
        Spec Sections:2.8 [27]
        Description:A combination of comments, whitespaces and processing instructions are valid as the Misc part of the prolog.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p27pass4.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op32pass1() {
    /*
        Test ID:o-p32pass1
        Test URI:p32pass1.xml
        Spec Sections:2.9 [32]
        Description:Double quotes can be used as delimeters for the value of a Standalone Document Declaration.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op32pass2() {
    /*
        Test ID:o-p32pass2
        Test URI:p32pass2.xml
        Spec Sections:2.9 [32]
        Description:Single quotes can be used as delimeters for the value of a Standalone Document Declaration.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op39pass1() {
    /*
        Test ID:o-p39pass1
        Test URI:p39pass1.xml
        Spec Sections:3 3.1 [39] [44]
        Description:Empty element tag may be used for any element which has no content.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op39pass2() {
    /*
        Test ID:o-p39pass2
        Test URI:p39pass2.xml
        Spec Sections:3 3.1 [39] [43]
        Description:Character data is valid element content.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op40pass1() {
    /*
        Test ID:o-p40pass1
        Test URI:p40pass1.xml
        Spec Sections:3.1 [40]
        Description:Elements content can be empty.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op40pass2() {
    /*
        Test ID:o-p40pass2
        Test URI:p40pass2.xml
        Spec Sections:3.1 [40]
        Description:Whitespace is valid within a Start-tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op40pass3() {
    /*
        Test ID:o-p40pass3
        Test URI:p40pass3.xml
        Spec Sections:3.1 [40] [41]
        Description:Attributes are valid within a Start-tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op40pass4() {
    /*
        Test ID:o-p40pass4
        Test URI:p40pass4.xml
        Spec Sections:3.1 [40]
        Description:Whitespace and Multiple Attributes are valid within a Start-tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40pass4.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op41pass1() {
    /*
        Test ID:o-p41pass1
        Test URI:p41pass1.xml
        Spec Sections:3.1 [41]
        Description:Attributes are valid within a Start-tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p41pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op41pass2() {
    /*
        Test ID:o-p41pass2
        Test URI:p41pass2.xml
        Spec Sections:3.1 [41]
        Description:Whitespace is valid within a Start-tags Attribute.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p41pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op42pass1() {
    /*
        Test ID:o-p42pass1
        Test URI:p42pass1.xml
        Spec Sections:3.1 [42]
        Description:Test shows proper syntax for an End-tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p42pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op42pass2() {
    /*
        Test ID:o-p42pass2
        Test URI:p42pass2.xml
        Spec Sections:3.1 [42]
        Description:Whitespace is valid after name in End-tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p42pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op44pass1() {
    /*
        Test ID:o-p44pass1
        Test URI:p44pass1.xml
        Spec Sections:3.1 [44]
        Description:Valid display of an Empty Element Tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op44pass2() {
    /*
        Test ID:o-p44pass2
        Test URI:p44pass2.xml
        Spec Sections:3.1 [44]
        Description:Empty Element Tags can contain an Attribute.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44pass2.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op44pass3() {
    /*
        Test ID:o-p44pass3
        Test URI:p44pass3.xml
        Spec Sections:3.1 [44]
        Description:Whitespace is valid in an Empty Element Tag following the end of the attribute value.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44pass3.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op44pass4() {
    /*
        Test ID:o-p44pass4
        Test URI:p44pass4.xml
        Spec Sections:3.1 [44]
        Description:Whitespace is valid after the name in an Empty Element Tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44pass4.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op44pass5() {
    /*
        Test ID:o-p44pass5
        Test URI:p44pass5.xml
        Spec Sections:3.1 [44]
        Description:Whitespace and Multiple Attributes are valid in an Empty Element Tag.
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44pass5.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op66pass1() {
    /*
        Test ID:o-p66pass1
        Test URI:p66pass1.xml
        Spec Sections:4.1 [66]
        Description:valid character references
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op74pass1() {
    /*
        Test ID:o-p74pass1
        Test URI:p74pass1.xml
        Spec Sections:4.2 [74]
        Description:PEDef is either an entity value or an external id
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p74pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[ignore]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn op75pass1() {
    /*
        Test ID:o-p75pass1
        Test URI:p75pass1.xml
        Spec Sections:4.2.2 [75]
        Description:valid external identifiers
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75pass1.xml")
            .unwrap()
            .as_str(),
    );
}

#[test]
#[cfg(all(test, feature = "test-conformance-xml"))]
fn oe2() {
    /*
        Test ID:o-e2
        Test URI:e2.xml
        Spec Sections:3.3.1 [58] [59] Errata [E2]
        Description:Validity Constraint: No duplicate tokens
    */

    test_oasis_invalid(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/e2.xml")
            .unwrap()
            .as_str(),
    );
}
