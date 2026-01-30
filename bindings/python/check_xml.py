import ccsds_ndm

xml_content = """<?xml version="1.0" encoding="UTF-8"?>
<ndm:aem xmlns:ndm="urn:ccsds:recommendation:navigation:schema:ndmxml"
         xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
         id="CCSDS_AEM_VERS" version="1.0">
    <header>
        <COMMENT>Test AEM</COMMENT>
        <CREATION_DATE>2023-01-01T00:00:00</CREATION_DATE>
        <ORIGINATOR>TEST</ORIGINATOR>
    </header>
    <body segmentInterface="NDM">
        <segment>
            <metadata>
                <OBJECT_NAME>SAT1</OBJECT_NAME>
                <OBJECT_ID>2023-001A</OBJECT_ID>
                <REF_FRAME_A>EME2000</REF_FRAME_A>
                <REF_FRAME_B>SC_BODY_1</REF_FRAME_B>
                <ATTITUDE_TYPE>QUATERNION</ATTITUDE_TYPE>
                <TIME_SYSTEM>UTC</TIME_SYSTEM>
                <START_TIME>2023-01-01T00:00:00</START_TIME>
                <STOP_TIME>2023-01-01T01:00:00</STOP_TIME>
                <INTERPOLATION_METHOD>LINEAR</INTERPOLATION_METHOD>
                <INTERPOLATION_DEGREE>1</INTERPOLATION_DEGREE>
            </metadata>
            <data>
                <attitudeState>
                    <EPOCH>2023-01-01T00:00:00</EPOCH>
                    <Q1>0.5</Q1>
                    <Q2>0.5</Q2>
                    <Q3>0.5</Q3>
                    <QC>0.5</QC>
                </attitudeState>
            </data>
        </segment>
    </body>
</ndm:aem>
"""

try:
    aem = ccsds_ndm.from_str(xml_content)
    print("XML parsing successful!")
    print(f"Originator: {aem.header.originator}")
except Exception as e:
    print(f"XML parsing failed: {e}")
