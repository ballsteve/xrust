/*

OASIS/NIST test cases

*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn op01pass2() {
    /*
        Test ID:o-p01pass2
        Test URI:p01pass2.xml
        Spec Sections:2.2 [1]
        Description:various Misc items where they can occur
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01pass2.xml").unwrap(),
    );
    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op06pass1() {
    /*
        Test ID:o-p06pass1
        Test URI:p06pass1.xml
        Spec Sections:2.3 [6]
        Description:various satisfactions of the Names production in a NAMES attribute
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p06pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op07pass1() {
    /*
        Test ID:o-p07pass1
        Test URI:p07pass1.xml
        Spec Sections:2.3 [7]
        Description:various valid Nmtoken 's in an attribute list declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p07pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op08pass1() {
    /*
        Test ID:o-p08pass1
        Test URI:p08pass1.xml
        Spec Sections:2.3 [8]
        Description:various satisfaction of an NMTOKENS attribute value.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p08pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op09pass1() {
    /*
        Test ID:o-p09pass1
        Test URI:p09pass1.xml
        Spec Sections:2.3 [9]
        Description:valid EntityValue's. Except for entity references, markup is not recognized.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p09pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op12pass1() {
    /*
        Test ID:o-p12pass1
        Test URI:p12pass1.xml
        Spec Sections:2.3 [12]
        Description:valid public IDs.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op22pass4() {
    /*
        Test ID:o-p22pass4
        Test URI:p22pass4.xml
        Spec Sections:2.8 [22]
        Description:XML decl and doctypedecl
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22pass4.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op22pass5() {
    /*
        Test ID:o-p22pass5
        Test URI:p22pass5.xml
        Spec Sections:2.8 [22]
        Description:just doctypedecl
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22pass5.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op22pass6() {
    /*
        Test ID:o-p22pass6
        Test URI:p22pass6.xml
        Spec Sections:2.8 [22]
        Description:S between decls is not required
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22pass6.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op28pass1() {
    /*
        Test ID:o-p28pass1
        Test URI:p28pass1.xml
        Spec Sections:3.1 [43] [44]
        Description:Empty-element tag must be used for element which are declared EMPTY.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p28pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op28pass3() {
    /*
        Test ID:o-p28pass3
        Test URI:p28pass3.xml
        Spec Sections:2.8 4.1 [28] [69]
        Description:Valid doctypedecl with Parameter entity reference. The declaration of a parameter entity must precede any reference to it.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p28pass3.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op28pass4() {
    /*
        Test ID:o-p28pass4
        Test URI:p28pass4.xml
        Spec Sections:2.8 4.2.2 [28] [75]
        Description:Valid doctypedecl with ExternalID as an External Entity declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p28pass4.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op28pass5() {
    /*
        Test ID:o-p28pass5
        Test URI:p28pass5.xml
        Spec Sections:2.8 4.1 [28] [69]
        Description:Valid doctypedecl with ExternalID as an External Entity. A parameter entity reference is also used.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p28pass5.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op29pass1() {
    /*
        Test ID:o-p29pass1
        Test URI:p29pass1.xml
        Spec Sections:2.8 [29]
        Description:Valid types of markupdecl.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p29pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op30pass1() {
    /*
        Test ID:o-p30pass1
        Test URI:p30pass1.xml
        Spec Sections:2.8 4.2.2 [30] [75]
        Description:Valid doctypedecl with ExternalID as an External Entity. The external entity has an element declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p30pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op30pass2() {
    /*
        Test ID:o-p30pass2
        Test URI:p30pass2.xml
        Spec Sections:2.8 4.2.2 4.3.1 [30] [75] [77]
        Description:Valid doctypedecl with ExternalID as an Enternal Entity. The external entity begins with a Text Declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p30pass2.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op31pass1() {
    /*
        Test ID:o-p31pass1
        Test URI:p31pass1.xml
        Spec Sections:2.8 [31]
        Description:external subset can be empty
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p31pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op31pass2() {
    /*
        Test ID:o-p31pass2
        Test URI:p31pass2.xml
        Spec Sections:2.8 3.4 4.2.2 [31] [62] [63] [75]
        Description:Valid doctypedecl with EXternalID as Enternal Entity. The external entity contains a parameter entity reference and condtional sections.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p31pass2.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op43pass1() {
    /*
        Test ID:o-p43pass1
        Test URI:p43pass1.xml
        Spec Sections:2.4 2.5 2.6 2.7 [15] [16] [18]
        Description:Valid use of character data, comments, processing instructions and CDATA sections within the start and end tag.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p43pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op45pass1() {
    /*
        Test ID:o-p45pass1
        Test URI:p45pass1.xml
        Spec Sections:3.2 [45]
        Description:valid element declarations
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p45pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op46pass1() {
    /*
        Test ID:o-p46pass1
        Test URI:p46pass1.xml
        Spec Sections:3.2 3.2.1 3.2.2 [45] [46] [47] [51]
        Description:Valid use of contentspec, element content models, and mixed content within an element type declaration.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op47pass1() {
    /*
        Test ID:o-p47pass1
        Test URI:p47pass1.xml
        Spec Sections:3.2 3.2.1 [45] [46] [47]
        Description:Valid use of contentspec, element content models, choices, sequences and content particles within an element type declaration. The optional character following a name or list governs the number of times the element or content particle may appear.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p47pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op48pass1() {
    /*
        Test ID:o-p48pass1
        Test URI:p48pass1.xml
        Spec Sections:3.2 3.2.1 [45] [46] [47]
        Description:Valid use of contentspec, element content models, choices, sequences and content particles within an element type declaration. The optional character following a name or list governs the number of times the element or content particle may appear.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p48pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op49pass1() {
    /*
        Test ID:o-p49pass1
        Test URI:p49pass1.xml
        Spec Sections:3.2 3.2.1 [45] [46] [47]
        Description:Valid use of contentspec, element content models, choices, and content particles within an element type declaration. The optional character following a name or list governs the number of times the element or content particle may appear. Whitespace is also valid between choices.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p49pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op50pass1() {
    /*
        Test ID:o-p50pass1
        Test URI:p50pass1.xml
        Spec Sections:3.2 3.2.1 [45] [46] [47]
        Description:Valid use of contentspec, element content models, sequences and content particles within an element type declaration. The optional character following a name or list governs the number of times the element or content particle may appear. Whitespace is also valid between sequences.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p50pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op51pass1() {
    /*
        Test ID:o-p51pass1
        Test URI:p51pass1.xml
        Spec Sections:3.2.2 [51]
        Description:valid Mixed contentspec's.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op52pass1() {
    /*
        Test ID:o-p52pass1
        Test URI:p52pass1.xml
        Spec Sections:3.3 [52]
        Description:valid AttlistDecls: No AttDef's are required, and the terminating S is optional, multiple ATTLISTS per element are OK, and multiple declarations of the same attribute are OK.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p52pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op53pass1() {
    /*
        Test ID:o-p53pass1
        Test URI:p53pass1.xml
        Spec Sections:3.3 [53]
        Description:a valid AttDef
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p53pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op54pass1() {
    /*
        Test ID:o-p54pass1
        Test URI:p54pass1.xml
        Spec Sections:3.3.1 [54]
        Description:the three kinds of attribute types
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p54pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op55pass1() {
    /*
        Test ID:o-p55pass1
        Test URI:p55pass1.xml
        Spec Sections:3.3.1 [55]
        Description:StringType = "CDATA"
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p55pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op56pass1() {
    /*
        Test ID:o-p56pass1
        Test URI:p56pass1.xml
        Spec Sections:3.3.1 [56]
        Description:the 7 tokenized attribute types
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p56pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op57pass1() {
    /*
        Test ID:o-p57pass1
        Test URI:p57pass1.xml
        Spec Sections:3.3.1 [57]
        Description:enumerated types are NMTOKEN or NOTATION lists
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p57pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op58pass1() {
    /*
        Test ID:o-p58pass1
        Test URI:p58pass1.xml
        Spec Sections:3.3.1 [58]
        Description:NOTATION enumeration has on or more items
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op59pass1() {
    /*
        Test ID:o-p59pass1
        Test URI:p59pass1.xml
        Spec Sections:3.3.1 [59]
        Description:NMTOKEN enumerations have one or more items
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p59pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op60pass1() {
    /*
        Test ID:o-p60pass1
        Test URI:p60pass1.xml
        Spec Sections:3.3.2 [60]
        Description:the four types of default values
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p60pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op61pass1() {
    /*
        Test ID:o-p61pass1
        Test URI:p61pass1.xml
        Spec Sections:3.4 [61]
        Description:valid conditional sections are INCLUDE and IGNORE
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p61pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op62pass1() {
    /*
        Test ID:o-p62pass1
        Test URI:p62pass1.xml
        Spec Sections:3.4 [62]
        Description:valid INCLUDE sections -- options S before and after keyword, sections can nest
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p62pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op63pass1() {
    /*
        Test ID:o-p63pass1
        Test URI:p63pass1.xml
        Spec Sections:3.4 [63]
        Description:valid IGNORE sections
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p63pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op64pass1() {
    /*
        Test ID:o-p64pass1
        Test URI:p64pass1.xml
        Spec Sections:3.4 [64]
        Description:IGNOREd sections ignore everything except section delimiters
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p64pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op68pass1() {
    /*
        Test ID:o-p68pass1
        Test URI:p68pass1.xml
        Spec Sections:4.1 [68]
        Description:Valid entity references. Also ensures that a charref to '&' isn't interpreted as an entity reference open delimiter
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p68pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op69pass1() {
    /*
        Test ID:o-p69pass1
        Test URI:p69pass1.xml
        Spec Sections:4.1 [69]
        Description:Valid PEReferences.
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p69pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op70pass1() {
    /*
        Test ID:o-p70pass1
        Test URI:p70pass1.xml
        Spec Sections:4.2 [70]
        Description:An EntityDecl is either a GEDecl or a PEDecl
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p70pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op71pass1() {
    /*
        Test ID:o-p71pass1
        Test URI:p71pass1.xml
        Spec Sections:4.2 [71]
        Description:Valid GEDecls
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p71pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
fn op72pass1() {
    /*
        Test ID:o-p72pass1
        Test URI:p72pass1.xml
        Spec Sections:4.2 [72]
        Description:Valid PEDecls
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p72pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op73pass1() {
    /*
        Test ID:o-p73pass1
        Test URI:p73pass1.xml
        Spec Sections:4.2 [73]
        Description:EntityDef is either Entity value or an external id, with an optional NDataDecl
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p73pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}

#[test]
#[ignore]
fn op76pass1() {
    /*
        Test ID:o-p76pass1
        Test URI:p76pass1.xml
        Spec Sections:4.2.2 [76]
        Description:valid NDataDecls
    */

    let testxml = Document::try_from(
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p76pass1.xml").unwrap(),
    );

    assert!(testxml.is_ok());
}
