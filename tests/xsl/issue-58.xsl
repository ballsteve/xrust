<?xml version="1.0" encoding="utf-8"?>
<xsl:stylesheet
	version="1.0"
	xmlns:dat="http://www.stormware.cz/schema/version_2/data.xsd"
	xmlns:int="http://www.stormware.cz/schema/version_2/intDoc.xsd"
	xmlns:xsl="http://www.w3.org/1999/XSL/Transform">

	<xsl:output method="xml" encoding="utf-8" indent="yes"/>

    <xsl:template match="child::Example">
        <dat:dataPack>
            <xsl:apply-templates/>
        </dat:dataPack>
    </xsl:template>
    <xsl:template match="child::Title">
        <int:head>
            <xsl:apply-templates/>
        </int:head>
    </xsl:template>
    <xsl:template match="child::Paragraph">
        <int:body>
            <xsl:apply-templates/>
        </int:body>
    </xsl:template>
</xsl:stylesheet>
