Quicktour
=========

Installation
------------

.. tabs::

   .. code-tab:: py

        pip install ccsds-ndm-py

   .. code-tab:: rust

      cargo add ccsds-ndm

Constructing a message
----------------------

Build a small complete message from explicit metadata and caller data, then generate
valid output. Generation validates the complete message before writing.

.. tabs::

   .. code-tab:: py

        import ccsds_ndm
        from ccsds_ndm import OdmHeader, Opm, OpmData, OpmMetadata, OpmSegment, StateVector

        header = OdmHeader("2023-01-01T00:00:00", "TEST")
        meta = OpmMetadata(
            object_name="SAT1",
            object_id="2023-001A",
            center_name="EARTH",
            ref_frame="GCRF",
            time_system="UTC",
        )
        state = StateVector(
            epoch="2023-01-01T00:00:00",
            x=7000.0, y=0.0, z=0.0, x_dot=0.0, y_dot=7.5, z_dot=0.0,
            comment=None,
        )
        opm = Opm(
            header=header,
            segment=OpmSegment(metadata=meta, data=OpmData(state_vector=state, comment=[])),
        )
        opm.to_file("output.opm", "kvn")

   .. code-tab:: rust

        use ccsds_ndm::Validate;
        use ccsds_ndm::common::{OdmHeader, StateVector};
        use ccsds_ndm::messages::opm::{Opm, OpmBody, OpmData, OpmMetadata, OpmSegment};
        use ccsds_ndm::types::{Position, Velocity};

        fn main() -> ccsds_ndm::error::Result<()> {
            let opm = Opm::builder()
                .version("3.0")
                .header(OdmHeader::builder()
                    .creation_date("2023-01-01T00:00:00".parse()?)
                    .originator("TEST")
                    .build())
                .body(OpmBody::builder().segment(OpmSegment::builder()
                    .metadata(OpmMetadata::builder()
                        .object_name("SAT1").object_id("2023-001A")
                        .center_name("EARTH").ref_frame("GCRF").time_system("UTC")
                        .build())
                    .data(OpmData::builder().state_vector(StateVector::builder()
                        .epoch("2023-01-01T00:00:00".parse()?)
                        .x(Position::new(7000.0, None)).y(Position::new(0.0, None)).z(Position::new(0.0, None))
                        .x_dot(Velocity::new(0.0, None)).y_dot(Velocity::new(7.5, None)).z_dot(Velocity::new(0.0, None))
                        .build()).build())
                    .build()).build())
                .build();
            opm.validate()?;
            ccsds_ndm::Message::Opm(opm).to_kvn_file("output.opm")?;
            Ok(())
        }


Parsing messages
----------------

**Generic Parsing**

Use the top-level functions when you want to handle any message type dynamically.

.. tabs::

   .. code-tab:: py

        import ccsds_ndm

        # Returns an Oem, Opm, etc.
        ndm = ccsds_ndm.from_file("example.oem")

   .. code-tab:: rust

        use ccsds_ndm::from_file;

        let ndm = from_file("example.oem")?;

**Type-Specific Parsing**

If you know the file type (e.g., OPM), you can parse it directly into the struct.

.. tabs::

   .. code-tab:: py

        import ccsds_ndm
        from ccsds_ndm import Opm

        # Returns an Opm object directly
        opm = Opm.from_file("example.opm")

   .. code-tab:: rust

        use ccsds_ndm::messages::opm::Opm;
        use ccsds_ndm::Ndm;
        use std::fs;

        let text = fs::read_to_string("example.opm")?;
        // Parses strict KVN for OPM
        let opm = Opm::from_kvn(&text)?;

Data Access
-----------

Once parsed, you can access the nested data structures.

.. tabs::

   .. code-tab:: py


        # Assuming 'ndm' is an Oem object
        for segment in ndm.segments:
            meta = segment.metadata
            print(f"Object: {meta.object_name} ({meta.object_id})")

            # State vectors are in segment.data.state_vector
            for sv in segment.data.state_vector:
                print(f"Epoch: {sv.epoch}, X: {sv.x}")

   .. code-tab:: rust

        use ccsds_ndm::Message;

        if let Message::Oem(oem) = ndm {
            for segment in oem.body.segment {
                let meta = segment.metadata;
                println!("Object: {} ({})", meta.object_name, meta.object_id);

                // State vectors are in segment.data.state_vector
                for sv in segment.data.state_vector {
                    println!("Epoch: {}, X: {}", sv.epoch, sv.x);
                }
            }
        }

Writing Messages
----------------

You can convert the in-memory structures back to KVN or XML.

.. tabs::

   .. code-tab:: py

        # Convert to XML string
        xml_string = ndm.to_str("xml")

        # Write to KVN file
        ndm.to_file("output.kvn", "kvn")

   .. code-tab:: rust

        // Convert to XML string
        let xml_string = ndm.to_xml()?;

        // Write to KVN file
        ndm.to_kvn_file("output.kvn")?;
