<xsl:stylesheet xmlns:xsl="http://www.w3.org/1999/XSL/Transform"
  version="3.0">

  <xsl:template match="child::doc">
    <HTML><BODY><xsl:apply-templates/></BODY></HTML>
  </xsl:template>
  <xsl:template match="child::heading1">
    <H1><xsl:apply-templates/></H1>
  </xsl:template>
  <xsl:template match="child::heading2">
    <H2><xsl:apply-templates/></H2>
  </xsl:template>
  <xsl:template match="child::para">
    <P><xsl:apply-templates/></P>
  </xsl:template>
  <xsl:template match="child::eol">
  </xsl:template>
</xsl:stylesheet>
