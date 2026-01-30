/*

University of Edinburgh XML 1.0 4th edition errata test suite.

*/
use qualname::NamespacePrefix;
use std::fs;
use xrust::item::{Node, NodeType};
use xrust::parser::xml;
use xrust::parser::{ParseError, ParserStateBuilder, StaticStateBuilder};
use xrust::trees::smite::RNode;

#[test]
fn parser_config_namespace_nodes_1() {
    let doc = r#"<doc xmlns="namespace"
                  xmlns:a="namespace1"
                  xmlns:b="namespace2"
                  xmlns:c="namespace3"
                  xmlns:d="namespace4"
                  xmlns:e="namespace5"
             >
                 <element1/>
                 <element2 xmlns="namespace6"/>
                 <element3 xmlns:f="namespace7"/>
                 <element4 xmlns:f="namespace8"/>
                 <element5>
                     <element6/>
                 </element5>
             </doc>"#;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());

    let doc = parseresult.clone().unwrap().first_child().unwrap();
    let mut docchildren = doc
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element);
    let element1 = docchildren.next().unwrap();
    let element2 = docchildren.next().unwrap();
    let element3 = docchildren.next().unwrap();
    let element4 = docchildren.next().unwrap();
    let element5 = docchildren.next().unwrap();
    let element6 = element5
        .child_iter()
        .filter(|n| n.node_type() == NodeType::Element)
        .next()
        .unwrap();

    assert_eq!(doc.namespace_iter().count(), 7);
    assert_eq!(element1.namespace_iter().count(), 7);
    assert_eq!(element2.namespace_iter().count(), 8);
    assert_eq!(element3.namespace_iter().count(), 8);
    assert_eq!(element4.namespace_iter().count(), 8);
    assert_eq!(element5.namespace_iter().count(), 7);
    assert_eq!(element6.namespace_iter().count(), 7);
}

#[test]
fn parser_config_default_attrs_1() {
    /*
        Conformance tests will determine if the ATTLIST functions are working,
        this tests only that it can be disabled.
    */
    let doc = r#"<!DOCTYPE doc [
        <!ELEMENT doc EMPTY>
        <!ATTLIST doc a CDATA "a" b CDATA "b" c CDATA #IMPLIED>
    ]>
    <doc/>"#;

    let testxml1 = RNode::new_document();
    let parseresult1 = xml::parse(
        testxml1,
        doc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    let testxml2 = RNode::new_document();
    let pc2 = ParserStateBuilder::new()
        .attribute_defaults(false)
        .doc(testxml2)
        .build();
    let ss = StaticStateBuilder::new()
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();
    let parseresult2 = xml::parse_with_state(doc, pc2, ss);

    assert!(parseresult1.is_ok());
    assert!(parseresult2.is_ok());

    assert_eq!(
        parseresult1
            .clone()
            .unwrap()
            .first_child()
            .unwrap()
            .attribute_iter()
            .count(),
        2
    );
    assert_eq!(
        parseresult2
            .clone()
            .unwrap()
            .first_child()
            .unwrap()
            .attribute_iter()
            .count(),
        0
    );
}

#[test]
fn parser_issue_94() {
    /*
        Github issue number 94

        Although rare, UTF-8 strings can start with a byte order mark, we strip this automatically.
    */

    let data = fs::read_to_string("tests/xml/issue-94.xml").unwrap();
    let source = RNode::new_document();

    let parseresult = xml::parse(
        source.clone(),
        &data,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok())
}

#[test]
fn parser_config_id_1() {
    /*
        Conformance tests will determine if the XML IDs are properly tracked by the parser,
        this tests only that it can be disabled.
    */

    let doc = r#"<!DOCTYPE root [
    <!ELEMENT doc ANY>
    <!ELEMENT element1 EMPTY>
    <!ATTLIST element1
	id	ID	#IMPLIED
	idref	IDREF	#IMPLIED
	>
]>
<doc>
    <element1 id="a1"/>
    <element1 id="a1"/>
</doc>
"#;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|_: &_| {
            Err(ParseError::NotWellFormed(
                "tried to resolve namespace prefix".to_string(),
            ))
        }),
    );

    eprintln!("parse1result {:?}", parseresult);
    assert!(parseresult.is_err());

    let testxml2 = RNode::new_document();
    let pc2 = ParserStateBuilder::new()
        .id_tracking(false)
        .doc(testxml2)
        .build();
    let ss = StaticStateBuilder::new()
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let parseresult2 = xml::parse_with_state(doc, pc2, ss);

    eprintln!("parseresult2 result {:?}", parseresult2);
    assert!(parseresult2.is_ok());
}

#[test]
fn parser_config_id_2() {
    /*
        Conformance tests will determine if the XML IDs are properly tracked by the parser,
        this tests only that it can be disabled.
    */

    let doc = r#"<!DOCTYPE root [
    <!ELEMENT doc ANY>
    <!ELEMENT element1 EMPTY>
    <!ATTLIST element1
	id	ID	#IMPLIED
	idref	IDREF	#IMPLIED
	>
]>
<doc>
    <element1 idref="a1"/>
</doc>
"#;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_err());

    let testxml2 = RNode::new_document();
    let pc2 = ParserStateBuilder::new()
        .id_tracking(false)
        .doc(testxml2)
        .build();
    let ss = StaticStateBuilder::new()
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let parseresult2 = xml::parse_with_state(doc, pc2, ss);

    assert!(parseresult2.is_ok());
}

#[test]
fn parser_config_id_3() {
    /*
        Conformance tests will determine if the XML IDs are properly tracked by the parser,
        this tests only that it can be disabled.

        When we disable XML ID tracking, the is-id and is-idrefs properties will not populate.
    */

    let doc = r#"<!DOCTYPE root [
    <!ELEMENT doc ANY>
    <!ATTLIST doc
	id	ID	#IMPLIED
	idref	IDREF	#IMPLIED
	>
]>
<doc id="a1"/>
"#;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert_eq!(
        parseresult
            .unwrap()
            .first_child()
            .unwrap()
            .attribute_iter()
            .next()
            .unwrap()
            .is_id(),
        true
    );

    let testxml2 = RNode::new_document();
    let pc2 = ParserStateBuilder::new()
        .doc(testxml2)
        .id_tracking(false)
        .build();
    let ss = StaticStateBuilder::new()
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();

    let parseresult2 = xml::parse_with_state(doc, pc2, ss);

    assert!(parseresult2.is_ok());
    assert_eq!(
        parseresult2
            .unwrap()
            .first_child()
            .unwrap()
            .attribute_iter()
            .next()
            .unwrap()
            .is_id(),
        false
    );
}

#[test]
fn parser_config_id_4() {
    /*
        Conformance tests will determine if the XML IDs are properly tracked by the parser,
        this tests only that it can be disabled.

        When we disable XML ID tracking, the is-id and is-idrefs properties will not populate.
    */

    let doc = r#"<!DOCTYPE root [
    <!ELEMENT doc ANY>
    <!ATTLIST doc
	idref	IDREF	#IMPLIED
	>
	<!ELEMENT element1 EMPTY>
    <!ATTLIST element1
	id	ID	#IMPLIED
	>
]>
<doc idref="a1">
    <element1 id="a1"/>
</doc>
"#;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
    assert_eq!(
        parseresult
            .unwrap()
            .first_child()
            .unwrap()
            .attribute_iter()
            .next()
            .unwrap()
            .is_idrefs(),
        true
    );

    let testxml2 = RNode::new_document();
    let pc2 = ParserStateBuilder::new()
        .doc(testxml2)
        .id_tracking(false)
        .build();
    let ss = StaticStateBuilder::new()
        .namespace(|_: &_| Err(ParseError::MissingNameSpace))
        .build();
    let parseresult2 = xml::parse_with_state(doc, pc2, ss);

    assert!(parseresult2.is_ok());
    assert_eq!(
        parseresult2
            .unwrap()
            .first_child()
            .unwrap()
            .attribute_iter()
            .next()
            .unwrap()
            .is_idrefs(),
        false
    );
}

#[test]
fn parser_issue_132_full() {
    let doc = r###"<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform" xmlns:mml="http://www.w3.org/1998/Math/MathML"
	xmlns:m="http://schemas.openxmlformats.org/officeDocument/2006/math">
      <xsl:output method="xml" encoding="UTF-16" />

      <!-- %% Global Definitions -->

      <!-- Every single unicode character that is recognized by OMML as an operator -->
      <xsl:variable name="sOperators"
		select="concat(
              '&#x00A8;&#x0021;&#x0022;&#x0023;&#x0026;&#x0028;&#x0029;&#x002B;&#x002C;&#x002D;&#x002E;&#x002F;&#x003A;',
              '&#x003B;&#x003C;&#x003D;&#x003E;&#x003F;&#x0040;&#x005B;&#x005C;&#x005D;&#x005E;&#x005F;&#x0060;&#x007B;',
              '&#x007C;&#x007D;&#x007E;&#x00A1;&#x00A6;&#x00AC;&#x00AF;&#x00B0;&#x00B1;&#x00B2;&#x00B3;&#x00B4;&#x00B7;&#x00B9;&#x00BF;',
              '&#x00D7;&#x007E;&#x00F7;&#x02C7;&#x02D8;&#x02D9;&#x02DC;&#x02DD;&#x0300;&#x0301;&#x0302;&#x0303;&#x0304;&#x0305;&#x0306;&#x0307;&#x0308;&#x0309;',
              '&#x030A;&#x030B;&#x030C;&#x030D;&#x030E;&#x030F;&#x0310;&#x0311;&#x0312;&#x0313;&#x0314;&#x0315;',
              '&#x0316;&#x0317;&#x0318;&#x0319;&#x031A;&#x031B;&#x031C;&#x031D;&#x031E;&#x031F;&#x0320;&#x0321;',
              '&#x0322;&#x0323;&#x0324;&#x0325;&#x0326;&#x0327;&#x0328;&#x0329;&#x032A;&#x032B;&#x032C;&#x032D;',
              '&#x032E;&#x032F;&#x0330;&#x0331;&#x0332;&#x0333;&#x0334;&#x0335;&#x0336;&#x0337;&#x0338;&#x033F;',
              '&#x2000;&#x2001;&#x2002;&#x2003;&#x2004;&#x2005;&#x2006;&#x2009;&#x200A;&#x2010;&#x2012;&#x2013;',
              '&#x2014;&#x2016;&#x2020;&#x2021;&#x2022;&#x2024;&#x2025;&#x2026;&#x2032;&#x2033;&#x2034;&#x203C;',
              '&#x2040;&#x2044;&#x204E;&#x204F;&#x2050;&#x2057;&#x2061;&#x2062;&#x2063;&#x2070;&#x2074;&#x2075;',
              '&#x2076;&#x2077;&#x2078;&#x2079;&#x207A;&#x207B;&#x207C;&#x207D;&#x207E;&#x2080;&#x2081;&#x2082;',
              '&#x2083;&#x2084;&#x2085;&#x2086;&#x2087;&#x2088;&#x2089;&#x208A;&#x208B;&#x208C;&#x208D;&#x208E;',
              '&#x20D0;&#x20D1;&#x20D2;&#x20D3;&#x20D4;&#x20D5;&#x20D6;&#x20D7;&#x20D8;&#x20D9;&#x20DA;&#x20DB;',
              '&#x20DC;&#x20DD;&#x20DE;&#x20DF;&#x20E0;&#x20E1;&#x20E4;&#x20E5;&#x20E6;&#x20E7;&#x20E8;&#x20E9;',
              '&#x20EA;&#x2140;&#x2146;&#x2190;&#x2191;&#x2192;&#x2193;&#x2194;&#x2195;&#x2196;&#x2197;&#x2198;&#x2199;',
              '&#x219A;&#x219B;&#x219C;&#x219D;&#x219E;&#x219F;&#x21A0;&#x21A1;&#x21A2;&#x21A3;&#x21A4;&#x21A5;',
              '&#x21A6;&#x21A7;&#x21A8;&#x21A9;&#x21AA;&#x21AB;&#x21AC;&#x21AD;&#x21AE;&#x21AF;&#x21B0;&#x21B1;',
              '&#x21B2;&#x21B3;&#x21B6;&#x21B7;&#x21BA;&#x21BB;&#x21BC;&#x21BD;&#x21BE;&#x21BF;&#x21C0;&#x21C1;',
              '&#x21C2;&#x21C3;&#x21C4;&#x21C5;&#x21C6;&#x21C7;&#x21C8;&#x21C9;&#x21CA;&#x21CB;&#x21CC;&#x21CD;',
              '&#x21CE;&#x21CF;&#x21D0;&#x21D1;&#x21D2;&#x21D3;&#x21D4;&#x21D5;&#x21D6;&#x21D7;&#x21D8;&#x21D9;',
              '&#x21DA;&#x21DB;&#x21DC;&#x21DD;&#x21DE;&#x21DF;&#x21E0;&#x21E1;&#x21E2;&#x21E3;&#x21E4;&#x21E5;',
              '&#x21E6;&#x21E7;&#x21E8;&#x21E9;&#x21F3;&#x21F4;&#x21F5;&#x21F6;&#x21F7;&#x21F8;&#x21F9;&#x21FA;',
              '&#x21FB;&#x21FC;&#x21FD;&#x21FE;&#x21FF;&#x2200;&#x2201;&#x2202;&#x2203;&#x2204;&#x2206;&#x2207;',
              '&#x2208;&#x2209;&#x220A;&#x220B;&#x220C;&#x220D;&#x220F;&#x2210;&#x2211;&#x2212;&#x2213;&#x2214;',
              '&#x2215;&#x2216;&#x2217;&#x2218;&#x2219;&#x221A;&#x221B;&#x221C;&#x221D;&#x2223;&#x2224;&#x2225;',
              '&#x2226;&#x2227;&#x2228;&#x2229;&#x222A;&#x222B;&#x222C;&#x222D;&#x222E;&#x222F;&#x2230;&#x2231;',
              '&#x2232;&#x2233;&#x2234;&#x2235;&#x2236;&#x2237;&#x2238;&#x2239;&#x223A;&#x223B;&#x223C;&#x223D;',
              '&#x223E;&#x2240;&#x2241;&#x2242;&#x2243;&#x2244;&#x2245;&#x2246;&#x2247;&#x2248;&#x2249;&#x224A;',
              '&#x224B;&#x224C;&#x224D;&#x224E;&#x224F;&#x2250;&#x2251;&#x2252;&#x2253;&#x2254;&#x2255;&#x2256;',
              '&#x2257;&#x2258;&#x2259;&#x225A;&#x225B;&#x225C;&#x225D;&#x225E;&#x225F;&#x2260;&#x2261;&#x2262;',
              '&#x2263;&#x2264;&#x2265;&#x2266;&#x2267;&#x2268;&#x2269;&#x226A;&#x226B;&#x226C;&#x226D;&#x226E;',
              '&#x226F;&#x2270;&#x2271;&#x2272;&#x2273;&#x2274;&#x2275;&#x2276;&#x2277;&#x2278;&#x2279;&#x227A;',
              '&#x227B;&#x227C;&#x227D;&#x227E;&#x227F;&#x2280;&#x2281;&#x2282;&#x2283;&#x2284;&#x2285;&#x2286;',
              '&#x2287;&#x2288;&#x2289;&#x228A;&#x228B;&#x228C;&#x228D;&#x228E;&#x228F;&#x2290;&#x2291;&#x2292;',
              '&#x2293;&#x2294;&#x2295;&#x2296;&#x2297;&#x2298;&#x2299;&#x229A;&#x229B;&#x229C;&#x229D;&#x229E;',
              '&#x229F;&#x22A0;&#x22A1;&#x22A2;&#x22A3;&#x22A5;&#x22A6;&#x22A7;&#x22A8;&#x22A9;&#x22AA;&#x22AB;',
              '&#x22AC;&#x22AD;&#x22AE;&#x22AF;&#x22B0;&#x22B1;&#x22B2;&#x22B3;&#x22B4;&#x22B5;&#x22B6;&#x22B7;',
              '&#x22B8;&#x22B9;&#x22BA;&#x22BB;&#x22BC;&#x22BD;&#x22C0;&#x22C1;&#x22C2;&#x22C3;&#x22C4;&#x22C5;',
              '&#x22C6;&#x22C7;&#x22C8;&#x22C9;&#x22CA;&#x22CB;&#x22CC;&#x22CD;&#x22CE;&#x22CF;&#x22D0;&#x22D1;',
              '&#x22D2;&#x22D3;&#x22D4;&#x22D5;&#x22D6;&#x22D7;&#x22D8;&#x22D9;&#x22DA;&#x22DB;&#x22DC;&#x22DD;',
              '&#x22DE;&#x22DF;&#x22E0;&#x22E1;&#x22E2;&#x22E3;&#x22E4;&#x22E5;&#x22E6;&#x22E7;&#x22E8;&#x22E9;',
              '&#x22EA;&#x22EB;&#x22EC;&#x22ED;&#x22EE;&#x22EF;&#x22F0;&#x22F1;&#x22F2;&#x22F3;&#x22F4;&#x22F5;',
              '&#x22F6;&#x22F7;&#x22F8;&#x22F9;&#x22FA;&#x22FB;&#x22FC;&#x22FD;&#x22FE;&#x22FF;&#x2305;&#x2306;',
              '&#x2308;&#x2309;&#x230A;&#x230B;&#x231C;&#x231D;&#x231E;&#x231F;&#x2322;&#x2323;&#x2329;&#x232A;',
              '&#x233D;&#x233F;&#x23B0;&#x23B1;&#x23DC;&#x23DD;&#x23DE;&#x23DF;&#x23E0;&#x2502;&#x251C;&#x2524;',
              '&#x252C;&#x2534;&#x2581;&#x2588;&#x2592;&#x25A0;&#x25A1;&#x25AD;&#x25B2;&#x25B3;&#x25B4;&#x25B5;',
              '&#x25B6;&#x25B7;&#x25B8;&#x25B9;&#x25BC;&#x25BD;&#x25BE;&#x25BF;&#x25C0;&#x25C1;&#x25C2;&#x25C3;',
              '&#x25C4;&#x25C5;&#x25CA;&#x25CB;&#x25E6;&#x25EB;&#x25EC;&#x25F8;&#x25F9;&#x25FA;&#x25FB;&#x25FC;',
              '&#x25FD;&#x25FE;&#x25FF;&#x2605;&#x2606;&#x2772;&#x2773;&#x27D1;&#x27D2;&#x27D3;&#x27D4;&#x27D5;',
              '&#x27D6;&#x27D7;&#x27D8;&#x27D9;&#x27DA;&#x27DB;&#x27DC;&#x27DD;&#x27DE;&#x27DF;&#x27E0;&#x27E1;',
              '&#x27E2;&#x27E3;&#x27E4;&#x27E5;&#x27E6;&#x27E7;&#x27E8;&#x27E9;&#x27EA;&#x27EB;&#x27F0;&#x27F1;',
              '&#x27F2;&#x27F3;&#x27F4;&#x27F5;&#x27F6;&#x27F7;&#x27F8;&#x27F9;&#x27FA;&#x27FB;&#x27FC;&#x27FD;',
              '&#x27FE;&#x27FF;&#x2900;&#x2901;&#x2902;&#x2903;&#x2904;&#x2905;&#x2906;&#x2907;&#x2908;&#x2909;',
              '&#x290A;&#x290B;&#x290C;&#x290D;&#x290E;&#x290F;&#x2910;&#x2911;&#x2912;&#x2913;&#x2914;&#x2915;',
              '&#x2916;&#x2917;&#x2918;&#x2919;&#x291A;&#x291B;&#x291C;&#x291D;&#x291E;&#x291F;&#x2920;&#x2921;',
              '&#x2922;&#x2923;&#x2924;&#x2925;&#x2926;&#x2927;&#x2928;&#x2929;&#x292A;&#x292B;&#x292C;&#x292D;',
              '&#x292E;&#x292F;&#x2930;&#x2931;&#x2932;&#x2933;&#x2934;&#x2935;&#x2936;&#x2937;&#x2938;&#x2939;',
              '&#x293A;&#x293B;&#x293C;&#x293D;&#x293E;&#x293F;&#x2940;&#x2941;&#x2942;&#x2943;&#x2944;&#x2945;',
              '&#x2946;&#x2947;&#x2948;&#x2949;&#x294A;&#x294B;&#x294C;&#x294D;&#x294E;&#x294F;&#x2950;&#x2951;',
              '&#x2952;&#x2953;&#x2954;&#x2955;&#x2956;&#x2957;&#x2958;&#x2959;&#x295A;&#x295B;&#x295C;&#x295D;',
              '&#x295E;&#x295F;&#x2960;&#x2961;&#x2962;&#x2963;&#x2964;&#x2965;&#x2966;&#x2967;&#x2968;&#x2969;',
              '&#x296A;&#x296B;&#x296C;&#x296D;&#x296E;&#x296F;&#x2970;&#x2971;&#x2972;&#x2973;&#x2974;&#x2975;',
              '&#x2976;&#x2977;&#x2978;&#x2979;&#x297A;&#x297B;&#x297C;&#x297D;&#x297E;&#x297F;&#x2980;&#x2982;',
              '&#x2983;&#x2984;&#x2985;&#x2986;&#x2987;&#x2988;&#x2989;&#x298A;&#x298B;&#x298C;&#x298D;&#x298E;',
              '&#x298F;&#x2990;&#x2991;&#x2992;&#x2993;&#x2994;&#x2995;&#x2996;&#x2997;&#x2998;&#x2999;&#x299A;',
              '&#x29B6;&#x29B7;&#x29B8;&#x29B9;&#x29C0;&#x29C1;&#x29C4;&#x29C5;&#x29C6;&#x29C7;&#x29C8;&#x29CE;',
              '&#x29CF;&#x29D0;&#x29D1;&#x29D2;&#x29D3;&#x29D4;&#x29D5;&#x29D6;&#x29D7;&#x29D8;&#x29D9;&#x29DA;',
              '&#x29DB;&#x29DF;&#x29E1;&#x29E2;&#x29E3;&#x29E4;&#x29E5;&#x29E6;&#x29EB;&#x29F4;&#x29F5;&#x29F6;',
              '&#x29F7;&#x29F8;&#x29F9;&#x29FA;&#x29FB;&#x29FC;&#x29FD;&#x29FE;&#x29FF;&#x2A00;&#x2A01;&#x2A02;',
              '&#x2A03;&#x2A04;&#x2A05;&#x2A06;&#x2A07;&#x2A08;&#x2A09;&#x2A0A;&#x2A0B;&#x2A0C;&#x2A0D;&#x2A0E;',
              '&#x2A0F;&#x2A10;&#x2A11;&#x2A12;&#x2A13;&#x2A14;&#x2A15;&#x2A16;&#x2A17;&#x2A18;&#x2A19;&#x2A1A;',
              '&#x2A1B;&#x2A1C;&#x2A1D;&#x2A1E;&#x2A1F;&#x2A20;&#x2A21;&#x2A22;&#x2A23;&#x2A24;&#x2A25;&#x2A26;',
              '&#x2A27;&#x2A28;&#x2A29;&#x2A2A;&#x2A2B;&#x2A2C;&#x2A2D;&#x2A2E;&#x2A2F;&#x2A30;&#x2A31;&#x2A32;',
              '&#x2A33;&#x2A34;&#x2A35;&#x2A36;&#x2A37;&#x2A38;&#x2A39;&#x2A3A;&#x2A3B;&#x2A3C;&#x2A3D;&#x2A3E;',
              '&#x2A3F;&#x2A40;&#x2A41;&#x2A42;&#x2A43;&#x2A44;&#x2A45;&#x2A46;&#x2A47;&#x2A48;&#x2A49;&#x2A4A;',
              '&#x2A4B;&#x2A4C;&#x2A4D;&#x2A4E;&#x2A4F;&#x2A50;&#x2A51;&#x2A52;&#x2A53;&#x2A54;&#x2A55;&#x2A56;',
              '&#x2A57;&#x2A58;&#x2A59;&#x2A5A;&#x2A5B;&#x2A5C;&#x2A5D;&#x2A5E;&#x2A5F;&#x2A60;&#x2A61;&#x2A62;',
              '&#x2A63;&#x2A64;&#x2A65;&#x2A66;&#x2A67;&#x2A68;&#x2A69;&#x2A6A;&#x2A6B;&#x2A6C;&#x2A6D;&#x2A6E;',
              '&#x2A6F;&#x2A70;&#x2A71;&#x2A72;&#x2A73;&#x2A74;&#x2A75;&#x2A76;&#x2A77;&#x2A78;&#x2A79;&#x2A7A;',
              '&#x2A7B;&#x2A7C;&#x2A7D;&#x2A7E;&#x2A7F;&#x2A80;&#x2A81;&#x2A82;&#x2A83;&#x2A84;&#x2A85;&#x2A86;',
              '&#x2A87;&#x2A88;&#x2A89;&#x2A8A;&#x2A8B;&#x2A8C;&#x2A8D;&#x2A8E;&#x2A8F;&#x2A90;&#x2A91;&#x2A92;',
              '&#x2A93;&#x2A94;&#x2A95;&#x2A96;&#x2A97;&#x2A98;&#x2A99;&#x2A9A;&#x2A9B;&#x2A9C;&#x2A9D;&#x2A9E;',
              '&#x2A9F;&#x2AA0;&#x2AA1;&#x2AA2;&#x2AA3;&#x2AA4;&#x2AA5;&#x2AA6;&#x2AA7;&#x2AA8;&#x2AA9;&#x2AAA;',
              '&#x2AAB;&#x2AAC;&#x2AAD;&#x2AAE;&#x2AAF;&#x2AB0;&#x2AB1;&#x2AB2;&#x2AB3;&#x2AB4;&#x2AB5;&#x2AB6;',
              '&#x2AB7;&#x2AB8;&#x2AB9;&#x2ABA;&#x2ABB;&#x2ABC;&#x2ABD;&#x2ABE;&#x2ABF;&#x2AC0;&#x2AC1;&#x2AC2;',
              '&#x2AC3;&#x2AC4;&#x2AC5;&#x2AC6;&#x2AC7;&#x2AC8;&#x2AC9;&#x2ACA;&#x2ACB;&#x2ACC;&#x2ACD;&#x2ACE;',
              '&#x2ACF;&#x2AD0;&#x2AD1;&#x2AD2;&#x2AD3;&#x2AD4;&#x2AD5;&#x2AD6;&#x2AD7;&#x2AD8;&#x2AD9;&#x2ADA;',
              '&#x2ADB;&#x2ADC;&#x2ADD;&#x2ADE;&#x2ADF;&#x2AE0;&#x2AE2;&#x2AE3;&#x2AE4;&#x2AE5;&#x2AE6;&#x2AE7;',
              '&#x2AE8;&#x2AE9;&#x2AEA;&#x2AEB;&#x2AEC;&#x2AED;&#x2AEE;&#x2AEF;&#x2AF0;&#x2AF2;&#x2AF3;&#x2AF4;',
              '&#x2AF5;&#x2AF6;&#x2AF7;&#x2AF8;&#x2AF9;&#x2AFA;&#x2AFB;&#x2AFC;&#x2AFD;&#x2AFE;&#x2AFF;&#x2B04;',
              '&#x2B06;&#x2B07;&#x2B0C;&#x2B0D;&#x3014;&#x3015;&#x3016;&#x3017;&#x3018;&#x3019;&#xFF01;&#xFF06;',
              '&#xFF08;&#xFF09;&#xFF0B;&#xFF0C;&#xFF0D;&#xFF0E;&#xFF0F;&#xFF1A;&#xFF1B;&#xFF1C;&#xFF1D;&#xFF1E;',
              '&#xFF1F;&#xFF20;&#xFF3B;&#xFF3C;&#xFF3D;&#xFF3E;&#xFF3F;&#xFF5B;&#xFF5C;&#xFF5D;')" />
    </xsl:stylesheet>
"###;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|_: &_| Err(ParseError::MissingNameSpace)),
    );

    assert!(parseresult.is_ok());
}

#[test]
fn parser_issue_132_minimal() {
    let doc = r###"<xsl:stylesheet version="1.0" xmlns:xsl="http://www.w3.org/1999/XSL/Transform">
      <test value="&#x003C;">empty</test>
    </xsl:stylesheet>
"###;

    let testxml = RNode::new_document();
    let parseresult = xml::parse(
        testxml,
        doc,
        Some(|p: &NamespacePrefix| {
            Err(ParseError::NotWellFormed(format!(
                "prefix: {}",
                p.to_string()
            )))
        }),
    );

    assert!(parseresult.is_ok());
}
