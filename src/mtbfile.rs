use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MtbFile {
    pub care_plans: Option<Vec<CarePlan>>,

    pub claim_responses: Option<Vec<ClaimResponse>>,

    pub claims: Option<Vec<Claim>>,

    pub consent: Consent,

    pub diagnoses: Option<Vec<Diagnosis>>,

    pub ecog_status: Option<Vec<EcogStatus>>,

    pub episode: Episode,

    pub family_member_diagnoses: Option<Vec<FamilyMemberDiagnosis>>,

    pub genetic_counselling_requests: Option<Vec<GeneticCounsellingRequest>>,

    pub histology_reevaluation_requests: Option<Vec<HistologyReevaluationRequest>>,

    pub histology_reports: Option<Vec<HistologyReport>>,

    pub last_guideline_therapies: Option<Vec<LastGuidelineTherapy>>,

    pub molecular_pathology_findings: Option<Vec<MolecularPathologyFinding>>,

    pub molecular_therapies: Option<Vec<MolecularTherapy>>,

    pub ngs_reports: Option<Vec<NgsReport>>,

    pub patient: Patient,

    pub previous_guideline_therapies: Option<Vec<PreviousGuidelineTherapy>>,

    pub rebiopsy_requests: Option<Vec<RebiopsyRequest>>,

    pub recommendations: Option<Vec<Recommendation>>,

    pub responses: Option<Vec<Response>>,

    pub specimens: Option<Vec<Specimen>>,

    pub study_inclusion_requests: Option<Vec<StudyInclusionRequest>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CarePlan {
    pub description: Option<String>,

    pub diagnosis: String,

    pub genetic_counselling_request: Option<String>,

    pub id: String,

    pub issued_on: Option<String>,

    pub no_target_finding: Option<NoTargetFinding>,

    pub patient: String,

    pub rebiopsy_requests: Option<Vec<String>>,

    pub recommendations: Option<Vec<String>>,

    pub study_inclusion_requests: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NoTargetFinding {
    pub diagnosis: String,

    pub issued_on: Option<String>,

    pub patient: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ClaimResponse {
    pub claim: String,

    pub id: String,

    pub issued_on: String,

    pub patient: String,

    pub reason: Option<Reason>,

    pub status: ClaimResponseStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Reason {
    #[serde(rename = "insufficient-evidence")]
    InsufficientEvidence,

    Other,

    #[serde(rename = "standard-therapy-not-exhausted")]
    StandardTherapyNotExhausted,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ClaimResponseStatus {
    Accepted,

    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Claim {
    pub id: String,

    pub issued_on: String,

    pub patient: String,

    pub therapy: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Consent {
    pub id: String,

    pub patient: String,

    pub status: ConsentStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ConsentStatus {
    Active,

    Rejected,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Diagnosis {
    pub guideline_treatment_status: Option<GuidelineTreatmentStatus>,

    pub histology_results: Option<Vec<String>>,

    pub icd10: Option<DiagnosisIcd10>,

    pub icd_o3_t: Option<IcdO3T>,

    pub id: String,

    pub patient: String,

    pub recorded_on: Option<String>,

    pub status_history: Option<Vec<StatusHistory>>,

    pub who_grade: Option<WhoGrade>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct DiagnosisIcd10 {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct IcdO3T {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatusHistory {
    pub date: String,

    pub status: DiagnosisStatus,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum DiagnosisStatus {
    Local,

    Metastasized,

    #[serde(rename = "tumor-free")]
    TumorFree,

    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WhoGrade {
    pub code: CodeEnum,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum CodeEnum {
    I,

    #[serde(rename = "II")]
    Ii,

    #[serde(rename = "III")]
    Iii,

    #[serde(rename = "IV")]
    Iv,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EcogStatus {
    pub effective_date: Option<String>,

    pub id: String,

    pub patient: String,

    pub value: EcogStatusValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EcogStatusValue {
    pub code: Ecog,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Episode {
    pub id: String,

    pub patient: String,

    pub period: EpisodePeriod,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EpisodePeriod {
    pub end: Option<String>,

    pub start: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FamilyMemberDiagnosis {
    pub id: String,

    pub patient: String,

    pub relationship: Relationship,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Relationship {
    pub code: RelationshipCode,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum RelationshipCode {
    #[serde(rename = "EXT")]
    Ext,

    #[serde(rename = "FAMMEMB")]
    Fammemb,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneticCounsellingRequest {
    pub id: String,

    pub issued_on: Option<String>,

    pub patient: String,

    pub reason: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReevaluationRequest {
    pub id: String,

    pub issued_on: Option<String>,

    pub patient: String,

    pub specimen: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HistologyReport {
    pub id: String,

    pub issued_on: Option<String>,

    pub patient: String,

    pub specimen: String,

    pub tumor_cell_content: Option<HistologyReportTumorCellContent>,

    pub tumor_morphology: Option<TumorMorphology>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistologyReportTumorCellContent {
    pub id: String,

    pub method: TumorCellContentMethod,

    pub specimen: String,

    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TumorCellContentMethod {
    Bioinformatic,

    Histologic,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TumorMorphology {
    pub id: String,

    pub note: Option<String>,

    pub patient: String,

    pub specimen: String,

    pub value: TumorMorphologyValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TumorMorphologyValue {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LastGuidelineTherapy {
    pub diagnosis: String,

    pub id: String,

    pub medication: Option<Vec<LastGuidelineTherapyMedication>>,

    pub patient: String,

    pub period: Option<LastGuidelineTherapyPeriod>,

    pub reason_stopped: Option<LastGuidelineTherapyReasonStopped>,

    pub therapy_line: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastGuidelineTherapyMedication {
    pub code: String,

    pub display: Option<String>,

    pub system: System,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum System {
    #[serde(rename = "ATC")]
    Atc,

    Unregistered,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastGuidelineTherapyPeriod {
    pub end: Option<String>,

    pub start: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LastGuidelineTherapyReasonStopped {
    pub code: GuidelineTherapyStopReason,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MolecularPathologyFinding {
    pub id: String,

    pub issued_on: Option<String>,

    pub note: String,

    pub patient: String,

    pub performing_institute: Option<String>,

    pub specimen: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MolecularTherapy {
    pub history: Vec<History>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct History {
    pub based_on: String,

    pub id: String,

    pub not_done_reason: Option<NotDoneReason>,

    pub note: Option<String>,

    pub patient: String,

    pub recorded_on: String,

    pub dosage: Option<Dosage>,

    pub medication: Option<Vec<HistoryMedication>>,

    pub period: Option<HistoryPeriod>,

    pub reason_stopped: Option<HistoryReasonStopped>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Dosage {
    #[serde(rename = ">=50%")]
    GreaterOrEqual50,

    #[serde(rename = "<50%")]
    Less50,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryMedication {
    pub code: String,

    pub display: Option<String>,

    pub system: System,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NotDoneReason {
    pub code: NotDoneReasonCode,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryPeriod {
    pub end: Option<String>,

    pub start: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HistoryReasonStopped {
    pub code: MolekularTherapyStopReason,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NgsReport {
    pub brcaness: Option<f64>,

    pub copy_number_variants: Option<Vec<CopyNumberVariant>>,

    pub dna_fusions: Option<Vec<DnaFusion>>,

    pub id: String,

    pub issue_date: String,

    pub metadata: Vec<Metadatum>,

    pub msi: Option<f64>,

    pub patient: String,

    pub rna_fusions: Option<Vec<RnaFusion>>,

    pub rna_seqs: Option<Vec<RnaSeq>>,

    pub sequencing_type: String,

    pub simple_variants: Option<Vec<SimpleVariant>>,

    pub specimen: String,

    pub tmb: Option<f64>,

    pub tumor_cell_content: Option<NgsReportTumorCellContent>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CopyNumberVariant {
    pub chromosome: String,

    pub cn_a: Option<f64>,

    pub cn_b: Option<f64>,

    pub copy_number_neutral_lo_h: Option<Vec<CopyNumberNeutralLoH>>,

    pub end_range: EndRange,

    pub id: String,

    pub relative_copy_number: Option<f64>,

    pub reported_affected_genes: Option<Vec<ReportedAffectedGene>>,

    pub reported_focality: Option<String>,

    pub start_range: StartRange,

    pub total_copy_number: Option<i64>,

    #[serde(rename = "type")]
    pub copy_number_variant_type: CnvType,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CopyNumberNeutralLoH {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum CnvType {
    #[serde(rename = "high-level-gain")]
    HighLevelGain,

    Loss,

    #[serde(rename = "low-level-gain")]
    LowLevelGain,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EndRange {
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ReportedAffectedGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartRange {
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DnaFusion {
    #[serde(rename = "fusionPartner3prime")]
    pub fusion_partner3_prime: Option<DnaFusionFusionPartner3Prime>,

    #[serde(rename = "fusionPartner5prime")]
    pub fusion_partner5_prime: Option<DnaFusionFusionPartner5Prime>,

    pub id: String,

    pub reported_num_reads: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnaFusionFusionPartner3Prime {
    pub chromosome: String,

    pub gene: PurpleGene,

    pub position: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PurpleGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnaFusionFusionPartner5Prime {
    pub chromosome: String,

    pub gene: FluffyGene,

    pub position: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FluffyGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadatum {
    pub kit_manufacturer: String,

    pub kit_type: String,

    pub pipeline: Option<String>,

    pub reference_genome: String,

    pub sequencer: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusion {
    pub cosmic_id: Option<String>,

    pub effect: Option<String>,

    #[serde(rename = "fusionPartner3prime")]
    pub fusion_partner3_prime: Option<RnaFusionFusionPartner3Prime>,

    #[serde(rename = "fusionPartner5prime")]
    pub fusion_partner5_prime: Option<RnaFusionFusionPartner5Prime>,

    pub id: String,

    pub reported_num_reads: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner3Prime {
    pub exon: String,

    pub gene: TentacledGene,

    pub position: f64,

    pub strand: Strand,

    pub transcript_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TentacledGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Strand {
    #[serde(rename = "+")]
    Empty,

    #[serde(rename = "-")]
    Strand,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RnaFusionFusionPartner5Prime {
    pub exon: String,

    pub gene: StickyGene,

    pub position: f64,

    pub strand: Strand,

    pub transcript_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StickyGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeq {
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RnaSeqGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleVariant {
    pub allelic_frequency: f64,

    pub alt_allele: String,

    pub amino_acid_change: Option<AminoAcidChange>,

    pub chromosome: String,

    pub cosmic_id: Option<String>,

    #[serde(rename = "dbSNPId")]
    pub db_snp_id: Option<String>,

    pub dna_change: Option<DnaChange>,

    pub gene: Option<SimpleVariantGene>,

    pub id: String,

    pub interpretation: Interpretation,

    pub read_depth: i64,

    pub ref_allele: String,

    pub start_end: StartEnd,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidChange {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DnaChange {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SimpleVariantGene {
    pub ensembl_id: Option<String>,

    pub hgnc_id: Option<String>,

    pub name: Option<String>,

    pub symbol: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Interpretation {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StartEnd {
    pub end: Option<f64>,

    pub start: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NgsReportTumorCellContent {
    pub id: String,

    pub method: TumorCellContentMethod,

    pub specimen: String,

    pub value: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Patient {
    pub birth_date: Option<String>,

    pub date_of_death: Option<String>,

    pub gender: Gender,

    pub id: String,

    pub insurance: Option<String>,

    #[serde(rename = "managingZPM")]
    pub managing_zpm: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Gender {
    Female,

    Male,

    Other,

    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PreviousGuidelineTherapy {
    pub diagnosis: String,

    pub id: String,

    pub medication: Option<Vec<PreviousGuidelineTherapyMedication>>,

    pub patient: String,

    pub therapy_line: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PreviousGuidelineTherapyMedication {
    pub code: String,

    pub display: Option<String>,

    pub system: System,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RebiopsyRequest {
    pub id: String,

    pub issued_on: Option<String>,

    pub patient: String,

    pub specimen: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Recommendation {
    pub diagnosis: String,

    pub id: String,

    pub issued_on: Option<String>,

    pub level_of_evidence: Option<LevelOfEvidence>,

    pub medication: Option<Vec<RecommendationMedication>>,

    pub ngs_report: Option<String>,

    pub patient: String,

    pub priority: Option<Priority>,

    pub supporting_variants: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelOfEvidence {
    pub addendums: Option<Vec<Addendum>>,

    pub grading: Grading,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Addendum {
    pub code: AddendumCode,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum AddendumCode {
    #[serde(rename = "is")]
    Is,

    #[serde(rename = "iv")]
    Iv,

    R,

    Z,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Grading {
    pub code: GradingCode,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct RecommendationMedication {
    pub code: String,

    pub display: Option<String>,

    pub system: System,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub effective_date: String,

    pub id: String,

    pub patient: String,

    pub therapy: String,

    pub value: ResponseValue,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseValue {
    pub code: Recist,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Specimen {
    pub collection: Option<Collection>,

    pub icd10: SpecimenIcd10,

    pub id: String,

    pub patient: String,

    #[serde(rename = "type")]
    pub specimen_type: Option<SpecimenType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Collection {
    pub date: String,

    pub localization: Localization,

    pub method: CollectionMethod,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum Localization {
    Metastasis,

    #[serde(rename = "primary-tumor")]
    PrimaryTumor,

    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub enum CollectionMethod {
    Biopsy,

    Cytology,

    #[serde(rename = "liquid-biopsy")]
    LiquidBiopsy,

    Resection,

    Unknown,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SpecimenIcd10 {
    pub code: String,

    pub display: Option<String>,

    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StudyInclusionRequest {
    pub id: String,

    pub issued_on: Option<String>,

    pub nct_number: String,

    pub patient: String,

    pub reason: String,
}
