OCM Technical Corrigendum 1 (June 2026)
=====================================

Source: `CCSDS 502.0-B-3 Technical Corrigendum 1 <https://ccsds.org/wp-content/uploads/gravity_forms/5-448e85c647331d9cbaf66c096458bdd5/2026/10/502x0b3e2tc1_tc2603.pdf>`_.
Extracted with ``pdftotext -layout``; page layout and publication wording are retained.

.. code-block:: text

                                                Recommended Standard CCSDS 502.0-B-3
                                                TECHNICAL CORRIGENDUM 1

                                                Issue Date: June 2026


   Orbit Data Messages
   TECHNICAL CORRIGENDUM 1

   The Management Council of the Consultative Committee for Space Data Systems (CCSDS) has
   authorized the publication of technical corrigendum 1 to CCSDS 502.0-B-3, issued June 2026.


   Section 6.2.11: USER_DEFINED data block enhancement for time or event sequences
   Per user request, the was enhanced to permit the sharing of either time- or event-
   based content histories within the USER_DEFINED data block. The new feature relies upon
   the establishment of an ICD to ensure that such a sequence of data is well
   understood. The specific changes were as follows:
   “… At most, only one User-Defined Parameters section shall appear in an OCM.
   User-Defined parameters keywords may be assigned multiple values (either comma- or space-
   separated), and optionally prepended and postpended by brackets, parentheses, or other characters
   in compliance with and as specified in an accompanying ICD.
   When described and enabled by ICD, User-Defined Parameters may be used to describe a sequence
   of entries. To do this, the “USER_DEFINED_COMPOSITION_YYY” construct is used, where:
   USER_DEFINED_COMPOSITION_ may not be used by any other user-defined parameters and
   specifically denotes that it will be followed by a sequence of data lines
   The “YYY” following “USER_DEFINED_COMPOSITION_” is user-selectable, in compliance
   with the ICD agreement;
   The “USER_DEFINED_COMPOSITION_YYY” keyword (with “YYY” replaced by the
   sequence ID in compliance with the accompanying ICD), shall be set equal to the comma-
   delimited ordered elements of information to be provided on each and every line immediately
   following the “USER_DEFINED_COMPOSITION_YYY =” specification line.

   Distribution Control Number: TC 2603                          Reference Number: CCSDS 502.0-B-3 Cor. 1

             NOTE     –   Current versions of CCSDS documents are maintained at the CCSDS Web site:

                                             http://www.ccsds.org/

   Correspondence regarding CCSDS documents should be addressed to

           CCSDS Secretariat
           Space Communications and Navigation Office, 7L70
           Space Operations Mission Directorate
           NASA Headquarters
           Washington, DC 20546-0001, USA

        TECHNICAL CORRIGENDUM 3 TO CCSDS 122.0-B-1 (Continued)

   The sequence of data lines is monotonically increasing (e.g., ordered by time, event, or sample
   number).
   The sequence of data lines is terminated by the presence of either another
   “USER_DEFINED_[…]” keyword, or by “USER_STOP”.
   For example, the following would be a valid time history within a USER_DEFINED data block:
   USER_START
   USER_DEFINED_COMPOSITION_GNSS = TIME_ABSOLUTE, PDOP, TDOP,
   GNSS_POS_SIGMA, GNSS_VEL_SIGMA
   2025-03-12T01:56:26.681583            2.1    0.8     0.01 0.00022
   2025-03-12T01:56:46.267761            1.7    0.8     0.01 0.00022
   2025-03-12T01:57:06.152206            1.4    0.6     0.01 0.00022
                           ⋮
   USER_STOP
   … Table 6-12 provides an overview of the OCM user-defined data section. Only those
   keywords shown in table 6-12 shall be used in an OCM user-defined data specification.”


   Section 6.2.5 (Trajectory State Time History) Section 6.2.7 (Covariance Time History):
   Upgrade of TRAJ_ID specification to “conditional”
   In order to resolve message ambiguities in the event where multiple trajectory state time
   histories or covariance time histories are provided within a single OCM, the requirement
   for TRAJ_ID to be provided has been elevated from “optional” to “conditional” in the
   trajectory data block and also introduced into the Covariance Time History block. This
   will allow the unambiguous mapping of each trajectory data block to its corresponding
   covariance data block. The following was modified to be “C” (conditional) in the trajectory
   state time history keyword Table 6-4:
   TRAJ_ID            Free-text field containing the identification number for this trajectory state time history block.
                      NOTE: Must be supplied if covariance time histories are supplied AND multiple trajectory time
                      histories (either for the same TRAJ_BASIS or for multiple TRAJ_BASIS keyword values) are
                      supplied in a single OCM.

   And the following was added as “C” (conditional) in the covariance time history Table 6-6:
   TRAJ_ID           Free-text field containing the identification number of a corresponding trajectory time history
                     block that this covariance time history block pairs with.
                     NOTE: Must be supplied if multiple trajectory and/or covariance blocks are supplied in a single
                     OCM.

   Section 6.2.4: Addition of phone book elements of information to OCM Metadata
   Optional point-of-contact information keywords seen to be necessary at the international
   level to coordinate and enable flight safety and support UN COPUOS Long Term
   Sustainability (LTS) guidelines have been added to the OCM to share point-of-contact
   information. The new optional keywords are reflected in Table 6-3:
   OPERATOR_URL                Free-text field containing the URL of the operator of the
                               space object, especially useful for sharing concepts of
                               operation for mitigation of collision risk or RFI or other
                               time-sensitive issues.




   Distribution Control Number: TC 2603                               Reference Number: CCSDS 122.0-B-1 Cor. 3

                                                    Page 2 of 4

        TECHNICAL CORRIGENDUM 3 TO CCSDS 122.0-B-1 (Continued)
   OPERATOR_EMAIL                Free-text field containing the operator’s email
                                 address to be used in non-emergencies (can be for a
                                 team or an individual).
   OPERATOR_PHONE                Free-text field containing the operator’s phone
                                 number to be used in non-emergencies (can be for a
                                 team or an individual).
   OPERATOR_AUTOMATED_URL        Free-text field containing the URL of the operator’s
                                 automated coordination or other machine-to-machine
                                 API, if available.
   EMERGENCY_EMAIL               Free-text field containing the email address to be used
                                 in the case of emergency (can be for a spacecraft
                                 operations team or an individual). Spacecraft operators
                                 should provide a PoC for someone who can be
                                 contacted in the case of perceived collision risk
                                 or other time-sensitive event. For example, could be a
                                 24/7 operations team or an on-duty operator contact
                                 who can either move the spacecraft
                                 OR immediately interact with others who can move the
                                 spacecraft if/when necessary.
   EMERGENCY_PHONE               Free-text field containing the phone number to be used
                                 in case of an emergency (can be for a spacecraft
                                 operations team or an individual).
   EMERGENCY_AVAILABILITY        Free-text field containing the hours of availability for
                                 their emergency contact (can be for a spacecraft
                                 operations team or an individual).
   SSA_EMAIL                     Free-text field containing the operator’s primary
                                 external SSA service provider email address (can be for
                                 a team or an individual). For operational spacecraft,
                                 operators should provide an SSA service PoC for
                                 someone who can be contacted in the case of
                                 perceived collision risk or other time-sensitive event.
   SSA_PHONE                     Free-text field containing the operator’s primary
                                 external SSA service provider phone number to be used
                                 in case of an emergency (can be for a team or an
                                 individual).
   OWNER_URL                     Free-text field containing the URL of the owner of the
                                 space object.
   COUNTRY                       Free-text field containing the name of the country,
                                 country code, or country abbreviation where the space
                                 object owner is basedregistered.

   Section 6.2.7: Clarification of event-based and time-based covariances
   To address past confusion on the use of the TCARTPV and CARTPV keyword values,
   clarifying text now clearly indicates that those beginning with a “T” correspond to event-
   based covariance matrices that include dispersions in time, and that more generally, the
   6x6 specifier (CARTPV) is required/appropriate for sharing covariance data tagged to a
   specific time as clarified in Table 6-6:
   COV_TYPE     Indicates covariance composition. Select from annex B, subsections B7 and B8.

                NOTE: All Cartesian covariance representations generally begin with “CART”. Those beginning
                with a “T” prepending “CART_...” denote that time also has an uncertainty associated with
                it. Thus, “CARTPV” denotes a 6x6 matrix having uncertainties in Position (x, y, z) and Velocity
                (x, y, z), while “TCARTPV” denotes a 7x7 matrix beginning with uncertainties in Time followed by
                uncertainties in Position, and Velocity. Uncertainties in time are specifically useful when
                characterizing errors for EVENTS such as “Main Engine Cutoff” or Acquisition of Signal, which
                are typically driven by the event’s occurrence rather than by a specific time.


   Distribution Control Number: TC 2603                                Reference Number: CCSDS 122.0-B-1 Cor. 3

                                                     Page 3 of 4

        TECHNICAL CORRIGENDUM 3 TO CCSDS 122.0-B-1 (Continued)

   Section 6.2.1: Replacement of “Orbit State” with “Trajectory State”
   During development of the OCM, the data block formerly named the “Orbit Data” block
   was renamed to be the “Trajectory Block”. But in the general overview section for the
   OCM, a few lingering “Orbit Data” descriptions were used. For consistency, these
   were renamed to “Trajectory Data” in the overview table for the OCM.




   Distribution Control Number: TC 2603                 Reference Number: CCSDS 122.0-B-1 Cor. 3

                                          Page 4 of 4
