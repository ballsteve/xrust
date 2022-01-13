<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		version="3.0">
  <xsl:output indent="yes"/>

  <xsl:strip-space elements="*"/>

  <xsl:template match="child::Test">
    <xsl:copy>
      <xsl:apply-templates/>
      <xsl:copy-of select="child::level1/child::data"/>
    </xsl:copy>
  </xsl:template>
  <xsl:template match="child::*">
    <xsl:copy>
      <xsl:apply-templates/>
    </xsl:copy>
  </xsl:template>
</xsl:stylesheet>
