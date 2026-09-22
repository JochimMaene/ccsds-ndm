use crate::common::assert_validation_field;
use ccsds_ndm::common::StateVector;
use ccsds_ndm::messages::opm::{KeplerianElements, OpmData};
use ccsds_ndm::types::{
    Angle, AngleUnits, Distance, Gm, GmUnits, Inclination, NonNegativeDouble, PositionUnits,
    Velocity,
};
#[test]
fn keplerian_elements_validation() {
    use ccsds_ndm::Validate;
    let mut kep = KeplerianElements::builder()
        .semi_major_axis(Distance::new(7000.0, Some(PositionUnits::Km)))
        .eccentricity(NonNegativeDouble::new(0.001).unwrap())
        .inclination(Inclination::new(45.0, Some(AngleUnits::Deg)).unwrap())
        .ra_of_asc_node(Angle::new(90.0, Some(AngleUnits::Deg)).unwrap())
        .arg_of_pericenter(Angle::new(180.0, Some(AngleUnits::Deg)).unwrap())
        .gm(Gm::new(398600.44, Some(GmUnits::Km3PerS2)).unwrap())
        .build();

    // Neither anomaly
    assert_validation_field(&kep.validate().unwrap_err(), "TRUE_ANOMALY");

    // Both anomalies
    kep.true_anomaly = Some(Angle::new(0.0, Some(AngleUnits::Deg)).unwrap());
    kep.mean_anomaly = Some(Angle::new(0.0, Some(AngleUnits::Deg)).unwrap());
    assert_validation_field(&kep.validate().unwrap_err(), "TRUE_ANOMALY");

    // Exactly one (true)
    kep.mean_anomaly = None;
    assert!(kep.validate().is_ok());

    // Exactly one (mean)
    kep.true_anomaly = None;
    kep.mean_anomaly = Some(Angle::new(0.0, Some(AngleUnits::Deg)).unwrap());
    assert!(kep.validate().is_ok());
}

#[test]
fn opm_data_validation() {
    use ccsds_ndm::Validate;
    let mut data = OpmData::builder()
        .state_vector(
            StateVector::builder()
                .epoch("2023-01-01T00:00:00".parse().unwrap())
                .x(Distance::new(1.0, None))
                .y(Distance::new(1.0, None))
                .z(Distance::new(1.0, None))
                .x_dot(Velocity::new(1.0, None))
                .y_dot(Velocity::new(1.0, None))
                .z_dot(Velocity::new(1.0, None))
                .build(),
        )
        .build();

    assert!(data.validate().is_ok());

    // With invalid KeplerianElements
    data.keplerian_elements = Some(
        KeplerianElements::builder()
            .semi_major_axis(Distance::new(7000.0, Some(PositionUnits::Km)))
            .eccentricity(NonNegativeDouble::new(0.001).unwrap())
            .inclination(Inclination::new(45.0, Some(AngleUnits::Deg)).unwrap())
            .ra_of_asc_node(Angle::new(90.0, Some(AngleUnits::Deg)).unwrap())
            .arg_of_pericenter(Angle::new(180.0, Some(AngleUnits::Deg)).unwrap())
            .gm(Gm::new(398600.44, Some(GmUnits::Km3PerS2)).unwrap())
            .build(),
    );
    assert_validation_field(&data.validate().unwrap_err(), "TRUE_ANOMALY");
}
