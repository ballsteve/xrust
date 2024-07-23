use std::collections::HashMap;
use std::rc::Rc;
use crate::{Error, Item};
use crate::trees::smite::{Node as SmiteNode, RNode};
use crate::parser::xml::{parse as xmlparse, parse_with_ns};
use crate::transform::context::{StaticContextBuilder};
use crate::xslt::from_document;

pub(crate) type Param = (String, String);


#[derive(Debug)]
pub(crate) enum PatternError<'a>{
    NotRelaxNG,
    MissingName,
    Other(&'a str)
}

type F = Box<dyn FnMut(&str) -> Result<(), Error>>;

pub(super) fn prepare(schemadoc: &RNode) -> Result<(RNode, HashMap<String,RNode>), PatternError> {
    //TODO implement

    let patternprepper = r#"<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet version="2.0"
                xmlns:rng="http://relaxng.org/ns/structure/1.0"
                xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
                exclude-result-prefixes="rng"
>

    <xsl:output
            method="xml"
            encoding="utf-8"
            indent="yes"
    />
    <xsl:mode name="Step4.1" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.2" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.3" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.4" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.5" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.6" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.7" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.8" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.9" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.10" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.11" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.12" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.13" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.14" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.15" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.16" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.17" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.18" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.19" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.20" on-no-match="shallow-copy"/>
    <xsl:mode name="Step4.21" on-no-match="shallow-copy"/>


    <xsl:template match="/">
        <xsl:variable name="s4.1" ><xsl:apply-templates mode="Step4.1"  select="/"      /></xsl:variable>
        <xsl:variable name="s4.2" ><xsl:apply-templates mode="Step4.2"  select="$s4.1"  /></xsl:variable>
        <xsl:variable name="s4.3" ><xsl:apply-templates mode="Step4.3"  select="$s4.2"  /></xsl:variable>
        <xsl:variable name="s4.4" ><xsl:apply-templates mode="Step4.4"  select="$s4.3"  /></xsl:variable>
        <xsl:variable name="s4.5" ><xsl:apply-templates mode="Step4.5"  select="$s4.4"  /></xsl:variable>
        <xsl:variable name="s4.6" ><xsl:apply-templates mode="Step4.6"  select="$s4.5"  /></xsl:variable>
        <xsl:variable name="s4.7" ><xsl:apply-templates mode="Step4.7"  select="$s4.6"  /></xsl:variable>
        <xsl:variable name="s4.8" ><xsl:apply-templates mode="Step4.8"  select="$s4.7"  /></xsl:variable>
        <xsl:variable name="s4.9" ><xsl:apply-templates mode="Step4.9"  select="$s4.8"  /></xsl:variable>
        <xsl:variable name="s4.10"><xsl:apply-templates mode="Step4.10" select="$s4.9"  /></xsl:variable>
        <xsl:variable name="s4.11"><xsl:apply-templates mode="Step4.11" select="$s4.10" /></xsl:variable>
        <xsl:variable name="s4.12"><xsl:apply-templates mode="Step4.12" select="$s4.11" /></xsl:variable>
        <xsl:variable name="s4.13"><xsl:apply-templates mode="Step4.13" select="$s4.12" /></xsl:variable>
        <xsl:variable name="s4.14"><xsl:apply-templates mode="Step4.14" select="$s4.13" /></xsl:variable>
        <xsl:variable name="s4.15"><xsl:apply-templates mode="Step4.15" select="$s4.14" /></xsl:variable>
        <xsl:variable name="s4.16"><xsl:apply-templates mode="Step4.16" select="$s4.15" /></xsl:variable>
        <xsl:variable name="s4.17"><xsl:apply-templates mode="Step4.17" select="$s4.16" /></xsl:variable>
        <xsl:variable name="s4.18"><xsl:apply-templates mode="Step4.18" select="$s4.17" /></xsl:variable>
        <xsl:variable name="s4.19"><xsl:apply-templates mode="Step4.19" select="$s4.18" /></xsl:variable>
        <xsl:variable name="s4.20"><xsl:apply-templates mode="Step4.20" select="$s4.19" /></xsl:variable>
        <xsl:variable name="s4.21"><xsl:apply-templates mode="Step4.21" select="$s4.20" /></xsl:variable>
        <xsl:copy-of select="$s4.21"/>
    </xsl:template>

    <!-- 4.1 Annotations -->
    <xsl:template mode="Step4.1" match="*[namespace-uri() != 'http://relaxng.org/ns/structure/1.0']"/>
    <xsl:template mode="Step4.1" match="@*[name() != 'ns' and
                                        name() != 'type' and
                                        name() != 'href' and
                                        name() != 'combine' and
                                        name() != 'datatypeLibrary' and
                                        name() != 'name']"/>
    <xsl:template mode="Step4.1" match="comment()"/>
    <xsl:template mode="Step4.1" match="processing-instruction()"/>
    <!-- 4.2 Whitespace -->
    <xsl:template mode="Step4.2" match="text()[normalize-space(.) = '' and ancestor::*[not(self::value or self::param)]]"/>

    <!-- 4.3 datatypeLibrary attribute -->
    <xsl:template mode="Step4.3" match="rng:*[name()='data' or name()='value'][not(@datatypeLibrary)]">
        <xsl:copy>
            <!-- Add or inherit "datatypeLibrary" attribute -->
            <xsl:if test="not(@datatypeLibrary)">
                <xsl:attribute name="datatypeLibrary">
                    <xsl:choose>
                        <!-- If nearest ancestor has the attribute, inherit it -->
                        <xsl:when test="ancestor::rng:*[@datatypeLibrary][1]/@datatypeLibrary">
                            <xsl:value-of select="ancestor::*[@datatypeLibrary][1]/@datatypeLibrary"/>
                        </xsl:when>
                        <!-- Otherwise, default to a predefined value or remove if desired -->
                        <xsl:otherwise>default_value</xsl:otherwise>
                    </xsl:choose>
                </xsl:attribute>
            </xsl:if>
            <!-- Copy existing attributes -->
            <xsl:apply-templates mode="Step4.3" select="@*"/>
            <!-- Process child nodes -->
            <xsl:apply-templates mode="Step4.3" select="node()"/>
        </xsl:copy>
    </xsl:template>

    <!-- 4.4 type attribute of value element -->
    <xsl:template mode="Step4.4" match="rng:value[not(@type)]">
        <xsl:copy>
            <xsl:attribute name="type">token</xsl:attribute>
            <xsl:attribute name="datatypeLibrary"/>
            <!-- Copy existing attributes -->
            <xsl:apply-templates mode="Step4.4" select="@*"/>
            <!-- Process child nodes -->
            <xsl:apply-templates mode="Step4.4" select="node()"/>
        </xsl:copy>
    </xsl:template>


    <!-- 4.5 href attribute -->
    <!-- 4.6 externalRef element -->
    <xsl:template mode="Step4.6" match="rng:externalRef">
        <xsl:value-of select="doc(@href)"/>
    </xsl:template>

    <!-- 4.7 include element -->
    <!--
    <xsl:template mode="Step1" match="rng:include"/>
    -->

    <!-- 4.8 name attribute of element and attribute elements -->
    <xsl:template mode="Step4.8" match="rng:element">
        <xsl:copy>
            <xsl:apply-templates mode="Step4.8" select="@*[name() != 'name']"/>
            <rng:name><xsl:value-of select="@name"/></rng:name>
            <xsl:apply-templates mode="Step4.8" select="node()"/>
        </xsl:copy>
    </xsl:template>
    <xsl:template mode="Step4.8" match="rng:attribute">
        <xsl:copy>
            <xsl:apply-templates mode="Step4.8" select="@*[name() != 'name']"/>
            <rng:name>
                <xsl:if test="@name and not(@ns)">
                    <xsl:attribute name="ns"></xsl:attribute>
                </xsl:if>
                <xsl:value-of select="@name"/>
            </rng:name>
            <xsl:apply-templates mode="Step4.8" select="node()"/>
        </xsl:copy>
    </xsl:template>

    <!-- 4.9 ns attribute -->
    <xsl:template  mode="Step4.9" match="rng:value[not(@ns)] | rng:name[not(@ns)] | rng:nsname[not(@ns)] ">
        <xsl:copy>
            <xsl:attribute name="ns" select="ancestor::*[value|name|nsname][@ns]"/>
            <xsl:apply-templates mode="Step4.9"/>
        </xsl:copy>
    </xsl:template>

    <xsl:template  mode="Step4.9" match="*[not(rng:name or rng:nsname or rng:value) and @ns]">
        <xsl:copy>
            <xsl:attribute name="ns"/>
            <xsl:apply-templates mode="Step4.9"/>
        </xsl:copy>
    </xsl:template>

    <!-- 4.10 QNames -->
    <!-- TODO -->


    <!-- 4.11 div -->
    <xsl:template mode="Step4.11" match="rng:div">
        <xsl:apply-templates mode="Step4.11"/>
    </xsl:template>


    <!-- 4.12 number of child elements -->
    <xsl:template mode="Step4.12" match="*[rng:define|rng:oneOrMore|rng:zeroOrMore|rng:optional|rng:list|rng:mixed][count(*) &gt; 1]">
        <xsl:copy>
            <rng:group>
                <xsl:apply-templates mode="Step4.12"/>
            </rng:group>
        </xsl:copy>
    </xsl:template>

    <xsl:template mode="Step41.12" match="rng:element[count(*) &gt; 2]">
        <xsl:copy>
            <xsl:copy select="rng:name"/>
            <rng:group>
                <xsl:apply-templates mode="Step4.12" select="*[name() != 'name']"/>
            </rng:group>
        </xsl:copy>
    </xsl:template>

    <xsl:template mode="Step4.12" match="rng:except[count(*) &gt; 1]">
        <xsl:copy>
            <rng:choice>
                <xsl:apply-templates mode="Step4.12"/>
            </rng:choice>
        </xsl:copy>
    </xsl:template>

    <xsl:template mode="Step4.12" match="rng:except[count(*) &gt; 1]">
        <xsl:copy>
            <rng:choice>
                <xsl:apply-templates mode="Step4.12"/>
            </rng:choice>
        </xsl:copy>
    </xsl:template>

    <xsl:template mode="Step4.12" match="rng:attribute[count(*) = 1 and name() = 'name']">
        <xsl:copy>
            <rng:text/>
        </xsl:copy>
    </xsl:template>

    <xsl:template mode="Step4.12" match="[rng:choice|rng:group|rng:interleave][count(*) = 1]">
        <xsl:apply-templates mode="Step4.12"/>
    </xsl:template>

    <xsl:template mode="Step4.12" match="rng:choice[count(*) &gt; 2]">
        <xsl:call-template name="choices_splitter">
            <xsl:with-param name="choices" select="./child::*"/>
        </xsl:call-template>
    </xsl:template>
    <xsl:template name="choices_splitter">
        <xsl:param name="choices"/>
        <xsl:choose>
            <xsl:when test="count($choices) = 1">
                <xsl:apply-templates mode="Step4.12" select="$choices"/>
            </xsl:when>
            <xsl:otherwise>
                <rng:choice>
                    <xsl:apply-templates mode="Step4.12" select="$choices[position() = 1]"/>
                    <xsl:call-template name="choices_splitter">
                        <xsl:with-param name="choices" select="$choices[position() &gt; 1]"/>
                    </xsl:call-template>
                </rng:choice>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template mode="Step4.12" match="rng:group[count(*) &gt; 2]">
        <xsl:call-template name="groups_splitter">
            <xsl:with-param name="groups" select="./child::*"/>
        </xsl:call-template>
    </xsl:template>
    <xsl:template name="groups_splitter">
        <xsl:param name="groups"/>
        <xsl:choose>
            <xsl:when test="count($groups) = 1">
                <xsl:apply-templates mode="Step4.12" select="$groups"/>
            </xsl:when>
            <xsl:otherwise>
                <rng:group>
                    <xsl:apply-templates mode="Step4.12" select="$groups[position() = 1]"/>
                    <xsl:call-template name="groups_splitter">
                        <xsl:with-param name="groups" select="$groups[position() &gt; 1]"/>
                    </xsl:call-template>
                </rng:group>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template mode="Step4.12" match="rng:interleave[count(*) &gt; 2]">
        <xsl:call-template name="interleave_splitter">
            <xsl:with-param name="interleaves" select="./child::*"/>
        </xsl:call-template>
    </xsl:template>
    <xsl:template name="interleave_splitter">
        <xsl:param name="interleaves"/>
        <xsl:choose>
            <xsl:when test="count($interleaves) = 1">
                <xsl:apply-templates mode="Step4.12" select="$interleaves"/>
            </xsl:when>
            <xsl:otherwise>
                <rng:group>
                    <xsl:apply-templates mode="Step4.12" select="$interleaves[position() = 1]"/>
                    <xsl:call-template name="interleave_splitter">
                        <xsl:with-param name="interleaves" select="$interleaves[position() &gt; 1]"/>
                    </xsl:call-template>
                </rng:group>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>


    <!-- 4.13  mixed element -->
    <xsl:template match="rng:mixed" mode="Step4.13">
        <rng:interleave>
            <xsl:apply-templates mode="Step4.13"/>
            <rng:text/>
        </rng:interleave>
    </xsl:template>

    <!-- 4.14 optional element -->
    <xsl:template match="rng:optional" mode="Step4.14">
        <rng:choice>
            <xsl:apply-templates mode="Step4.14"/>
            <rng:empty/>
        </rng:choice>
    </xsl:template>

    <!-- 4.15 zeroOrMore element -->
    <xsl:template match="rng:zeroOrMore" mode="Step4.15">
        <rng:choice>
            <rng:oneOrMore>
                <xsl:apply-templates mode="Step4.15"/>
            </rng:oneOrMore>
            <rng:empty/>
        </rng:choice>
    </xsl:template>

    <!-- 4.16 Constraints -->
    <!-- TODO -->

    <!-- 4.17 combine attribute -->
    <xsl:template mode="Step4.17" match="rng:grammar">
        <xsl:copy>
            <xsl:for-each-group select=".//rng:define[@combine='choice']" group-by="@name">
                <rng:define>
                    <xsl:attribute name="name" select="@name"/>
                    <rng:choice>
                        <xsl:value-of select="current-group()"/>
                    </rng:choice>
                </rng:define>
            </xsl:for-each-group>
            <xsl:for-each-group select="//rng:define[@combine='interleave']" group-by="@name">
                <rng:define>
                    <xsl:attribute name="name" select="@name"/>
                    <rng:interleave>
                        <xsl:value-of select="current-group()"/>
                    </rng:interleave>
                </rng:define>
            </xsl:for-each-group>
            <xsl:apply-templates mode="Step4.17"/>
        </xsl:copy>
    </xsl:template>

    <!-- 4.18 grammar element -->
    <xsl:template mode="Step4.18" match="/*[not(local-name()='grammar')]">
        <rng:grammar>
            <rng:start>
                <xsl:copy-of select="."/>
            </rng:start>
            <xsl:copy-of select="//rng:define"/>
        </rng:grammar>
    </xsl:template>

    <xsl:template mode="Step4.18" match="/rng:grammar">
        <xsl:copy>
            <xsl:choose>
                <xsl:when test="rng:group/rng:start">
                    <xsl:copy-of select="rng:group/rng:start"/>
                </xsl:when>
                <xsl:otherwise>
                    <rng:start>
                        <xsl:copy-of select="*"/>
                    </rng:start>
                </xsl:otherwise>
            </xsl:choose>
            <xsl:copy-of select="//rng:define"/>
        </xsl:copy>
    </xsl:template>

    <!-- All except root node -->
    <xsl:template mode="Step4.18" match="rng:grammar[parent::*]"/>

    <!-- 4.19. define and ref elements -->
    <xsl:template mode="Step4.19" match="rng:element[not(ancestor::rng:define)]">
        <rng:ref>
            <xsl:attribute name="name">
                <xsl:value-of select="./rng:name"/>
            </xsl:attribute>
        </rng:ref>
    </xsl:template>
    <xsl:template mode="Step4.19" match="/rng:grammar">
        <xsl:copy>
            <xsl:apply-templates mode="Step4.19"/>
            <xsl:for-each select="//rng:element[not(ancestor::rng:define)]">
                <rng:define>
                    <xsl:attribute name="name">
                        <xsl:value-of select="./rng:name"/>
                    </xsl:attribute>
                    <xsl:copy-of select="/rng:grammar/rng:start/*"/>
                </rng:define>
            </xsl:for-each>
        </xsl:copy>
    </xsl:template>

    <!-- 4.20. notAllowed element -->
    <xsl:template mode="Step4.20" match="rng:attribute[rng:notAllowed]"><rng:notAllowed/></xsl:template>
    <xsl:template mode="Step4.20" match="rng:list[rng:notAllowed]"><rng:notAllowed/></xsl:template>
    <xsl:template mode="Step4.20" match="rng:group[rng:notAllowed]"><rng:notAllowed/></xsl:template>
    <xsl:template mode="Step4.20" match="rng:interleave[rng:notAllowed]"><rng:notAllowed/></xsl:template>
    <xsl:template mode="Step4.20" match="rng:oneOrMore[rng:notAllowed]"><rng:notAllowed/></xsl:template>

    <xsl:template mode="Step4.20" match="rng:choice[count(rng:notAllowed) = 2]">
        <rng:notAllowed/>
    </xsl:template>

    <xsl:template mode="Step4.20" match="rng:choice[count(rng:notAllowed) != 2]">
        <xsl:variable name="child1">
            <xsl:apply-templates mode="Step4.20" select="*[1]"/>
        </xsl:variable>
        <xsl:variable name="child2">
            <xsl:apply-templates mode="Step4.20" select="*[2]"/>
        </xsl:variable>
        <xsl:choose>
            <xsl:when test="name($child1)= 'notAllowed' and name($child2)= 'notAllowed'">
                <rng:notAllowed/>
            </xsl:when>
            <xsl:otherwise>
                <rng:choice>
                    <xsl:copy-of select="$child1"/>
                    <xsl:copy-of select="$child2"/>
                </rng:choice>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template mode="Step4.20" match="rng:except[rng:notAllowed]"/>


    <!-- 4.21. empty element -->
    <xsl:template mode="Step4.21" match="rng:group">
        <xsl:variable name="child1">
            <xsl:apply-templates mode="Step4.21" select="*[1]"/>
        </xsl:variable>
        <xsl:variable name="child2">
            <xsl:apply-templates mode="Step4.21" select="*[2]"/>
        </xsl:variable>
        <xsl:choose>
            <xsl:when test="name($child1)= 'empty' and name($child2)= 'empty'">
                <rng:empty/>
            </xsl:when>
            <xsl:when test="name($child1)= 'empty'">
                <xsl:copy-of select="$child2"/>
            </xsl:when>
            <xsl:when test="name($child2)= 'empty'">
                <xsl:copy-of select="$child1"/>
            </xsl:when>
            <xsl:otherwise>
                <rng:group>
                    <xsl:copy-of select="$child1"/>
                    <xsl:copy-of select="$child2"/>
                </rng:group>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template mode="Step4.21" match="rng:interleave">
        <xsl:variable name="child1">
            <xsl:apply-templates mode="Step4.21" select="*[1]"/>
        </xsl:variable>
        <xsl:variable name="child2">
            <xsl:apply-templates mode="Step4.21" select="*[2]"/>
        </xsl:variable>
        <xsl:choose>
            <xsl:when test="name($child1)= 'empty' and name($child2)= 'empty'">
                <rng:empty/>
            </xsl:when>
            <xsl:when test="name($child1)= 'empty'">
                <xsl:copy-of select="$child2"/>
            </xsl:when>
            <xsl:when test="name($child2)= 'empty'">
                <xsl:copy-of select="$child1"/>
            </xsl:when>
            <xsl:otherwise>
                <rng:interleave>
                    <xsl:copy-of select="$child1"/>
                    <xsl:copy-of select="$child2"/>
                </rng:interleave>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

    <xsl:template mode="Step4.21" match="rng:choice">
        <xsl:variable name="child1">
            <xsl:apply-templates mode="Step4.21" select="*[1]"/>
        </xsl:variable>
        <xsl:variable name="child2">
            <xsl:apply-templates mode="Step4.21" select="*[2]"/>
        </xsl:variable>
        <xsl:choose>
            <xsl:when test="name($child1)= 'empty' and name($child2)= 'empty'">
                <rng:empty/>
            </xsl:when>
            <xsl:when test="name($child2)= 'empty'">
                <rng:choice>
                    <xsl:copy-of select="$child2"/>
                    <xsl:copy-of select="$child1"/>
                </rng:choice>
            </xsl:when>
            <xsl:otherwise>
                <rng:choice>
                    <xsl:copy-of select="$child1"/>
                    <xsl:copy-of select="$child2"/>
                </rng:choice>
            </xsl:otherwise>
        </xsl:choose>
    </xsl:template>

</xsl:stylesheet>"#;

    let (styledoc, stylens) = parse_from_str_with_ns(patternprepper).expect("TODO: panic message");
    let mut stctxt = StaticContextBuilder::new()
        .message(|_| Ok(()))
        .parser(|_s| Ok(Rc::new(SmiteNode::new())))
        .fetcher(|url| Ok(String::new()))
        .build();
    let mut c = from_document(
        styledoc,
        stylens,
        None,
        |s| Ok(Rc::new(SmiteNode::new())),
        |_| Ok(String::new()),
    );
    match c {
        Ok(mut ctxt) => {
            ctxt.context(vec![Item::Node(schemadoc.clone())], 0);
            ctxt.result_document(Rc::new(SmiteNode::new()));
            ctxt.populate_key_values(&mut stctxt, schemadoc.clone()).expect("TODO: panic message");
            let rest =  ctxt.evaluate(&mut stctxt);
            println!("res-{:?}",rest);
        }
        Err(e) => {
            println!("reserre-{:?}",e);

        }
    }

        /*
    let mut ci = schemadoc.child_iter();
    let pat = ci.next().unwrap();
    let mut refs = HashMap::new();
    for r in ci {
        refs.insert(r.name().get_localname(), r);
    }
    Ok((pat, refs))

         */
    //println!("res-{:?}",rest);
    Ok((Rc::new(SmiteNode::new()), HashMap::new()))
}

fn parse_from_str(s: &str) -> Result<RNode, Error> {
    let doc = Rc::new(SmiteNode::new());
    xmlparse(doc.clone(), s, None)?;
    Ok(doc)
}

fn parse_from_str_with_ns(s: &str) -> Result<(RNode, Vec<HashMap<String, String>>), Error> {
    let doc = Rc::new(SmiteNode::new());
    let r = parse_with_ns(doc.clone(), s, None)?;
    Ok(r)
}