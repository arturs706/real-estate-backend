use serde::{Serialize, Deserialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;
use chrono::{NaiveDate, DateTime, Utc};




#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "certificate_type", rename_all = "lowercase")]
pub enum CertificateType {
    ElectricalSafetyCertificate,
    GasSafetyCertificate,
    EnergyPerformanceCertificate,
    FireSafetyCertificate,
    WaterSafetyCertificate,
    BuildingRegulationComplianceCertificate,
    PATTestingCertificate,
    FENSACertificate,
    AsbestosSurveyCertificate,
    StructuralSafetyCertificate,
    LandlordInsuranceCertificate,
    HealthAndSafetyRiskAssessment,
    BoilerServiceCertificate,
    ChimneySafetyCertificate,
    FoodHygieneCertificate,
    WasteCarrierCertificate,
    SmokeAndCO2DetectorComplianceCertificate,
}


#[derive(Clone, Serialize, Deserialize, Debug, sqlx::Type)]
#[sqlx(type_name = "responsibility_type ", rename_all = "lowercase")]
pub enum ResponsibilityType {
    Landlord,
    PropertyManager,
    Manager,
    Developer,
    Broker,
    SalesAgent,
    Legal,
    Investor,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Certificate {
    pub certificate_id: Uuid,
    pub property_id: Uuid,
    pub certificate_type: CertificateType,
    pub expiry_date: Option<NaiveDate>,
    pub responsibility_name: ResponsibilityType,
    pub start_date: NaiveDate,
    pub updated_at: Option<DateTime<Utc>>,
    pub created_at: Option<DateTime<Utc>>,
}

