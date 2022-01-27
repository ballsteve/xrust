<xsl:stylesheet xmlns:xsl='http://www.w3.org/1999/XSL/Transform'
		xmlns:xlink="http://www.w3.org/1999/xlink"
		version="3.0">
  <xsl:output indent="yes"/>

  <xsl:strip-space elements="*"/>

  <xsl:template match="child::Test">
    <html>
      <xsl:apply-templates/>
    </html>
  </xsl:template>
  <xsl:template match="child::level1">
    <body id="top">
      <xsl:apply-templates/>
    </body>
  </xsl:template>
  <xsl:template match="child::data">
    <div role="data" class="child">
      <xsl:apply-templates/>
    </div>
  </xsl:template>
  <xsl:template match="child::deeper">
    <span class="deep">
      <xsl:apply-templates/>
    </span>
  </xsl:template>
  <xsl:template match="child::link">
    <a xlink:href="urn:test-data">
      <xsl:apply-templates/>
    </a>
  </xsl:template>
</xsl:stylesheet>
