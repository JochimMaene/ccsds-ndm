// SPDX-FileCopyrightText: 2025 Jochim Maene <jochim.maene+github@gmail.com>
//
// SPDX-License-Identifier: MPL-2.0

//! Supported CCSDS Navigation Data Message (NDM) types.
//!
//! This module contains the struct definitions for all supported messages. Each message type
//! implements the [`Ndm`](crate::traits::Ndm) trait for unified parsing and serialization.
//!
//! # Message Summary
//!
//! | Type | Name | Description | Standard |
//! |------|------|-------------|----------|
//! | **OPM** | [Orbit Parameter Message](opm) | Single state vector at a specific epoch (Keplerian, Cartesian). | CCSDS 502.0-B-3 |
//! | **OMM** | [Orbit Mean-Elements Message](omm) | Mean orbital elements (e.g., TLE/SGP4 data). | CCSDS 502.0-B-3 |
//! | **OEM** | [Orbit Ephemeris Message](oem) | Orbit state time series (ephemeris) and covariance. | CCSDS 502.0-B-3 |
//! | **OCM** | [Orbit Comprehensive Message](ocm) | Complex orbit data including maneuvers, perturbations, and covariance. | CCSDS 502.0-B-3 |
//! | **CDM** | [Conjunction Data Message](cdm) | Collision assessment data between two space objects. | CCSDS 508.0-B-1 |
//! | **TDM** | [Tracking Data Message](tdm) | Ground station tracking data (ranges, doppler, angles). | CCSDS 503.0-B-2 |
//! | **RDM** | [Reentry Data Message](rdm) | Reentry prediction information and impact windows. | CCSDS 508.1-B-1 |
//! | **APM** | [Attitude Parameter Message](apm) | Single attitude state (quaternion/Euler) at an epoch. | CCSDS 504.0-B-1 |
//! | **AEM** | [Attitude Ephemeris Message](aem) | Attitude state time series. | CCSDS 504.0-B-1 |
//! | **ACM** | [Attitude Comprehensive Message](acm) | detailed attitude data including maneuvers. | CCSDS 504.0-B-1 |
//!
//! # Common Structures
//!
//! Most messages share a common structure:
//! 1.  **Header**: Metadata about the message creation (originator, date, version).
//! 2.  **Body**: Contains one or more **Segments**.
//! 3.  **Segment**: The core data unit, consisting of:
//!     - **Metadata**: Contextual info (object name, reference frames, time system).
//!     - **Data**: The actual navigation data (state vectors, covariance, parameters).

pub mod acm;
pub mod aem;
pub mod apm;
pub mod cdm;
pub mod ndm;
pub mod ocm;
pub mod oem;
pub mod omm;
pub mod opm;
pub mod rdm;
pub mod tdm;
