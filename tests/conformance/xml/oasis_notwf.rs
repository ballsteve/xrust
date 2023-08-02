/*

OASIS/NIST test cases

*/

use std::convert::TryFrom;
use std::fs;
use xrust::Document;

#[test]
fn op01fail1() {
    /*
        Test ID:o-p01fail1
        Test URI:p01fail1.xml
        Spec Sections:2.1 [1]
        Description:S cannot occur before the prolog
    */
    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op01fail2() {
    /*
        Test ID:o-p01fail2
        Test URI:p01fail2.xml
        Spec Sections:2.1 [1]
        Description:comments cannot occur before the prolog
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op01fail3() {
    /*
        Test ID:o-p01fail3
        Test URI:p01fail3.xml
        Spec Sections:2.1 [1]
        Description:only one document element
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op01fail4() {
    /*
        Test ID:o-p01fail4
        Test URI:p01fail4.xml
        Spec Sections:2.1 [1]
        Description:document element must be complete.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p01fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail1() {
    /*
        Test ID:o-p02fail1
        Test URI:p02fail1.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail10() {
    /*
        Test ID:o-p02fail10
        Test URI:p02fail10.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail10.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail11() {
    /*
        Test ID:o-p02fail11
        Test URI:p02fail11.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail11.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail12() {
    /*
        Test ID:o-p02fail12
        Test URI:p02fail12.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail12.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail13() {
    /*
        Test ID:o-p02fail13
        Test URI:p02fail13.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail13.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail14() {
    /*
        Test ID:o-p02fail14
        Test URI:p02fail14.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail14.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail15() {
    /*
        Test ID:o-p02fail15
        Test URI:p02fail15.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail15.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail16() {
    /*
        Test ID:o-p02fail16
        Test URI:p02fail16.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail16.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail17() {
    /*
        Test ID:o-p02fail17
        Test URI:p02fail17.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail17.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail18() {
    /*
        Test ID:o-p02fail18
        Test URI:p02fail18.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail18.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail19() {
    /*
        Test ID:o-p02fail19
        Test URI:p02fail19.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail19.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail2() {
    /*
        Test ID:o-p02fail2
        Test URI:p02fail2.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail20() {
    /*
        Test ID:o-p02fail20
        Test URI:p02fail20.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail20.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail21() {
    /*
        Test ID:o-p02fail21
        Test URI:p02fail21.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail21.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail22() {
    /*
        Test ID:o-p02fail22
        Test URI:p02fail22.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail22.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail23() {
    /*
        Test ID:o-p02fail23
        Test URI:p02fail23.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail23.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail24() {
    /*
        Test ID:o-p02fail24
        Test URI:p02fail24.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail24.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail25() {
    /*
        Test ID:o-p02fail25
        Test URI:p02fail25.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail25.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail26() {
    /*
        Test ID:o-p02fail26
        Test URI:p02fail26.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail26.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail27() {
    /*
        Test ID:o-p02fail27
        Test URI:p02fail27.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail27.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail28() {
    /*
        Test ID:o-p02fail28
        Test URI:p02fail28.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail28.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail29() {
    /*
        Test ID:o-p02fail29
        Test URI:p02fail29.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail29.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail3() {
    /*
        Test ID:o-p02fail3
        Test URI:p02fail3.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail30() {
    /*
        Test ID:o-p02fail30
        Test URI:p02fail30.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail30.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail31() {
    /*
        Test ID:o-p02fail31
        Test URI:p02fail31.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail31.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail4() {
    /*
        Test ID:o-p02fail4
        Test URI:p02fail4.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail5() {
    /*
        Test ID:o-p02fail5
        Test URI:p02fail5.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail6() {
    /*
        Test ID:o-p02fail6
        Test URI:p02fail6.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail7() {
    /*
        Test ID:o-p02fail7
        Test URI:p02fail7.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail7.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail8() {
    /*
        Test ID:o-p02fail8
        Test URI:p02fail8.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail8.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op02fail9() {
    /*
        Test ID:o-p02fail9
        Test URI:p02fail9.xml
        Spec Sections:2.2 [2]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p02fail9.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail1() {
    /*
        Test ID:o-p03fail1
        Test URI:p03fail1.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail10() {
    /*
        Test ID:o-p03fail10
        Test URI:p03fail10.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail10.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail11() {
    /*
        Test ID:o-p03fail11
        Test URI:p03fail11.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail11.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail12() {
    /*
        Test ID:o-p03fail12
        Test URI:p03fail12.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail12.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail13() {
    /*
        Test ID:o-p03fail13
        Test URI:p03fail13.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail13.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail14() {
    /*
        Test ID:o-p03fail14
        Test URI:p03fail14.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail14.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail15() {
    /*
        Test ID:o-p03fail15
        Test URI:p03fail15.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail15.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail16() {
    /*
        Test ID:o-p03fail16
        Test URI:p03fail16.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail16.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail17() {
    /*
        Test ID:o-p03fail17
        Test URI:p03fail17.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail17.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail18() {
    /*
        Test ID:o-p03fail18
        Test URI:p03fail18.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail18.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail19() {
    /*
        Test ID:o-p03fail19
        Test URI:p03fail19.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail19.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail2() {
    /*
        Test ID:o-p03fail2
        Test URI:p03fail2.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail20() {
    /*
        Test ID:o-p03fail20
        Test URI:p03fail20.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail20.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail21() {
    /*
        Test ID:o-p03fail21
        Test URI:p03fail21.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail21.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail22() {
    /*
        Test ID:o-p03fail22
        Test URI:p03fail22.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail22.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail23() {
    /*
        Test ID:o-p03fail23
        Test URI:p03fail23.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail23.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail24() {
    /*
        Test ID:o-p03fail24
        Test URI:p03fail24.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail24.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail25() {
    /*
        Test ID:o-p03fail25
        Test URI:p03fail25.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail25.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail26() {
    /*
        Test ID:o-p03fail26
        Test URI:p03fail26.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail26.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail27() {
    /*
        Test ID:o-p03fail27
        Test URI:p03fail27.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail27.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail28() {
    /*
        Test ID:o-p03fail28
        Test URI:p03fail28.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail28.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail29() {
    /*
        Test ID:o-p03fail29
        Test URI:p03fail29.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail29.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail3() {
    /*
        Test ID:o-p03fail3
        Test URI:p03fail3.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail4() {
    /*
        Test ID:o-p03fail4
        Test URI:p03fail4.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail5() {
    /*
        Test ID:o-p03fail5
        Test URI:p03fail5.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail7() {
    /*
        Test ID:o-p03fail7
        Test URI:p03fail7.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail7.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail8() {
    /*
        Test ID:o-p03fail8
        Test URI:p03fail8.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail8.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op03fail9() {
    /*
        Test ID:o-p03fail9
        Test URI:p03fail9.xml
        Spec Sections:2.3 [3]
        Description:Use of illegal character within XML document.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p03fail9.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op04fail1() {
    /*
        Test ID:o-p04fail1
        Test URI:p04fail1.xml
        Spec Sections:2.3 [4]
        Description:Name contains invalid character.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p04fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op04fail2() {
    /*
        Test ID:o-p04fail2
        Test URI:p04fail2.xml
        Spec Sections:2.3 [4]
        Description:Name contains invalid character.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p04fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op04fail3() {
    /*
        Test ID:o-p04fail3
        Test URI:p04fail3.xml
        Spec Sections:2.3 [4]
        Description:Name contains invalid character.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p04fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op05fail1() {
    /*
        Test ID:o-p05fail1
        Test URI:p05fail1.xml
        Spec Sections:2.3 [5]
        Description:a Name cannot start with a digit
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p05fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op05fail2() {
    /*
        Test ID:o-p05fail2
        Test URI:p05fail2.xml
        Spec Sections:2.3 [5]
        Description:a Name cannot start with a '.'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p05fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op05fail3() {
    /*
        Test ID:o-p05fail3
        Test URI:p05fail3.xml
        Spec Sections:2.3 [5]
        Description:a Name cannot start with a "-"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p05fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op05fail4() {
    /*
        Test ID:o-p05fail4
        Test URI:p05fail4.xml
        Spec Sections:2.3 [5]
        Description:a Name cannot start with a CombiningChar
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p05fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op05fail5() {
    /*
        Test ID:o-p05fail5
        Test URI:p05fail5.xml
        Spec Sections:2.3 [5]
        Description:a Name cannot start with an Extender
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p05fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op09fail1() {
    /*
        Test ID:o-p09fail1
        Test URI:p09fail1.xml
        Spec Sections:2.3 [9]
        Description:EntityValue excludes '%'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p09fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op09fail2() {
    /*
        Test ID:o-p09fail2
        Test URI:p09fail2.xml
        Spec Sections:2.3 [9]
        Description:EntityValue excludes '&'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p09fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op09fail3() {
    /*
        Test ID:o-p09fail3
        Test URI:p09fail3.xml
        Spec Sections:2.3 [9]
        Description:incomplete character reference
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p09fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op09fail4() {
    /*
        Test ID:o-p09fail4
        Test URI:p09fail4.xml
        Spec Sections:2.3 [9]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p09fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op09fail5() {
    /*
        Test ID:o-p09fail5
        Test URI:p09fail5.xml
        Spec Sections:2.3 [9]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p09fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op10fail1() {
    /*
        Test ID:o-p10fail1
        Test URI:p10fail1.xml
        Spec Sections:2.3 [10]
        Description:attribute values exclude '<'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p10fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op10fail2() {
    /*
        Test ID:o-p10fail2
        Test URI:p10fail2.xml
        Spec Sections:2.3 [10]
        Description:attribute values exclude '&'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p10fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op10fail3() {
    /*
        Test ID:o-p10fail3
        Test URI:p10fail3.xml
        Spec Sections:2.3 [10]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p10fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op11fail1() {
    /*
        Test ID:o-p11fail1
        Test URI:p11fail1.xml
        Spec Sections:2.3 [11]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p11fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op11fail2() {
    /*
        Test ID:o-p11fail2
        Test URI:p11fail2.xml
        Spec Sections:2.3 [11]
        Description:cannot contain delimiting quotes
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p11fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op12fail1() {
    /*
        Test ID:o-p12fail1
        Test URI:p12fail1.xml
        Spec Sections:2.3 [12]
        Description:'"' excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op12fail2() {
    /*
        Test ID:o-p12fail2
        Test URI:p12fail2.xml
        Spec Sections:2.3 [12]
        Description:'\' excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op12fail3() {
    /*
        Test ID:o-p12fail3
        Test URI:p12fail3.xml
        Spec Sections:2.3 [12]
        Description:entity references excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op12fail4() {
    /*
        Test ID:o-p12fail4
        Test URI:p12fail4.xml
        Spec Sections:2.3 [12]
        Description:'>' excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op12fail5() {
    /*
        Test ID:o-p12fail5
        Test URI:p12fail5.xml
        Spec Sections:2.3 [12]
        Description:'<' excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op12fail6() {
    /*
        Test ID:o-p12fail6
        Test URI:p12fail6.xml
        Spec Sections:2.3 [12]
        Description:built-in entity refs excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op12fail7() {
    /*
        Test ID:o-p12fail7
        Test URI:p12fail7.xml
        Spec Sections:2.3 [13]
        Description:The public ID has a tab character, which is disallowed
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p12fail7.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op14fail1() {
    /*
        Test ID:o-p14fail1
        Test URI:p14fail1.xml
        Spec Sections:2.4 [14]
        Description:'<' excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p14fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op14fail2() {
    /*
        Test ID:o-p14fail2
        Test URI:p14fail2.xml
        Spec Sections:2.4 [14]
        Description:'&' excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p14fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op14fail3() {
    /*
        Test ID:o-p14fail3
        Test URI:p14fail3.xml
        Spec Sections:2.4 [14]
        Description:"]]>" excluded
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p14fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op15fail1() {
    /*
        Test ID:o-p15fail1
        Test URI:p15fail1.xml
        Spec Sections:2.5 [15]
        Description:comments can't end in '-'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p15fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op15fail2() {
    /*
        Test ID:o-p15fail2
        Test URI:p15fail2.xml
        Spec Sections:2.5 [15]
        Description:one comment per comment (contrasted with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p15fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op15fail3() {
    /*
        Test ID:o-p15fail3
        Test URI:p15fail3.xml
        Spec Sections:2.5 [15]
        Description:can't include 2 or more adjacent '-'s
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p15fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op16fail1() {
    /*
        Test ID:o-p16fail1
        Test URI:p16fail1.xml
        Spec Sections:2.6 [16]
        Description:"xml" is an invalid PITarget
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p16fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op16fail2() {
    /*
        Test ID:o-p16fail2
        Test URI:p16fail2.xml
        Spec Sections:2.6 [16]
        Description:a PITarget must be present
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p16fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op16fail3() {
    /*
        Test ID:o-p16fail3
        Test URI:p16fail3.xml
        Spec Sections:2.6 [16]
        Description:S after PITarget is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p16fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op18fail1() {
    /*
        Test ID:o-p18fail1
        Test URI:p18fail1.xml
        Spec Sections:2.7 [18]
        Description:no space before "CDATA"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p18fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op18fail2() {
    /*
        Test ID:o-p18fail2
        Test URI:p18fail2.xml
        Spec Sections:2.7 [18]
        Description:no space after "CDATA"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p18fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op18fail3() {
    /*
        Test ID:o-p18fail3
        Test URI:p18fail3.xml
        Spec Sections:2.7 [18]
        Description:CDSect's can't nest
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p18fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op22fail1() {
    /*
        Test ID:o-p22fail1
        Test URI:p22fail1.xml
        Spec Sections:2.8 [22]
        Description:prolog must start with XML decl
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op22fail2() {
    /*
        Test ID:o-p22fail2
        Test URI:p22fail2.xml
        Spec Sections:2.8 [22]
        Description:prolog must start with XML decl
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p22fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op23fail1() {
    /*
        Test ID:o-p23fail1
        Test URI:p23fail1.xml
        Spec Sections:2.8 [23]
        Description:"xml" must be lower-case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op23fail2() {
    /*
        Test ID:o-p23fail2
        Test URI:p23fail2.xml
        Spec Sections:2.8 [23]
        Description:VersionInfo must be supplied
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op23fail3() {
    /*
        Test ID:o-p23fail3
        Test URI:p23fail3.xml
        Spec Sections:2.8 [23]
        Description:VersionInfo must come first
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op23fail4() {
    /*
        Test ID:o-p23fail4
        Test URI:p23fail4.xml
        Spec Sections:2.8 [23]
        Description:SDDecl must come last
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op23fail5() {
    /*
        Test ID:o-p23fail5
        Test URI:p23fail5.xml
        Spec Sections:2.8 [23]
        Description:no SGML-type PIs
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p23fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op24fail1() {
    /*
        Test ID:o-p24fail1
        Test URI:p24fail1.xml
        Spec Sections:2.8 [24]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p24fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op24fail2() {
    /*
        Test ID:o-p24fail2
        Test URI:p24fail2.xml
        Spec Sections:2.8 [24]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p24fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op25fail1() {
    /*
        Test ID:o-p25fail1
        Test URI:p25fail1.xml
        Spec Sections:2.8 [25]
        Description:Comment is illegal in VersionInfo.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p25fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op26fail1() {
    /*
        Test ID:o-p26fail1
        Test URI:p26fail1.xml
        Spec Sections:2.8 [26]
        Description:Illegal character in VersionNum.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p26fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op26fail2() {
    /*
        Test ID:o-p26fail2
        Test URI:p26fail2.xml
        Spec Sections:2.8 [26]
        Description:Illegal character in VersionNum.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p26fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op27fail1() {
    /*
        Test ID:o-p27fail1
        Test URI:p27fail1.xml
        Spec Sections:2.8 [27]
        Description:References aren't allowed in Misc, even if they would resolve to valid Misc.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p27fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op28fail1() {
    /*
        Test ID:o-p28fail1
        Test URI:p28fail1.xml
        Spec Sections:2.8 [28]
        Description:only declarations in DTD.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p28fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op29fail1() {
    /*
        Test ID:o-p29fail1
        Test URI:p29fail1.xml
        Spec Sections:2.8 [29]
        Description:A processor must not pass unknown declaration types.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p29fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op30fail1() {
    /*
        Test ID:o-p30fail1
        Test URI:p30fail1.xml
        Spec Sections:2.8 [30]
        Description:An XML declaration is not the same as a TextDecl
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p30fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op31fail1() {
    /*
        Test ID:o-p31fail1
        Test URI:p31fail1.xml
        Spec Sections:2.8 [31]
        Description:external subset excludes doctypedecl
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p31fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op32fail1() {
    /*
        Test ID:o-p32fail1
        Test URI:p32fail1.xml
        Spec Sections:2.9 [32]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op32fail2() {
    /*
        Test ID:o-p32fail2
        Test URI:p32fail2.xml
        Spec Sections:2.9 [32]
        Description:quote types must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op32fail3() {
    /*
        Test ID:o-p32fail3
        Test URI:p32fail3.xml
        Spec Sections:2.9 [32]
        Description:initial S is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op32fail4() {
    /*
        Test ID:o-p32fail4
        Test URI:p32fail4.xml
        Spec Sections:2.9 [32]
        Description:quotes are required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op32fail5() {
    /*
        Test ID:o-p32fail5
        Test URI:p32fail5.xml
        Spec Sections:2.9 [32]
        Description:yes or no must be lower case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p32fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op39fail1() {
    /*
        Test ID:o-p39fail1
        Test URI:p39fail1.xml
        Spec Sections:3 [39]
        Description:start-tag requires end-tag
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op39fail2() {
    /*
        Test ID:o-p39fail2
        Test URI:p39fail2.xml
        Spec Sections:3 [39]
        Description:end-tag requires start-tag
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op39fail3() {
    /*
        Test ID:o-p39fail3
        Test URI:p39fail3.xml
        Spec Sections:3 [39]
        Description:XML documents contain one or more elements
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op39fail4() {
    /*
        Test ID:o-p39fail4
        Test URI:p39fail4.xml
        Spec Sections:2.8 [23]
        Description:XML declarations must be correctly terminated
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op39fail5() {
    /*
        Test ID:o-p39fail5
        Test URI:p39fail5.xml
        Spec Sections:2.8 [23]
        Description:XML declarations must be correctly terminated
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p39fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op40fail1() {
    /*
        Test ID:o-p40fail1
        Test URI:p40fail1.xml
        Spec Sections:3.1 [40]
        Description:S is required between attributes
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op40fail2() {
    /*
        Test ID:o-p40fail2
        Test URI:p40fail2.xml
        Spec Sections:3.1 [40]
        Description:tags start with names, not nmtokens
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op40fail3() {
    /*
        Test ID:o-p40fail3
        Test URI:p40fail3.xml
        Spec Sections:3.1 [40]
        Description:tags start with names, not nmtokens
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op40fail4() {
    /*
        Test ID:o-p40fail4
        Test URI:p40fail4.xml
        Spec Sections:3.1 [40]
        Description:no space before name
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p40fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op41fail1() {
    /*
        Test ID:o-p41fail1
        Test URI:p41fail1.xml
        Spec Sections:3.1 [41]
        Description:quotes are required (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p41fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op41fail2() {
    /*
        Test ID:o-p41fail2
        Test URI:p41fail2.xml
        Spec Sections:3.1 [41]
        Description:attribute name is required (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p41fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op41fail3() {
    /*
        Test ID:o-p41fail3
        Test URI:p41fail3.xml
        Spec Sections:3.1 [41]
        Description:Eq required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p41fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op42fail1() {
    /*
        Test ID:o-p42fail1
        Test URI:p42fail1.xml
        Spec Sections:3.1 [42]
        Description:no space before name
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p42fail1.xml").unwrap(),
        None,
        None,
    ));
    assert!(testxml.is_err());
}

#[test]
fn op42fail2() {
    /*
        Test ID:o-p42fail2
        Test URI:p42fail2.xml
        Spec Sections:3.1 [42]
        Description:cannot end with "/>"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p42fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op42fail3() {
    /*
        Test ID:o-p42fail3
        Test URI:p42fail3.xml
        Spec Sections:3.1 [42]
        Description:no NET (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p42fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op43fail1() {
    /*
        Test ID:o-p43fail1
        Test URI:p43fail1.xml
        Spec Sections:3.1 [43]
        Description:no non-comment declarations
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p43fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op43fail2() {
    /*
        Test ID:o-p43fail2
        Test URI:p43fail2.xml
        Spec Sections:3.1 [43]
        Description:no conditional sections
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p43fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op43fail3() {
    /*
        Test ID:o-p43fail3
        Test URI:p43fail3.xml
        Spec Sections:3.1 [43]
        Description:no conditional sections
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p43fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op44fail1() {
    /*
        Test ID:o-p44fail1
        Test URI:p44fail1.xml
        Spec Sections:3.1 [44]
        Description:Illegal space before Empty element tag.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op44fail2() {
    /*
        Test ID:o-p44fail2
        Test URI:p44fail2.xml
        Spec Sections:3.1 [44]
        Description:Illegal space after Empty element tag.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op44fail3() {
    /*
        Test ID:o-p44fail3
        Test URI:p44fail3.xml
        Spec Sections:3.1 [44]
        Description:Illegal comment in Empty element tag.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op44fail4() {
    /*
        Test ID:o-p44fail4
        Test URI:p44fail4.xml
        Spec Sections:3.1 [44]
        Description:Whitespace required between attributes.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op44fail5() {
    /*
        Test ID:o-p44fail5
        Test URI:p44fail5.xml
        Spec Sections:3.1 [44]
        Description:Duplicate attribute name is illegal.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p44fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op45fail1() {
    /*
        Test ID:o-p45fail1
        Test URI:p45fail1.xml
        Spec Sections:3.2 [45]
        Description:ELEMENT must be upper case.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p45fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op45fail2() {
    /*
        Test ID:o-p45fail2
        Test URI:p45fail2.xml
        Spec Sections:3.2 [45]
        Description:S before contentspec is required.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p45fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op45fail3() {
    /*
        Test ID:o-p45fail3
        Test URI:p45fail3.xml
        Spec Sections:3.2 [45]
        Description:only one content spec
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p45fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op45fail4() {
    /*
        Test ID:o-p45fail4
        Test URI:p45fail4.xml
        Spec Sections:3.2 [45]
        Description:no comments in declarations (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p45fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op46fail1() {
    /*
        Test ID:o-p46fail1
        Test URI:p46fail1.xml
        Spec Sections:3.2 [46]
        Description:no parens on declared content
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op46fail2() {
    /*
        Test ID:o-p46fail2
        Test URI:p46fail2.xml
        Spec Sections:3.2 [46]
        Description:no inclusions (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op46fail3() {
    /*
        Test ID:o-p46fail3
        Test URI:p46fail3.xml
        Spec Sections:3.2 [46]
        Description:no exclusions (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op46fail4() {
    /*
        Test ID:o-p46fail4
        Test URI:p46fail4.xml
        Spec Sections:3.2 [46]
        Description:no space before occurrence
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op46fail5() {
    /*
        Test ID:o-p46fail5
        Test URI:p46fail5.xml
        Spec Sections:3.2 [46]
        Description:single group
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op46fail6() {
    /*
        Test ID:o-p46fail6
        Test URI:p46fail6.xml
        Spec Sections:3.2 [46]
        Description:can't be both declared and modeled
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p46fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op47fail1() {
    /*
        Test ID:o-p47fail1
        Test URI:p47fail1.xml
        Spec Sections:3.2.1 [47]
        Description:Invalid operator '|' must match previous operator ','
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p47fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op47fail2() {
    /*
        Test ID:o-p47fail2
        Test URI:p47fail2.xml
        Spec Sections:3.2.1 [47]
        Description:Illegal character '-' in Element-content model
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p47fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op47fail3() {
    /*
        Test ID:o-p47fail3
        Test URI:p47fail3.xml
        Spec Sections:3.2.1 [47]
        Description:Optional character must follow a name or list
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p47fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op47fail4() {
    /*
        Test ID:o-p47fail4
        Test URI:p47fail4.xml
        Spec Sections:3.2.1 [47]
        Description:Illegal space before optional character
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p47fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op48fail1() {
    /*
        Test ID:o-p48fail1
        Test URI:p48fail1.xml
        Spec Sections:3.2.1 [48]
        Description:Illegal space before optional character
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p48fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op48fail2() {
    /*
        Test ID:o-p48fail2
        Test URI:p48fail2.xml
        Spec Sections:3.2.1 [48]
        Description:Illegal space before optional character
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p48fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op49fail1() {
    /*
        Test ID:o-p49fail1
        Test URI:p49fail1.xml
        Spec Sections:3.2.1 [49]
        Description:connectors must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p49fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op50fail1() {
    /*
        Test ID:o-p50fail1
        Test URI:p50fail1.xml
        Spec Sections:3.2.1 [50]
        Description:connectors must match
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p50fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail1() {
    /*
        Test ID:o-p51fail1
        Test URI:p51fail1.xml
        Spec Sections:3.2.2 [51]
        Description:occurrence on #PCDATA group must be *
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail2() {
    /*
        Test ID:o-p51fail2
        Test URI:p51fail2.xml
        Spec Sections:3.2.2 [51]
        Description:occurrence on #PCDATA group must be *
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail3() {
    /*
        Test ID:o-p51fail3
        Test URI:p51fail3.xml
        Spec Sections:3.2.2 [51]
        Description:#PCDATA must come first
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail4() {
    /*
        Test ID:o-p51fail4
        Test URI:p51fail4.xml
        Spec Sections:3.2.2 [51]
        Description:occurrence on #PCDATA group must be *
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail5() {
    /*
        Test ID:o-p51fail5
        Test URI:p51fail5.xml
        Spec Sections:3.2.2 [51]
        Description:only '|' connectors
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail6() {
    /*
        Test ID:o-p51fail6
        Test URI:p51fail6.xml
        Spec Sections:3.2.2 [51]
        Description:Only '|' connectors and occurrence on #PCDATA group must be *
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op51fail7() {
    /*
        Test ID:o-p51fail7
        Test URI:p51fail7.xml
        Spec Sections:3.2.2 [51]
        Description:no nested groups
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p51fail7.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op52fail1() {
    /*
        Test ID:o-p52fail1
        Test URI:p52fail1.xml
        Spec Sections:3.3 [52]
        Description:A name is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p52fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op52fail2() {
    /*
        Test ID:o-p52fail2
        Test URI:p52fail2.xml
        Spec Sections:3.3 [52]
        Description:A name is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p52fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op53fail1() {
    /*
        Test ID:o-p53fail1
        Test URI:p53fail1.xml
        Spec Sections:3.3 [53]
        Description:S is required before default
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p53fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op53fail2() {
    /*
        Test ID:o-p53fail2
        Test URI:p53fail2.xml
        Spec Sections:3.3 [53]
        Description:S is required before type
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p53fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op53fail3() {
    /*
        Test ID:o-p53fail3
        Test URI:p53fail3.xml
        Spec Sections:3.3 [53]
        Description:type is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p53fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op53fail4() {
    /*
        Test ID:o-p53fail4
        Test URI:p53fail4.xml
        Spec Sections:3.3 [53]
        Description:default is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p53fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op53fail5() {
    /*
        Test ID:o-p53fail5
        Test URI:p53fail5.xml
        Spec Sections:3.3 [53]
        Description:name is requried
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p53fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op54fail1() {
    /*
        Test ID:o-p54fail1
        Test URI:p54fail1.xml
        Spec Sections:3.3.1 [54]
        Description:don't pass unknown attribute types
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p54fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op55fail1() {
    /*
        Test ID:o-p55fail1
        Test URI:p55fail1.xml
        Spec Sections:3.3.1 [55]
        Description:must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p55fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op56fail1() {
    /*
        Test ID:o-p56fail1
        Test URI:p56fail1.xml
        Spec Sections:3.3.1 [56]
        Description:no IDS type
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p56fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op56fail2() {
    /*
        Test ID:o-p56fail2
        Test URI:p56fail2.xml
        Spec Sections:3.3.1 [56]
        Description:no NUMBER type
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p56fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op56fail3() {
    /*
        Test ID:o-p56fail3
        Test URI:p56fail3.xml
        Spec Sections:3.3.1 [56]
        Description:no NAME type
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p56fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op56fail4() {
    /*
        Test ID:o-p56fail4
        Test URI:p56fail4.xml
        Spec Sections:3.3.1 [56]
        Description:no ENTITYS type- types must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p56fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op56fail5() {
    /*
        Test ID:o-p56fail5
        Test URI:p56fail5.xml
        Spec Sections:3.3.1 [56]
        Description:types must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p56fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op57fail1() {
    /*
        Test ID:o-p57fail1
        Test URI:p57fail1.xml
        Spec Sections:3.3.1 [57]
        Description:no keyword for NMTOKEN enumeration
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p57fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail1() {
    /*
        Test ID:o-p58fail1
        Test URI:p58fail1.xml
        Spec Sections:3.3.1 [58]
        Description:at least one value required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail2() {
    /*
        Test ID:o-p58fail2
        Test URI:p58fail2.xml
        Spec Sections:3.3.1 [58]
        Description:separator must be '|'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail3() {
    /*
        Test ID:o-p58fail3
        Test URI:p58fail3.xml
        Spec Sections:3.3.1 [58]
        Description:notations are NAMEs, not NMTOKENs -- note: Leaving the invalid notation undeclared would cause a validating parser to fail without checking the name syntax, so the notation is declared with an invalid name. A parser that reports error positions should report an error at the AttlistDecl on line 6, before reaching the notation declaration.
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail4() {
    /*
        Test ID:o-p58fail4
        Test URI:p58fail4.xml
        Spec Sections:3.3.1 [58]
        Description:NOTATION must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail5() {
    /*
        Test ID:o-p58fail5
        Test URI:p58fail5.xml
        Spec Sections:3.3.1 [58]
        Description:S after keyword is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail6() {
    /*
        Test ID:o-p58fail6
        Test URI:p58fail6.xml
        Spec Sections:3.3.1 [58]
        Description:parentheses are require
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail7() {
    /*
        Test ID:o-p58fail7
        Test URI:p58fail7.xml
        Spec Sections:3.3.1 [58]
        Description:values are unquoted
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail7.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op58fail8() {
    /*
        Test ID:o-p58fail8
        Test URI:p58fail8.xml
        Spec Sections:3.3.1 [58]
        Description:values are unquoted
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p58fail8.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op59fail1() {
    /*
        Test ID:o-p59fail1
        Test URI:p59fail1.xml
        Spec Sections:3.3.1 [59]
        Description:at least one required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p59fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op59fail2() {
    /*
        Test ID:o-p59fail2
        Test URI:p59fail2.xml
        Spec Sections:3.3.1 [59]
        Description:separator must be ","
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p59fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op59fail3() {
    /*
        Test ID:o-p59fail3
        Test URI:p59fail3.xml
        Spec Sections:3.3.1 [59]
        Description:values are unquoted
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p59fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op60fail1() {
    /*
        Test ID:o-p60fail1
        Test URI:p60fail1.xml
        Spec Sections:3.3.2 [60]
        Description:keywords must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p60fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op60fail2() {
    /*
        Test ID:o-p60fail2
        Test URI:p60fail2.xml
        Spec Sections:3.3.2 [60]
        Description:S is required after #FIXED
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p60fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op60fail3() {
    /*
        Test ID:o-p60fail3
        Test URI:p60fail3.xml
        Spec Sections:3.3.2 [60]
        Description:only #FIXED has both keyword and value
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p60fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op60fail4() {
    /*
        Test ID:o-p60fail4
        Test URI:p60fail4.xml
        Spec Sections:3.3.2 [60]
        Description:#FIXED required value
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p60fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op60fail5() {
    /*
        Test ID:o-p60fail5
        Test URI:p60fail5.xml
        Spec Sections:3.3.2 [60]
        Description:only one default type
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p60fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op61fail1() {
    /*
        Test ID:o-p61fail1
        Test URI:p61fail1.xml
        Spec Sections:3.4 [61]
        Description:no other types, including TEMP, which is valid in SGML
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p61fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op62fail1() {
    /*
        Test ID:o-p62fail1
        Test URI:p62fail1.xml
        Spec Sections:3.4 [62]
        Description:INCLUDE must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p62fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op62fail2() {
    /*
        Test ID:o-p62fail2
        Test URI:p62fail2.xml
        Spec Sections:3.4 [62]
        Description:no spaces in terminating delimiter
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p62fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op63fail1() {
    /*
        Test ID:o-p63fail1
        Test URI:p63fail1.xml
        Spec Sections:3.4 [63]
        Description:IGNORE must be upper case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p63fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op63fail2() {
    /*
        Test ID:o-p63fail2
        Test URI:p63fail2.xml
        Spec Sections:3.4 [63]
        Description:delimiters must be balanced
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p63fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op64fail1() {
    /*
        Test ID:o-p64fail1
        Test URI:p64fail1.xml
        Spec Sections:3.4 [64]
        Description:section delimiters must balance
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p64fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[ignore]
fn op64fail2() {
    /*
        Test ID:o-p64fail2
        Test URI:p64fail2.xml
        Spec Sections:3.4 [64]
        Description:section delimiters must balance
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p64fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op66fail1() {
    /*
        Test ID:o-p66fail1
        Test URI:p66fail1.xml
        Spec Sections:4.1 [66]
        Description:terminating ';' is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op66fail2() {
    /*
        Test ID:o-p66fail2
        Test URI:p66fail2.xml
        Spec Sections:4.1 [66]
        Description:no S after '&#'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op66fail3() {
    /*
        Test ID:o-p66fail3
        Test URI:p66fail3.xml
        Spec Sections:4.1 [66]
        Description:no hex digits in numeric reference
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op66fail4() {
    /*
        Test ID:o-p66fail4
        Test URI:p66fail4.xml
        Spec Sections:4.1 [66]
        Description:only hex digits in hex references
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op66fail5() {
    /*
        Test ID:o-p66fail5
        Test URI:p66fail5.xml
        Spec Sections:4.1 [66]
        Description:no references to non-characters
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
#[should_panic]
fn op66fail6() {
    /*
        Test ID:o-p66fail6
        Test URI:p66fail6.xml
        Spec Sections:4.1 [66]
        Description:no references to non-characters
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p66fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op68fail1() {
    /*
        Test ID:o-p68fail1
        Test URI:p68fail1.xml
        Spec Sections:4.1 [68]
        Description:terminating ';' is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p68fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op68fail2() {
    /*
        Test ID:o-p68fail2
        Test URI:p68fail2.xml
        Spec Sections:4.1 [68]
        Description:no S after '&'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p68fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op68fail3() {
    /*
        Test ID:o-p68fail3
        Test URI:p68fail3.xml
        Spec Sections:4.1 [68]
        Description:no S before ';'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p68fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op69fail1() {
    /*
        Test ID:o-p69fail1
        Test URI:p69fail1.xml
        Spec Sections:4.1 [69]
        Description:terminating ';' is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p69fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op69fail2() {
    /*
        Test ID:o-p69fail2
        Test URI:p69fail2.xml
        Spec Sections:4.1 [69]
        Description:no S after '%'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p69fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op69fail3() {
    /*
        Test ID:o-p69fail3
        Test URI:p69fail3.xml
        Spec Sections:4.1 [69]
        Description:no S before ';'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p69fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op70fail1() {
    /*
        Test ID:o-p70fail1
        Test URI:p70fail1.xml
        Spec Sections:4.2 [70]
        Description:This is neither
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p70fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op71fail1() {
    /*
        Test ID:o-p71fail1
        Test URI:p71fail1.xml
        Spec Sections:4.2 [71]
        Description:S is required before EntityDef
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p71fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op71fail2() {
    /*
        Test ID:o-p71fail2
        Test URI:p71fail2.xml
        Spec Sections:4.2 [71]
        Description:Entity name is a Name, not an NMToken
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p71fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op71fail3() {
    /*
        Test ID:o-p71fail3
        Test URI:p71fail3.xml
        Spec Sections:4.2 [71]
        Description:no S after "<!"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p71fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op71fail4() {
    /*
        Test ID:o-p71fail4
        Test URI:p71fail4.xml
        Spec Sections:4.2 [71]
        Description:S is required after "<!ENTITY"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p71fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op72fail1() {
    /*
        Test ID:o-p72fail1
        Test URI:p72fail1.xml
        Spec Sections:4.2 [72]
        Description:S is required after "<!ENTITY"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p72fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op72fail2() {
    /*
        Test ID:o-p72fail2
        Test URI:p72fail2.xml
        Spec Sections:4.2 [72]
        Description:S is required after '%'
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p72fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op72fail3() {
    /*
        Test ID:o-p72fail3
        Test URI:p72fail3.xml
        Spec Sections:4.2 [72]
        Description:S is required after name
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p72fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op72fail4() {
    /*
        Test ID:o-p72fail4
        Test URI:p72fail4.xml
        Spec Sections:4.2 [72]
        Description:Entity name is a name, not an NMToken
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p72fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op73fail1() {
    /*
        Test ID:o-p73fail1
        Test URI:p73fail1.xml
        Spec Sections:4.2 [73]
        Description:No typed replacement text
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p73fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op73fail2() {
    /*
        Test ID:o-p73fail2
        Test URI:p73fail2.xml
        Spec Sections:4.2 [73]
        Description:Only one replacement value
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p73fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op73fail3() {
    /*
        Test ID:o-p73fail3
        Test URI:p73fail3.xml
        Spec Sections:4.2 [73]
        Description:No NDataDecl on replacement text
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p73fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op73fail4() {
    /*
        Test ID:o-p73fail4
        Test URI:p73fail4.xml
        Spec Sections:4.2 [73]
        Description:Value is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p73fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op73fail5() {
    /*
        Test ID:o-p73fail5
        Test URI:p73fail5.xml
        Spec Sections:4.2 [73]
        Description:No NDataDecl without value
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p73fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op74fail1() {
    /*
        Test ID:o-p74fail1
        Test URI:p74fail1.xml
        Spec Sections:4.2 [74]
        Description:no NDataDecls on parameter entities
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p74fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op74fail2() {
    /*
        Test ID:o-p74fail2
        Test URI:p74fail2.xml
        Spec Sections:4.2 [74]
        Description:value is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p74fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op74fail3() {
    /*
        Test ID:o-p74fail3
        Test URI:p74fail3.xml
        Spec Sections:4.2 [74]
        Description:only one value
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p74fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op75fail1() {
    /*
        Test ID:o-p75fail1
        Test URI:p75fail1.xml
        Spec Sections:4.2.2 [75]
        Description:S required after "PUBLIC"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op75fail2() {
    /*
        Test ID:o-p75fail2
        Test URI:p75fail2.xml
        Spec Sections:4.2.2 [75]
        Description:S required after "SYSTEM"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op75fail3() {
    /*
        Test ID:o-p75fail3
        Test URI:p75fail3.xml
        Spec Sections:4.2.2 [75]
        Description:S required between literals
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op75fail4() {
    /*
        Test ID:o-p75fail4
        Test URI:p75fail4.xml
        Spec Sections:4.2.2 [75]
        Description:"SYSTEM" implies only one literal
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op75fail5() {
    /*
        Test ID:o-p75fail5
        Test URI:p75fail5.xml
        Spec Sections:4.2.2 [75]
        Description:only one keyword
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75fail5.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op75fail6() {
    /*
        Test ID:o-p75fail6
        Test URI:p75fail6.xml
        Spec Sections:4.2.2 [75]
        Description:"PUBLIC" requires two literals (contrast with SGML)
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p75fail6.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op76fail1() {
    /*
        Test ID:o-p76fail1
        Test URI:p76fail1.xml
        Spec Sections:4.2.2 [76]
        Description:S is required before "NDATA"
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p76fail1.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op76fail2() {
    /*
        Test ID:o-p76fail2
        Test URI:p76fail2.xml
        Spec Sections:4.2.2 [76]
        Description:"NDATA" is upper-case
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p76fail2.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op76fail3() {
    /*
        Test ID:o-p76fail3
        Test URI:p76fail3.xml
        Spec Sections:4.2.2 [76]
        Description:notation name is required
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p76fail3.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}

#[test]
fn op76fail4() {
    /*
        Test ID:o-p76fail4
        Test URI:p76fail4.xml
        Spec Sections:4.2.2 [76]
        Description:notation names are Names
    */

    let testxml = Document::try_from((
        fs::read_to_string("tests/conformance/xml/xmlconf/oasis/p76fail4.xml").unwrap(),
        None,
        None,
    ));

    assert!(testxml.is_err());
}
