Quicktour
=========

Installation
------------

.. tabs::

   .. code-tab:: py

        pip install ccsds-ndm-py

   .. code-tab:: rust

      cargo add ccsds-ndm

Parsing Messages
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

        from ccsds_ndm import Opm

        # Returns an Opm object directly
        opm = Opm.from_file("example.opm")

   .. code-tab:: rust

        use ccsds_ndm::messages::opm::Opm;

        // Returns an Opm struct directly
        let opm = Opm::from_file("example.opm")?;

Data Access
-----------

Once parsed, you can access the nested data structures.

.. tabs::

   .. code-tab:: py


        # Assuming 'ndm' is an Oem object
        for segment in ndm.segments:
            meta = segment.metadata
            print(f"Object: {meta.object_name} ({meta.object_id})")
            
            # State vectors are in segment.data.state_vectors
            for sv in segment.data.state_vectors:
                print(f"Epoch: {sv.epoch}, X: {sv.x}")

   .. code-tab:: rust

        if let MessageType::Oem(oem) = ndm {
            for segment in oem.segments {
                let meta = segment.metadata;
                println!("Object: {} ({})", meta.object_name, meta.object_id);
                
                // State vectors are in segment.data.state_vectors
                for sv in segment.data.state_vectors {
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
