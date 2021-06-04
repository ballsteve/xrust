# Compliance

This is the status of compliance to the XDM, XPath, XQuery, and XSLT standards.

## XDM

Concept|Status|Notes
-------|-----|-----
Item|yes|
Sequence|yes|
Data type: xs:untyped|no|
Data type: xs:untypedAtomic|no|
Data type: xs:anyAtomicType|no|
Data type: xs:duration|no|
Data type: xs:dayTimeDuration|no|
Data type: xs:yearMonthDuration|no|
Data type: xs:dateTime|no|
Data type: xs:dateTimeStamp|no|
Data type: xs:date|no|
Data type: xs:time|no|
Data type: xs:gYearMonth|no|
Data type: xs:gYear|no|
Data type: xs:gMonthDay|no|
Data type: xs:gMonth|no|
Data type: xs:gDay|no|
Data type: xs:string|yes|
Data type: xs:boolean|yes|
Data type: xs:normalizedString|yes|
Data type: xs:token|no|
Data type: xs:language|no|
Data type: xs:ID|no|
Data type: xs:IDREF|no|
Data type: xs:IDREFS|no|
Data type: xs:NMTOKEN|no|
Data type: xs:NMTOKENS|no|
Data type: xs:ENTITY|no|
Data type: xs:ENTITIES|no|
Data type: xs:Name|no|
Data type: xs:NCName|no|
Data type: xs:numeric|no|
Data type: xs:decimal|yes|
Data type: xs:float|yes|
Data type: xs:double|yes|
Data type: xs:integer|yes|
Data type: xs:nonPositiveInteger|yes|
Data type: xs:negativeInteger|yes|
Data type: xs:long|no|
Data type: xs:int|no|
Data type: xs:short|no|
Data type: xs:byte|no|
Data type: xs:nonNegativeInteger|yes|
Data type: xs:unsignedLong|no|
Data type: xs:unsignedInt|no|
Data type: xs:unsignedShort|no|
Data type: xs:unsignedByte|no|
Data type: xs:positiveInteger|yes|
Data type: xs:base64Binary|no|
Data type: xs:hexBinary|no|
Data type: xs:QName|no|
Data type: xs:NOTATION|no|
Functions|no|
Map|no|
Array|no|
Document node|yes|roxmltree|\
Element node|yes|roxmltree, json
Attribute node|no|
Namespace node|no|
PI node|yes|roxmltree
Comment node|yes|roxmltree
Text node|yes|roxmltree, json
Accessor|no|

## XPath

Concept|Status|Notes
-------|-----|-----
Primary expression: Literals|yes|
Primary expression: Variable references|yes|
Primary expression: Parenthesized expressions|yes|
Primary expression: Context item|yes|
Primary expression: Static function calls|yes|
Primary expression: Named function calls|yes|
Primary expression: Inline function expressions|no|
Postfix expression: Filter|yes|
Postfix expression: Dynamic function calls|no|
Path expression: /|yes|
Path expression: steps|yes|
Path expression: axes|partial|
Axis: child|yes|
Axis: self|yes|
Axis: descendant|yes|roxmltree
Axis: descendant-or-self|yes|roxmltree
Axis: ancestor|yes|
Axis: ancestor-or-self|yes|
Axis: parent|yes|
Axis: following|yes|
Axis: following-sibling|yes|
Axis: preceding|yes|
Axis: preceding-sibling|yes|
Axis: attribute|no|
Axis: namespace|no|
Path expression: Node tests|yes|
Path expression: Predicates within steps|yes|
Path expression: Unabbreviated syntax|yes|
Path expression: Abbreviated syntax|no|
Sequence expression: constructing sequences|yes|
Sequence expression: combining node sequences|yes|
Arithmetic expressions: |yes|
String concatenation expressions|yes|
Comparison expression: value|yes|
Comparison expression: general|yes|
Comparison expression: node|no|
Logical expression: value|yes|
For expression: value|yes|
Let expression: value|yes|
Maps|no|
Arrays|no|
Conditional expression|yes|
Quantified expression|no|
Instance of|no|
Cast|no|
Castable|no|
Constructor functions|no|
Treat|no|
Simple map operator: !|no|
Arrow operator: =>|no|
Unary expression|no|
Comments|yes|

## XPath Functions

Version 1.0 functions. Version 2.0 - 3.1 all not supported (yet).

Concept|Status|Notes
-------|-----|-----
count|yes|
last|yes|
position|yes|
id|no|requires validating parser
local-name|yes|Argument not yet implemented
namespace-uri|no|
name|yes|Argument not yet implemented; qnames not implemented
string|yes|
concat|yes|
starts-with|yes|
contains|yes|
substring-before|yes|
substring-after|yes|
substring|yes|
normalize-space|yes|
translate|yes|
boolean|yes|
not|yes|
true|yes|
false|yes|
lang|no|
number|yes|
sum|yes|
floor|yes|
ceiling|yes|
round|yes|

## XSLT

Concept|Status|Notes
-------|-----|-----
xsl:accept|no|
xsl:accept/@component|no|
xsl:accept/@names|no|
xsl:accept/@visibility|no|
xsl:accumulator|no|
xsl:accumulator/@name|no|
xsl:accumulator/@as|no|
xsl:accumulator/@streamable|no|
xsl:accumulator-rule|no|
xsl:accumulator-rule/@match|no|
xsl:accumulator-rule/@phase|no|
xsl:accumulator-rule/@select|no|
xsl:analyze-string|no|
xsl:analyze-string/@select|no|
xsl:analyze-string/@regex|no|
xsl:analyze-string/@flags|no|
xsl:apply-imports|no|
xsl:apply-templates|yes|
xsl:apply-templates/@select|yes|
xsl:apply-templates/@mode|no|
xsl:assert|no|
xsl:assert/@test|no|
xsl:assert/@select|no|
xsl:assert/@error-code|no|
xsl:attribute|no|
xsl:attribute/@name|no|
xsl:attribute/@namespace|no|
xsl:attribute/@select|no|
xsl:attribute/@separator|no|
xsl:attribute/@type|no|
xsl:attribute/@validation|no|
xsl:attribute-set|no|
xsl:attribute-set/@name|no|
xsl:attribute-set/@use-attribute-sets|no|
xsl:attribute-set/@visibility|no|
xsl:attribute-set/@streamable|no|
xsl:break|no|
xsl:break/@select|no|
xsl:call-template|no|
xsl:call-template/@name|no|
xsl:catch|no|
xsl:catch/@select|no|
xsl:catch/@errors|no|
xsl:character-map|no|
xsl:character-map/@name|no|
xsl:character-map/@use-character-maps|no|
xsl:choose|no|
xsl:comment|no|
xsl:comment/@select|no|
xsl:context-item|no|
xsl:context-item/@as|no|
xsl:context-item/@use|no|
xsl:copy|no|
xsl:copy/@select|no|
xsl:copy/@copy-namespaces|no|
xsl:copy/@inherit-namespaces|no|
xsl:copy/@use-attribute-sets|no|
xsl:copy/@type|no|
xsl:copy/@validation|no|
xsl:copy-of|no|
xsl:copy-of/@select|no|
xsl:copy-of/@copy-accumulators|no|
xsl:copy-of/@copy-namespaces|no|
xsl:copy-of/@type|no|
xsl:copy-of/@validation|no|
xsl:decimal-format|no|
xsl:decimal-format/@name|no|
xsl:decimal-format/@decimal-separator|no|
xsl:decimal-format/@grouping-separator|no|
xsl:decimal-format/@infinity|no|
xsl:decimal-format/@minus-sign|no|
xsl:decimal-format/@exponent-separator|no|
xsl:decimal-format/@NaN|no|
xsl:decimal-format/@percent|no|
xsl:decimal-format/@per-mille|no|
xsl:decimal-format/@zero-digit|no|
xsl:decimal-format/@digit|no|
xsl:decimal-format/@pattern-separator|no|
xsl:document|no|
xsl:document/@validation|no|
xsl:document/@type|no|
xsl:element|no|
xsl:element/@name|no|
xsl:element/@namespace|no|
xsl:element/@inherit-namespaces|no|
xsl:element/@use-attribute-sets|no|
xsl:element/@type|no|
xsl:element/@validation|no|
xsl:evaluate|no|
xsl:evaluate/@xpath|no|
xsl:evaluate/@as|no|
xsl:evaluate/@base-uri|no|
xsl:evaluate/@with-params|no|
xsl:evaluate/@context-item|no|
xsl:evaluate/@namespace-context|no|
xsl:evaluate/@schema-aware|no|
xsl:expose|no|
xsl:expose/@component|no|
xsl:expose/@names|no|
xsl:expose/@visibility|no|
xsl:fallback|no|
xsl:for-each|no|
xsl:for-each/@select|no|
xsl:for-each-group|no|
xsl:for-each-group/@select|no|
xsl:for-each-group/@group-by|no|
xsl:for-each-group/@group-adjacent|no|
xsl:for-each-group/@group-starting-with|no|
xsl:for-each-group/@group-ending-with|no|
xsl:for-each-group/@composite|no|
xsl:for-each-group/@collation|no|
xsl:fork|no|
xsl:function|no|
xsl:function/@name|no|
xsl:function/@as|no|
xsl:function/@visibility|no|
xsl:function/@streamability|no|
xsl:function/@override-extension-function|no|
xsl:function/@new-each-time|no|
xsl:function/@cache|no|
xsl:global-context-item|no|
xsl:global-context-item/@as|no|
xsl:global-context-item/@use|no|
xsl:if|no|
xsl:if/@test|no|
xsl:import|no|
xsl:import/@href|no|
xsl:import-schema|no|
xsl:import-schema/@namespace|no|
xsl:import-schema/@schema-location|no|
xsl:include|no|
xsl:include/@href|no|
xsl:iterate|no|
xsl:iterate/@select|no|
xsl:key|no|
xsl:key/@name|no|
xsl:key/@match|no|
xsl:key/@use|no|
xsl:key/@composite|no|
xsl:key/@collation|no|
xsl:map|no|
xsl:map-entry|no|
xsl:map-entry/@key|no|
xsl:map-entry/@select|no|
xsl:matching-substring|no|
xsl:merge|no|
xsl:merge-action|no|
xsl:merge-key|no|
xsl:merge-key/@select|no|
xsl:merge-key/@lang|no|
xsl:merge-key/@order|no|
xsl:merge-key/@collation|no|
xsl:merge-key/@case-order|no|
xsl:merge-key/@data-type|no|
xsl:merge-source|no|
xsl:merge-source/@name|no|
xsl:merge-source/@for-each-item|no|
xsl:merge-source/@for-each-source|no|
xsl:merge-source/@select|no|
xsl:merge-source/@streamable|no|
xsl:merge-source/@use-accumulators|no|
xsl:merge-source/@sort-before-merge|no|
xsl:merge-source/@validation|no|
xsl:merge-source/@type|no|
xsl:message|no|
xsl:message/@select|no|
xsl:message/@terminate|no|
xsl:message/@error-code|no|
xsl:mode|no|
xsl:mode/@name|no|
xsl:mode/@streamable|no|
xsl:mode/@use-accumulators|no|
xsl:mode/@on-no-match|no|
xsl:mode/@on-multiple-match|no|
xsl:mode/@warning-on-no-match|no|
xsl:mode/@warning-on-multiple-match|no|
xsl:mode/@typed|no|
xsl:mode/@visibility|no|
xsl:namespace|no|
xsl:namespace/@name|no|
xsl:namespace/@select|no|
xsl:namespace-alias|no|
xsl:namespace-alias/@stylesheet-prefix|no|
xsl:namespace-alias/@result-prefix|no|
xsl:next-iteration|no|
xsl:next-match|no|
xsl:non-matching-substring|no|
xsl:number|no|
xsl:number/@value|no|
xsl:number/@select|no|
xsl:number/@level|no|
xsl:number/@count|no|
xsl:number/@from|no|
xsl:number/@format|no|
xsl:number/@lang|no|
xsl:number/@letter-value|no|
xsl:number/@ordinal|no|
xsl:number/@start-at|no|
xsl:number/@grouping-separator|no|
xsl:number/@grouping-size|no|
xsl:on-completion|no|
xsl:on-completion/@select|no|
xsl:on-empty|no|
xsl:on-empty/@select|no|
xsl:on-non-empty|no|
xsl:on-non-empty/@select|no|
xsl:otherwise|no|
xsl:output|no|
xsl:output/@name|no|
xsl:output/@method|no|
xsl:output/@allow-duplicate-names|no|
xsl:output/@build-tree|no|
xsl:output/@byte-order-mark|no|
xsl:output/@cdata-section-elements|no|
xsl:output/@doctype-public|no|
xsl:output/@doctype-system|no|
xsl:output/@encoding|no|
xsl:output/@escape-uri-attributes|no|
xsl:output/@html-version|no|
xsl:output/@include-content-type|no|
xsl:output/@indent|no|
xsl:output/@item-separator|no|
xsl:output/@json-node-output-method|no|
xsl:output/@media-type|no|
xsl:output/@normalization-form|no|
xsl:output/@omit-xml-declaration|no|
xsl:output/@parameter-document|no|
xsl:output/@standalone|no|
xsl:output/@suppress-indentation|no|
xsl:output/@undeclare-prefixes|no|
xsl:output/@use-character-maps|no|
xsl:output/@version|no|
xsl:output-character|no|
xsl:output-character/@character|no|
xsl:output-character/@string|no|
xsl:override|no|
xsl:package|no|
xsl:package/@id|no|
xsl:package/@name|no|
xsl:package/@package-version|no|
xsl:package/@version|no|
xsl:package/@input-type-annotations|no|
xsl:package/@declared-modes|no|
xsl:package/@default-mode|no|
xsl:package/@default-validation|no|
xsl:package/@default-collation|no|
xsl:package/@extension-element-prefixes|no|
xsl:package/@exclude-result-prefixes|no|
xsl:package/@expand-text|no|
xsl:package/@use-when|no|
xsl:package/@xpath-default-namespace|no|
xsl:param|no|
xsl:param/@name|no|
xsl:param/@select|no|
xsl:param/@as|no|
xsl:param/@required|no|
xsl:param/@tunnel|no|
xsl:param/@static|no|
xsl:perform-sort|no|
xsl:perform-sort/@select|no|
xsl:preserve-space|no|
xsl:preserve-space/@elements|no|
xsl:processing-instruction|no|
xsl:processing-instruction/@name|no|
xsl:processing-instruction/@select|no|
xsl:result-document|no|
xsl:result-document/@format|no|
xsl:result-document/@href|no|
xsl:result-document/@validation|no|
xsl:result-document/@type|no|
xsl:result-document/@method|no|
xsl:result-document/@allow-duplicate-names|no|
xsl:result-document/@build-tree|no|
xsl:result-document/@byte-order-mark|no|
xsl:result-document/@cdata-section-elements|no|
xsl:result-document/@doctype-public|no|
xsl:result-document/@doctype-system|no|
xsl:result-document/@encoding|no|
xsl:result-document/@escape-uri-attributes|no|
xsl:result-document/@html-version|no|
xsl:result-document/@include-content-type|no|
xsl:result-document/@indent|no|
xsl:result-document/@item-separator|no|
xsl:result-document/@json-node-output-method|no|
xsl:result-document/@media-type|no|
xsl:result-document/@normalization-form|no|
xsl:result-document/@omit-xml-declaration|no|
xsl:result-document/@parameter-document|no|
xsl:result-document/@standalone|no|
xsl:result-document/@suppress-indentation|no|
xsl:result-document/@undeclare-prefixes|no|
xsl:result-document/@use-character-maps|no|
xsl:result-document/@output-version|no|
xsl:sequence|yes|
xsl:sequence/@select|yes|
xsl:sort|no|
xsl:sort/@select|no|
xsl:sort/@lang|no|
xsl:sort/@order|no|
xsl:sort/@collation|no|
xsl:sort/@stable|no|
xsl:sort/@case-order|no|
xsl:sort/@data-type|no|
xsl:source-document|no|
xsl:source-document/@href|no|
xsl:source-document/@streamable|no|
xsl:source-document/@use-accumulators|no|
xsl:source-document/@validation|no|
xsl:source-document/@type|no|
xsl:strip-space|no|
xsl:strip-space/@elements|no|
xsl:stylesheet|yes|
xsl:stylesheet/@id|no|
xsl:stylesheet/@version|no|
xsl:stylesheet/@default-mode|no|
xsl:stylesheet/@default-validation|no|
xsl:stylesheet/@input-type-annotations|no|
xsl:stylesheet/@default-collation|no|
xsl:stylesheet/@extension-element-prefixes|no|
xsl:stylesheet/@exclude-result-prefixes|no|
xsl:stylesheet/@expand-text|no|
xsl:stylesheet/@use-when|no|
xsl:stylesheet/@xpath-default-namespace|no|
xsl:template|yes|
xsl:template/@match|yes|
xsl:template/@name|no|
xsl:template/@priority|no|
xsl:template/@mode|no|
xsl:template/@as|no|
xsl:template/@visibility|no|
xsl:text|no|
xsl:text/@disable-output-escaping|no|
xsl:transform|yes|
xsl:try|no|
xsl:try/@select|no|
xsl:try/@rollback-output|no|
xsl:use-package|no|
xsl:use-package/@name|no|
xsl:use-package/@package-version|no|
xsl:value-of|no|
xsl:value-of/@select|no|
xsl:value-of/@separator|no|
xsl:value-of/@disable-output-escaping|no|
xsl:variable|no|
xsl:variable/@name|no|
xsl:variable/@select|no|
xsl:variable/@as|no|
xsl:variable/@static|no|
xsl:variable/@visibility|no|
xsl:when|no|
xsl:when/@test|no|
xsl:where-populated|no|
xsl:with-param|no|
xsl:with-param/@name|no|
xsl:with-param/@select|no|
xsl:with-param/@as|no|
xsl:with-param/@tunnel|no|


