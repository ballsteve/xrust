use std::fs;
use std::rc::Rc;
//use xrust::item::Node;
use xrust::parser::{ParserConfig, xml};
use xrust::trees::smite::{Node as SmiteNode};
use xrust::validators::relaxng::validate_relaxng;


#[test]
#[ignore]
fn relaxng_incorrect_001_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/001/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_002_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/002/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_003_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/003/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_004_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/004/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_005_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/005/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_006_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/006/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_007_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/007/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_008_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/008/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_009_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/009/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_010_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/010/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_011_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/011/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_012_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/012/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_013_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/013/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_014_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/014/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_015_1(){

    /*
        Spec Sections: 3
        Description: Various possible syntax errors.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/015/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_016_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/016/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_017_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/017/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_018_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/018/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_019_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/019/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_020_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/020/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_021_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/021/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_022_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/022/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_023_1(){

    /*
        Spec Sections: 3
        Description: Tests for obsolete syntax
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/023/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_024_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/024/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_025_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/025/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_026_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/026/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_027_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/027/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_028_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/028/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_029_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/029/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_030_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/030/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_031_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/031/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_032_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/032/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_033_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/033/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_034_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/034/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_035_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/035/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_036_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/036/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_037_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/037/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_038_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/038/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_039_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/039/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_040_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/040/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_041_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/041/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_042_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/042/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_043_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/043/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_044_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/044/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_045_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/045/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_046_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/046/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_047_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/047/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_048_1(){

    /*
        Spec Sections: 3
        Description: Tests for missing attributes and child elements
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/048/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_049_1(){

    /*
        Spec Sections: 3
        Description: Checking of ns attribute
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/049/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/049/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_050_1(){

    /*
        Spec Sections: 3
        Description: Checking of ns attribute
        Description: No checking of ns attribute is performed
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/050/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/050/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_053_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/053/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_054_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/054/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/054/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_055_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/055/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/055/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_056_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/056/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_057_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/057/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_058_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/058/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_059_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must conform to RFC 2396
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/059/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/059/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_060_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must not be relative
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/060/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_061_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must not be relative
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/061/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_062_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must not be relative
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/062/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_063_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must not be relative
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/063/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_064_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/064/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/064/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_065_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/065/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/065/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_066_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/066/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/066/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_067_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must not contain fragment identifier
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/067/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_068_1(){

    /*
        Spec Sections: 3
        Description: Checking of datatypeLibrary attribute
        Description: Value of datatypeLibrary attribute must not contain fragment identifier
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/068/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_069_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/069/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/069/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_070_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/070/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_071_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/071/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_072_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/072/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_073_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/073/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_074_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/074/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_075_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/075/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/075/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_076_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/076/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_077_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/077/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_078_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/078/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_079_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/079/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_080_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/080/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_081_1(){

    /*
        Spec Sections: 3
        Description: Tests for QName and NCNames in schemas
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/081/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_082_1(){

    /*
        Spec Sections: 3
        Description: Tests for elements that allow only a single pattern child.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/082/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_083_1(){

    /*
        Spec Sections: 3
        Description: Tests for elements that allow only a single pattern child.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/083/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_084_1(){

    /*
        Spec Sections: 3
        Description: Tests for elements that allow only a single pattern child.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/084/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_085_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/085/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_086_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/086/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_087_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/087/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_088_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/088/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/088/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_089_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/089/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/089/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_090_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/090/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/090/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_091_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/091/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/091/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_092_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/092/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/092/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_093_1(){

    /*
        Spec Sections: 3
        Description: Tests for foreign element and attribute handling.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/093/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/093/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_094_1(){

    /*
        Spec Sections: 4.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/094/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/094/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_095_1(){

    /*
        Spec Sections: 4.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/095/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/095/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_095_2(){

    /*
        Spec Sections: 4.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/095/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/095/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_096_1(){

    /*
        Spec Sections: 4.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/096/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/096/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_097_1(){

    /*
        Spec Sections: 4.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/097/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/097/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_098_1(){

    /*
        Spec Sections: 4.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/098/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/098/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_099_1(){

    /*
        Spec Sections: 4.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_099_2(){

    /*
        Spec Sections: 4.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_099_3(){

    /*
        Spec Sections: 4.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_099_4(){

    /*
        Spec Sections: 4.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_099_5(){

    /*
        Spec Sections: 4.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/099/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_100_1(){

    /*
        Spec Sections: 4.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/100/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/100/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_100_2(){

    /*
        Spec Sections: 4.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/100/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/100/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_101_1(){

    /*
        Spec Sections: 4.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/101/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/101/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_101_2(){

    /*
        Spec Sections: 4.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/101/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/101/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_102_1(){

    /*
        Spec Sections: 4.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/102/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_103_1(){

    /*
        Spec Sections: 4.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/103/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/103/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_103_2(){

    /*
        Spec Sections: 4.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/103/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/103/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_104_1(){

    /*
        Spec Sections: 4.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/104/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/104/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_104_2(){

    /*
        Spec Sections: 4.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/104/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/104/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_105_1(){

    /*
        Spec Sections: 4.6
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/105/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_106_1(){

    /*
        Spec Sections: 4.6
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/106/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_107_1(){

    /*
        Spec Sections: 4.6
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/107/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_108_1(){

    /*
        Spec Sections: 4.6
        Description: Same value of href before resolution, but not a loop.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/108/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/108/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_108_2(){

    /*
        Spec Sections: 4.6
        Description: Same value of href before resolution, but not a loop.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/108/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/108/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_109_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/109/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/109/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_109_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/109/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/109/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_110_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/110/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/110/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_110_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/110/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/110/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_111_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/111/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/111/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_111_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/111/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/111/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_112_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/112/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_113_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/113/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_114_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/114/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_115_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/115/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/115/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_115_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/115/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/115/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_116_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/116/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_117_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/117/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/117/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_117_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/117/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/117/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_118_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/118/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_119_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/119/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/119/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_119_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/119/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/119/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_120_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/120/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/120/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_120_2(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/120/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/120/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_120_3(){

    /*
        Spec Sections: 4.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/120/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/120/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_121_1(){

    /*
        Spec Sections: 4.7
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/121/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_122_1(){

    /*
        Spec Sections: 4.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/122/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/122/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_122_2(){

    /*
        Spec Sections: 4.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/122/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/122/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_123_1(){

    /*
        Spec Sections: 4.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/123/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/123/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_123_2(){

    /*
        Spec Sections: 4.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/123/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/123/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_124_1(){

    /*
        Spec Sections: 4.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/124/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/124/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_124_2(){

    /*
        Spec Sections: 4.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/124/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/124/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_125_1(){

    /*
        Spec Sections: 4.6 4.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/125/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/125/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_125_2(){

    /*
        Spec Sections: 4.6 4.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/125/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/125/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_126_1(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/126/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/126/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_126_2(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/126/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/126/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_127_1(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/127/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/127/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_127_2(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/127/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/127/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_128_1(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/128/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/128/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_128_2(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/128/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/128/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_129_1(){

    /*
        Spec Sections: 4.10
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/129/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_130_1(){

    /*
        Spec Sections: 4.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/130/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/130/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_130_2(){

    /*
        Spec Sections: 4.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/130/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/130/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_131_1(){

    /*
        Spec Sections: 4.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/131/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/131/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_131_2(){

    /*
        Spec Sections: 4.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/131/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/131/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_132_1(){

    /*
        Spec Sections: 4.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/132/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/132/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_132_2(){

    /*
        Spec Sections: 4.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/132/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/132/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_133_1(){

    /*
        Spec Sections: 4.11
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/133/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/133/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_133_2(){

    /*
        Spec Sections: 4.11
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/133/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/133/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_134_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_134_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_134_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_134_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_134_5(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_134_6(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_134_7(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_134_8(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/134/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_135_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_135_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_135_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_135_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_135_5(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_135_6(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_135_7(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_135_8(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/135/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_136_1(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_136_2(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_136_3(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_136_4(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_136_5(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_136_6(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_136_7(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_136_8(){

    /*
        Spec Sections: 4.12 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/136/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_137_1(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_137_2(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_137_3(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_137_4(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_137_5(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_137_6(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_137_7(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_137_8(){

    /*
        Spec Sections: 4.12 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/137/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_138_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/138/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/138/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_138_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/138/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/138/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_139_1(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_139_2(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_139_3(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_139_4(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_139_5(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_139_6(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/6.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_139_7(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_139_8(){

    /*
        Spec Sections: 4.12 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/139/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_140_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_140_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_140_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_140_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_140_5(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_140_6(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_140_7(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_140_8(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/140/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_141_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_141_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_141_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_141_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/141/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_142_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_142_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_142_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_142_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_142_5(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/142/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_143_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/143/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/143/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_143_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/143/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/143/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_143_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/143/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/143/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_144_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/144/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/144/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_144_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/144/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/144/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_144_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/144/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/144/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_145_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_145_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_145_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_145_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_145_5(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_145_6(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_145_7(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_145_8(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/145/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_146_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_146_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_146_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_146_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/146/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_147_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_147_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_147_3(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_147_4(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_147_5(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_147_6(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/6.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_147_7(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_147_8(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/147/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_148_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/148/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/148/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_148_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/148/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/148/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_149_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/149/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/149/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_149_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/149/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/149/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_150_1(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/150/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/150/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_150_2(){

    /*
        Spec Sections: 4.12
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/150/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/150/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_151_1(){

    /*
        Spec Sections: 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_151_2(){

    /*
        Spec Sections: 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_151_3(){

    /*
        Spec Sections: 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_151_4(){

    /*
        Spec Sections: 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_151_5(){

    /*
        Spec Sections: 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_151_6(){

    /*
        Spec Sections: 4.13
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/151/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_152_1(){

    /*
        Spec Sections: 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_152_2(){

    /*
        Spec Sections: 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_152_3(){

    /*
        Spec Sections: 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_152_4(){

    /*
        Spec Sections: 4.14
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/152/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_153_1(){

    /*
        Spec Sections: 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_153_2(){

    /*
        Spec Sections: 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_153_3(){

    /*
        Spec Sections: 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_153_4(){

    /*
        Spec Sections: 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_153_5(){

    /*
        Spec Sections: 4.15
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/153/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_154_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/154/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_155_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/155/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_156_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/156/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_157_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/157/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_158_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/158/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_159_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/159/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_160_1(){

    /*
        Spec Sections: 4.16
        Description: Tests that 4.16 is before 4.20.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/160/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_161_1(){

    /*
        Spec Sections: 4.16
        Description: Tests that 4.16 is before removal of unreachable definitions.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/161/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_162_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/162/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_163_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/163/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/163/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_164_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/164/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_165_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/165/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_166_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/166/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_167_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/167/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_168_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/168/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_169_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/169/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_170_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/170/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_171_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/171/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_172_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/172/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_173_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/173/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_174_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/174/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_175_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/175/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_176_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/176/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/176/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_177_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/177/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_178_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/178/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_179_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/179/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_180_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/180/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_181_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/181/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_182_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/182/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_183_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/183/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_184_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/184/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_185_1(){

    /*
        Spec Sections: 4.16
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/185/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_186_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/186/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_187_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/187/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_188_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/188/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_189_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/189/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_190_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_190_2(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_190_3(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_190_4(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/190/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_191_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_191_2(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_191_3(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_191_4(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/191/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_192_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/192/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_193_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/193/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_194_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_194_2(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_194_3(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_194_4(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/194/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_195_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_195_2(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_195_3(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_195_4(){

    /*
        Spec Sections: 4.17
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/195/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_196_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/196/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_197_1(){

    /*
        Spec Sections: 4.17
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/197/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_198_1(){

    /*
        Spec Sections: 4.18
        Description: grammar must have a start
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/198/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_199_1(){

    /*
        Spec Sections: 4.18
        Description: 4.17 is before 4.18
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/199/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_200_1(){

    /*
        Spec Sections: 4.18
        Description: 4.17 is before 4.19
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/200/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_201_1(){

    /*
        Spec Sections: 4.18
        Description: every ref must have a def
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/201/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_202_1(){

    /*
        Spec Sections: 4.18
        Description: 4.17 is before 4.18
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/202/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_203_1(){

    /*
        Spec Sections: 4.18
        Description: 4.17 is before 4.19
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/203/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_204_1(){

    /*
        Spec Sections: 4.18
        Description: every parentRef must have a def
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/204/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_205_1(){

    /*
        Spec Sections: 4.18
        Description: 4.17 is before 4.18
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/205/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_206_1(){

    /*
        Spec Sections: 4.18
        Description: 4.17 is before 4.19
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/206/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_207_1(){

    /*
        Spec Sections: 4.18
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/207/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_208_1(){

    /*
        Spec Sections: 4.18
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/208/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/208/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_209_1(){

    /*
        Spec Sections: 4.18
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/209/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/209/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_209_2(){

    /*
        Spec Sections: 4.18
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/209/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/209/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_210_1(){

    /*
        Spec Sections: 4.18
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/210/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/210/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_210_2(){

    /*
        Spec Sections: 4.18
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/210/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/210/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_211_1(){

    /*
        Spec Sections: 4.19
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/211/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_212_1(){

    /*
        Spec Sections: 4.19
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/212/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/212/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_212_2(){

    /*
        Spec Sections: 4.19
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/212/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/212/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_212_3(){

    /*
        Spec Sections: 4.19
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/212/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/212/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_213_1(){

    /*
        Spec Sections: 4.19
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/213/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/213/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_214_1(){

    /*
        Spec Sections: 4.19 4.20
        Description: Tests that recursion detection happens before
normalization of notAllowed.
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/214/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_215_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/215/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/215/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_215_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/215/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/215/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_216_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/216/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/216/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_216_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/216/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/216/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_216_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/216/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/216/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_217_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/217/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/217/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_217_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/217/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/217/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_218_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/218/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/218/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_218_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/218/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/218/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_219_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_219_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_219_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_219_4(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/219/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_220_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/220/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/220/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_220_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/220/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/220/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_220_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/220/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/220/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_221_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_221_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_221_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_221_4(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/221/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_222_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_222_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_222_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_222_4(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/222/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_223_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_223_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_223_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_223_4(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/223/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_224_1(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/224/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/224/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_224_2(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/224/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/224/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_224_3(){

    /*
        Spec Sections: 6.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/224/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/224/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_225_1(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/225/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/225/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_225_2(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/225/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/225/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_225_3(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/225/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/225/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_226_1(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_226_2(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_226_3(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_226_4(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_226_5(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_226_6(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_226_7(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/226/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_227_1(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_227_2(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_227_3(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_227_4(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_227_5(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_227_6(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_227_7(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/227/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_228_1(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_228_2(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_228_3(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_228_4(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_228_5(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_228_6(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_228_7(){

    /*
        Spec Sections: 6.2.1
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/228/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_229_1(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_229_2(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_229_3(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_229_4(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_229_5(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_229_6(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/229/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_230_1(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/230/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/230/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_230_2(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/230/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/230/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_231_1(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_231_2(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_231_3(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_231_4(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/231/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_232_1(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_232_2(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_232_3(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_232_4(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/232/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_233_1(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_233_2(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_233_3(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_233_4(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/233/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_234_1(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_234_2(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_234_3(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_234_4(){

    /*
        Spec Sections: 6.2.2
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/234/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_235_1(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_235_2(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_235_3(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_235_4(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_235_5(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_235_6(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_235_7(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/235/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_236_1(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/236/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/236/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_236_2(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/236/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/236/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_236_3(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/236/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/236/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_237_1(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_237_2(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_237_3(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_237_4(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_237_5(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/237/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_238_1(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/238/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/238/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_238_2(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/238/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/238/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_238_3(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/238/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/238/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_239_1(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_239_2(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_239_3(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_239_4(){

    /*
        Spec Sections: 6.2.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/239/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_240_1(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_240_2(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_240_3(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_240_4(){

    /*
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/240/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_241_1(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_241_2(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_241_3(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_241_4(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_241_5(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/241/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_242_1(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_242_2(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_242_3(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_242_4(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_242_5(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_242_6(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/242/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_243_1(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_243_2(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_243_3(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_243_4(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_243_5(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_243_6(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/6.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/243/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_244_1(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_244_2(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_244_3(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_244_4(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_244_5(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_244_6(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/6.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_244_7(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/7.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_244_8(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/244/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_245_1(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_245_2(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_245_3(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_245_4(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_245_5(){

    /*
        Spec Sections: 6.2.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/245/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_246_1(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_246_2(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_246_3(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_246_4(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/246/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_247_1(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_247_2(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_247_3(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_247_4(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_247_5(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_247_6(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/247/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_248_1(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_248_2(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_248_3(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_248_4(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_248_5(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_248_6(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/248/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_249_1(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_249_2(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_249_3(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_249_4(){

    /*
        Spec Sections: 6.2.5
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/249/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_250_1(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_250_2(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_250_3(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_250_4(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_250_5(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_250_6(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/250/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_251_1(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_251_2(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_251_3(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_251_4(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_251_5(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_251_6(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_251_7(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_251_8(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/251/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_252_1(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_252_2(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_252_3(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_252_4(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_252_5(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/252/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_253_1(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_253_2(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_253_3(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_253_4(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_253_5(){

    /*
        Spec Sections: 6.2.6
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/253/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_254_1(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_254_2(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_254_3(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_254_4(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_254_5(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_254_6(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/254/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_255_1(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_255_2(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_255_3(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_255_4(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_255_5(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_255_6(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_255_7(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/255/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_256_1(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/256/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/256/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_256_2(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/256/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/256/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_256_3(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/256/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/256/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_257_1(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_257_2(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_257_3(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_257_4(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_257_5(){

    /*
        Spec Sections: 6.2.7
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/257/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_258_1(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/258/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/258/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_258_2(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/258/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/258/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_258_3(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/258/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/258/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_259_1(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/259/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/259/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_259_2(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/259/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/259/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_259_3(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/259/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/259/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_260_1(){

    /*
        Spec Sections: 6.2.7 6.2.8 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_260_2(){

    /*
        Spec Sections: 6.2.7 6.2.8 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_260_3(){

    /*
        Spec Sections: 6.2.7 6.2.8 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_260_4(){

    /*
        Spec Sections: 6.2.7 6.2.8 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_260_5(){

    /*
        Spec Sections: 6.2.7 6.2.8 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/260/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_261_1(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_261_2(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_261_3(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_261_4(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_261_5(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_261_6(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_261_7(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/261/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_262_1(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_262_2(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_262_3(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_262_4(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_262_5(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_262_6(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_262_7(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_262_8(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/262/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_263_1(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_263_2(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_263_3(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_263_4(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_263_5(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_263_6(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/6.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_263_7(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_263_8(){

    /*
        Spec Sections: 6.2.7 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/8.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/263/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_264_1(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_264_2(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_264_3(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_264_4(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/264/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_265_1(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/265/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/265/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_266_1(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/266/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/266/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_267_1(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/267/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/267/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_267_2(){

    /*
        Spec Sections: 6.2.8
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/267/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/267/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_268_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_268_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_268_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_268_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_268_5(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_268_6(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/268/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_269_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_269_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_269_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_269_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_269_5(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_269_6(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/269/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_270_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_270_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_270_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_270_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/270/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_271_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_271_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_271_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_271_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/271/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_272_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_272_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_272_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_272_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_272_5(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_272_6(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/272/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_273_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_273_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_273_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_273_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/273/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_274_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_274_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_274_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_274_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_274_5(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_274_6(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/274/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_275_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_275_2(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_275_3(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_275_4(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/4.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_275_5(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/5.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_275_6(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_275_7(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/275/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_276_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/276/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_277_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/277/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_278_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/278/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_279_1(){

    /*
        Spec Sections: 6.2.9
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/279/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_280_1(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/280/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/280/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_280_2(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/280/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/280/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_280_3(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/280/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/280/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_281_1(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_281_2(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_281_3(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_281_4(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/281/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_282_1(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_282_2(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_282_3(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_282_4(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/282/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_283_1(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/283/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/283/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_283_2(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/283/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/283/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_283_3(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/283/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/283/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_284_1(){

    /*
        Spec Sections: 6.2.10
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/284/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/284/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_285_1(){

    /*
        Spec Sections: 7.1.1
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/285/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_286_1(){

    /*
        Spec Sections: 7.1.1
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/286/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_287_1(){

    /*
        Spec Sections: 7.1.1
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/287/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_288_1(){

    /*
        Spec Sections: 7.1.1
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/288/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_289_1(){

    /*
        Spec Sections: 7.1.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/289/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_290_1(){

    /*
        Spec Sections: 7.1.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/290/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_291_1(){

    /*
        Spec Sections: 7.1.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/291/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_292_1(){

    /*
        Spec Sections: 7.1.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/292/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_293_1(){

    /*
        Spec Sections: 7.1.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/293/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_294_1(){

    /*
        Spec Sections: 7.1.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/294/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_295_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/295/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_296_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/296/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_297_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/297/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_298_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/298/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_299_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/299/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_300_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/300/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_301_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/301/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_302_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/302/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_303_1(){

    /*
        Spec Sections: 7.1.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/303/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_304_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/304/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_305_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/305/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_306_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/306/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_307_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/307/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_308_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/308/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_309_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/309/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_310_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/310/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_311_1(){

    /*
        Spec Sections: 7.1.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/311/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_312_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/312/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_313_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/313/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_314_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/314/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_315_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/315/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_316_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/316/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_317_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/317/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_318_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/318/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_319_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/319/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_320_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/320/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_321_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/321/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_322_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/322/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_323_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/323/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_324_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/324/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_325_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/325/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_326_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/326/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_327_1(){

    /*
        Spec Sections: 7.1.5
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/327/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_328_1(){

    /*
        Spec Sections: 7.1.5 7 4.18
        Description: Tests that constraints are post-normalization
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/328/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/328/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_329_1(){

    /*
        Spec Sections: 7.1.5 7 4.18
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/329/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_330_1(){

    /*
        Spec Sections: 7.1.1 7 4.20
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/330/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/330/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_331_1(){

    /*
        Spec Sections: 7.1.1 7 4.20
        Description: The nested attribute element is normalized out because
of the not allowed.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/331/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/331/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_332_1(){

    /*
        Spec Sections: 7.1.2 7 4.12
        Description: The group element is normalized out.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/332/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/332/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_333_1(){

    /*
        Spec Sections: 7.1.2 7 4.21
        Description: The group element is normalized out.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/333/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/333/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_334_1(){

    /*
        Spec Sections: 7.1.2 7 4.20
        Description: The attribute elements are all normalized out.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/334/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/334/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_335_1(){

    /*
        Spec Sections: 7.2
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/335/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_336_1(){

    /*
        Spec Sections: 7.2 4.20
        Description: Checks that normalization of notAllowed happens
before string sequence checking.
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/336/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/336/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_337_1(){

    /*
        Spec Sections: 4.20 7.2
        Description: notAllowed in an element is not normalized
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/337/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_338_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/338/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_339_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/339/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_340_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/340/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/340/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_341_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/341/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_342_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/342/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_343_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/343/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_344_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/344/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_345_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_345_2(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_345_3(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_345_4(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_345_5(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/345/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_346_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/346/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_347_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/347/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_348_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/348/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_349_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/349/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_350_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/350/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_351_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/351/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_352_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/352/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_353_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/353/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/353/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_353_2(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/353/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/353/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_353_3(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/353/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/353/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_354_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/354/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/354/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_355_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/355/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/355/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_356_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/356/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_357_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/357/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_358_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/358/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_359_1(){

    /*
        Spec Sections: 7.3
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/359/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_360_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/360/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_361_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/361/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_362_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/362/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_363_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/363/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_364_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/364/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_365_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/365/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_366_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/366/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_367_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/367/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_368_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/368/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/368/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_369_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/369/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/369/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_incorrect_370_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/370/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_incorrect_371_1(){

    /*
        Spec Sections: 7.4
    */

    let docfile = "<doc/>".to_string();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/371/i.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_372_1(){

    /*
        Description: Regressions
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/372/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/372/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_373_1(){

    /*
        Description: Regressions
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/373/1.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/373/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_374_1(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_374_2(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_3(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_4(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_5(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_6(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_7(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_8(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/8.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_374_9(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/9.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/374/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_375_1(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/375/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/375/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_375_2(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/375/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/375/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_376_1(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/376/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/376/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_376_2(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/376/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/376/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_376_3(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/376/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/376/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_377_1(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/377/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/377/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_377_2(){

    /*
        Description: Validation error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/377/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/377/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_378_1(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_378_2(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_378_3(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_378_4(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/378/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_379_1(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_379_2(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_379_3(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_379_4(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_379_5(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/379/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_380_1(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_380_2(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_380_3(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_380_4(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_380_5(){

    /*
        Description: Datatype problems
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/380/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_381_1(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/381/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/381/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_381_2(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/381/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/381/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_381_3(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/381/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/381/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_382_1(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_382_2(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_382_3(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_382_4(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/382/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_383_1(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_valid_383_2(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/2.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_383_3(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/3.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_383_4(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_383_5(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/383/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_384_1(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_384_2(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_384_3(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/3.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_384_4(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/4.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_384_5(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/5.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_384_6(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/6.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_invalid_384_7(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/7.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/384/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

#[test]
#[ignore]
fn relaxng_valid_385_1(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/385/1.v.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/385/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_ok());

}

#[test]
#[ignore]
fn relaxng_invalid_385_2(){

    /*
        Description: Datatype error reporting
    */

    let docfile = fs::read_to_string("tests/conformance/relaxng/jamesclark/385/2.i.xml").unwrap();
    let doc = Rc::new(SmiteNode::new());
    let _ = xml::parse(doc.clone(), docfile.as_str(), None);

    let schemafile = fs::read_to_string("tests/conformance/relaxng/jamesclark/385/c.rng").unwrap();
    let sch = Rc::new(SmiteNode::new());
    let _ = xml::parse(sch.clone(), schemafile.as_str(), None);

    let result = validate_relaxng(&doc, &sch);
    assert!(result.is_err());

}

