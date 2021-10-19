<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		version="3.0">
  <xsl:output indent="yes"/>

  <xsl:strip-space elements="*"/>

  <xsl:template match="child::*">
    <xsl:copy>
      <xsl:apply-templates select="attribute::*"/>
      <xsl:apply-templates select="child::node()"/>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="attribute::*">
    <xsl:copy/>
  </xsl:template>
</xsl:stylesheet>
