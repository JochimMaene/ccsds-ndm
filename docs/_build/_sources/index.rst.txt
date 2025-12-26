CCSDS NDM Documentation
=======================

Navigation Data Messages (NDM)
------------------------------

The CCSDS Navigation Data Messages (NDM) are a set of international standards for the exchange of spacecraft navigation data.
They facilitate interoperability between space agencies and commercial operators by defining common formats for orbit, attitude, and tracking data.

This library provides parsers and serializers for the following message types:

*   **OEM (Orbit Ephemeris Message)**: Orbit state (position/velocity) and covariance time series.
*   **OPM (Orbit Parameter Message)**: Single state vector and orbital parameters (Keplerian elements).
*   **OMM (Orbit Mean-Elements Message)**: Mean orbital elements.
*   **OCM (Orbit Comprehensive Message)**: Detailed orbit data including physical properties, covariance, and maneuvers.
*   **CDM (Conjunction Data Message)**: Collision assessment data.
*   **TDM (Tracking Data Message)**: Ground station tracking measurements.
*   **RDM (Re-entry Data Message)**: Re-entry prediction information.
*   **AEM (Attitude Ephemeris Message)**: Attitude state (quaternions/Euler angles) time series.
*   **APM (Attitude Parameter Message)**: Single attitude state and parameters.
*   **ACM (Attitude Comprehensive Message)**: Detailed attitude data.

For a detailed overview of the NDM conceptual framework, refer to the :doc:`ccsds-books/ndm`.
For the specific Attitude Data Messages standard, see :doc:`ccsds-books/adm`.

.. toctree::
   :maxdepth: 2
   :caption: User Guide

   guide/quicktour


.. toctree::
   :maxdepth: 2
   :caption: API Reference

   api/python

.. toctree::
   :maxdepth: 1
   :caption: CCSDS Standards

   ccsds-books/ndm
   ccsds-books/adm
