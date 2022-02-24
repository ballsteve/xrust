<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		version="3.0">
  <xsl:output indent="yes"/>

  <xsl:strip-space elements="*"/>

  <xsl:template match="child::Test">
    <result>
      <xsl:apply-templates/>
    </result>
  </xsl:template>
  <xsl:template match="child::data">
    <xsl:choose>
      <xsl:when test="attribute::role eq 'test'">This is correct</xsl:when>
      <xsl:when test="attribute::role eq 'test'">This is not correct</xsl:when>
      <xsl:otherwise>This is something else</xsl:otherwise>
    </xsl:choose>
  </xsl:template>
</xsl:stylesheet>
