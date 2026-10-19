"""Keep constructor arguments and combined constituents precise in generated stubs."""

from ccsds_ndm import (
    CdmMetadata,
    CdmObjectType,
    CombinedNdm,
    CovarianceMethodType,
    ManeuverableType,
    OcmMetadata,
    Oem,
    RdmMetadata,
    ReferenceFrameType,
    TdmMetadata,
)


def construct(oem: Oem) -> None:
    CdmMetadata(
        object=CdmObjectType.Object1,
        object_designator="12345",
        catalog_name="SATCAT",
        object_name="SAT1",
        international_designator="2023-001A",
        ref_frame=ReferenceFrameType.Eme2000,
        covariance_method=CovarianceMethodType.Calculated,
        maneuverable=ManeuverableType.No,
    )
    CdmMetadata(
        object=42,  # type: ignore[arg-type]
        object_designator="12345",
        catalog_name="SATCAT",
        object_name="SAT1",
        international_designator="2023-001A",
        ref_frame=42,  # type: ignore[arg-type]
        covariance_method=42,  # type: ignore[arg-type]
        maneuverable=42,  # type: ignore[arg-type]
    )
    combined = CombinedNdm([oem])
    combined.messages = (oem,)
    CombinedNdm([combined])  # type: ignore[list-item]
    combined.messages = [combined]  # type: ignore[list-item]
    OcmMetadata(
        epoch_tzero="2023-01-01T00:00:00",
        time_system="UTC",
        object_type=42,  # type: ignore[arg-type]
    )
    RdmMetadata(
        object_name="SAT1",
        international_designator="2023-001A",
        epoch_tzero="2023-01-01T00:00:00",
        center_name="EARTH",
        time_system="UTC",
        controlled_reentry=42,  # type: ignore[arg-type]
        object_type=42,  # type: ignore[arg-type]
    )
    TdmMetadata(
        time_system="UTC",
        participant_1="SAT1",
        mode=42,  # type: ignore[arg-type]
        path=42,  # type: ignore[arg-type]
        corrections_applied=42,  # type: ignore[arg-type]
    )
