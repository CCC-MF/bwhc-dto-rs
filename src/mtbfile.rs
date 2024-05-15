use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MtbFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub care_plans: Option<Vec<CarePlan>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claim_responses: Option<Vec<ClaimResponse>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub claims: Option<Vec<Claim>>,

    pub consent: Consent,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub diagnoses: Option<Vec<Diagnosis>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ecog_status: Option<Vec<EcogStatus>>,

    pub episode: Episode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_member_diagnoses: Option<Vec<FamilyMemberDiagnosis>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counselling_requests: Option<Vec<GeneticCounsellingRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_reevaluation_requests: Option<Vec<HistologyReevaluationRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_reports: Option<Vec<HistologyReport>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_guideline_therapies: Option<Vec<LastGuidelineTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_pathology_findings: Option<Vec<MolecularPathologyFinding>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_therapies: Option<Vec<MolecularTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ngs_reports: Option<Vec<NgsReport>>,

    pub patient: Patient,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_guideline_therapies: Option<Vec<PreviousGuidelineTherapy>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebiopsy_requests: Option<Vec<RebiopsyRequest>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<Recommendation>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<Vec<Response>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub specimens: Option<Vec<Specimen>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_inclusion_requests: Option<Vec<StudyInclusionRequest>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CarePlan {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    pub diagnosis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub genetic_counselling_request: Option<String>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_target_finding: Option<NoTargetFinding>,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rebiopsy_requests: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendations: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub study_inclusion_requests: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NoTargetFinding {
    pub diagnosis: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
    pub claim: String,

    pub id: String,

    pub issued_on: String,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<Reason>,

    pub status: ClaimResponseStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    #[serde(rename = "insufficient-evidence")]
    InsufficientEvidence,

    Other,

    #[serde(rename = "standard-therapy-not-exhausted")]
    StandardTherapyNotExhausted,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClaimResponseStatus {
    Accepted,

    Rejected,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: String,

    pub issued_on: String,

    pub patient: String,

    pub therapy: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Consent {
    pub id: String,

    pub patient: String,

    pub status: ConsentStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStatus {
    Active,

    Rejected,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnosis {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guideline_treatment_status: Option<GuidelineTreatmentStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub histology_results: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd10: Option<DiagnosisIcd10>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icd_o3_t: Option<IcdO3T>,

    pub id: String,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub recorded_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_history: Option<Vec<StatusHistory>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub who_grade: Option<WhoGrade>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GuidelineTreatmentStatus {
    Exhausted,

    Impossible,

    #[serde(rename = "no-guidelines-available")]
    NoGuidelinesAvailable,

    #[serde(rename = "non-exhausted")]
    NonExhausted,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DiagnosisIcd10 {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IcdO3T {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatusHistory {
    pub date: String,

    pub status: DiagnosisStatus,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosisStatus {
    Local,

    Metastasized,

    #[serde(rename = "tumor-free")]
    TumorFree,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WhoGrade {
    pub code: CodeEnum,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum CodeEnum {
    I,

    #[serde(rename = "II")]
    Ii,

    #[serde(rename = "III")]
    Iii,

    #[serde(rename = "IV")]
    Iv,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EcogStatus {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<String>,

    pub id: String,

    pub patient: String,

    pub value: EcogStatusValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EcogStatusValue {
    pub code: Ecog,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Ecog {
    #[serde(rename = "0")]
    The0,

    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,

    #[serde(rename = "4")]
    The4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Episode {
    pub id: String,

    pub patient: String,

    pub period: EpisodePeriod,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EpisodePeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyMemberDiagnosis {
    pub id: String,

    pub patient: String,

    pub relationship: Relationship,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Relationship {
    pub code: RelationshipCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum RelationshipCode {
    #[serde(rename = "EXT")]
    Ext,

    #[serde(rename = "FAMMEMB")]
    Fammemb,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneticCounsellingRequest {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: String,

    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReevaluationRequest {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: String,

    pub specimen: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReport {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: String,

    pub specimen: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<HistologyReportTumorCellContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_morphology: Option<TumorMorphology>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistologyReportTumorCellContent {
    pub id: String,

    pub method: TumorCellContentMethod,

    pub specimen: String,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TumorCellContentMethod {
    Bioinformatic,

    Histologic,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumorMorphology {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    pub patient: String,

    pub specimen: String,

    pub value: TumorMorphologyValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TumorMorphologyValue {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LastGuidelineTherapy {
    pub diagnosis: String,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<LastGuidelineTherapyMedication>>,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<LastGuidelineTherapyPeriod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_stopped: Option<LastGuidelineTherapyReasonStopped>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastGuidelineTherapyMedication {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub system: System,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum System {
    #[serde(rename = "ATC")]
    Atc,

    Unregistered,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastGuidelineTherapyPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LastGuidelineTherapyReasonStopped {
    pub code: GuidelineTherapyStopReason,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum GuidelineTherapyStopReason {
    #[serde(rename = "chronic-remission")]
    ChronicRemission,

    Deterioration,

    Other,

    #[serde(rename = "patient-wish")]
    PatientWish,

    Progression,

    Toxicity,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MolecularPathologyFinding {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub note: String,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub performing_institute: Option<String>,

    pub specimen: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MolecularTherapy {
    pub history: Vec<History>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub based_on: String,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_done_reason: Option<NotDoneReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,

    pub patient: String,

    pub recorded_on: String,

    pub status: MolecularTherapyStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dosage: Option<Dosage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<HistoryMedication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<HistoryPeriod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_stopped: Option<HistoryReasonStopped>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Dosage {
    #[serde(rename = ">=50%")]
    Dosage50,

    #[serde(rename = "<50%")]
    The50,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryMedication {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub system: System,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NotDoneReason {
    pub code: NotDoneReasonCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum NotDoneReasonCode {
    #[serde(rename = "continued-externally")]
    ContinuedExternally,

    #[serde(rename = "lost-to-fu")]
    LostToFu,

    #[serde(rename = "medical-reason")]
    MedicalReason,

    #[serde(rename = "no-indication")]
    NoIndication,

    Other,

    #[serde(rename = "other-therapy-chosen")]
    OtherTherapyChosen,

    #[serde(rename = "patient-death")]
    PatientDeath,

    #[serde(rename = "patient-refusal")]
    PatientRefusal,

    #[serde(rename = "payment-pending")]
    PaymentPending,

    #[serde(rename = "payment-refused")]
    PaymentRefused,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryPeriod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<String>,

    pub start: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryReasonStopped {
    pub code: MolekularTherapyStopReason,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MolekularTherapyStopReason {
    #[serde(rename = "continued-externally")]
    ContinuedExternally,

    Deterioration,

    #[serde(rename = "medical-reason")]
    MedicalReason,

    Other,

    #[serde(rename = "other-therapy-chosen")]
    OtherTherapyChosen,

    #[serde(rename = "patient-death")]
    PatientDeath,

    #[serde(rename = "patient-wish")]
    PatientWish,

    #[serde(rename = "payment-ended")]
    PaymentEnded,

    Progression,

    Remission,

    Toxicity,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum MolecularTherapyStatus {
    Completed,

    #[serde(rename = "not-done")]
    NotDone,

    #[serde(rename = "on-going")]
    OnGoing,

    Stopped,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NgsReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brcaness: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_variants: Option<Vec<CopyNumberVariant>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_fusions: Option<Vec<DnaFusion>>,

    pub id: String,

    pub issue_date: String,

    pub metadata: Vec<Metadatum>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub msi: Option<f64>,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_fusions: Option<Vec<RnaFusion>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub rna_seqs: Option<Vec<RnaSeq>>,

    pub sequencing_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub simple_variants: Option<Vec<SimpleVariant>>,

    pub specimen: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tmb: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tumor_cell_content: Option<NgsReportTumorCellContent>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyNumberVariant {
    pub chromosome: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_a: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cn_b: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copy_number_neutral_lo_h: Option<Vec<CopyNumberNeutralLoH>>,

    pub end_range: EndRange,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_copy_number: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_affected_genes: Option<Vec<ReportedAffectedGene>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_focality: Option<String>,

    pub start_range: StartRange,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_copy_number: Option<i64>,

    #[serde(rename = "type")]
    pub copy_number_variant_type: CnvType,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CopyNumberNeutralLoH {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CnvType {
    #[serde(rename = "high-level-gain")]
    HighLevelGain,

    Loss,

    #[serde(rename = "low-level-gain")]
    LowLevelGain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EndRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportedAffectedGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartRange {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DnaFusion {
    #[serde(rename = "fusionPartner3prime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_partner3_prime: Option<DnaFusionFusionPartner3Prime>,

    #[serde(rename = "fusionPartner5prime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_partner5_prime: Option<DnaFusionFusionPartner5Prime>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_num_reads: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnaFusionFusionPartner3Prime {
    pub chromosome: String,

    pub gene: PurpleGene,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurpleGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnaFusionFusionPartner5Prime {
    pub chromosome: String,

    pub gene: FluffyGene,

    pub position: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FluffyGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadatum {
    pub kit_manufacturer: String,

    pub kit_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pipeline: Option<String>,

    pub reference_genome: String,

    pub sequencer: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosmic_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub effect: Option<String>,

    #[serde(rename = "fusionPartner3prime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_partner3_prime: Option<RnaFusionFusionPartner3Prime>,

    #[serde(rename = "fusionPartner5prime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fusion_partner5_prime: Option<RnaFusionFusionPartner5Prime>,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reported_num_reads: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner3Prime {
    pub exon: String,

    pub gene: TentacledGene,

    pub position: f64,

    pub strand: Strand,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TentacledGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Strand {
    #[serde(rename = "+")]
    Empty,

    #[serde(rename = "-")]
    Strand,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner5Prime {
    pub exon: String,

    pub gene: StickyGene,

    pub position: f64,

    pub strand: Strand,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StickyGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeq {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cohort_ranking: Option<i64>,

    pub ensembl_id: String,

    pub entrez_id: String,

    pub fragments_per_kilobase_million: f64,

    #[serde(rename = "fromNGS")]
    pub from_ngs: bool,

    pub gene: RnaSeqGene,

    pub id: String,

    pub library_size: i64,

    pub raw_counts: i64,

    pub tissue_corrected_expression: bool,

    pub transcript_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeqGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleVariant {
    pub allelic_frequency: f64,

    pub alt_allele: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub amino_acid_change: Option<AminoAcidChange>,

    pub chromosome: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cosmic_id: Option<String>,

    #[serde(rename = "dbSNPId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub db_snp_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dna_change: Option<DnaChange>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gene: Option<SimpleVariantGene>,

    pub id: String,

    pub interpretation: Interpretation,

    pub read_depth: i64,

    pub ref_allele: String,

    pub start_end: StartEnd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AminoAcidChange {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DnaChange {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SimpleVariantGene {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ensembl_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hgnc_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Interpretation {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StartEnd {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NgsReportTumorCellContent {
    pub id: String,

    pub method: TumorCellContentMethod,

    pub specimen: String,

    pub value: f64,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_death: Option<String>,

    pub gender: Gender,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub insurance: Option<String>,

    #[serde(rename = "managingZPM")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub managing_zpm: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreviousGuidelineTherapy {
    pub diagnosis: String,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<PreviousGuidelineTherapyMedication>>,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub therapy_line: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviousGuidelineTherapyMedication {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub system: System,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RebiopsyRequest {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub patient: String,

    pub specimen: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub diagnosis: String,

    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level_of_evidence: Option<LevelOfEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication: Option<Vec<RecommendationMedication>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ngs_report: Option<String>,

    pub patient: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub supporting_variants: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LevelOfEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addendums: Option<Vec<Addendum>>,

    pub grading: Grading,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Addendum {
    pub code: AddendumCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AddendumCode {
    #[serde(rename = "is")]
    Is,

    #[serde(rename = "iv")]
    Iv,

    R,

    Z,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Grading {
    pub code: GradingCode,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GradingCode {
    #[serde(rename = "m1A")]
    M1A,

    #[serde(rename = "m1B")]
    M1B,

    #[serde(rename = "m1C")]
    M1C,

    #[serde(rename = "m2A")]
    M2A,

    #[serde(rename = "m2B")]
    M2B,

    #[serde(rename = "m2C")]
    M2C,

    M3,

    M4,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecommendationMedication {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    pub system: System,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Priority {
    #[serde(rename = "1")]
    The1,

    #[serde(rename = "2")]
    The2,

    #[serde(rename = "3")]
    The3,

    #[serde(rename = "4")]
    The4,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub effective_date: String,

    pub id: String,

    pub patient: String,

    pub therapy: String,

    pub value: ResponseValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseValue {
    pub code: Recist,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Recist {
    #[serde(rename = "CR")]
    Cr,

    #[serde(rename = "MR")]
    Mr,

    #[serde(rename = "NA")]
    Na,

    #[serde(rename = "NYA")]
    Nya,

    #[serde(rename = "PD")]
    Pd,

    #[serde(rename = "PR")]
    Pr,

    #[serde(rename = "SD")]
    Sd,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Specimen {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection: Option<Collection>,

    pub icd10: SpecimenIcd10,

    pub id: String,

    pub patient: String,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specimen_type: Option<SpecimenType>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Collection {
    pub date: String,

    pub localization: Localization,

    pub method: CollectionMethod,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Localization {
    Metastasis,

    #[serde(rename = "primary-tumor")]
    PrimaryTumor,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CollectionMethod {
    Biopsy,

    Cytology,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Resection,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecimenIcd10 {
    pub code: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SpecimenType {
    #[serde(rename = "cryo-frozen")]
    CryoFrozen,

    #[serde(rename = "FFPE")]
    Ffpe,

    #[serde(rename = "fresh-tissue")]
    FreshTissue,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Unknown,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StudyInclusionRequest {
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issued_on: Option<String>,

    pub nct_number: String,

    pub patient: String,

    pub reason: String,
}
