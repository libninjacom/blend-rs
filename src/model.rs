use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct RawApplicationExportData(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowUpId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PartyId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ReferenceNumber(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LosId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct CrmId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LosPrimaryKey(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum DocumentType {
    #[serde(rename = "LOE_PUBLIC_RECORD:BANKRUPTCY")]
    LoePublicRecordBankruptcy,
    #[serde(rename = "LOE_PUBLIC_RECORD:TAX_LIEN")]
    LoePublicRecordTaxLien,
    #[serde(rename = "LOE_PUBLIC_RECORD:JUDGMENT")]
    LoePublicRecordJudgment,
    #[serde(rename = "LOE_LARGE_DEPOSITS:TRANSFER")]
    LoeLargeDepositsTransfer,
    #[serde(rename = "LOE_LARGE_DEPOSITS:SALE_OF_ASSET")]
    LoeLargeDepositsSaleOfAsset,
    #[serde(rename = "LOE_LARGE_DEPOSITS:INCOME")]
    LoeLargeDepositsIncome,
    #[serde(rename = "LOE_LARGE_DEPOSITS:GIFT")]
    LoeLargeDepositsGift,
    #[serde(rename = "LOE_LARGE_DEPOSITS:OTHER")]
    LoeLargeDepositsOther,
    #[serde(rename = "BANK_STATEMENT")]
    BankStatement,
    #[serde(rename = "BANK_STATEMENT:_MUTUAL_FUND_ACCOUNT")]
    BankStatementMutualFundAccount,
    #[serde(rename = "BANK_STATEMENT:_STOCK_ACCOUNT")]
    BankStatementStockAccount,
    #[serde(rename = "FINANCIAL_STATEMENT:_BALANCE_SHEET")]
    FinancialStatementBalanceSheet,
    #[serde(rename = "FINANCIAL_STATEMENT:_INCOME")]
    FinancialStatementIncome,
    #[serde(rename = "IRS1040")]
    Irs1040,
    #[serde(rename = "IRS1041")]
    Irs1041,
    #[serde(rename = "IRS1120")]
    Irs1120,
    #[serde(rename = "IRS1120_S")]
    Irs1120S,
    #[serde(rename = "LLC_BTR_DOC")]
    LlcBtrDoc,
    #[serde(rename = "IRSW2")]
    Irsw2,
    #[serde(rename = "CRAT4")]
    Crat4,
    #[serde(rename = "1003")]
    DocumentType1003,
    #[serde(rename = "4506t")]
    DocumentType4506T,
    #[serde(rename = "TAX_TRANSCRIPT")]
    TaxTranscript,
    #[serde(rename = "DOCUSIGN_CERT_OF_COMP")]
    DocusignCertOfComp,
    #[serde(rename = "PAY_STUB")]
    PayStub,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_GENERAL")]
    PropertyInsurancePolicyGeneral,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_MINE_SUBSIDENCE")]
    PropertyInsurancePolicyMineSubsidence,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_INSECT_INFESTATION")]
    PropertyInsurancePolicyInsectInfestation,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_HOMEOWNERS")]
    PropertyInsurancePolicyHomeowners,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_VOLCANO")]
    PropertyInsurancePolicyVolcano,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_FLOOD")]
    PropertyInsurancePolicyFlood,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_HAZARD")]
    PropertyInsurancePolicyHazard,
    #[serde(rename = "INSURANCE_AGENT_CONTACT_INFO")]
    InsuranceAgentContactInfo,
    #[serde(rename = "PURCHASE_AGREEMENT")]
    PurchaseAgreement,
    #[serde(rename = "OTHER_DISCLOSURES")]
    OtherDisclosures,
    #[serde(rename = "REQUEST_FOR_COPY_OF_TAX_RETURN:_IRS4506_T")]
    RequestForCopyOfTaxReturnIrs4506T,
    #[serde(rename = "RETIREMENT_ACCOUNT_STATEMENT")]
    RetirementAccountStatement,
    #[serde(rename = "SOCIAL_SECURITY_AWARD_LETTER")]
    SocialSecurityAwardLetter,
    #[serde(rename = "TRUST_AGREEMENT")]
    TrustAgreement,
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT")]
    VerificationOfEmployment,
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT_REVERIFY")]
    VerificationOfEmploymentReverify,
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT_RETRIEVED")]
    VerificationOfEmploymentRetrieved,
    #[serde(rename = "VERIFICATION_OF_INCOME")]
    VerificationOfIncome,
    #[serde(rename = "PENSION_AWARD_LETTER")]
    PensionAwardLetter,
    #[serde(rename = "ANNUITY_AWARD_LETTER")]
    AnnuityAwardLetter,
    #[serde(rename = "DEFERRED_COMPENSATION_AWARD_LETTER")]
    DeferredCompensationAwardLetter,
    #[serde(rename = "VERIFICATION_OF_MORTGAGE_OR_RENT")]
    VerificationOfMortgageOrRent,
    #[serde(rename = "VERIFICATION_OF_RENT")]
    VerificationOfRent,
    #[serde(rename = "VERIFICATION_OF_MORTGAGE")]
    VerificationOfMortgage,
    #[serde(rename = "RETIREMENT_LIQUIDITY_TERMS")]
    RetirementLiquidityTerms,
    #[serde(rename = "LETTER_OF_EXPLANATION:_CREDIT_INQUIRY")]
    LetterOfExplanationCreditInquiry,
    #[serde(rename = "LETTER_OF_EXPLANATION:_DEROGATORY_CREDIT")]
    LetterOfExplanationDerogatoryCredit,
    #[serde(rename = "LETTER_OF_EXPLANATION:_ADDRESS_VERIFICATION")]
    LetterOfExplanationAddressVerification,
    #[serde(rename = "LETTER_OF_EXPLANATION:_JOB_GAP")]
    LetterOfExplanationJobGap,
    #[serde(rename = "LETTER_OF_EXPLANATION:_LARGE_DEPOSITS")]
    LetterOfExplanationLargeDeposits,
    #[serde(rename = "LETTER_OF_EXPLANATION:SHORT_SALE")]
    LetterOfExplanationShortSale,
    #[serde(rename = "PROPERTY_TAX_BILL")]
    PropertyTaxBill,
    #[serde(rename = "MORTGAGE_STATEMENT")]
    MortgageStatement,
    #[serde(rename = "BORROWER_CONSENT_FORM")]
    BorrowerConsentForm,
    #[serde(rename = "BORROWER_INCOME_VERIFICATION_CONSENT_FORM")]
    BorrowerIncomeVerificationConsentForm,
    #[serde(rename = "APPRAISAL_WAIVER")]
    AppraisalWaiver,
    #[serde(rename = "INTENT_TO_PROCEED")]
    IntentToProceed,
    #[serde(rename = "NATIONAL_IDENTIFICATION")]
    NationalIdentification,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_MASTERPOLICY_ASSN")]
    PropertyInsurancePolicyMasterpolicyAssn,
    #[serde(rename = "MARRIAGE_CERTIFICATE")]
    MarriageCertificate,
    #[serde(rename = "DEATH_CERTIFICATE")]
    DeathCertificate,
    #[serde(rename = "DIVORCE_DECREE")]
    DivorceDecree,
    #[serde(rename = "NATIONAL_IDENTIFICATION:_SOCIAL_SECURITY_CARD")]
    NationalIdentificationSocialSecurityCard,
    #[serde(rename = "CONDOMINIUM_OCCUPANCY_CERTIFICATE")]
    CondominiumOccupancyCertificate,
    #[serde(rename = "HOMEOWNERS_ASSOCIATION_CERTIFICATION")]
    HomeownersAssociationCertification,
    #[serde(rename = "BANK_DEPOSIT_SLIP")]
    BankDepositSlip,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_HURRICANE")]
    PropertyInsurancePolicyHurricane,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_LEASEHOLD")]
    PropertyInsurancePolicyLeasehold,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_PERSONAL_PROPERTY")]
    PropertyInsurancePolicyPersonalProperty,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_STORM")]
    PropertyInsurancePolicyStorm,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_TORNADO")]
    PropertyInsurancePolicyTornado,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_WIND")]
    PropertyInsurancePolicyWind,
    #[serde(rename = "CREDIT_REPORT")]
    CreditReport,
    #[serde(rename = "PERMANENT_RESIDENT_IDENTIFICATION")]
    PermanentResidentIdentification,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_CONDOMINIUM")]
    PropertyInsurancePolicyCondominium,
    #[serde(rename = "APPRAISAL_REPORT_SINGLE FAMILY")]
    AppraisalReportSingleFamily,
    #[serde(rename = "APPRAISAL_REPORT_2_-_4_UNIT")]
    AppraisalReport24Unit,
    #[serde(rename = "APPRAISAL_REPORT_MANUFACTURED_HOME")]
    AppraisalReportManufacturedHome,
    #[serde(rename = "APPRAISAL_REPORT_COOP")]
    AppraisalReportCoop,
    #[serde(rename = "APPRAISAL_REPORT_EXTERIOR_ONLY")]
    AppraisalReportExteriorOnly,
    #[serde(rename = "APPRAISAL_REPORT_AVM")]
    AppraisalReportAvm,
    #[serde(rename = "APPRAISAL_REPORT_RENT_SURVEY")]
    AppraisalReportRentSurvey,
    #[serde(rename = "APPRAISAL_REPORT_OPERATING_INCOME_SCHEDULE")]
    AppraisalReportOperatingIncomeSchedule,
    #[serde(rename = "APPRAISAL_REPORT_MARKET_CONDITIONS_ADDENDUM")]
    AppraisalReportMarketConditionsAddendum,
    #[serde(rename = "APPRAISAL_REPORT_UPDATE_AND_OR_COMPLETION_REPORT")]
    AppraisalReportUpdateAndOrCompletionReport,
    #[serde(rename = "APPRAISAL_REPORT_PROPERTY_CONDITION_REPORT")]
    AppraisalReportPropertyConditionReport,
    #[serde(rename = "APPRAISAL_REPORT_DESK_REVIEW")]
    AppraisalReportDeskReview,
    #[serde(rename = "SETTLEMENT_STATEMENT_HUD1")]
    SettlementStatementHud1,
    #[serde(rename = "VISA")]
    Visa,
    #[serde(rename = "PASSPORT")]
    Passport,
    #[serde(rename = "NATIONAL_IDENTIFICATION_NON_PERMANENT_RESIDENT")]
    NationalIdentificationNonPermanentResident,
    #[serde(rename = "DD_214")]
    Dd214,
    #[serde(rename = "VA_STATEMENT_OF_SERVICE")]
    VaStatementOfService,
    #[serde(rename = "VA_CERTIFICATE_OF_ELIGIBILITY")]
    VaCertificateOfEligibility,
    #[serde(rename = "CREDIT_AUTHORIZATION")]
    CreditAuthorization,
    #[serde(rename = "CREDIT_CARD_AUTHORIZATION")]
    CreditCardAuthorization,
    #[serde(rename = "BIRTH_CERTIFICATE")]
    BirthCertificate,
    #[serde(rename = "INVOICE_UTILITY_BILL")]
    InvoiceUtilityBill,
    #[serde(rename = "ADDRESS_VERIFICATION")]
    AddressVerification,
    #[serde(rename = "LETTER_OF_EXPLANATION_NAME_VARIATION")]
    LetterOfExplanationNameVariation,
    #[serde(rename = "POWER_OF_ATTORNEY")]
    PowerOfAttorney,
    #[serde(rename = "RENTAL_AGREEMENT")]
    RentalAgreement,
    #[serde(rename = "RENTAL_SECURITY_DEPOSIT")]
    RentalSecurityDeposit,
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT_LETTER")]
    VerificationOfEmploymentLetter,
    #[serde(rename = "VERIFICATION_OF_EMPLOYMENT_WRITTEN")]
    VerificationOfEmploymentWritten,
    #[serde(rename = "BUSINESS_LICENSE")]
    BusinessLicense,
    #[serde(rename = "IRS1099")]
    Irs1099,
    #[serde(rename = "K1")]
    K1,
    #[serde(rename = "IRS1065")]
    Irs1065,
    #[serde(rename = "SOCIAL_SECURITY_PROOF_OF_RECEIPT")]
    SocialSecurityProofOfReceipt,
    #[serde(rename = "EMPLOYMENT_OFFER_LETTER")]
    EmploymentOfferLetter,
    #[serde(rename = "RENTAL_INCOME_PROOF_OF_RECEIPT")]
    RentalIncomeProofOfReceipt,
    #[serde(rename = "RETIREMENT_AWARD_LETTER")]
    RetirementAwardLetter,
    #[serde(rename = "VA_CHILD_CARE_CERTIFICATE")]
    VaChildCareCertificate,
    #[serde(rename = "VA_AWARD_LETTER")]
    VaAwardLetter,
    #[serde(rename = "BONUS_AND_COMMISSION_DOCUMENTATION")]
    BonusAndCommissionDocumentation,
    #[serde(rename = "CHILD_SUPPORT_INCOME")]
    ChildSupportIncome,
    #[serde(rename = "ALIMONY_INCOME")]
    AlimonyIncome,
    #[serde(rename = "CHILD_SUPPORT_LIABILITY")]
    ChildSupportLiability,
    #[serde(rename = "ALIMONY_LIABILITY")]
    AlimonyLiability,
    #[serde(rename = "GENERATED_ASSET_STATEMENT")]
    GeneratedAssetStatement,
    #[serde(rename = "ACHDEBIT_AUTHORIZATION")]
    AchdebitAuthorization,
    #[serde(rename = "GIFT_LETTER")]
    GiftLetter,
    #[serde(rename = "PROOF_OF_LIQUIDATION")]
    ProofOfLiquidation,
    #[serde(rename = "CREDIT_REPORT_NON_TRADITIONAL")]
    CreditReportNonTraditional,
    #[serde(rename = "CREDIT_SUPPLEMENT")]
    CreditSupplement,
    #[serde(rename = "BANKRUPTCY_DISCHARGE_NOTICE")]
    BankruptcyDischargeNotice,
    #[serde(rename = "BANKRUPTCY_FILING")]
    BankruptcyFiling,
    #[serde(rename = "SATISFACTION_OF_SECURITY_INSTRUMENT_LIEN_RELEASE")]
    SatisfactionOfSecurityInstrumentLienRelease,
    #[serde(rename = "FRAUD_ALERT")]
    FraudAlert,
    #[serde(rename = "STATEMENT_CREDIT_CARD_OR_INSTALLMENT_ACCT")]
    StatementCreditCardOrInstallmentAcct,
    #[serde(rename = "JUDGMENT")]
    Judgment,
    #[serde(rename = "COSIGNED_LIABILITY")]
    CosignedLiability,
    #[serde(rename = "PURCHASE_AGREEMENT_ADDENDUM")]
    PurchaseAgreementAddendum,
    #[serde(rename = "HOME_INSPECTION_REPORT")]
    HomeInspectionReport,
    #[serde(rename = "PEST_INSPECTION_REPORT")]
    PestInspectionReport,
    #[serde(rename = "ROOF_INSPECTION_REPORT")]
    RoofInspectionReport,
    #[serde(rename = "POOL_INSPECTION_REPORT")]
    PoolInspectionReport,
    #[serde(rename = "EARNEST_MONEY_DEPOSIT")]
    EarnestMoneyDeposit,
    #[serde(rename = "FLOOD_HAZARD_NOTICE")]
    FloodHazardNotice,
    #[serde(rename = "FLOOD_CERTIFICATION")]
    FloodCertification,
    #[serde(rename = "TAX_CERTIFICATE")]
    TaxCertificate,
    #[serde(rename = "AMENDATORY_CLAUSE")]
    AmendatoryClause,
    #[serde(rename = "PAYOFF_STATEMENT")]
    PayoffStatement,
    #[serde(rename = "CLOSING_PROTECTION_LETTER")]
    ClosingProtectionLetter,
    #[serde(rename = "SCHEDULE_OF_REAL_ESTATE")]
    ScheduleOfRealEstate,
    #[serde(rename = "BORROWER_CERTIFICATION")]
    BorrowerCertification,
    #[serde(rename = "OCCUPANCY_CERTIFICATION")]
    OccupancyCertification,
    #[serde(rename = "TRUTH_IN_LENDING")]
    TruthInLending,
    #[serde(rename = "GOOD_FAITH_ESTIMATE")]
    GoodFaithEstimate,
    #[serde(rename = "ESCROW_WAIVER")]
    EscrowWaiver,
    #[serde(rename = "LOAN_ESTIMATE")]
    LoanEstimate,
    #[serde(rename = "CLOSING_DISCLOSURE")]
    ClosingDisclosure,
    #[serde(rename = "GENERAL_LOAN_ACKNOWLEDGMENT")]
    GeneralLoanAcknowledgment,
    #[serde(rename = "HUD_VA_ADDENDUM")]
    HudVaAddendum,
    #[serde(rename = "BORROWER_CORRESPONDENCE:_LETTER_OF_EXPLANATION")]
    BorrowerCorrespondenceLetterOfExplanation,
    #[serde(rename = "BORROWER_CORRESPONDENCE:_LETTER_OF_EXPLANATION_OCCUPANCY")]
    BorrowerCorrespondenceLetterOfExplanationOccupancy,
    #[serde(rename = "BORROWER_CORRESPONDENCE:_LETTER_OF_EXPLANATION_CASHOUT")]
    BorrowerCorrespondenceLetterOfExplanationCashout,
    #[serde(rename = "MORTGAGE_INSURANCE")]
    MortgageInsurance,
    #[serde(rename = "NET_TANGIBLE_BENEFIT")]
    NetTangibleBenefit,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "HOA_BILL")]
    HoaBill,
    #[serde(rename = "ARTICLES_OF_INCORPORATION")]
    ArticlesOfIncorporation,
    #[serde(rename = "BYLAWS_OPERATING_AGREEMENT")]
    BylawsOperatingAgreement,
    #[serde(rename = "AFFILIATED_BUSINESS_ARRANGEMENT_DISCLOSURE")]
    AffiliatedBusinessArrangementDisclosure,
    #[serde(rename = "DISCLOSURES_TRACKING_REPORT")]
    DisclosuresTrackingReport,
    #[serde(rename = "ENERGY_EFFICIENT_MORTGAGE_WORKSHEET")]
    EnergyEfficientMortgageWorksheet,
    #[serde(rename = "IMPORTANT_NOTICE_TO_HOMEBUYER")]
    ImportantNoticeToHomebuyer,
    #[serde(rename = "FAIR_LENDING_NOTICE")]
    FairLendingNotice,
    #[serde(rename = "FOR_YOUR_PROTECTION_HOME_INSPECTION")]
    ForYourProtectionHomeInspection,
    #[serde(rename = "RATE_LOCK_AGREEMENT")]
    RateLockAgreement,
    #[serde(rename = "BOND_CERTIFICATE")]
    BondCertificate,
    #[serde(rename = "CUSTOMER_IDENTIFICATION_VERIFICATION")]
    CustomerIdentificationVerification,
    #[serde(rename = "VERIFICATION_OF_SSN")]
    VerificationOfSsn,
    #[serde(rename = "RESIDUAL_INCOME_LETTER")]
    ResidualIncomeLetter,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "DU_UNDERWRITING_FINDINGS")]
    DuUnderwritingFindings,
    #[serde(rename = "DU_UNDERWRITING_LOG")]
    DuUnderwritingLog,
    #[serde(rename = "DU_UNDERWRITING_DEFAULTS")]
    DuUnderwritingDefaults,
    #[serde(rename = "OTHER_APPLICATION")]
    OtherApplication,
    #[serde(rename = "PRE_QUALIFICATION_LETTER")]
    PreQualificationLetter,
    #[serde(rename = "APPRAISAL")]
    Appraisal,
    #[serde(rename = "PRE_APPROVAL_LETTER")]
    PreApprovalLetter,
    #[serde(rename = "PRE_APPROVAL_LETTER:_PREVIEW")]
    PreApprovalLetterPreview,
    #[serde(rename = "COMPLIANCE_REPORT")]
    ComplianceReport,
    #[serde(rename = "LOAN_SNAPSHOT")]
    LoanSnapshot,
    #[serde(rename = "HMDA_REPORT")]
    HmdaReport,
    #[serde(rename = "ATTACHMENT")]
    Attachment,
    #[serde(rename = "LETTER_OF_EXPLANATION:_GENERATED_CREDIT_INQUIRY")]
    LetterOfExplanationGeneratedCreditInquiry,
    #[serde(rename = "LETTER_OF_EXPLANATION:_GENERATED_ADDRESS_VERIFICATION")]
    LetterOfExplanationGeneratedAddressVerification,
    #[serde(rename = "GENERATED_GIFT_LETTER")]
    GeneratedGiftLetter,
    #[serde(rename = "LETTER_OF_EXPLANATION:_GENERATED_LATE_PAYMENT")]
    LetterOfExplanationGeneratedLatePayment,
    #[serde(rename = "LETTER_OF_EXPLANATION:_GENERATED_PUBLIC_RECORD")]
    LetterOfExplanationGeneratedPublicRecord,
    #[serde(rename = "LETTER_OF_EXPLANATION:_GENERATED_LARGE_DEPOSIT")]
    LetterOfExplanationGeneratedLargeDeposit,
    #[serde(rename = "MEDICAL_RESIDENCY_FELLOWSHIP")]
    MedicalResidencyFellowship,
    #[serde(rename = "MEDICAL_LICENSE_DIPLOMA")]
    MedicalLicenseDiploma,
    #[serde(rename = "NEAREST_LIVING_RELATIVE")]
    NearestLivingRelative,
    #[serde(rename = "EMPLOYMENT_CONTRACT")]
    EmploymentContract,
    #[serde(rename = "GENERATED_COST_WORKSHEET")]
    GeneratedCostWorksheet,
    #[serde(rename = "FNMA_NETPAY_REPORT_ASSETS")]
    FnmaNetpayReportAssets,
    #[serde(rename = "FNMA_NETPAY_REPORT_INCOME")]
    FnmaNetpayReportIncome,
    #[serde(rename = "FNMA_NETPAY_BORROWERS_REQUEST")]
    FnmaNetpayBorrowersRequest,
    #[serde(rename = "FNMA_NETPAY_GSE_API_XML")]
    FnmaNetpayGseApiXml,
    #[serde(rename = "KNOW_YOUR_CUSTOMER")]
    KnowYourCustomer,
    #[serde(rename = "CLOSING_PACKAGE")]
    ClosingPackage,
    #[serde(rename = "PROMISSORY_NOTE")]
    PromissoryNote,
    #[serde(rename = "DEED_OF_TRUST")]
    DeedOfTrust,
    #[serde(rename = "TITLE_DOCUMENT")]
    TitleDocument,
    #[serde(rename = "CLOSING_SETTLEMENT_ONLY")]
    ClosingSettlementOnly,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ApplicationExperienceType {
    #[serde(rename = "FULL_APPLICATION")]
    FullApplication,
    #[serde(rename = "LENDER_ENTERED")]
    LenderEntered,
    #[serde(rename = "POST_SUBMISSION")]
    PostSubmission,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum HomeLendingSolutionSubType {
    #[serde(rename = "CANADA_MORTGAGE")]
    CanadaMortgage,
    #[serde(rename = "HELOAN")]
    Heloan,
    #[serde(rename = "HELOC")]
    Heloc,
    #[serde(rename = "MORTGAGE")]
    Mortgage,
    #[serde(rename = "MORTGAGE_MODIFICATION")]
    MortgageModification,
    #[serde(rename = "OTHER")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AffordabilityRangeSchema(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveStatusSchema(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivedStatusSchema(pub bool);
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationLinks {
    #[serde(rename = "mismo")]
    pub mismo: Option<serde_json::Value>,
    #[serde(rename = "fannie")]
    pub fannie: Option<serde_json::Value>,
}
impl std::fmt::Display for ApplicationLinks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PropertyUsageType {
    #[serde(rename = "PRIMARY_RESIDENCE")]
    PrimaryResidence,
    #[serde(rename = "SECOND_HOME")]
    SecondHome,
    #[serde(rename = "PRIMARY_AND_INVESTMENT")]
    PrimaryAndInvestment,
    #[serde(rename = "INVESTMENT")]
    Investment,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EstimatedClosingDate(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LoanAmount(pub f64);
#[derive(Debug, Serialize, Deserialize)]
pub struct LosMilestoneSchema {
    #[serde(rename = "losMilestone")]
    ///ENUM describing that status of the application. The following milestones can be configured to be displayed to the Borrower on their landing page after submitting their application in Blend (SUBMITTED, PROCESSING, UNDERWRITING, CONDITIONAL APPROVAL, APPROVED, CLOSING, CLOSED, FUNDED). The other milestones are only visible to the lender application.
    pub los_milestone: Option<String>,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: Option<String>,
}
impl std::fmt::Display for LosMilestoneSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SingleLosMilestoneSchema {
    #[serde(rename = "losMilestone")]
    ///ENUM describing that status of the application. The following milestones can be configured to be displayed to the Borrower on their landing page after submitting their application in Blend (SUBMITTED, PROCESSING, UNDERWRITING, CONDITIONAL APPROVAL, APPROVED, CLOSING, CLOSED, FUNDED). The other milestones are only visible to the lender application.
    pub los_milestone: String,
}
impl std::fmt::Display for SingleLosMilestoneSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LosMilestoneName {
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "PREQUALIFIED")]
    Prequalified,
    #[serde(rename = "PREAPPROVED")]
    Preapproved,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "UNDERWRITING")]
    Underwriting,
    #[serde(rename = "CONDITIONAL_APPROVAL")]
    ConditionalApproval,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "CLOSING")]
    Closing,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "APPRAISAL_ORDERED")]
    AppraisalOrdered,
    #[serde(rename = "APPRAISAL_RECEIVED")]
    AppraisalReceived,
    #[serde(rename = "CLEAR_TO_CLOSE")]
    ClearToClose,
    #[serde(rename = "DOCS_SENT")]
    DocsSent,
    #[serde(rename = "FUNDED")]
    Funded,
    #[serde(rename = "SET_UP")]
    SetUp,
    #[serde(rename = "DECISIONED")]
    Decisioned,
    #[serde(rename = "CONDITIONS_SUBMITTED")]
    ConditionsSubmitted,
    #[serde(rename = "APPRAISAL_APPROVED")]
    AppraisalApproved,
    #[serde(rename = "RATE_LOCKED")]
    RateLocked,
    #[serde(rename = "CLOSING_DISCLOSURE_SENT")]
    ClosingDisclosureSent,
    #[serde(rename = "DENIED")]
    Denied,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxpayerIdentifierSchema {
    #[serde(rename = "value")]
    ///The identifier itself, such as an applicant's social security number.
    pub value: String,
    #[serde(rename = "type")]
    ///ENUM describing the type of identifier being used.
    pub type_: String,
}
impl std::fmt::Display for TaxpayerIdentifierSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressSchema {
    #[serde(rename = "streetAddressLine1")]
    pub street_address_line1: String,
    #[serde(rename = "streetAddressLine2")]
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    ///Two-character US state code
    pub state: String,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code
    pub zip_code: String,
    #[serde(rename = "zipCodePlusFour")]
    ///Four-digit additional US zip code numbers
    pub zip_code_plus_four: Option<String>,
    #[serde(rename = "countyName")]
    ///County name
    pub county_name: Option<String>,
}
impl std::fmt::Display for AddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartialAddressSchema {
    #[serde(rename = "streetAddressLine1")]
    pub street_address_line1: Option<String>,
    #[serde(rename = "streetAddressLine2")]
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    ///Two-digit US state code
    pub state: String,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code
    pub zip_code: String,
    #[serde(rename = "zipCodePlusFour")]
    ///Four-digit additional US zip code numbers
    pub zip_code_plus_four: Option<String>,
    #[serde(rename = "countyName")]
    pub county_name: Option<String>,
}
impl std::fmt::Display for PartialAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressWithCountyNameSchema {
    #[serde(rename = "streetAddressLine1")]
    pub street_address_line1: String,
    #[serde(rename = "streetAddressLine2")]
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    ///Two-character US state code
    pub state: String,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code
    pub zip_code: String,
    #[serde(rename = "zipCodePlusFour")]
    ///Four-digit additional US zip code numbers
    pub zip_code_plus_four: Option<String>,
    #[serde(rename = "countyName")]
    pub county_name: Option<String>,
}
impl std::fmt::Display for AddressWithCountyNameSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NameSchema {
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "suffixName")]
    pub suffix_name: Option<String>,
}
impl std::fmt::Display for NameSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowerPairSchema {
    #[serde(rename = "primaryBorrowerId")]
    ///UUID of the first applicant on a specific 1003
    pub primary_borrower_id: Option<String>,
    #[serde(rename = "secondaryBorrowerId")]
    ///UUID of the second applicant on a specific 1003
    pub secondary_borrower_id: Option<String>,
}
impl std::fmt::Display for BorrowerPairSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SignerInfoSchema {
    #[serde(rename = "name")]
    ///Full name of signer
    pub name: String,
    #[serde(rename = "partyId")]
    ///Party UUID of signer
    pub party_id: String,
    #[serde(rename = "status")]
    ///Signing status
    pub status: String,
    #[serde(rename = "completedAt")]
    ///UTC Timestamp of signature
    pub completed_at: String,
}
impl std::fmt::Display for SignerInfoSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentAdditionalEntities {
    #[serde(rename = "entityType")]
    ///Entities ids of a given entity type
    pub entity_type: Option<Vec<String>>,
}
impl std::fmt::Display for DocumentAdditionalEntities {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationRequestPartySchema {
    #[serde(rename = "name")]
    pub name: NameSchema,
    #[serde(rename = "email")]
    ///Party's email
    pub email: String,
    #[serde(rename = "taxpayerIdentifier")]
    pub taxpayer_identifier: Option<TaxpayerIdentifierSchema>,
    #[serde(rename = "dateOfBirth")]
    ///UTC Timestamp of the Date of Birth
    pub date_of_birth: Option<String>,
    #[serde(rename = "homePhone")]
    ///Nine-digit home phone number
    pub home_phone: Option<String>,
    #[serde(rename = "currentAddress")]
    pub current_address: Option<AddressParamSchema>,
    #[serde(rename = "mailingAddress")]
    pub mailing_address: Option<AddressParamSchema>,
}
impl std::fmt::Display for ApplicationRequestPartySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IncomeSchema {
    #[serde(rename = "type")]
    ///ENUM describing what type of income this is
    pub type_: String,
    #[serde(rename = "yearlyIncome")]
    pub yearly_income: Option<serde_json::Value>,
    #[serde(rename = "description")]
    ///A description of this income.
    pub description: Option<String>,
}
impl std::fmt::Display for IncomeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerSchemaBase {
    #[serde(rename = "status")]
    ///ENUM describing whether this employer is current or previous
    pub status: String,
    #[serde(rename = "name")]
    ///The employer's name.
    pub name: Option<String>,
    #[serde(rename = "type")]
    ///Employment type
    pub type_: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<AddressSchema>,
    #[serde(rename = "phoneNumber")]
    ///Employer phone number
    pub phone_number: Option<f64>,
    #[serde(rename = "startDate")]
    ///UTC Timestamp of the start of employment
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    ///UTC Timestamp of the end of employment
    pub end_date: Option<String>,
    #[serde(rename = "incomes")]
    pub incomes: Option<Vec<IncomeSchema>>,
}
impl std::fmt::Display for EmployerSchemaBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerResponseSchema {
    #[serde(rename = "employers")]
    pub employers: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for EmployerResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerIngestSchema {
    #[serde(rename = "employers")]
    pub employers: Option<Vec<EmployerSchemaBase>>,
}
impl std::fmt::Display for EmployerIngestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BorrowerAddressSchema {
    #[serde(rename = "type")]
    ///Type of address
    pub type_: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<AddressSchema>,
}
impl std::fmt::Display for BorrowerAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartyAddressSchema {
    #[serde(rename = "type")]
    ///Type of address
    pub type_: Option<String>,
    #[serde(rename = "address")]
    pub address: Option<AddressSchema>,
    #[serde(rename = "moveInDate")]
    ///UTC Timestamp of the move in for current address
    pub move_in_date: Option<String>,
}
impl std::fmt::Display for PartyAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressParamSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignmentRequestSchema {
    #[serde(rename = "assignees")]
    pub assignees: Option<Vec<UserAssignmentSchema>>,
}
impl std::fmt::Display for UserAssignmentRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignmentResponseSchema(pub Vec<UserAssignmentSchema>);
#[derive(Debug, Serialize, Deserialize)]
pub struct UserAssignmentSchema {
    #[serde(rename = "userId")]
    ///UUID of a valid lender user
    pub user_id: String,
    #[serde(rename = "role")]
    ///Assignment role (defaults to ASSIGNEE)
    pub role: Option<String>,
}
impl std::fmt::Display for UserAssignmentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LosPartyIdSchema(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PartySchema {
    #[serde(rename = "id")]
    ///The UUID of the Party in Blend's system. The static identifier that should be used to connect the party's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "type")]
    ///Identifying type of party to the application
    pub type_: String,
    #[serde(rename = "name")]
    pub name: Option<NameSchema>,
    #[serde(rename = "nameId")]
    ///A unique identifier of the party passed to Blend during SSO login.
    pub name_id: Option<String>,
    #[serde(rename = "email")]
    ///Party email
    pub email: Option<String>,
    #[serde(rename = "status")]
    ///An ENUM for the status of the party in the application.
    pub status: Option<String>,
    #[serde(rename = "dateOfBirth")]
    ///UTC timestamp representing the date of birth
    pub date_of_birth: Option<String>,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Option<Vec<PhoneNumberSchema>>,
    #[serde(rename = "econsent")]
    pub econsent: Option<EconsentSchema>,
    #[serde(rename = "relationship")]
    pub relationship: Option<RelationshipSchema>,
    #[serde(rename = "veteranStatus")]
    pub veteran_status: Option<VeteranStatusSchema>,
    #[serde(rename = "losPartyId")]
    ///The UUID or GUID of the current party in the LOS.
    pub los_party_id: Option<String>,
    #[serde(rename = "credit")]
    pub credit: Option<PartyCreditSchema>,
    #[serde(rename = "liabilities")]
    pub liabilities: Option<Vec<PartyLiabilitySchema>>,
    #[serde(rename = "incomes")]
    pub incomes: Option<Vec<ConsumerLendingIncomeSchema>>,
    #[serde(rename = "verificationRefId")]
    pub verification_ref_id: Option<VerificationRefIdSchema>,
    #[serde(rename = "addresses")]
    pub addresses: Option<Vec<PartyAddressSchema>>,
    #[serde(rename = "taxpayerIdentifier")]
    pub taxpayer_identifier: Option<TaxpayerIdentifierSchema>,
    #[serde(rename = "firstTimeHomeBuyer")]
    ///Indicates whether borrower is a first-time home buyer.
    pub first_time_home_buyer: Option<bool>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<CustomMetadata>,
}
impl std::fmt::Display for PartySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EconsentSchema {
    #[serde(rename = "status")]
    ///State of econsent
    pub status: Option<String>,
    #[serde(rename = "date")]
    ///UTC Timestamp of when Econsent was last updated
    pub date: Option<String>,
    #[serde(rename = "ip")]
    ///IP Address from which econsent was given
    pub ip: Option<String>,
}
impl std::fmt::Display for EconsentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields {
    #[serde(rename = "fieldName")]
    pub field_name: Option<CustomFieldSchema>,
}
impl std::fmt::Display for CustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomMetadata {
    #[serde(rename = "fieldName")]
    pub field_name: Option<CustomFieldSchema>,
}
impl std::fmt::Display for CustomMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFieldSchema {
    #[serde(rename = "fieldValue")]
    ///value of the field
    pub field_value: Option<String>,
    #[serde(rename = "createdAt")]
    ///UTC Timestamp of when the field was created or updated
    pub created_at: Option<String>,
}
impl std::fmt::Display for CustomFieldSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCustomFields {}
impl std::fmt::Display for RequestCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RequestCustomMetadata {}
impl std::fmt::Display for RequestCustomMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostEconsentSchema {
    #[serde(rename = "status")]
    ///State of econsent
    pub status: String,
    #[serde(rename = "date")]
    ///UTC Timestamp of when Econsent was given. Optional. Null is accepted.
    pub date: Option<String>,
    #[serde(rename = "ip")]
    ///IP Address from which econsent was given. Optional. Null is accepted.
    pub ip: Option<String>,
}
impl std::fmt::Display for PostEconsentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RelationshipSchema {
    #[serde(rename = "maritalStatus")]
    ///Marital Status
    pub marital_status: Option<String>,
}
impl std::fmt::Display for RelationshipSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VeteranStatusSchema {
    #[serde(rename = "status")]
    ///Veteran Status
    pub status: Option<String>,
}
impl std::fmt::Display for VeteranStatusSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OtherMortgageSchema {
    #[serde(rename = "creditLimit")]
    ///Credit Limit in dollars and cents
    pub credit_limit: Option<f64>,
    #[serde(rename = "creditorName")]
    ///Creditor Name
    pub creditor_name: Option<String>,
    #[serde(rename = "lienAmount")]
    ///Lien Amount
    pub lien_amount: Option<f64>,
    #[serde(rename = "lienType")]
    ///Lien Type ENUM
    pub lien_type: Option<String>,
    #[serde(rename = "loanAmount")]
    ///Loan Amount
    pub loan_amount: Option<f64>,
    #[serde(rename = "monthlyPayment")]
    ///Monthly Payment in dollars and cents
    pub monthly_payment: Option<f64>,
    #[serde(rename = "type")]
    ///Mortgage Type
    pub type_: Option<String>,
    #[serde(rename = "willPayOff")]
    ///Will pay off this mortgage at or before closing
    pub will_pay_off: Option<bool>,
}
impl std::fmt::Display for OtherMortgageSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationRefIdSchema {
    #[serde(rename = "id")]
    ///The most up to date UUID used to reference verified assets, income, and/or employment
    pub id: String,
    #[serde(rename = "lastUpdated")]
    ///UTC Timestamp of when this verification reference id was generated.
    pub last_updated: String,
}
impl std::fmt::Display for VerificationRefIdSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartyLiabilitySchema {
    #[serde(rename = "type")]
    ///ENUM indicating the type of liability
    pub type_: String,
    #[serde(rename = "monthlyPayment")]
    ///Monthly payment amount
    pub monthly_payment: f64,
}
impl std::fmt::Display for PartyLiabilitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartyCreditSchema {
    #[serde(rename = "creditReferenceNumber")]
    ///Unique identifier for the credit pull as returned by the credit provider.
    pub credit_reference_number: Option<String>,
    #[serde(rename = "creditPullDate")]
    ///UTC Timestamp of when credit was pulled for this party.
    pub credit_pull_date: Option<String>,
    #[serde(rename = "creditPullType")]
    ///Pull Type of the active credit report for this party.
    pub credit_pull_type: Option<String>,
    #[serde(rename = "equifaxScore")]
    ///Party's Equifax credit score.
    pub equifax_score: Option<f64>,
    #[serde(rename = "experianScore")]
    ///Party's Experian credit score.
    pub experian_score: Option<f64>,
    #[serde(rename = "transUnionScore")]
    ///Party's TransUnion credit score.
    pub trans_union_score: Option<f64>,
    #[serde(rename = "estimatedCreditScore")]
    ///Party's self-stated credit score.
    pub estimated_credit_score: Option<f64>,
}
impl std::fmt::Display for PartyCreditSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PartyResponseSchema {
    #[serde(rename = "id")]
    ///The UUID of the Party in Blend's system. The static identifier that should be used to connect the party's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<NameSchema>,
    #[serde(rename = "losPartyId")]
    ///Party ID in LOS
    pub los_party_id: Option<String>,
    #[serde(rename = "econsent")]
    pub econsent: Option<EconsentSchema>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<CustomMetadata>,
}
impl std::fmt::Display for PartyResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PhoneNumberSchema {
    #[serde(rename = "phoneNumber")]
    ///String representation of the phone number
    pub phone_number: Option<String>,
    #[serde(rename = "phoneNumberType")]
    ///Type of phone number
    pub phone_number_type: Option<String>,
}
impl std::fmt::Display for PhoneNumberSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RealtorRequestSchema {
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "name")]
    pub name: NameSchema,
    #[serde(rename = "contact")]
    pub contact: Option<serde_json::Value>,
}
impl std::fmt::Display for RealtorRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationDataSchema {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "hint")]
    pub hint: Option<String>,
}
impl std::fmt::Display for VerificationDataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSchema {
    #[serde(rename = "id")]
    ///The UUID of the Document in Blend's system. The static identifier that should be used to connect the document's identity across Blend and external integrations.
    pub id: Option<String>,
    #[serde(rename = "name")]
    ///Document Filename
    pub name: Option<String>,
    #[serde(rename = "type")]
    ///Blend document type
    pub type_: Option<String>,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: Option<String>,
    #[serde(rename = "losType")]
    ///LOS document type (used with Blend document templating)
    pub los_type: Option<String>,
    #[serde(rename = "losTypeId")]
    ///LOS document id (used for external tracking)
    pub los_type_id: Option<String>,
    #[serde(rename = "created")]
    ///UTC Timestamp of document creation
    pub created: Option<String>,
    #[serde(rename = "partyIds")]
    ///UUIDs of the Parties associated with this document
    pub party_ids: Option<Vec<String>>,
    #[serde(rename = "downloadUrl")]
    ///URL where the Document can be Downloaded from
    pub download_url: Option<String>,
    #[serde(rename = "lastExportedAt")]
    ///UTC Timestamp of last export for document
    pub last_exported_at: Option<String>,
    #[serde(rename = "text")]
    ///Description used in UI
    pub text: Option<String>,
    #[serde(rename = "category")]
    ///Document's hierarchical category
    pub category: Option<String>,
    #[serde(rename = "signerInfo")]
    pub signer_info: Option<SignerInfoSchema>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<CustomMetadata>,
    #[serde(rename = "additionalEntities")]
    ///Additional entities ids associated with the document
    pub additional_entities: Option<DocumentAdditionalEntities>,
}
impl std::fmt::Display for DocumentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRequestSchema {
    #[serde(rename = "file")]
    ///the file to be attached to the application
    pub file: String,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "type")]
    ///Blend document type
    pub type_: Option<String>,
    #[serde(rename = "losType")]
    ///LOS document type (used with Blend document templating)
    pub los_type: Option<String>,
    #[serde(rename = "losTypeId")]
    ///LOS document id (used for external tracking)
    pub los_type_id: Option<String>,
    #[serde(rename = "partyIds")]
    ///UUIDs of the Parties associated with this document
    pub party_ids: Option<Vec<String>>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "status")]
    ///ENUM describing the Document Status
    pub status: Option<String>,
    #[serde(rename = "shareWithAllParties")]
    ///If set to true, uploaded document becomes accessible to borrowers; otherwise, only lender can view the document
    pub share_with_all_parties: Option<bool>,
    #[serde(rename = "customFields")]
    ///field name of the custom field
    pub custom_fields: Option<String>,
    #[serde(rename = "customMetadata")]
    ///field name of the custom metadata
    pub custom_metadata: Option<String>,
}
impl std::fmt::Display for DocumentRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentExportStatusSchema {
    #[serde(rename = "exportedAt")]
    ///UTC timestamp of export time
    pub exported_at: Option<String>,
    #[serde(rename = "exportedBy")]
    ///ID of the user who initiated the export call
    pub exported_by: Option<String>,
    #[serde(rename = "partnerId")]
    ///Role of the user who initiated the export call
    pub partner_id: Option<String>,
    #[serde(rename = "exportedTo")]
    ///The UUID or GUID of the current resource in the LOS AFTER initial export to the LOS. For loans this may match the loan reference number depending on the LOS and if the customer has selected to use only GUIDs instead of UUIDs in their LOS settings. For all other resources, it is the static identifier of this resource in the LOS.
    pub exported_to: Option<String>,
}
impl std::fmt::Display for DocumentExportStatusSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSchema {
    #[serde(rename = "id")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "referenceNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub reference_number: Option<String>,
    #[serde(rename = "losId")]
    ///The UUID or GUID of the current resource in the LOS AFTER initial export to the LOS. For loans this may match the loan reference number depending on the LOS and if the customer has selected to use only GUIDs instead of UUIDs in their LOS settings. For all other resources, it is the static identifier of this resource in the LOS.
    pub los_id: Option<String>,
    #[serde(rename = "crmId")]
    ///The unique identifier of the application in a lender's Customer Relationship Management (CRM) system (e.g. Salesforce, Velocify, etc.). This value should not change over time and should be used to connect the application's identity between Blend and the CRM.
    pub crm_id: Option<String>,
    #[serde(rename = "status")]
    ///An ENUM for the status of the application. This field is being deprecated but still controls lender UI and TRID
    pub status: Option<String>,
    #[serde(rename = "applicationStatus")]
    ///An ENUM for the status of the application. This field and Party.status replaces status.
    pub application_status: Option<String>,
    #[serde(rename = "parties")]
    ///An array of all the parties belonging to the application
    pub parties: Option<Vec<PartySchema>>,
    #[serde(rename = "property")]
    ///Object storing information about the property corresponding to a home lending application
    pub property: Option<PropertySchema>,
    #[serde(rename = "leadId")]
    ///Lead ID associated with this application in the system that referred it to Blend. Primarily used for LeadGen integration use cases.
    pub lead_id: Option<String>,
    #[serde(rename = "solutionSubType")]
    ///An ENUM for the type of solution to which the application corresponds
    pub solution_sub_type: Option<String>,
    #[serde(rename = "loanPurposeType")]
    ///An ENUM describing the purpose of the application
    pub loan_purpose_type: Option<String>,
    #[serde(rename = "communityId")]
    pub community_id: Option<String>,
    #[serde(rename = "communityName")]
    ///The community that the home resides in
    pub community_name: Option<String>,
    #[serde(rename = "referralBrokerageConsentGiven")]
    ///A boolean indicating whether the borrower has opted into using Blend Realty
    pub referral_brokerage_consent_given: Option<bool>,
    #[serde(rename = "appSource")]
    ///The tag corresponding to a marketing campaign. This identifier can be used to gauge which links are most effective within certain campaigns.
    pub app_source: Option<String>,
    #[serde(rename = "createdDate")]
    ///UTC timestamp of the application creation time
    pub created_date: Option<String>,
    #[serde(rename = "createdBy")]
    pub created_by: Option<CreatedByObject>,
    #[serde(rename = "estimatedClosingDate")]
    ///UTC timestamp of the estimated date to close on the subject property
    pub estimated_closing_date: Option<String>,
    #[serde(rename = "exportedDate")]
    ///UTC timestamp of the application exported time
    pub exported_date: Option<String>,
    #[serde(rename = "preApprovalLetterDatetime")]
    ///UTC timestamp of when the Preapproval Letter was generated
    pub pre_approval_letter_datetime: Option<String>,
    #[serde(rename = "applicationExperienceType")]
    ///ENUM describing the type of borrower experience for this application
    pub application_experience_type: Option<String>,
    #[serde(rename = "assignees")]
    pub assignees: Option<UserAssignmentResponseSchema>,
    #[serde(rename = "applicationTemplateId")]
    ///Application template applied to this application. Determines what kind and how much information the applicant will be required to enter to complete the application.
    pub application_template_id: Option<String>,
    #[serde(rename = "loanAmount")]
    ///The amount of money (dollars and cents) for which the applicant is applying
    pub loan_amount: Option<f64>,
    #[serde(rename = "purchasePrice")]
    ///The purchase price for the subject property of the loan
    pub purchase_price: Option<f64>,
    #[serde(rename = "applicationSource")]
    ///Identifier of where the application originally came from. Used to trace which system programmatically created an application in Blend when it wasn't initiated by a Lender or a Borrower via Blend's web application.
    pub application_source: Option<ApplicationSourceSchema>,
    #[serde(rename = "archivedStatus")]
    ///If set to true, will archive the referenced application
    pub archived_status: Option<bool>,
    #[serde(rename = "links")]
    pub links: Option<ApplicationLinks>,
    #[serde(rename = "interestRate")]
    ///The interest rate in percentage points
    pub interest_rate: Option<f64>,
    #[serde(rename = "mortgageType")]
    ///ENUM describing the type of the mortgage
    pub mortgage_type: Option<String>,
    #[serde(rename = "tridTriggeredDate")]
    ///UTC timestamp of when TRID was triggered on the application
    pub trid_triggered_date: Option<String>,
    #[serde(rename = "debtToIncomeRatio")]
    ///The combined debt to income ratio (DTI) of all borrowers on the loan
    pub debt_to_income_ratio: Option<f64>,
    #[serde(rename = "affordabilityRange")]
    pub affordability_range: Option<AffordabilityRangeSchema>,
}
impl std::fmt::Display for ApplicationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PropertySchema {
    #[serde(rename = "address")]
    pub address: Option<AddressSchema>,
    #[serde(rename = "type")]
    ///ENUM describing the type of property to which the application pertains
    pub type_: Option<String>,
    #[serde(rename = "usageType")]
    ///ENUM describing the property's intended usage category
    pub usage_type: Option<String>,
    #[serde(rename = "searchType")]
    ///ENUM describing the applicant's search stage
    pub search_type: Option<String>,
    #[serde(rename = "searchTimeline")]
    ///ENUM describing the applicant's timeline for searching for a property
    pub search_timeline: Option<String>,
    #[serde(rename = "value")]
    ///The property's value in dollars and cents
    pub value: Option<f64>,
    #[serde(rename = "otherMortgages")]
    ///Other mortgages on the subject property (only relevant for refinances)
    pub other_mortgages: Option<Vec<OtherMortgageSchema>>,
}
impl std::fmt::Display for PropertySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationSourceSchema {
    #[serde(rename = "type")]
    ///Type of system the application came from. If set to Other, it is recommended to also set a name.
    pub type_: String,
    #[serde(rename = "name")]
    ///Name of the system creating the application. For example, Salesforce or Empower.
    pub name: Option<String>,
}
impl std::fmt::Display for ApplicationSourceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationTemplateIdSchema(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationRequestSchema {
    #[serde(rename = "solutionSubType")]
    ///ENUM for the subtype of home loan application being created
    pub solution_sub_type: Option<String>,
    #[serde(rename = "applicationExperienceType")]
    ///ENUM describing the type of borrower experience for this application
    pub application_experience_type: Option<String>,
    #[serde(rename = "loanPurposeType")]
    ///ENUM describing the reason for this home loan application
    pub loan_purpose_type: Option<String>,
    #[serde(rename = "property")]
    ///Object storing information about the property corresponding to a home lending application
    pub property: Option<PropertySchema>,
    #[serde(rename = "loanAmount")]
    ///The amount of money (dollars and cents) for which the applicant is applying
    pub loan_amount: Option<f64>,
    #[serde(rename = "purchasePrice")]
    ///The purchase price for the subject property of the loan. This field is only supported for new URLA mortgage applications.
    pub purchase_price: Option<f64>,
    #[serde(rename = "communityId")]
    pub community_id: Option<String>,
    #[serde(rename = "party")]
    pub party: ApplicationRequestPartySchema,
    #[serde(rename = "leadId")]
    ///Lead ID associated with this application in the system that referred it to Blend. Primarily used for LeadGen integration use cases.
    pub lead_id: Option<String>,
    #[serde(rename = "crmId")]
    ///The unique identifier of the application in a lender's Customer Relationship Management (CRM) system (e.g. Salesforce, Velocify, etc.). This value should not change over time and should be used to connect the application's identity between Blend and the CRM.
    pub crm_id: Option<String>,
    #[serde(rename = "losId")]
    ///The UUID or GUID of the current resource in the LOS AFTER initial export to the LOS. For loans this may match the loan reference number depending on the LOS and if the customer has selected to use only GUIDs instead of UUIDs in their LOS settings. For all other resources, it is the static identifier of this resource in the LOS.
    pub los_id: Option<String>,
    #[serde(rename = "referenceNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub reference_number: Option<String>,
    #[serde(rename = "sendEmailInvite")]
    ///If set to true, sends an email invite to borrower with a link to start the application
    pub send_email_invite: Option<bool>,
    #[serde(rename = "applicationTemplateId")]
    ///Application template applied to this application. Determines what kind and how much information the applicant will be required to enter to complete the application.
    pub application_template_id: Option<String>,
    #[serde(rename = "applicationSource")]
    ///Identifier of where the application originally came from. Used to trace which system programmatically created an application in Blend when it wasn't initiated by a Lender or a Borrower via Blend's web application.
    pub application_source: Option<ApplicationSourceSchema>,
    #[serde(rename = "interestRate")]
    ///The interest rate in percentage points
    pub interest_rate: Option<f64>,
    #[serde(rename = "mortgageType")]
    ///ENUM describing the type of the mortgage
    pub mortgage_type: Option<String>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<CustomMetadata>,
    #[serde(rename = "branchIdOverride")]
    ///A specific branch ID, used for origination attribution, that when set takes precedence over the originating user's
    pub branch_id_override: Option<String>,
}
impl std::fmt::Display for ApplicationRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageSchema {
    #[serde(rename = "id")]
    ///The UUID of the Package in Blend's system. The static identifier that should be used to connect the package's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "losId")]
    ///The UUID or GUID of the current resource in the LOS AFTER initial export to the LOS. For loans this may match the loan reference number depending on the LOS and if the customer has selected to use only GUIDs instead of UUIDs in their LOS settings. For all other resources, it is the static identifier of this resource in the LOS.
    pub los_id: Option<String>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "status")]
    ///ENUM describing the status of the package
    pub status: String,
    #[serde(rename = "displayStatus")]
    pub display_status: Option<String>,
    #[serde(rename = "sentDate")]
    ///UTC Timestamp of package sent date
    pub sent_date: Option<String>,
    #[serde(rename = "recipients")]
    pub recipients: Option<Vec<serde_json::Value>>,
    #[serde(rename = "documents")]
    ///Array of document IDs associated with this Package
    pub documents: Option<Vec<String>>,
    #[serde(rename = "name")]
    ///Optional name field for package when type is 'DOCUMENT_PACKAGE' or 'OTHER_DISCLOSURE,' forbidden otherwise
    pub name: Option<String>,
}
impl std::fmt::Display for PackageSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TabSchema {
    #[serde(rename = "approveTabs")]
    pub approve_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "signHereTabs")]
    pub sign_here_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "listTabs")]
    pub list_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "dateSignedTabs")]
    pub date_signed_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "checkboxTabs")]
    pub checkbox_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "textTabs")]
    pub text_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "initialHereTabs")]
    pub initial_here_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "dateTabs")]
    pub date_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "noteTabs")]
    pub note_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "radioGroupTabs")]
    pub radio_group_tabs: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for TabSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LenderCreationRequestSchema {
    #[serde(rename = "name")]
    ///Lender Full Name
    pub name: String,
    #[serde(rename = "email")]
    ///Lender Email
    pub email: String,
    #[serde(rename = "permittedSolutionSubTypes")]
    ///Set of allowable solution types
    pub permitted_solution_sub_types: Vec<String>,
    #[serde(rename = "requireTwoFactorAuth")]
    ///MFA Required
    pub require_two_factor_auth: Option<bool>,
    #[serde(rename = "loginMethod")]
    ///Login Method
    pub login_method: String,
    #[serde(rename = "roleNames")]
    ///Roles for Lender
    pub role_names: Vec<String>,
    #[serde(rename = "phone")]
    pub phone: Option<serde_json::Value>,
    #[serde(rename = "nmlsId")]
    ///Nationwide Multistate Licensing System ID number
    pub nmls_id: Option<String>,
    #[serde(rename = "losUsername")]
    ///Lender's LOS Username
    pub los_username: Option<String>,
    #[serde(rename = "employeeId")]
    ///Lender's Employee Id
    pub employee_id: Option<String>,
    #[serde(rename = "branchId")]
    ///Lender's Branch Id
    pub branch_id: Option<String>,
    #[serde(rename = "licensedStates")]
    ///States Lender is licensed in
    pub licensed_states: Option<Vec<String>>,
}
impl std::fmt::Display for LenderCreationRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LenderUpdateRequestSchema {
    #[serde(rename = "id")]
    ///The UUID of the Lender User in Blend's system. The static identifier that should be used to connect the user's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "email")]
    ///Lender Email. This can only be updated for SSO users. Note that you must submit a Blend Support request to enable the "Allow SSO lender email updates through API" configuration to enable this field.
    pub email: Option<String>,
    #[serde(rename = "name")]
    ///Lender Full Name
    pub name: Option<String>,
    #[serde(rename = "permittedSolutionSubTypes")]
    ///Allowable solution type
    pub permitted_solution_sub_types: Option<String>,
    #[serde(rename = "requireTwoFactorAuth")]
    ///MFA Required
    pub require_two_factor_auth: Option<bool>,
    #[serde(rename = "loginMethod")]
    ///Login Method
    pub login_method: Option<String>,
    #[serde(rename = "roleNames")]
    ///Roles for Lender
    pub role_names: Option<Vec<String>>,
    #[serde(rename = "phone")]
    pub phone: Option<serde_json::Value>,
    #[serde(rename = "nmlsId")]
    ///Nationwide Multistate Licensing System ID number
    pub nmls_id: Option<String>,
    #[serde(rename = "losUsername")]
    ///Lender's LOS Username
    pub los_username: Option<String>,
    #[serde(rename = "employeeId")]
    ///Lender's Employee Id
    pub employee_id: Option<String>,
    #[serde(rename = "branchId")]
    ///Lender's Branch Id
    pub branch_id: Option<String>,
    #[serde(rename = "creditInternalAccountIdentifiers")]
    pub credit_internal_account_identifiers: Option<CreditInternalAccountIdentifiers>,
    #[serde(rename = "licensedStates")]
    ///States Lender is licensed in
    pub licensed_states: Option<Vec<String>>,
}
impl std::fmt::Display for LenderUpdateRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LenderRecordSchema {
    #[serde(rename = "name")]
    ///Lender Full Name
    pub name: Option<String>,
    #[serde(rename = "firstName")]
    ///Lender First Name
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    ///Lender Last Name
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    ///Lender Email
    pub email: Option<String>,
    #[serde(rename = "permittedSolutionSubTypes")]
    ///Loan Subtypes Permitted
    pub permitted_solution_sub_types: Option<Vec<String>>,
    #[serde(rename = "requireTwoFactorAuth")]
    ///MFA Required
    pub require_two_factor_auth: bool,
    #[serde(rename = "loginMethod")]
    ///Login Method
    pub login_method: Option<String>,
    #[serde(rename = "roleNames")]
    ///Roles for Lender
    pub role_names: Option<Vec<String>>,
    #[serde(rename = "phone")]
    pub phone: Option<serde_json::Value>,
    #[serde(rename = "nmlsId")]
    ///Nationwide Multistate Licensing System ID number
    pub nmls_id: Option<String>,
    #[serde(rename = "losUsername")]
    ///Lender's LOS Username
    pub los_username: Option<String>,
    #[serde(rename = "employeeId")]
    ///Lender's Employee Id
    pub employee_id: Option<String>,
    #[serde(rename = "branchId")]
    ///Lender's Branch Id
    pub branch_id: Option<String>,
    #[serde(rename = "id")]
    ///The UUID of the Lender User in Blend's system. The static identifier that should be used to connect the user's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "status")]
    ///ENUM describing the Lender User's status in Blend
    pub status: String,
    #[serde(rename = "creditInternalAccountIdentifiers")]
    ///List of all the active creditInternalAccountIdentifiers
    pub credit_internal_account_identifiers: Option<
        Vec<CreditInternalAccountIdentifiers>,
    >,
    #[serde(rename = "licensedStates")]
    ///States Lender is licensed in
    pub licensed_states: Option<Vec<String>>,
}
impl std::fmt::Display for LenderRecordSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountApplicationSchema {
    #[serde(rename = "_id")]
    ///Internal application ID
    pub id: String,
    #[serde(rename = "referenceId")]
    ///Customer reference ID for application.
    pub reference_id: Option<String>,
    #[serde(rename = "productReferenceId")]
    ///Customer reference ID for product bundle.
    pub product_reference_id: Option<String>,
    #[serde(rename = "applicants")]
    ///List of applicants applying to open an account. Maximum of two.
    pub applicants: Vec<ApplicantSchema>,
    #[serde(rename = "accounts")]
    ///List of accounts requested to be opened.
    pub accounts: Vec<AccountSchema>,
    #[serde(rename = "funding")]
    ///List of funding sources and amounts to be transferred to new accounts.
    pub funding: Option<Vec<FundingSchema>>,
    #[serde(rename = "membershipQualification")]
    pub membership_qualification: serde_json::Value,
    #[serde(rename = "dynamicAccountOptions")]
    ///answers to configured account setup questions
    pub dynamic_account_options: Option<DynamicAccountOptionsAnswerSchema>,
    #[serde(rename = "debitCardIndicator")]
    ///Indicates whether applicant is requesting a debit card.
    pub debit_card_indicator: Option<bool>,
    #[serde(rename = "debitCardDesign")]
    ///Selected debit card design. Values configured per customer.
    pub debit_card_design: Option<String>,
    #[serde(rename = "promoCode")]
    ///Promo code for the application.
    pub promo_code: Option<String>,
    #[serde(rename = "overdraftPrivilegeIndicator")]
    ///Indicates whether applicant is requesting overdraft privileges
    pub overdraft_privilege_indicator: Option<bool>,
    #[serde(rename = "beneficiariesConsentIndicator")]
    ///Indicates whether applicant consented to adding beneficiaries
    pub beneficiaries_consent_indicator: Option<bool>,
    #[serde(rename = "approvalStatus")]
    ///Status of the application approval process.
    pub approval_status: Option<String>,
    #[serde(rename = "rejectedApplicants")]
    ///List of applicants removed from the current application via customer's decisioning policy.
    pub rejected_applicants: Option<Vec<RejectedApplicantSchema>>,
    #[serde(rename = "metadata")]
    ///additional info about the application
    pub metadata: Option<serde_json::Value>,
    #[serde(rename = "associatedApplicationId")]
    ///Application Id of Associated Application from Single App Flow
    pub associated_application_id: Option<String>,
    #[serde(rename = "beneficiaries")]
    ///Beneficiaries collected at application-level.
    pub beneficiaries: Option<Vec<BeneficiarySchema>>,
    #[serde(rename = "createdAt")]
    ///The timestamp for when the application is created
    pub created_at: Option<String>,
    #[serde(rename = "submittedAt")]
    ///The timestamp for when the application is submitted
    pub submitted_at: Option<String>,
}
impl std::fmt::Display for AccountApplicationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicantSchema {
    #[serde(rename = "_id")]
    ///Internal applicant ID
    pub id: String,
    #[serde(rename = "type")]
    ///Applicant type, primary, secondary or minor
    pub type_: String,
    #[serde(rename = "email")]
    ///Applicant email address
    pub email: String,
    #[serde(rename = "name")]
    pub name: NameSchema,
    #[serde(rename = "dateOfBirth")]
    ///Applicant birth date, UNIX timestamp (milliseconds)
    pub date_of_birth: String,
    #[serde(rename = "socialSecurityNumber")]
    ///Applicant SSN
    pub social_security_number: String,
    #[serde(rename = "creditConsentIndicator")]
    ///Indicates whether applicant consented to credit policy for Identity Verification purposes.
    pub credit_consent_indicator: Option<bool>,
    #[serde(rename = "addresses")]
    ///List of applicant addresses, current or mailing
    pub addresses: Vec<AccountApplicationAddressSchema>,
    #[serde(rename = "phoneNumbers")]
    ///List of phone numbers associated with applicant
    pub phone_numbers: Vec<AccountApplicationPhoneNumberSchema>,
    #[serde(rename = "identityVerification")]
    ///Information regarding id verification for applicant
    pub identity_verification: Option<IdentityVerificationSchema>,
    #[serde(rename = "identityDocumentation")]
    ///ID Document
    pub identity_documentation: Option<IdentityDocumentationSchema>,
    #[serde(rename = "employmentStatus")]
    ///current listed employment status
    pub employment_status: Option<String>,
    #[serde(rename = "employers")]
    ///List of applicant's employers
    pub employers: Option<Vec<EmployerSchema>>,
    #[serde(rename = "employments")]
    ///List of applicant's employments
    pub employments: Option<Vec<EmploymentSchema>>,
    #[serde(rename = "coreBankingId")]
    ///Unique identifier for customer's core banking system.
    pub core_banking_id: Option<String>,
    #[serde(rename = "maritalStatus")]
    ///Applicant marital status
    pub marital_status: Option<String>,
    #[serde(rename = "citizenshipStatus")]
    ///Applicant citizenship status
    pub citizenship_status: Option<String>,
    #[serde(rename = "sex")]
    ///Applicant sex
    pub sex: Option<String>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<CustomMetadata>,
}
impl std::fmt::Display for ApplicantSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RejectedApplicantSchema {
    #[serde(rename = "_id")]
    ///Internal applicant ID
    pub id: String,
    #[serde(rename = "type")]
    ///Applicant type
    pub type_: String,
    #[serde(rename = "email")]
    ///Applicant email address
    pub email: String,
    #[serde(rename = "name")]
    pub name: NameSchema,
    #[serde(rename = "dateOfBirth")]
    ///Applicant birth date, UNIX timestamp (milliseconds)
    pub date_of_birth: String,
    #[serde(rename = "socialSecurityNumber")]
    ///Applicant SSN
    pub social_security_number: String,
    #[serde(rename = "creditConsentIndicator")]
    ///Indicates whether applicant consented to credit policy for Identity Verification purposes.
    pub credit_consent_indicator: Option<bool>,
    #[serde(rename = "addresses")]
    ///List of applicant addresses, current or mailing
    pub addresses: Vec<AccountApplicationAddressSchema>,
    #[serde(rename = "phoneNumbers")]
    ///List of phone numbers associated with applicant
    pub phone_numbers: Vec<AccountApplicationPhoneNumberSchema>,
    #[serde(rename = "identityVerificationStatus")]
    ///Information regarding id verification for applicant
    pub identity_verification_status: Option<IdentityVerificationSchema>,
    #[serde(rename = "identityDocumentation")]
    ///ID Document
    pub identity_documentation: Option<IdentityDocumentationSchema>,
    #[serde(rename = "employers")]
    ///List of applicant's employers
    pub employers: Option<Vec<EmployerSchema>>,
    #[serde(rename = "employments")]
    ///List of applicant's employments
    pub employments: Option<Vec<EmploymentSchema>>,
    #[serde(rename = "coreBankingId")]
    ///Unique identifier for customer's core banking system.
    pub core_banking_id: Option<String>,
    #[serde(rename = "maritalStatus")]
    ///Applicant marital status
    pub marital_status: Option<String>,
    #[serde(rename = "citizenshipStatus")]
    ///Applicant citizenship status
    pub citizenship_status: Option<String>,
    #[serde(rename = "sex")]
    ///Applicant sex
    pub sex: Option<String>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<CustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<CustomMetadata>,
}
impl std::fmt::Display for RejectedApplicantSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AoReviewerSchema {
    #[serde(rename = "_id")]
    ///Internal reviewer ID
    pub id: String,
    #[serde(rename = "fullName")]
    ///The full name of the reviewer
    pub full_name: Option<String>,
}
impl std::fmt::Display for AoReviewerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountApplicationAddressSchema {
    #[serde(rename = "_id")]
    ///Address ID
    pub id: String,
    #[serde(rename = "type")]
    ///Address type, current or mailing
    pub type_: String,
    #[serde(rename = "lineText")]
    ///First line of address
    pub line_text: String,
    #[serde(rename = "additionalLineText")]
    ///Second (optional) line of address
    pub additional_line_text: Option<String>,
    #[serde(rename = "cityName")]
    ///City name
    pub city_name: String,
    #[serde(rename = "countyName")]
    ///County name
    pub county_name: Option<String>,
    #[serde(rename = "stateCode")]
    ///2-digit state or territory code
    pub state_code: String,
    #[serde(rename = "zipCode")]
    ///5-digit zip code
    pub zip_code: String,
}
impl std::fmt::Display for AccountApplicationAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountApplicationPhoneNumberSchema {
    #[serde(rename = "type")]
    ///Type of phone number
    pub type_: String,
    #[serde(rename = "value")]
    ///Phone number
    pub value: f64,
}
impl std::fmt::Display for AccountApplicationPhoneNumberSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthorizedUserSchema {
    #[serde(rename = "_id")]
    ///Internal authorized user ID
    pub id: String,
    #[serde(rename = "email")]
    ///Authorized User email address
    pub email: String,
    #[serde(rename = "name")]
    pub name: NameSchema,
    #[serde(rename = "dateOfBirth")]
    ///Authorized User birth date, UNIX timestamp (milliseconds)
    pub date_of_birth: String,
    #[serde(rename = "socialSecurityNumber")]
    ///Authorized User SSN
    pub social_security_number: String,
    #[serde(rename = "addresses")]
    ///List of authorized User addresses, current or mailing
    pub addresses: Vec<AccountApplicationAddressSchema>,
    #[serde(rename = "phoneNumbers")]
    ///List of phone numbers associated with authorized User
    pub phone_numbers: Vec<AccountApplicationPhoneNumberSchema>,
    #[serde(rename = "identityVerification")]
    ///Information regarding id verification for applicant
    pub identity_verification: Option<IdentityVerificationSchema>,
}
impl std::fmt::Display for AuthorizedUserSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BeneficiarySchema {
    #[serde(rename = "_id")]
    ///Internal beneficiary ID
    pub id: String,
    #[serde(rename = "type")]
    ///Beneficiary type, primary or contingent
    pub type_: String,
    #[serde(rename = "subType")]
    ///Beneficiary sub-type, INDIVIDUAL, TRUST, CHARITY, CHURCH, or FOUNDATION
    pub sub_type: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<NameSchema>,
    #[serde(rename = "relation")]
    ///Beneficiary relationship
    pub relation: Option<String>,
    #[serde(rename = "allocation")]
    ///Percentage allocated to the given beneficiary
    pub allocation: Option<f64>,
    #[serde(rename = "address")]
    pub address: Option<AccountApplicationAddressSchema>,
    #[serde(rename = "email")]
    ///Beneficiary email address
    pub email: Option<String>,
    #[serde(rename = "dateOfBirth")]
    ///Beneficiary birth date, UNIX timestamp (milliseconds)
    pub date_of_birth: Option<String>,
    #[serde(rename = "socialSecurityNumber")]
    ///Beneficiary SSN
    pub social_security_number: Option<String>,
    #[serde(rename = "entityName")]
    ///Entity name of the given beneficiary (if the beneficiary is not a individual)
    pub entity_name: Option<String>,
    #[serde(rename = "taxpayerIdentificationNumber")]
    ///Taxpayer identification number of the entity (if the beneficiary is not a individual)
    pub taxpayer_identification_number: Option<String>,
    #[serde(rename = "phoneNumbers")]
    ///List of phone numbers associated with the given beneficiary
    pub phone_numbers: Option<Vec<AccountApplicationPhoneNumberSchema>>,
}
impl std::fmt::Display for BeneficiarySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditInternalAccountIdentifiers {
    #[serde(rename = "pullType")]
    ///Pull type
    pub pull_type: String,
    #[serde(rename = "creditPullScenario")]
    ///Credit Pull Scenario
    pub credit_pull_scenario: String,
    #[serde(rename = "creditProvider")]
    ///Credit Provider
    pub credit_provider: String,
    #[serde(rename = "creditAccountIdentifier")]
    ///Credit Account Identifier
    pub credit_account_identifier: String,
}
impl std::fmt::Display for CreditInternalAccountIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityDocumentationSchema {
    #[serde(rename = "type")]
    ///Type of ID document provided by applicant
    pub type_: String,
    #[serde(rename = "value")]
    ///ID document number
    pub value: String,
    #[serde(rename = "stateCode")]
    ///2-digit code of state that issued the ID document
    pub state_code: Option<String>,
    #[serde(rename = "country")]
    ///Country that issued the ID document
    pub country: Option<String>,
    #[serde(rename = "province")]
    ///Province that issued the ID document
    pub province: Option<String>,
    #[serde(rename = "tribe")]
    ///Tribe that issued the ID document
    pub tribe: Option<String>,
    #[serde(rename = "issueDate")]
    ///Date when ID document was issued, UNIX Timestamp (milliseconds)
    pub issue_date: f64,
    #[serde(rename = "expiryDate")]
    ///Date when ID document expires, UNIX Timestamp (milliseconds)
    pub expiry_date: f64,
}
impl std::fmt::Display for IdentityDocumentationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IdentityVerificationSchema {
    #[serde(rename = "_id")]
    ///Internal ID of identity verification process
    pub id: String,
    #[serde(rename = "statusType")]
    ///Result of identity verification process
    pub status_type: Option<String>,
    #[serde(rename = "provider")]
    ///Provider metadata object
    pub provider: Option<serde_json::Value>,
    #[serde(rename = "type")]
    ///Type of identity verification
    pub type_: Option<String>,
}
impl std::fmt::Display for IdentityVerificationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingAccountSchema {
    #[serde(rename = "_id")]
    ///Internal account ID
    pub id: String,
    #[serde(rename = "referenceId")]
    ///External ID or code to reference this account. e.g. share code.
    pub reference_id: Option<String>,
    #[serde(rename = "type")]
    ///Account type
    pub type_: Option<String>,
    #[serde(rename = "description")]
    ///Additional text describing account
    pub description: Option<String>,
    #[serde(rename = "accountNumber")]
    ///Account number
    pub account_number: Option<f64>,
    #[serde(rename = "routingNumber")]
    ///Routing number
    pub routing_number: Option<String>,
}
impl std::fmt::Display for FundingAccountSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AccountSchema {
    #[serde(rename = "_id")]
    ///Internal account ID
    pub id: String,
    #[serde(rename = "referenceId")]
    ///External ID or code to reference this account. e.g. share code.
    pub reference_id: Option<String>,
    #[serde(rename = "type")]
    ///Account type
    pub type_: Option<String>,
    #[serde(rename = "description")]
    ///Additional text describing account
    pub description: Option<String>,
    #[serde(rename = "accountNumber")]
    ///Account number
    pub account_number: Option<f64>,
    #[serde(rename = "routingNumber")]
    ///Routing number
    pub routing_number: Option<String>,
    #[serde(rename = "isPrimary")]
    ///Indicator for primary account on application
    pub is_primary: Option<bool>,
    #[serde(rename = "authorizedUsers")]
    ///Authorized Users collected for the given account.
    pub authorized_users: Option<Vec<AuthorizedUserSchema>>,
    #[serde(rename = "beneficiaries")]
    ///Beneficiaries collected for the given account.
    pub beneficiaries: Option<Vec<BeneficiarySchema>>,
}
impl std::fmt::Display for AccountSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DynamicAccountOptionsAnswerSchema {}
impl std::fmt::Display for DynamicAccountOptionsAnswerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct MembershipQualificationAnswerSchema {}
impl std::fmt::Display for MembershipQualificationAnswerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FundingSchema {
    #[serde(rename = "sourceAccount")]
    pub source_account: FundingAccountSchema,
    #[serde(rename = "destinationAccount")]
    pub destination_account: FundingAccountSchema,
    #[serde(rename = "amount")]
    ///Amount being transferred
    pub amount: f64,
}
impl std::fmt::Display for FundingSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployerSchema {
    #[serde(rename = "_id")]
    ///Internal employer ID
    pub id: String,
    #[serde(rename = "type")]
    ///Type of employment.
    pub type_: String,
    #[serde(rename = "referenceId")]
    ///External employer reference ID. e.g. select employee group number.
    pub reference_id: Option<String>,
    #[serde(rename = "name")]
    ///Employer name
    pub name: String,
    #[serde(rename = "address")]
    pub address: Option<AccountApplicationAddressSchema>,
}
impl std::fmt::Display for EmployerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmploymentSchema {
    #[serde(rename = "type")]
    ///Type of employment.
    pub type_: String,
    #[serde(rename = "jobTitle")]
    ///Job Title
    pub job_title: Option<String>,
    #[serde(rename = "annualIncome")]
    ///Annual income range for given employment
    pub annual_income: Option<String>,
    #[serde(rename = "hireDate")]
    ///Date of hire for given employment
    pub hire_date: Option<String>,
}
impl std::fmt::Display for EmploymentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchApplicationSchema {
    #[serde(rename = "referenceId")]
    ///Optional reference ID for matching records with Blend's application object
    pub reference_id: Option<String>,
    #[serde(rename = "approvalStatus")]
    ///Decision regarding opening the account
    pub approval_status: Option<String>,
    #[serde(rename = "accounts")]
    ///List of accounts with their account/routing numbers
    pub accounts: Option<Vec<PatchAccountSchema>>,
    #[serde(rename = "applicants")]
    ///List of applicants with their member numbers
    pub applicants: Option<Vec<PatchApplicantSchema>>,
    #[serde(rename = "integrationEvent")]
    pub integration_event: Option<IntegrationEventSchema>,
}
impl std::fmt::Display for PatchApplicationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchAccountSchema {
    #[serde(rename = "_id")]
    ///Internal account ID
    pub id: String,
    #[serde(rename = "accountNumber")]
    ///Account number of booked account
    pub account_number: Option<String>,
    #[serde(rename = "routingNumber")]
    ///Routing number of booked account
    pub routing_number: Option<String>,
    #[serde(rename = "accessLimits")]
    ///access limits for the given account
    pub access_limits: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for PatchAccountSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchApplicantSchema {
    #[serde(rename = "_id")]
    ///Internal applicant ID
    pub id: String,
    #[serde(rename = "memberNumber")]
    ///Member number of applicant
    pub member_number: Option<String>,
    #[serde(rename = "coreBankingId")]
    ///Applicant identifier in customer's core banking system
    pub core_banking_id: Option<String>,
    #[serde(rename = "customFields")]
    ///Custom fields on PATCH or POST requests for supported resources
    pub custom_fields: Option<RequestCustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on PATCH or POST requests for supported resources
    pub custom_metadata: Option<RequestCustomMetadata>,
}
impl std::fmt::Display for PatchApplicantSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IntegrationEventSchema {
    #[serde(rename = "type")]
    ///The type of integration event which is being completed
    pub type_: String,
    #[serde(rename = "status")]
    ///The final status of the integration event which is being completed
    pub status: String,
    #[serde(rename = "failureCode")]
    ///Corresponding reason for integration event failure
    pub failure_code: Option<String>,
    #[serde(rename = "failureMessage")]
    ///Message explaining why the integration event failed (It's only displayed when a failureCode is not provided)
    pub failure_message: Option<String>,
}
impl std::fmt::Display for IntegrationEventSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageBorrowerMetadataSchema {
    #[serde(rename = "id")]
    ///The UUID of the Party in Blend's system. The static identifier that should be used to connect the party's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "tabs")]
    pub tabs: Option<TabSchema>,
    #[serde(rename = "docDeliveryType")]
    pub doc_delivery_type: Option<String>,
}
impl std::fmt::Display for PackageBorrowerMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageThirdPartyRecipientMetadataSchema {
    #[serde(rename = "id")]
    ///The UUID of the Party in Blend's system. The static identifier that should be used to connect the Party's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "tabs")]
    pub tabs: Option<TabSchema>,
    #[serde(rename = "docDeliveryType")]
    pub doc_delivery_type: Option<String>,
}
impl std::fmt::Display for PackageThirdPartyRecipientMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageLenderMetadataSchema {
    #[serde(rename = "tabs")]
    pub tabs: Option<TabSchema>,
    #[serde(rename = "docDeliveryType")]
    pub doc_delivery_type: Option<String>,
}
impl std::fmt::Display for PackageLenderMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LosMilestoneErrorSchema {
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: Option<String>,
    #[serde(rename = "code")]
    pub code: Option<f64>,
    #[serde(rename = "reason")]
    pub reason: Option<String>,
}
impl std::fmt::Display for LosMilestoneErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportStatusSchema {
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "status")]
    ///ENUM describing the status of this application's export to an external system
    pub status: String,
    #[serde(rename = "reason")]
    ///Further explanation for the assigned status
    pub reason: Option<String>,
    #[serde(rename = "lastUpdated")]
    ///UTC Timestamp of last update to status
    pub last_updated: Option<String>,
}
impl std::fmt::Display for ExportStatusSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostExportStatusSchema {
    #[serde(rename = "status")]
    ///ENUM describing the status of this application's export to an external system
    pub status: String,
    #[serde(rename = "reason")]
    ///Further explanation for the assigned status
    pub reason: Option<String>,
}
impl std::fmt::Display for PostExportStatusSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NotAuthorizedErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for NotAuthorizedErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalServerErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for InternalServerErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BulkRequestErrorSchema {
    #[serde(rename = "id")]
    ///The UUID of the referenced resource in Blend's system. The static identifier that should be used to connect the resource's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "code")]
    pub code: f64,
    #[serde(rename = "reason")]
    pub reason: String,
}
impl std::fmt::Display for BulkRequestErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ApplicationNotFoundErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for ApplicationNotFoundErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConflictErrorSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidRequestErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for InvalidRequestErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct InvalidUpdateErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for InvalidUpdateErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AuthenticationRequiredErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for AuthenticationRequiredErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RateLimitedErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for RateLimitedErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingLoanSchema {
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "losPrimaryKey")]
    ///The unique identifier of the loan in a lender's Loan Origination System, or LOS (e.g. Empower, Encompass, etc.). This value should not change over time and should be used to connect the loan's identity between Blend and the LOS.
    pub los_primary_key: Option<String>,
    #[serde(rename = "crmPrimaryKey")]
    ///The unique identifier of the application in a lender's Customer Relationship Management (CRM) system (e.g. Salesforce, Velocify, etc.). This value should not change over time and should be used to connect the application's identity between Blend and the CRM.
    pub crm_primary_key: Option<String>,
    #[serde(rename = "loanNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub loan_number: Option<String>,
    #[serde(rename = "creatorUserId")]
    ///The user ID for the user that originated the loan.
    pub creator_user_id: Option<String>,
    #[serde(rename = "primaryAssigneeUserId")]
    ///The user id of the primary assignee LO on the loan.
    pub primary_assignee_user_id: Option<String>,
    #[serde(rename = "primaryAssigneeName")]
    ///The full name of the primary assignee
    pub primary_assignee_name: Option<String>,
    #[serde(rename = "primaryAssigneeEmail")]
    ///The email address of the primary assignee
    pub primary_assignee_email: Option<String>,
    #[serde(rename = "allCurrentAssignees")]
    ///A JSON array containing the user ID, name, and email of all current assignees on the loan
    pub all_current_assignees: Option<String>,
    #[serde(rename = "loanType")]
    ///The subtype of the loan application.
    pub loan_type: Option<String>,
    #[serde(rename = "loanPurposeType")]
    ///The purpose of the loan application.
    pub loan_purpose_type: Option<String>,
    #[serde(rename = "appSourceName")]
    ///Canonical name of where this application came from, such as the name of the LOS or CRM
    pub app_source_name: Option<String>,
    #[serde(rename = "appLeadSource")]
    ///Specifies how an app was created.
    pub app_lead_source: Option<String>,
    #[serde(rename = "appProgram")]
    ///The loan program of the application, possible values include - STATE_AGENCY, VA, FHA, CONVENTIONAL, LOCAL_AGENCY, USARURAL_DEVELOPMENT, PUBLIC_AND_INDIAN_HOUSING, OTHER
    pub app_program: Option<String>,
    #[serde(rename = "appMarketingTag")]
    ///The tag corresponding to a marketing campaign. This identifier can be used to gauge which links are most effective within certain campaigns.
    pub app_marketing_tag: Option<String>,
    #[serde(rename = "assetState")]
    ///The registered address state on the asset
    pub asset_state: Option<String>,
    #[serde(rename = "assetPurchasePrice")]
    ///The purchase price of the asset
    pub asset_purchase_price: Option<f64>,
    #[serde(rename = "loanApplicationStatus")]
    ///The current status of the loan application
    pub loan_application_status: Option<String>,
    #[serde(rename = "los_milestone")]
    ///An optional enumerated field sent by lenders who have shared this data via the home-lending/applications/los-milestones endpoint. Possible values include SUBMITTED. UNDERWRITING, CONDITIONAL_APPROVAL, APPROVED, CLOSING, CLOSED
    pub los_milestone: Option<String>,
    #[serde(rename = "losMilestoneLastUpdatedDatetime")]
    ///The UTC timestamp for when an LOS milestone was last posted to Blend via the home-lending/applications/los-milestones
    pub los_milestone_last_updated_datetime: Option<String>,
    #[serde(rename = "firstCreditPulledDatetime")]
    ///The first UTC timestamp of when the borrower’s credit was pulled through Blend
    pub first_credit_pulled_datetime: Option<String>,
    #[serde(rename = "primaryBorrowerSsnFlag")]
    ///A flag denoting if the borrower has provided their SSN
    pub primary_borrower_ssn_flag: Option<bool>,
    #[serde(rename = "primaryBorrowerCreditConsentDatetime")]
    ///The date that the borrower provided credit consent
    pub primary_borrower_credit_consent_datetime: Option<String>,
    #[serde(rename = "primaryBorrowerEconsentFlag")]
    ///A flag denoting that the borrower has provided e-consent
    pub primary_borrower_econsent_flag: Option<bool>,
    #[serde(rename = "coborrowerSsnFlag")]
    ///A flag denoting if the coborrower has provided their SSN
    pub coborrower_ssn_flag: Option<bool>,
    #[serde(rename = "coborrowerCreditConsentDatetime")]
    ///The date that the borrower provided credit consent
    pub coborrower_credit_consent_datetime: Option<String>,
    #[serde(rename = "coborrowerEconsentFlag")]
    ///A flag denoting that the co-borrower has provided e-consent
    pub coborrower_econsent_flag: Option<bool>,
    #[serde(rename = "followUpOnlyFlag")]
    ///A flag that indicates that the application was a follow-up only loan - the lender used Blend to issue follow ups to the borrower
    pub follow_up_only_flag: Option<bool>,
    #[serde(rename = "connectivityFlag")]
    ///A flag that indicates that the borrower successfully generated an asset statement
    pub connectivity_flag: Option<bool>,
    #[serde(rename = "blendIncomeSuccessFlag")]
    ///A flag that indicates whether the loan had at least one Blend Income Report generated (i.e. a borrower's income was successfully verified)
    pub blend_income_success_flag: Option<bool>,
    #[serde(rename = "twnSuccessFlag")]
    ///A flag that indicates that a successful match was found on the borrower using The Work Number
    pub twn_success_flag: Option<bool>,
    #[serde(rename = "hasRealtorFlag")]
    ///A flag that indicates the borrower selected they are working with a realtor on their application
    pub has_realtor_flag: Option<bool>,
    #[serde(rename = "unassignedOnSubmitFlag")]
    ///For Mortgage/Heloc/Heloan only - A flag that indicates the loan was unassigned upon submit. This field will not change if a loan is assigned. 'TRUE' = the loan was unassigned at submission; 'FALSE' = the loan was assigned at submission; NULL = the loan may not have been submitted yet, or is is a non home lending product
    pub unassigned_on_submit_flag: Option<bool>,
    #[serde(rename = "signupUrl")]
    ///For Mortgage/Heloc/Heloan only - The sign-up URL the borrower used to apply. Can be used with the unassignedOnsubmitFlag field to diagnose faulty referral links
    pub signup_url: Option<String>,
    #[serde(rename = "preApprovalLetterDatetime")]
    ///The timestamp for when a Preapproval Letter was generated in Blend (UTC)
    pub pre_approval_letter_datetime: Option<String>,
    #[serde(rename = "createdDatetime")]
    ///The creation time of the loan (in UTC)
    pub created_datetime: Option<String>,
    #[serde(rename = "daysSinceLastTouch")]
    ///Number of days since the loan was last touched (from when the report was generated)
    pub days_since_last_touch: Option<f64>,
    #[serde(rename = "borrowerStartedDatetime")]
    ///The timestamp when the borrower(s) started the loan application
    pub borrower_started_datetime: Option<String>,
    #[serde(rename = "borrowerSubmittedDatetime")]
    ///The timestamp when borrower submitted the application
    pub borrower_submitted_datetime: Option<String>,
    #[serde(rename = "takeOverDatetime")]
    ///The timestamp when the loan was first taken over by the LO (in UTC)
    pub take_over_datetime: Option<String>,
    #[serde(rename = "estExportedDatetime")]
    ///The timestamp when the application was exported. Field is appended with estimated, as this field is not meant to be used for compliance and reconciliation purposes
    pub est_exported_datetime: Option<String>,
    #[serde(rename = "estLastUpdatedDatetime")]
    ///The timestamp the loan was last updated by the system. Can be used to filter on to find loans that have been updated. Field is appended with estimated, as this field is not meant to be used for compliance and reconciliation purposes and specific actions may not be considered as a "last updated"
    pub est_last_updated_datetime: Option<String>,
    #[serde(rename = "estLastTouchDatetime")]
    ///The timestamp the loan was last touched (by a borrower or lender user). Field is appended with estimated, as this field is not meant to be used for compliance and reconciliation purposes and specific actions may not be considered as a "last touched"
    pub est_last_touch_datetime: Option<String>,
    #[serde(rename = "lastTouchedByUserId")]
    ///The ID of the user to last touch the loan (can be borrower or lender user)
    pub last_touched_by_user_id: Option<String>,
    #[serde(rename = "estCompletedDatetime")]
    ///The application completion time of the loan (in UTC)
    pub est_completed_datetime: Option<String>,
    #[serde(rename = "estArchivedDatetime")]
    ///The timestamp when the application was archived. Field is appended with estimated, as this field is not meant to be used for compliance and reconciliation purposes
    pub est_archived_datetime: Option<String>,
    #[serde(rename = "estDeletedDatetime")]
    ///The timestamp when the application was deleted. Field is appended with estimated, as this field is not meant to be used for compliance and reconciliation purposes
    pub est_deleted_datetime: Option<String>,
    #[serde(rename = "estPurgedDatetime")]
    ///The timestamp when the application was purged. Field is appended with estimated, as this field is not meant to be used for compliance and reconciliation purposes ("estimated" in this case does not predict when a loan will be purged. This field is only for loans that have already been purged)
    pub est_purged_datetime: Option<String>,
    #[serde(rename = "appStartToSubmitMinutes")]
    ///For a submitted loan, the time between app start and app submit
    pub app_start_to_submit_minutes: Option<f64>,
    #[serde(rename = "asOfDate")]
    ///The date for when the report was generated
    pub as_of_date: Option<String>,
}
impl std::fmt::Display for ReportingLoanSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingDocumentSchema {
    #[serde(rename = "documentId")]
    ///The unique ID for the document upload
    pub document_id: String,
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "uploadedByUserId")]
    ///The User ID for the user who uploaded the document (can be 'system', or either a borrower or lender user ID)
    pub uploaded_by_user_id: Option<String>,
    #[serde(rename = "uploadedByUserType")]
    ///The type of user that uploaded the document. Eg "borrower", "lender", "system", "internal_api", "external_api". If the source is system or from the API, those are generally documents uploaded by Blend, or they can also be from internal/third party integrations that use the APIs to upload documents
    pub uploaded_by_user_type: Option<String>,
    #[serde(rename = "type")]
    ///Blend document type
    pub type_: String,
    #[serde(rename = "source")]
    ///The source of where the document came from. Can be 'connectivity' for documents pulled through Blend's connectivity, or 'user' for all other documents
    pub source: String,
    #[serde(rename = "createdDatetime")]
    ///The UTC timestamp for when the document was uploaded
    pub created_datetime: String,
    #[serde(rename = "loanType")]
    ///The specific product (Mortgage, HELOAN, HELOC, Auto, etc)
    pub loan_type: Option<String>,
    #[serde(rename = "asOfDate")]
    ///The date for when the report was generated
    pub as_of_date: String,
}
impl std::fmt::Display for ReportingDocumentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingBorrowerSchema {
    #[serde(rename = "userId")]
    ///User id
    pub user_id: String,
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "loanNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub loan_number: Option<String>,
    #[serde(rename = "partyId")]
    ///The UUID of the party in Blend's system. Equivalent to borrowerId
    pub party_id: String,
    #[serde(rename = "role")]
    ///The role of the user
    pub role: Option<String>,
    #[serde(rename = "loginMethod")]
    ///The method the user used to login
    pub login_method: Option<String>,
    #[serde(rename = "activationDatetime")]
    ///The UTC timestamp of when the borrower activated their Blend user
    pub activation_datetime: Option<String>,
    #[serde(rename = "invitedDatetime")]
    ///The UTC timestamp of when the borrower was invited to Blend
    pub invited_datetime: Option<String>,
    #[serde(rename = "creditPulledDatetime")]
    ///The latest UTC timestamp of when the borrower’s credit was pulled through Blend
    pub credit_pulled_datetime: Option<String>,
    #[serde(rename = "assetsPulledDatetime")]
    ///The latest UTC timestamp of when the borrower connected their assets through Blend
    pub assets_pulled_datetime: Option<String>,
    #[serde(rename = "incomeVerifiedDatetime")]
    ///The most recent timestamp (in UTC) that a Blend Income Report was generated for that borrower (i.e. the borrower's income was successfully verified)
    pub income_verified_datetime: Option<String>,
    #[serde(rename = "workflowMilestone")]
    ///The latest completed borrower milestone within Blend
    pub workflow_milestone: Option<String>,
    #[serde(rename = "userAgent")]
    ///The most recent borrower user agent. Can be used to derive the last device type used.
    pub user_agent: Option<String>,
    #[serde(rename = "lastUpdatedDatetime")]
    ///The timestamp (in UTC) of the latest action occurring for the borrower out of fields available in this report. Please note, this timestamp only updates if any of the above fields are updated - it does not apply to actions outside this report, so this field does not necessarily correspond to the last time the borrower took action on the loan. If all other timestamp fields are null, this timestamp defaults to the time the borrower user was created (and the borrower may not yet have been invited by the loan officer)
    pub last_updated_datetime: Option<String>,
}
impl std::fmt::Display for ReportingBorrowerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingLenderSchema {
    #[serde(rename = "userId")]
    ///Unique User ID associated with the Lender User
    pub user_id: String,
    #[serde(rename = "nmlsId")]
    ///The NMLS ID of the lender
    pub nmls_id: Option<String>,
    #[serde(rename = "branchId")]
    ///The Branch ID entered into the lender user profile (optional field maintained manually by the client)
    pub branch_id: Option<String>,
    #[serde(rename = "employeeId")]
    ///The lender user's employee ID (optional field maintained manually by the client)
    pub employee_id: Option<String>,
    #[serde(rename = "fullName")]
    ///The full name of the lender
    pub full_name: String,
    #[serde(rename = "email")]
    ///The email address of the lender
    pub email: String,
    #[serde(rename = "userRoles")]
    ///A comma separated list of user roles attributed to the lender
    pub user_roles: Option<String>,
    #[serde(rename = "losUsername")]
    ///The lender user's LOS Username in Blend
    pub los_username: Option<String>,
    #[serde(rename = "activationDatetime")]
    ///The UTC timestamp for when the lender user accepted Terms of Service
    pub activation_datetime: Option<String>,
    #[serde(rename = "invitedDatetime")]
    ///The invitation time of the lender (in UTC)
    pub invited_datetime: Option<String>,
    #[serde(rename = "lastLoginDatetime")]
    ///The UTC timestamp for the last time the account was opened/used
    pub last_login_datetime: Option<String>,
    #[serde(rename = "totalCreatedApplications")]
    ///The total number of created application applications assigned to the lender user in the last 90 days
    pub total_created_applications: Option<f64>,
    #[serde(rename = "borrowerSubmits")]
    ///The total number of loans submitted by the borrower with this lender as the primary assignee
    pub borrower_submits: Option<f64>,
    #[serde(rename = "percentLoansSubmitted")]
    ///The percent of loans submitted out of the total number of created applications assigned to the lender user
    pub percent_loans_submitted: Option<f64>,
    #[serde(rename = "percentSubmitsWithGas")]
    ///The percent of submitted loans with a Generated Asset Statement on the previous date
    pub percent_submits_with_gas: Option<f64>,
    #[serde(rename = "lastBorrowerSubmitDatetime")]
    ///The UTC timestamp for the last time the account was opened/used
    pub last_borrower_submit_datetime: Option<String>,
    #[serde(rename = "averageNpsScore")]
    ///The average NPS score of all submitted applications assigned to the lender user
    pub average_nps_score: Option<f64>,
    #[serde(rename = "calculatedNpsScore")]
    ///The calculated NPS score of all submitted applications assigned to the lender user
    pub calculated_nps_score: Option<f64>,
    #[serde(rename = "percentLoansUtilizingFollowups")]
    ///The percent of the lender user's loans utilizing follow-ups within the date filters provided
    pub percent_loans_utilizing_followups: Option<f64>,
    #[serde(rename = "followupsManuallyRequested")]
    ///The number of manually requested follow-ups by the lender user within the date filters provided
    pub followups_manually_requested: Option<f64>,
    #[serde(rename = "followupsBorrowerCompleted")]
    ///The number of follow-ups requested by this lender within the date filters provided that were completed by borrowers
    pub followups_borrower_completed: Option<f64>,
    #[serde(rename = "followupsPending")]
    ///The number of follow-ups requested within the date filters that have not been completed by the borrower
    pub followups_pending: Option<f64>,
    #[serde(rename = "followupsAccepted")]
    ///The number of follow-ups requested within the date filters that were accepted by this lender
    pub followups_accepted: Option<f64>,
    #[serde(rename = "followupsRejected")]
    ///The number of follow-ups requested within the date filters that were rejected by this lender
    pub followups_rejected: Option<f64>,
    #[serde(rename = "asOfDate")]
    ///The date that the report was generated
    pub as_of_date: Option<String>,
}
impl std::fmt::Display for ReportingLenderSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingActivitySchema {
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "loanNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub loan_number: Option<String>,
    #[serde(rename = "activityId")]
    ///Activity id
    pub activity_id: String,
    #[serde(rename = "userId")]
    ///User id
    pub user_id: String,
    #[serde(rename = "type")]
    ///Activity type
    pub type_: Option<String>,
    #[serde(rename = "startedDatetime")]
    ///Start time of the activity (in UTC)
    pub started_datetime: Option<String>,
    #[serde(rename = "completedDatetime")]
    ///Completion time of the activity (in UTC)
    pub completed_datetime: Option<String>,
}
impl std::fmt::Display for ReportingActivitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingNpsSchema {
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "loanNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub loan_number: Option<String>,
    #[serde(rename = "datetime")]
    ///submit time of NPS
    pub datetime: Option<String>,
    #[serde(rename = "comment")]
    ///NPS Comment submitted by borrower
    pub comment: Option<String>,
    #[serde(rename = "score")]
    ///NPS Score (0-10)
    pub score: Option<i64>,
}
impl std::fmt::Display for ReportingNpsSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingDataVerificationUsageSchema {
    #[serde(rename = "week")]
    ///The start date of the week
    pub week: String,
    #[serde(rename = "numberLoansSubmitted")]
    ///The total number of borrower submitted loans for the week
    pub number_loans_submitted: f64,
    #[serde(rename = "numberGasGenerated")]
    ///The total number of Generated Asset Statements (GAS) on submitted loans for that week. This only includes instances where a GAS was created from source data in a borrower’s bank account that is eligible for GSE rep and warrant programs (D1C/AIM). GAS does not include successful asset connectivity in cases that Blend is unable to access high enough quality source data for the GSE programs, but will pull a PDF instead and the borrower will not be prompted to upload bank statements (Assets Statement Fallback)
    pub number_gas_generated: f64,
    #[serde(rename = "numberLoansWithGas")]
    ///The total number of borrower submitted loans that week that have at least one Generated Asset Statement - this number does not include Assets Statement (PDF) Fallbacks; see description above
    pub number_loans_with_gas: f64,
    #[serde(rename = "percentSubmitsWithGas")]
    ///The percent of borrower submitted loans that week that have at least one Generated Asset Statement - this percentage does not include Assets Statement (PDF) Fallbacks; see description above
    pub percent_submits_with_gas: f64,
    #[serde(rename = "numberAssetsStatementFallback")]
    ///The total number of Assets Statement (PDF) Fallbacks created on submitted loans for that week. An Assets Statement Fallback is generated in cases where Blend is unable to access high enough quality source data for the GSE programs to generate a GAS, but will pull a PDF instead and the borrower will not be prompted to upload bank statements; this number does not include Generated Asset Statements (GAS) that are eligible for GSE rep and warrant programs (D1C/AIM)
    pub number_assets_statement_fallback: Option<f64>,
    #[serde(rename = "numberLoansWithAssetsStatementFallback")]
    ///The total number of borrower submitted loans that week that have at least one Asset Statement (PDF) Fallback - this number does not include Generated Asset Statements; see description above
    pub number_loans_with_assets_statement_fallback: Option<f64>,
    #[serde(rename = "percentSubmitsWithAssetsStatementFallback")]
    ///The percent of borrower submitted loans that week that have at least one Asset Statement (PDF) Fallback - this number does not include Generated Asset Statements; see description above
    pub percent_submits_with_assets_statement_fallback: Option<f64>,
    #[serde(rename = "numberAssetVerification")]
    ///The total number of Asset Verifications for borrower submitted loans that week, including either Generated Asset Statements (GAS) or Asset Statement Fallbacks (PDF Fallbacks); see descriptions above for details
    pub number_asset_verification: Option<f64>,
    #[serde(rename = "numberLoansWithAssetVerification")]
    ///The total number of borrower submitted loans that have at least one Asset Verification, including either Generated Asset Statements (GAS) or Asset Statement Fallbacks (PDF Fallbacks); see descriptions above for details
    pub number_loans_with_asset_verification: Option<f64>,
    #[serde(rename = "percentSubmitsWithAssetVerification")]
    ///The percent of borrower submitted loans that have at least one Asset Verification, including either Generated Asset Statements (GAS) or Asset Statement Fallbacks (PDF Fallbacks); see descriptions above for details
    pub percent_submits_with_asset_verification: Option<f64>,
    #[serde(rename = "numberBlendIncomeVerifications")]
    ///The total number of Blend Income Reports generated on borrower submitted loans in that week
    pub number_blend_income_verifications: Option<f64>,
    #[serde(rename = "numberLoansWithBlendIncomeVerification")]
    ///The total number of borrower submitted loans for that week with at least one Blend Income Verification
    pub number_loans_with_blend_income_verification: Option<f64>,
    #[serde(rename = "percentSubmitsWithBlendIncomeVerification")]
    ///The percent of borrower submitted loans for that week with at least one Blend Income Verification
    pub percent_submits_with_blend_income_verification: Option<f64>,
    #[serde(rename = "numberTwnInstantAttempt")]
    ///The total number of TWN instant attempts for that week
    pub number_twn_instant_attempt: f64,
    #[serde(rename = "numberTwnInstantSuccess")]
    ///The total number of TWN instant successes for that week
    pub number_twn_instant_success: f64,
    #[serde(rename = "asOfDate")]
    ///The effective date of the report
    pub as_of_date: String,
    #[serde(rename = "deployment")]
    ///The tenant making the API call
    pub deployment: Option<String>,
}
impl std::fmt::Display for ReportingDataVerificationUsageSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingFollowupsSchema {
    #[serde(rename = "followUpId")]
    ///The UUID of the followup in Blend's system. The static identifier that should be used to access followups across Blend and external integrations.
    pub follow_up_id: String,
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "loanNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub loan_number: Option<String>,
    #[serde(rename = "loanType")]
    ///The specific product (Mortgage, HELOAN, HELOC, Auto, etc)
    pub loan_type: Option<String>,
    #[serde(rename = "requestType")]
    ///The type of request that created the followup. Can be AUTOMATED, MANUAL, or SUGGESTED
    pub request_type: Option<String>,
    #[serde(rename = "followUpType")]
    ///The followup requested
    pub follow_up_type: Option<String>,
    #[serde(rename = "followUpText")]
    ///Optional text containing additional details about the follow-up. Can be set by Blend, or custom text
    pub follow_up_text: Option<String>,
    #[serde(rename = "followUpCreatedDatetime")]
    ///The timestamp when the followup was created in Blend
    pub follow_up_created_datetime: Option<String>,
    #[serde(rename = "currentStatus")]
    ///The current status of the followup. Can be Suggested, Declined, Requested, Pending Review, Completed, Completed Externally, Rejected, Cancelled
    pub current_status: Option<String>,
    #[serde(rename = "requestedDatetime")]
    ///The UTC timestamp when the followup was requested
    pub requested_datetime: Option<String>,
    #[serde(rename = "requestedUserId")]
    ///The user_id that requested the followup. Can be 'system' or a Lender User ID
    pub requested_user_id: Option<String>,
    #[serde(rename = "requestedUserName")]
    ///Full name of the Lender User that requested the followup
    pub requested_user_name: Option<String>,
    #[serde(rename = "requestedUserEmail")]
    ///Email address of the Lender User that requested the followup
    pub requested_user_email: Option<String>,
    #[serde(rename = "completedDatetime")]
    ///The timestamp when the followup was completed by the borrower
    pub completed_datetime: Option<String>,
    #[serde(rename = "completedUserId")]
    ///The User ID that completed the followup (generally associated with a Borrower)
    pub completed_user_id: Option<String>,
    #[serde(rename = "reviewAction")]
    ///The action taken by the reviewing lender. Can be ACCEPT or REJECT.
    pub review_action: Option<String>,
    #[serde(rename = "reviewedDatetime")]
    ///The timestamp when the follow-up was reviewed (in UTC)
    pub reviewed_datetime: Option<String>,
    #[serde(rename = "reviewedUserId")]
    ///The Lender User ID that reviewed the followup
    pub reviewed_user_id: Option<String>,
    #[serde(rename = "cancelledDatetime")]
    ///The timestamp when the followup was cancelled (in UTC)
    pub cancelled_datetime: Option<String>,
    #[serde(rename = "cancelledUserId")]
    ///The lender User ID that cancelled the followup
    pub cancelled_user_id: Option<String>,
    #[serde(rename = "asOfDate")]
    ///The date that the report was generated
    pub as_of_date: String,
    #[serde(rename = "clientFollowupReferenceId")]
    ///Unique id passed by client for referencing follow-up
    pub client_followup_reference_id: Option<String>,
}
impl std::fmt::Display for ReportingFollowupsSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingLenderPabActivitiesSchema {
    #[serde(rename = "activityId")]
    ///Unique ID of the activity
    pub activity_id: String,
    #[serde(rename = "loanId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub loan_id: String,
    #[serde(rename = "loanNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub loan_number: Option<String>,
    #[serde(rename = "loanType")]
    ///The specific product (Mortgage, HELOAN, HELOC, Auto, etc)
    pub loan_type: Option<String>,
    #[serde(rename = "loanPurposeType")]
    ///REFINANCE or PURCHASE for Mortgage loans. Will be set to OTHER or null for other loan types.
    pub loan_purpose_type: Option<String>,
    #[serde(rename = "appLeadSource")]
    ///Specifies how an app was created (BORROWER, LENDER, INGEST, PUBLIC_API)
    pub app_lead_source: Option<String>,
    #[serde(rename = "userId")]
    ///User ID for the user that completed the action (can be the user ID for a borrower or lender, or 'system')
    pub user_id: Option<String>,
    #[serde(rename = "userType")]
    ///The type for the user that completed the action. Can be system, borrower, or lender
    pub user_type: Option<String>,
    #[serde(rename = "lenderUserName")]
    ///If userType is a lender, the full name of the lender user that completed the activity
    pub lender_user_name: Option<String>,
    #[serde(rename = "lenderUserEmail")]
    ///If userType is a lender, the email address of the lender user that completed the activity
    pub lender_user_email: Option<String>,
    #[serde(rename = "activityType")]
    ///Which PAB Activity was done on the loan. Can be 'ecoa updated', 'pulled credit' 'selected a product' 'sent pre-approval letter' 'ran desktop underwriter'
    pub activity_type: Option<String>,
    #[serde(rename = "activityDatetime")]
    ///The UTC timestamp for when the activity was done on the loan
    pub activity_datetime: Option<String>,
    #[serde(rename = "activitySuccess")]
    ///Field indicating whether the activity was successful or not
    pub activity_success: Option<bool>,
    #[serde(rename = "creditPulltype")]
    ///For credit pull events - the type of credit pull (Hard or Soft)
    pub credit_pulltype: Option<String>,
    #[serde(rename = "creditBureaus")]
    ///For credit pull events - a JSON array containing the bureaus where credit was pulled from
    pub credit_bureaus: Option<String>,
    #[serde(rename = "creditBureautype")]
    ///For credit pull events - the type of pull (single or tri)
    pub credit_bureautype: Option<String>,
    #[serde(rename = "creditJoint")]
    ///For credit pull events - if the credit was pulled jointly.
    pub credit_joint: Option<bool>,
    #[serde(rename = "creditActiontype")]
    ///For credit pull events - the action on the pull
    pub credit_actiontype: Option<String>,
    #[serde(rename = "duRecommendation")]
    ///For desktop underwriting events - what the recommendation is
    pub du_recommendation: Option<String>,
}
impl std::fmt::Display for ReportingLenderPabActivitiesSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReportingDepositAccountApplicationsSchema {
    #[serde(rename = "asOfDatetime")]
    ///UTC datetime of when the data in the report was last refreshed.
    pub as_of_datetime: String,
    #[serde(rename = "applicationCreatedDatetime")]
    ///UTC datetime of when the application was created.
    pub application_created_datetime: String,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "applicationNumber")]
    ///Application number displayed in the Blend Workspace.
    pub application_number: Option<String>,
    #[serde(rename = "leadSource")]
    ///Type of party that created the application, either "BORROWER" (i.e. the applicant) or "LENDER".
    pub lead_source: Option<String>,
    #[serde(rename = "appSourceMarketingTag")]
    ///Marketing identifier of the top-of-funnel campaign to which the application should be attributed.
    pub app_source_marketing_tag: Option<String>,
    #[serde(rename = "applicantSignupUrl")]
    ///URL of the page through which the primary applicant first created a Blend account. This URL may contain useful referral information if the applicant first arrived at the signup page through a lender-user's custom link or invitation. If there is no URL associated with the primary applicant, this field falls back on the first signup URL associated with the application overall (i.e. that of a coapplicant), if any.
    pub applicant_signup_url: Option<String>,
    #[serde(rename = "isSingleAppFlowApplication")]
    ///Boolean indicator for whether the deposit account application was created (and prefilled) via the "Single App Flow" feature, wherein the applicant had actually started by submitting an application for another product, such as a credit card or a loan, for which the applicant must hold a deposit account in order to be eligible. In this situation, the applicant would have been taken from that original application into a new deposit account application with as many fields prefilled as possible based on the applicant's responses in preceding application.
    pub is_single_app_flow_application: Option<bool>,
    #[serde(rename = "prefillSourceApplicationId")]
    ///In the case of a "Single App Flow" application, the application_id of the applicant's previous credit card or loan application from which the new required deposit account application was prefilled.
    pub prefill_source_application_id: Option<String>,
    #[serde(rename = "prefillSourceApplicationType")]
    ///IIn the case of a "Single App Flow" application, the product for which the applicant was originally applying, i.e. the product associated with the prefill_source_application_id; for example "Credit Card", "Personal Loan", "Auto Loan", etc.
    pub prefill_source_application_type: Option<String>,
    #[serde(rename = "primaryAssignedReviewerUserId")]
    ///Blend user ID of the application's primary assigned lender-user.
    pub primary_assigned_reviewer_user_id: Option<String>,
    #[serde(rename = "primaryAssignedReviewerFullName")]
    ///Full name in the primary assigned reviewer's user profile.
    pub primary_assigned_reviewer_full_name: Option<String>,
    #[serde(rename = "primaryAssignedReviewerEmail")]
    ///Email address of the primary assigned reviewer.
    pub primary_assigned_reviewer_email: Option<String>,
    #[serde(rename = "primaryAssignedReviewerBranchId")]
    ///Branch ID in the primary assigned reviewer's user profile.
    pub primary_assigned_reviewer_branch_id: Option<String>,
    #[serde(rename = "primaryAssignedReviewerEmployeeId")]
    ///Employee ID in the primary assigned reviewer's user profile.
    pub primary_assigned_reviewer_employee_id: Option<String>,
    #[serde(rename = "primaryAssignedReviewerNmlsId")]
    ///Nationwide Multistate Licensing System ID in the primary assigned reviewer's user profile.
    pub primary_assigned_reviewer_nmls_id: Option<String>,
    #[serde(rename = "productBundleReferenceId")]
    ///Product bundle ID associated with the application (a single application can open multiple accounts, e.g. "Checking & Savings").
    pub product_bundle_reference_id: Option<String>,
    #[serde(rename = "totalNumberofAccountsInBundle")]
    ///Number of accounts in the product bundle that will be opened if the application is approved.
    pub total_numberof_accounts_in_bundle: Option<i64>,
    #[serde(rename = "debitCardSelected")]
    ///Booelan indicator for whether the applicant opted in for a debit card.
    pub debit_card_selected: Option<bool>,
    #[serde(rename = "overdraftProtectionSelected")]
    ///Boolean indicator for whether the applicant opted in for overdraft protection.
    pub overdraft_protection_selected: Option<bool>,
    #[serde(rename = "numberOfCheckingAccounts")]
    ///Number of checking accounts in the product bundle that will be opened if the application is approved.
    pub number_of_checking_accounts: Option<i64>,
    #[serde(rename = "checkingProductReferenceIds")]
    ///Product ID(s) of the specific checking product(s) in the bundle; may be the same as the productBundleReferenceId if the application is for a single checking account. This field will contain a comma-separated list of IDs when the application is for a multi-account bundle containing more than one checking account.
    pub checking_product_reference_ids: Option<String>,
    #[serde(rename = "checkingProductNames")]
    ///Full humanized name(s) of the checking product(s) in the bundle. This field will contain a comma-separated list of names when the application is for a multi-account bundle with more than one checking account.
    pub checking_product_names: Option<String>,
    #[serde(rename = "numberOfSavingsAccounts")]
    ///Number of savings accounts in the product bundle that will be opened if the application is approved.
    pub number_of_savings_accounts: Option<i64>,
    #[serde(rename = "savingsProductReferenceIds")]
    ///Product ID(s) of the specific savings product(s) in the bundle; may be the same as the productBundleReferenceId if the application is for a single savings account. This field will contain a comma-separated list of identifiers when the application is for a multi-account bundle containing more than one savings account.
    pub savings_product_reference_ids: Option<String>,
    #[serde(rename = "savingsProductNames")]
    ///Full humanized name(s) of the savings product(s) in the bundle. This field will contain a comma-separated list of names when the application is for a multi-account bundle with more than one savings account.
    pub savings_product_names: Option<String>,
    #[serde(rename = "numberofCdAccounts")]
    ///Number of certificates of deposit (CDs) in the product bundle that will be opened if the application is approved.
    pub numberof_cd_accounts: Option<i64>,
    #[serde(rename = "cdProductReferenceIds")]
    ///Product ID(s) of the specific CD product(s) in the bundle; may be the same as the productBundleReferenceId if the application is for a single CD. This field will contain a comma-separated list of identifiers when the application is for a multi-account bundle containing more than one CD.
    pub cd_product_reference_ids: Option<String>,
    #[serde(rename = "cdProductNames")]
    ///Full humanized name(s) of the CD product(s) in the bundle. This field will contain a comma-separated list of names when the application is for a multi-account bundle with more than one CD.
    pub cd_product_names: Option<String>,
    #[serde(rename = "cdTerms")]
    ///Chosen term length(s) of the CD(s) in the bundle, appearing as strings such as "12_MONTHS" and "36_MONTHS". This field will contain a comma-separated list of term lengths when the application is for a multi-account bundle with more than one CD.
    pub cd_terms: Option<String>,
    #[serde(rename = "numberofHsaAccounts")]
    ///Number of health savings accounts (HSAs) in the product bundle that will be opened if the application is approved.
    pub numberof_hsa_accounts: Option<i64>,
    #[serde(rename = "hsaProductReferenceIds")]
    ///Product ID(s) of the specific HSA product(s) in the bundle; may be the same as the productBundleReferenceId if the application is for a single HSA. This field will contain a comma-separated list of identifiers when the application is for a multi-account bundle containing more than one HSA.
    pub hsa_product_reference_ids: Option<String>,
    #[serde(rename = "hsaProductNames")]
    ///Full humanized name(s) of the HSA product(s) in the bundle. This field will contain a comma-separated list of names when the application is for a multi-account bundle with more than one HSA.
    pub hsa_product_names: Option<String>,
    #[serde(rename = "hsaPlanTypes")]
    ///Chosen plan type(s) of the HSA(s) in the bundle, either "INDIVIDUAL" or "FAMILY". This field will contain a comma-separated list of plan types when the application is for a multi-account bundle with more than one HSA.
    pub hsa_plan_types: Option<String>,
    #[serde(rename = "totalNumberOfApplicants")]
    ///Number of applicants on the application (primary, co-, and minor applicants).
    pub total_number_of_applicants: Option<i64>,
    #[serde(rename = "numberOfPrimaryApplicants")]
    ///Number of primary applicants on the application. Every application should have one primary applicant.
    pub number_of_primary_applicants: Option<i64>,
    #[serde(rename = "primaryApplicantEmploymentStatus")]
    ///Employment status of the primary applicant. There are five statuses the applicant can choose from, including "EMPLOYED", "SELF_EMPLOYED", "UNEMPLOYED", "RETIRED", and "NEVER_EMPLOYED".
    pub primary_applicant_employment_status: Option<String>,
    #[serde(rename = "primaryApplicantIsExistingMember")]
    ///Boolean indicator that is TRUE if the primary applicant indicated that they are an existing member of the institution.
    pub primary_applicant_is_existing_member: Option<bool>,
    #[serde(rename = "numberOfCoapplicants")]
    ///Number of coapplicants on the application.
    pub number_of_coapplicants: Option<i64>,
    #[serde(rename = "coapplicantEmploymentStatus")]
    ///Employment status of the coapplicant. There are five statuses the coapplicant can choose from, including "EMPLOYED", "SELF_EMPLOYED", "UNEMPLOYED", "RETIRED", and "NEVER_EMPLOYED".
    pub coapplicant_employment_status: Option<String>,
    #[serde(rename = "numberOfMinorApplicants")]
    ///Number of minor applicants (i.e. persons under 18) on the application.
    pub number_of_minor_applicants: Option<i64>,
    #[serde(rename = "firstUiTrackingEventDatetime")]
    ///UTC datetime of the first tracking event associated with the application. The presence of an event should indicate that the applicant at least allowed the UI to load.
    pub first_ui_tracking_event_datetime: Option<String>,
    #[serde(rename = "firstMinorInfoWorkflowEventDatetime")]
    ///UTC datetime of the first tracking event in the "Minor Info" workflow.
    pub first_minor_info_workflow_event_datetime: Option<String>,
    #[serde(rename = "firstPersonalInfoWorkflowEventDatetime")]
    ///UTC datetime of the first tracking event in the "Personal Info" workflow.
    pub first_personal_info_workflow_event_datetime: Option<String>,
    #[serde(rename = "hasActivityBeyondInitialDetailsTask")]
    ///Boolean indicator compiling all possible downfunnel evidence that the application made it past the initial "Get Applicant Details" task in the "Personal Info" workflow. The field is intended to be used as a more qualified definition of "started", with a value of TRUE identifying applications that were not abandoned on the first page.
    pub has_activity_beyond_initial_details_task: Option<bool>,
    #[serde(rename = "deviceTypestart")]
    ///Type of device ("Desktop", "Mobile", or "Tablet") used for the application's first user action, which should correspond to clicking "continue" on the first page of the application.
    pub device_typestart: Option<String>,
    #[serde(rename = "firstIdvWorkflowEventDatetime")]
    ///UTC datetime of the first tracking event in the "Identity Verfication" workflow.
    pub first_idv_workflow_event_datetime: Option<String>,
    #[serde(rename = "firstIdvWorkflowSubmitDatetime")]
    ///UTC datetime of the first click on "Submit and Continue" on the final page of the "Identity Verification" workflow, which should submit the applicant's information to the third-party identity verification (IDV) provider (Alloy or Socure). The exception to this is when the applicant is an existing member of the institution, and the institution has configured its application to "offboard" existing members from the application without submission to the IDV provider.
    pub first_idv_workflow_submit_datetime: Option<String>,
    #[serde(rename = "firstIdvWorkflowAdditionalInfoRequestedDatetime")]
    ///UTC datetime of the first time an applicant was sent to the supplementary "Step Up" task in the "Identity Verification" workflow, where additional identifying information is requested, if the initial ID information provided was found to be insufficient for evaluation/decisioning.
    pub first_idv_workflow_additional_info_requested_datetime: Option<String>,
    #[serde(rename = "firstAccountSetupWorkflowEventDatetime")]
    ///UTC datetime of the first tracking event in the "Account Setup" workflow.
    pub first_account_setup_workflow_event_datetime: Option<String>,
    #[serde(rename = "has_beneficiaries_added")]
    ///Boolean indicator for whether any beneficiaries were specified and allocated for the new account(s) in the "Account Setup" workflow.
    pub has_beneficiaries_added: Option<bool>,
    #[serde(rename = "applicationSubmittedDatetime")]
    ///UTC datetime of application submission.
    pub application_submitted_datetime: Option<String>,
    #[serde(rename = "applicationSubmissionStatus")]
    ///Supplementary status indicator that may be able to confirm application submission even in the case of a lapse in the event tracking that produces applicationSubmittedDatetime; either "SUBMITTED" or "NOT SUBMITTED".
    pub application_submission_status: Option<String>,
    #[serde(rename = "totalNumberOfFollowupsRequested")]
    ///Number of follow-up tasks requested of the applicant by a lender-user through Blend's "Follow-Ups" tool.
    pub total_number_of_followups_requested: Option<i64>,
    #[serde(rename = "firstAutomatedApprovalDatetime")]
    ///UTC datetime of the first automated approval decision on the application emitted by the third-party identity verification (IDV) provider (Alloy or Socure). An approval decision will not be emitted unless and until the application is submitted.
    pub first_automated_approval_datetime: Option<String>,
    #[serde(rename = "firstAutomatedReferralDatetime")]
    ///UTC datetime of the first automated referral on the application, where the third-party identity verification (IDV) provider (Alloy or Socure) flags the application for manual review by a lender-user. A referral decision will not be made unless and until the application is submitted.
    pub first_automated_referral_datetime: Option<String>,
    #[serde(rename = "firstAutomatedRejectionDatetime")]
    ///UTC datetime of the first automated rejection decision by the third-party identity verification (IDV) provider (Alloy or Socure) or by the "system" (i.e. application configurations set by the institution to reject/"offboard" certain applicants, like existing members, on the spot). A rejection decision can be emitted prior to application submission if the applicant fails the basic preliminary IDV screening (because of a low Qualifile score) and triggers an "adverse action notice" (AAN) in the "Identity Verification" workflow, or if they are rejected by system rules.
    pub first_automated_rejection_datetime: Option<String>,
    #[serde(rename = "firstAutomatedRejectionSource")]
    ///Source of the first automated rejection decision, either "IDV provider" (Alloy or Socure) or "system" (i.e. application configurations set by the institution to reject/"offboard" certain applicants, like existing members, on the spot).
    pub first_automated_rejection_source: Option<String>,
    #[serde(rename = "primaryApplicantRejectedViaAan")]
    ///Booelan indicator for whether the primary applicant was rejected via "adverse action notice" (AAN) based on their "Qualifile" score in the "Identity Verification" workflow, which would result in an immediate rejection of the application without proceeding to the "Account Setup" workflow. If the applicant has not yet submitted their information for IDV evaulation, this field will be null.
    pub primary_applicant_rejected_via_aan: Option<bool>,
    #[serde(rename = "coapplicantRejectedViaAan")]
    ///Booelan indicator for whether the coapplicant was rejected via "adverse action notice" (AAN) based on their "Qualifile" score in the "Identity Verification" workflow. If there is no coapplicant on the application, or the coapplicant has not yet submitted their information for IDV evaulation, this field will be null.
    pub coapplicant_rejected_via_aan: Option<bool>,
    #[serde(rename = "firstManualApprovalDatetime")]
    ///UTC datetime of the first manual approval decision on the application issued by a lender-user. An application cannot be reviewed or approved unless and until the application is submitted.
    pub first_manual_approval_datetime: Option<String>,
    #[serde(rename = "firstManualApprovalReviewerUserId")]
    ///Blend user ID of the lender-user who manually approved the application.
    pub first_manual_approval_reviewer_user_id: Option<String>,
    #[serde(rename = "firstManualApprovalReviewerFullName")]
    ///Full name in the user profile of the lender-user who manually approved the application.
    pub first_manual_approval_reviewer_full_name: Option<String>,
    #[serde(rename = "firstManualApprovalReviewerEmail")]
    ///Email address of the lender-user who manually approved the application.
    pub first_manual_approval_reviewer_email: Option<String>,
    #[serde(rename = "firstManualApprovalReviewerBranchId")]
    ///Branch ID in the user profile of the lender-user who manually approved the application.
    pub first_manual_approval_reviewer_branch_id: Option<String>,
    #[serde(rename = "firstManualApprovalReviewerEmployeeId")]
    ///Employee ID in the user profile of the lender-user who manually approved the application.
    pub first_manual_approval_reviewer_employee_id: Option<String>,
    #[serde(rename = "firstManualApprovalReviewerNmlsId")]
    ///Nationwide Multistate Licensing System ID in the user profile of the lender-user who manually approved the application.
    pub first_manual_approval_reviewer_nmls_id: Option<String>,
    #[serde(rename = "firstManualRejectionDatetime")]
    ///UTC datetime of the first manual rejection decision on the application issued by a lender user. An application cannot be reviewed or rejected unless and until the application is submitted.
    pub first_manual_rejection_datetime: Option<String>,
    #[serde(rename = "firstManualRejectionReviewerUserId")]
    ///Blend user ID of the lender-user who manually rejected the application.
    pub first_manual_rejection_reviewer_user_id: Option<String>,
    #[serde(rename = "firstManualRejectionReviewerFullName")]
    ///Full name in the user profile of the lender-user who manually rejected the application.
    pub first_manual_rejection_reviewer_full_name: Option<String>,
    #[serde(rename = "firstManualRejectionReviewerEmail")]
    ///Email address of the lender-user who manually rejected the application.
    pub first_manual_rejection_reviewer_email: Option<String>,
    #[serde(rename = "firstManualRejectionReviewerBranchId")]
    ///Branch ID in the user profile of the lender-user who manually rejected the application.
    pub first_manual_rejection_reviewer_branch_id: Option<String>,
    #[serde(rename = "firstManualRejectionReviewerEmployeeId")]
    ///Employee ID in the user profile of the lender-user who manually rejected the application.
    pub first_manual_rejection_reviewer_employee_id: Option<String>,
    #[serde(rename = "firstManualRejectionReviewerNmlsId")]
    ///Nationwide Multistate Licensing System ID in the user profile of the lender-user who manually rejected the application.
    pub first_manual_rejection_reviewer_nmls_id: Option<String>,
    #[serde(rename = "latestDecisionDatetime")]
    ///UTC datetime of the latest decision on the application, automated or manual, approval or rejection. See descriptions of first_automated_approval_datetime, first_automated_rejection_datetime, first_manual_approval_datetime, and first_manual_rejection_datetime for detail on each decision type and outcome. Referrals are not considered true "decisions" and thus are ignored by this field.
    pub latest_decision_datetime: Option<String>,
    #[serde(rename = "latestDecisionType")]
    ///Type, either "automated" or "manual", of the latest decision on the appliction, approval or rejection. Referrals are not considered true "decisions" and thus are ignored by this field.
    pub latest_decision_type: Option<String>,
    #[serde(rename = "latestDecisionOutcome")]
    ///Outcome, either "approve" or "reject", of the latest decision on the application, automated or manual. Referrals are not considered true "decisions" and thus are ignored by this field.
    pub latest_decision_outcome: Option<String>,
    #[serde(rename = "latestDecisionAutomatedRejectionSource")]
    ///If the latest decision on the application is an automated rejection, the source of that rejection, either "IDV provider" (Alloy or Socure) or "system" (i.e. application configurations set by the institution to reject/"offboard" certain applicants, like existing members, on the spot).
    pub latest_decision_automated_rejection_source: Option<String>,
    #[serde(rename = "latestDecisionReviewerUserId")]
    ///If the latest decision on the application is manual, the Blend user ID of the lender-user who made that decision.
    pub latest_decision_reviewer_user_id: Option<String>,
    #[serde(rename = "latestDecisionReviewerFullName")]
    ///If the latest decision on the application is manual, the full name in the user profile of the lender-user who made that decision.
    pub latest_decision_reviewer_full_name: Option<String>,
    #[serde(rename = "latestDecisionReviewerEmail")]
    ///If the latest decision on the application is manual, the email address of the lender-user who made that decision.
    pub latest_decision_reviewer_email: Option<String>,
    #[serde(rename = "latestDecisionReviewerBranchId")]
    ///If the latest decision on the application is manual, the branch ID in the user profile of the lender-user who made that decision.
    pub latest_decision_reviewer_branch_id: Option<String>,
    #[serde(rename = "latestDecisionReviewerEmployeeId")]
    ///If the latest decision on the application is manual, the employee ID in the user profile of the lender-user who made the decision.
    pub latest_decision_reviewer_employee_id: Option<String>,
    #[serde(rename = "latestDecisionReviewerNmlsId")]
    ///If the latest decision on the application is manual, the Nationwide Multistate Licensing System ID in the user profile of the lender-user who made that latest decision.
    pub latest_decision_reviewer_nmls_id: Option<String>,
    #[serde(rename = "applicationApprovalStatus")]
    ///Approval status of the application, either "APPROVED", "REFERRED", or "REJECTED" if the application has reached a decisioning point, or null if not.
    pub application_approval_status: Option<String>,
    #[serde(rename = "accountBookedToCoreDatetime")]
    ///UTC datetime that the newly opened account(s) was successfully booked to the institution's core banking system.
    pub account_booked_to_core_datetime: Option<String>,
    #[serde(rename = "applicationBookingStatus")]
    ///Supplementary status indicator that may be able to confirm that the newly opened account(s) was successfully booked even in the case of a lapse in the event tracking that produces accountBookedToCoreDatetime; either "BOOKED" or null.
    pub application_booking_status: Option<String>,
    #[serde(rename = "firstFundingWorkflowEventDatetime")]
    ///UTC datetime of the first tracking event in the "Opening Deposit" workflow.
    pub first_funding_workflow_event_datetime: Option<String>,
    #[serde(rename = "firstOpeningDepositTransactionSubmittedDatetime")]
    ///UTC datetime of the first deposit transaction submitted through the "Opening Deposit" workflow. The exception to this is for institutions that have configured their application to preemptively require the applicant to complete the opening deposit tasks in the "Account Setup" workflow prior to application submission, in which case this field will still report a datetime once those tasks have been completed, even if the application was never subsequently submitted, approved, and booked to the core banking system, and accordingly no opening deposits were actually transferred.
    pub first_opening_deposit_transaction_submitted_datetime: Option<String>,
    #[serde(rename = "numberOfNewAccountsFunded")]
    ///Number of newly opened accounts funded with an initial deposit submitted through the "Opening Deposit" workflow. The exception to this is for institutions that have configured their application to preemptively require the applicant to complete the opening deposit tasks in the "Account Setup" workflow prior to application submission, in which case this field will still report the number of intended desintation accounts once those tasks have been completed, even if the application was never subsequently submitted, approved, and booked to the core banking system, and accordingly no opening deposits were actually transferred.
    pub number_of_new_accounts_funded: Option<i64>,
    #[serde(rename = "openingDepositAmounts")]
    ///Dollar amount(s) of the opening deposit(s) submitted through the "Opening Deposit" workflow, rounded to the nearest increment of 50. This field will contain a comma-separated list of amounts when multiple newly opened accounts that originated from the same application were funded. The exception to this is for institutions that have configured their application to preemptively require the applicant to complete the opening deposit tasks in the "Account Setup" workflow prior to application submission, in which case this field will still report the intended deposit amount(s) once those tasks have been completed, even if the application was never subsequently submitted, approved, and booked to the core banking system, and accordingly no opening deposits were actually transferred.
    pub opening_deposit_amounts: Option<String>,
    #[serde(rename = "totalAmountDeposited")]
    ///Sum of the dollar amounts (each rounded to the nearest increment of 50 prior to summation) of the opening deposits submitted through the "Opening Deposit" workflow; the sum will be the same as the value in opening_deposit_amounts if only one new account was funded. The exception to this is for institutions that have configured their application to preemptively require the applicant to complete the opening deposit tasks in the "Account Setup" workflow prior to application submission, in which case this field will still report the intended total deposit amount once those tasks have been completed, even if the application was never subsequently submitted, approved, and booked to the core banking system, and accordingly no opening deposits were actually transferred.
    pub total_amount_deposited: Option<i64>,
    #[serde(rename = "openingDepositSourceAccountConnectionMethod")]
    ///Method used to connect an external source account for the transfer of an opening deposit in the "Opening Deposit" workflow. The applicant can either use the Plaid integration to log in to a financial institution, or manually enter the account and routing numbers of the source account. This field will have a value of either "Plaid", "manual entry", or "unknown" when first_opening_deposit_transaction_submitted_datetime is populated, and null otherwise. The exception to this is for institutions that have configured their application to preemptively require the applicant to complete the opening deposit tasks in the "Account Setup" workflow prior to application submission, in which case this field will still report the chosen connection method once those tasks have been completed, even if the application was never subsequently submitted, approved, and booked to the core banking system, and accordingly no opening deposits were actually transferred.
    pub opening_deposit_source_account_connection_method: Option<String>,
    #[serde(rename = "openingDepositSourceAccountType")]
    ///Type of source account used to fund the opening deposit, either "checking", "savings", or "unknown". The account type (checking or savings) is known for all Plaid-connected source accounts, whereas this information is only collected for manually entered source accounts if the instutition has configured the application to ask the applicant to specify the account type. The exception to this is for institutions that have configured their application to preemptively require the applicant to complete the opening deposit tasks in the "Account Setup" workflow prior to application submission, in which case this field will still report the intended source account type once those tasks have been completed, even if the application was never subsequently submitted, approved, and booked to the core banking system, and accordingly no opening deposits were actually transferred.
    pub opening_deposit_source_account_type: Option<String>,
    #[serde(rename = "applicationFundingStatus")]
    ///Funding status of the application. This field will be "FUNDED VIA BLEND" if an opening deposit for at least one newly opened account was submitted through the "Opening Deposit" workflow (i.e. if application_booking_status = "BOOKED" and first_opening_deposit_transaction_submitted_datetime exists), and null otherwise.
    pub application_funding_status: Option<String>,
    #[serde(rename = "applicationStatusOverall")]
    ///Overall latest status of the application (i.e. the most down-funnel status reached to date), either "NOT SUBMITTED", "SUBMITTED", "REFERRED", "APPROVED", "REJECTED", "BOOKED", or "FUNDED VIA BLEND".
    pub application_status_overall: Option<String>,
    #[serde(rename = "firstOnlineBankingRegistrationWorkflowEventDatetime")]
    ///UTC datetime of the first tracking event in the "Online Banking Registration" workflow.
    pub first_online_banking_registration_workflow_event_datetime: Option<String>,
    #[serde(rename = "onlineBankingRegistrationCompletedDatetime")]
    ///UTC datetime of when the new account holder succesfully created and submitted login credentials for online banking access at the institution via the "Online Banking Registration" workflow.
    pub online_banking_registration_completed_datetime: Option<String>,
    #[serde(rename = "applicationLateTouchDatetime")]
    ///UTC datetime of when the application was last touched (by either an applicant or a lender-user).
    pub application_late_touch_datetime: Option<String>,
    #[serde(rename = "applicationLastTouchedByUserId")]
    ///Blend user ID of the applicant or lender-user who last touched the application.
    pub application_last_touched_by_user_id: Option<String>,
    #[serde(rename = "applicationLastUpdatedDatetime")]
    ///UTC datetime of when the application record was last updated by the system.
    pub application_last_updated_datetime: Option<String>,
    #[serde(rename = "applicationArchivedDatetime")]
    ///UTC datetime of when the application was archived in Blend.
    pub application_archived_datetime: Option<String>,
    #[serde(rename = "applicationDeletedDatetime")]
    ///UTC datetime of when the application was deleted by a user.
    pub application_deleted_datetime: Option<String>,
    #[serde(rename = "applicationPurgedDatetime")]
    ///UTC datetime of when the application was purged from Blend production.
    pub application_purged_datetime: Option<String>,
}
impl std::fmt::Display for ReportingDepositAccountApplicationsSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventSchema {
    #[serde(rename = "eventId")]
    ///Blend UUID of the event
    pub event_id: String,
    #[serde(rename = "entityId")]
    ///A Blend UUID. If this is set, all events for the given entity are returned.
    pub entity_id: Option<String>,
    #[serde(rename = "status")]
    ///The latest status of the event, updated by the API
    pub status: String,
    #[serde(rename = "eventType")]
    ///Event type
    pub event_type: String,
    #[serde(rename = "createdAt")]
    ///The UTC timestamp of the event creation. ISO format
    pub created_at: String,
    #[serde(rename = "statusSetAt")]
    ///The UTC timestamp of the latest status that was posted for the event. ISO format
    pub status_set_at: String,
}
impl std::fmt::Display for EventSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventWithDataSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EventDescribePath {
    #[serde(rename = "type")]
    ///The readable name of the event type relative to the system relevant to the described path.
    pub type_: String,
    #[serde(rename = "system")]
    ///The system producing or receiving the event.
    pub system: String,
    #[serde(rename = "version")]
    ///The version of the system producing or receiving the events. May be null for systems with no versioning.
    pub version: String,
}
impl std::fmt::Display for EventDescribePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventDescribeMetadata {
    #[serde(rename = "inbound")]
    pub inbound: EventDescribePath,
    #[serde(rename = "outbound")]
    pub outbound: EventDescribePath,
    #[serde(rename = "validDecorators")]
    pub valid_decorators: Vec<String>,
}
impl std::fmt::Display for EventDescribeMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaFlagSchema {
    #[serde(rename = "presence")]
    ///A string flag that denotes whether the described parent is required, optional, or conditionally allowed.
    pub presence: String,
    #[serde(rename = "allowUnknown")]
    ///Describes whether or not the consumer should allow unknown additional keys or explicitly decline them when parsing an object.
    pub allow_unknown: Option<bool>,
}
impl std::fmt::Display for SchemaFlagSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventDescribeChildSchema {
    #[serde(rename = "type")]
    ///The data type of the child object, which could be an compound or primitive data type.
    pub type_: String,
    #[serde(rename = "flags")]
    pub flags: SchemaFlagSchema,
    #[serde(rename = "invalids")]
    pub invalids: Vec<String>,
    #[serde(rename = "rules")]
    pub rules: Vec<serde_json::Value>,
}
impl std::fmt::Display for EventDescribeChildSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventDescribeSchemaSchema {
    #[serde(rename = "type")]
    ///The data type of the schema, which can be any primitive or an object.
    pub type_: String,
    #[serde(rename = "flags")]
    pub flags: SchemaFlagSchema,
    #[serde(rename = "children")]
    pub children: serde_json::Value,
}
impl std::fmt::Display for EventDescribeSchemaSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventDescribeSchema {
    #[serde(rename = "metadata")]
    pub metadata: EventDescribeMetadata,
    #[serde(rename = "schema")]
    ///A key value pair where the key is the property name and the value is an object defining acceptable values for that property.
    pub schema: Option<EventDescribeSchemaSchema>,
}
impl std::fmt::Display for EventDescribeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventStatusSchema {
    #[serde(rename = "status")]
    ///The latest status of the event, updated by the API
    pub status: String,
    #[serde(rename = "occurredAt")]
    ///The UTC timestamp of the latest status that was posted for the event, in ISO format
    pub occurred_at: String,
}
impl std::fmt::Display for EventStatusSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventStatusRequestSchema {
    #[serde(rename = "status")]
    ///The status of the event
    pub status: String,
    #[serde(rename = "error")]
    ///Only used if an error occurs when processing an event
    pub error: Option<serde_json::Value>,
}
impl std::fmt::Display for EventStatusRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventInvalidRequestSchema {
    #[serde(rename = "error")]
    ///Error message
    pub error: String,
    #[serde(rename = "display")]
    ///Error message description
    pub display: String,
    #[serde(rename = "trackingId")]
    ///Provide this to Blend when troubleshooting
    pub tracking_id: String,
}
impl std::fmt::Display for EventInvalidRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventUnauthorizedSchema {
    #[serde(rename = "error")]
    ///Error message
    pub error: String,
    #[serde(rename = "display")]
    ///Error message description
    pub display: String,
    #[serde(rename = "trackingId")]
    ///Provide this to Blend when troubleshooting
    pub tracking_id: String,
}
impl std::fmt::Display for EventUnauthorizedSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventMissingObjectSchema {
    #[serde(rename = "error")]
    ///Error message
    pub error: String,
    #[serde(rename = "display")]
    ///Error message description
    pub display: String,
    #[serde(rename = "trackingId")]
    ///Provide this to Blend when troubleshooting
    pub tracking_id: String,
}
impl std::fmt::Display for EventMissingObjectSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EventInternalErrorSchema {
    #[serde(rename = "error")]
    ///Error message
    pub error: String,
    #[serde(rename = "display")]
    ///Error message description
    pub display: String,
    #[serde(rename = "trackingId")]
    ///Provide this to Blend when troubleshooting
    pub tracking_id: String,
}
impl std::fmt::Display for EventInternalErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnderwritingResultSchema {
    #[serde(rename = "id")]
    ///The UUID of the Underwriting Result in Blend's system.
    pub id: String,
    #[serde(rename = "ausProduct")]
    ///The AUS system that was used to run the result.
    pub aus_product: String,
    #[serde(rename = "runData")]
    ///A collection of information about a specific AUS run in Blend.
    pub run_data: Option<serde_json::Value>,
    #[serde(rename = "recommendation")]
    ///The returned recommendation from the AUS.
    pub recommendation: Option<serde_json::Value>,
    #[serde(rename = "findingsDocId")]
    ///The UUID for the document that came with the findings for this AUS run.
    pub findings_doc_id: Option<String>,
    #[serde(rename = "runParameters")]
    ///The loan attributes used for this AUS run, such as debt-to-income ratio.
    pub run_parameters: Option<serde_json::Value>,
}
impl std::fmt::Display for UnderwritingResultSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostFollowUpSchema {
    #[serde(rename = "applicationId")]
    ///ID of the application to add the follow-up to
    pub application_id: String,
    #[serde(rename = "type")]
    ///Types of follow-ups that can be created
    pub type_: String,
    #[serde(rename = "comments")]
    ///Comments displayed to the party on the follow-up
    pub comments: Option<String>,
    #[serde(rename = "clientFollowUpReferenceId")]
    ///Unique id passed by client for referencing follow-up
    pub client_follow_up_reference_id: Option<String>,
    #[serde(rename = "dueDate")]
    ///UTC Timestamp of follow-up due date
    pub due_date: Option<String>,
    #[serde(rename = "context")]
    pub context: serde_json::Value,
}
impl std::fmt::Display for PostFollowUpSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchFollowUpSchema {
    #[serde(rename = "status")]
    pub status: Option<String>,
    #[serde(rename = "comments")]
    ///Comments displayed to the party on the follow-up
    pub comments: Option<String>,
    #[serde(rename = "clientFollowUpReferenceId")]
    ///Unique id passed by client for referencing follow-up
    pub client_follow_up_reference_id: Option<String>,
    #[serde(rename = "dueDate")]
    ///UTC Timestamp of follow-up due date
    pub due_date: Option<String>,
    #[serde(rename = "context")]
    pub context: Option<serde_json::Value>,
}
impl std::fmt::Display for PatchFollowUpSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowUpSchema {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "applicationId")]
    pub application_id: String,
    #[serde(rename = "type")]
    ///Types of follow-ups that can be returned. This is a superset of FollowUpType that includes the 'SYSTEM' follow-up type corresponding to follow-ups that were requested by Blend automatically, and the 'DOCUMENT_WET_SIGNATURE' follow-up type which is the type of wet-sign follow-ups issued when an external e-sign ('DOCUMENT_SIGNATURE' type) follow-up request fails.
    pub type_: String,
    #[serde(rename = "status")]
    /**Follow-up statuses
* **REQUESTED** - Follow-up has been requested and is awaiting completion by the borrower
* **IN_PROGRESS** - Follow-up has been started by at least one borrower
* **COMPLETED** - Follow-up has been completed by the borrower and if a document request, reviewed and accepted by the lender in the Blend lender UI.
* **COMPLETED_EXTERNALLY** - Follow-up has been marked as completed externally in the Blend lender UI
* **PENDING_REVIEW** - Follow-up has been completed by the borrower and can be reviewed and accepted or rejected by the lender in the Blend lender UI
* **REJECTED** - Follow-up has been completed by the borrower and was reviewed and rejected by the lender in the Blend lender UI
*/
    pub status: String,
    #[serde(rename = "comments")]
    ///Comments displayed to the party on the follow-up
    pub comments: Option<String>,
    #[serde(rename = "clientFollowUpReferenceId")]
    ///Unique id passed by client for referencing follow-up
    pub client_follow_up_reference_id: Option<String>,
    #[serde(rename = "dueDate")]
    ///UTC Timestamp of follow-up due date
    pub due_date: Option<String>,
    #[serde(rename = "requestedAt")]
    ///UTC Timestamp of when the follow-up was requested
    pub requested_at: String,
    #[serde(rename = "requestedBy")]
    ///The ID of the user that requested the follow-up, or 'SYSTEM' for automatically requested follow-ups
    pub requested_by: String,
    #[serde(rename = "context")]
    pub context: serde_json::Value,
}
impl std::fmt::Display for FollowUpSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BasePostFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
}
impl std::fmt::Display for BasePostFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseFollowUpContext(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct PostDocumentRequestFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "document")]
    pub document: serde_json::Value,
}
impl std::fmt::Display for PostDocumentRequestFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchDocumentRequestFollowUpContext {
    #[serde(rename = "document")]
    pub document: Option<serde_json::Value>,
}
impl std::fmt::Display for PatchDocumentRequestFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentRequestFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "document")]
    pub document: serde_json::Value,
}
impl std::fmt::Display for DocumentRequestFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostDocumentSignatureFollowUpContext {
    #[serde(rename = "document")]
    pub document: serde_json::Value,
}
impl std::fmt::Display for PostDocumentSignatureFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchDocumentSignatureFollowUpContext {
    #[serde(rename = "document")]
    pub document: Option<serde_json::Value>,
}
impl std::fmt::Display for PatchDocumentSignatureFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentSignatureFollowUpContext {
    #[serde(rename = "document")]
    pub document: serde_json::Value,
}
impl std::fmt::Display for DocumentSignatureFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchReviewDocumentsFollowUpContext {
    #[serde(rename = "title")]
    ///Review documents follow-up display title.
    pub title: Option<String>,
}
impl std::fmt::Display for PatchReviewDocumentsFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewDocumentsFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "documents")]
    pub documents: Vec<serde_json::Value>,
    #[serde(rename = "title")]
    ///Review documents follow-up display title.
    pub title: Option<String>,
}
impl std::fmt::Display for ReviewDocumentsFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PatchPaymentCollectionFollowUpContext {
    #[serde(rename = "lineItems")]
    pub line_items: Option<Vec<PaymentCollectionLineItem>>,
    #[serde(rename = "title")]
    ///Payment collection follow-up display title.
    pub title: Option<String>,
}
impl std::fmt::Display for PatchPaymentCollectionFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCollectionFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "lineItems")]
    pub line_items: Vec<PaymentCollectionLineItem>,
    #[serde(rename = "title")]
    ///Payment collection follow-up display title.
    pub title: Option<String>,
}
impl std::fmt::Display for PaymentCollectionFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCollectionTitle(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct PaymentCollectionLineItem {
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "value")]
    pub value: f64,
}
impl std::fmt::Display for PaymentCollectionLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentFollowUpSubContext(pub Vec<serde_json::Value>);
#[derive(Debug, Serialize, Deserialize)]
pub struct PostTaxReturnFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "taxReturnYear")]
    pub tax_return_year: f64,
}
impl std::fmt::Display for PostTaxReturnFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxReturnFollowUpContext {}
impl std::fmt::Display for TaxReturnFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostTaxTranscriptsFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "taxTranscriptYears")]
    pub tax_transcript_years: Option<Vec<f64>>,
}
impl std::fmt::Display for PostTaxTranscriptsFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxTranscriptsFollowUpContext {}
impl std::fmt::Display for TaxTranscriptsFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PostW2FollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "w2Year")]
    pub w2_year: f64,
}
impl std::fmt::Display for PostW2FollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct W2FollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "w2Year")]
    pub w2_year: Option<f64>,
}
impl std::fmt::Display for W2FollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SystemFollowUpContext {
    #[serde(rename = "partyId")]
    pub party_id: String,
    #[serde(rename = "description")]
    pub description: Option<String>,
}
impl std::fmt::Display for SystemFollowUpContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FollowUpStatus {
    #[serde(rename = "REQUESTED")]
    Requested,
    #[serde(rename = "IN_PROGRESS")]
    InProgress,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "COMPLETED_EXTERNALLY")]
    CompletedExternally,
    #[serde(rename = "PENDING_REVIEW")]
    PendingReview,
    #[serde(rename = "REJECTED")]
    Rejected,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PostFollowUpType {
    #[serde(rename = "DOCUMENT_REQUEST")]
    DocumentRequest,
    #[serde(rename = "DOCUMENT_SIGNATURE")]
    DocumentSignature,
    #[serde(rename = "ASSET_STATEMENTS")]
    AssetStatements,
    #[serde(rename = "PAYMENT_COLLECTION")]
    PaymentCollection,
    #[serde(rename = "PAYSTUBS")]
    Paystubs,
    #[serde(rename = "REVIEW_DOCUMENTS")]
    ReviewDocuments,
    #[serde(rename = "TAX_RETURN")]
    TaxReturn,
    #[serde(rename = "TAX_TRANSCRIPTS")]
    TaxTranscripts,
    #[serde(rename = "W2")]
    W2,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum FollowUpType {
    #[serde(rename = "DOCUMENT_REQUEST")]
    DocumentRequest,
    #[serde(rename = "DOCUMENT_SIGNATURE")]
    DocumentSignature,
    #[serde(rename = "DOCUMENT_WET_SIGNATURE")]
    DocumentWetSignature,
    #[serde(rename = "ASSET_STATEMENTS")]
    AssetStatements,
    #[serde(rename = "PAYMENT_COLLECTION")]
    PaymentCollection,
    #[serde(rename = "PAYSTUBS")]
    Paystubs,
    #[serde(rename = "REVIEW_DOCUMENTS")]
    ReviewDocuments,
    #[serde(rename = "TAX_RETURN")]
    TaxReturn,
    #[serde(rename = "TAX_TRANSCRIPTS")]
    TaxTranscripts,
    #[serde(rename = "W2")]
    W2,
    #[serde(rename = "SYSTEM")]
    System,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PostDocumentType {
    #[serde(rename = "1003")]
    PostDocumentType1003,
    #[serde(rename = "4506t")]
    PostDocumentType4506T,
    #[serde(rename = "ACHDEBIT_AUTHORIZATION")]
    AchdebitAuthorization,
    #[serde(rename = "ADDRESS_VERIFICATION")]
    AddressVerification,
    #[serde(rename = "AFFILIATED_BUSINESS_ARRANGEMENT_DISCLOSURE")]
    AffiliatedBusinessArrangementDisclosure,
    #[serde(rename = "ALIMONY_INCOME")]
    AlimonyIncome,
    #[serde(rename = "ALIMONY_LIABILITY")]
    AlimonyLiability,
    #[serde(rename = "APPRAISAL")]
    Appraisal,
    #[serde(rename = "APPRAISAL_WAIVER")]
    AppraisalWaiver,
    #[serde(rename = "ARTICLES_OF_INCORPORATION")]
    ArticlesOfIncorporation,
    #[serde(rename = "BANKRUPTCY_DISCHARGE_NOTICE")]
    BankruptcyDischargeNotice,
    #[serde(rename = "BANKRUPTCY_FILING")]
    BankruptcyFiling,
    #[serde(rename = "BANK_DEPOSIT_SLIP")]
    BankDepositSlip,
    #[serde(rename = "BANK_STATEMENT")]
    BankStatement,
    #[serde(rename = "BANK_STATEMENT:_MUTUAL_FUND_ACCOUNT")]
    BankStatementMutualFundAccount,
    #[serde(rename = "BANK_STATEMENT:_STOCK_ACCOUNT")]
    BankStatementStockAccount,
    #[serde(rename = "BIRTH_CERTIFICATE")]
    BirthCertificate,
    #[serde(rename = "BONUS_AND_COMMISSION_DOCUMENTATION")]
    BonusAndCommissionDocumentation,
    #[serde(rename = "BORROWER_CERTIFICATION")]
    BorrowerCertification,
    #[serde(rename = "BORROWER_CONSENT_FORM")]
    BorrowerConsentForm,
    #[serde(rename = "BORROWER_CORRESPONDENCE:_LETTER_OF_EXPLANATION")]
    BorrowerCorrespondenceLetterOfExplanation,
    #[serde(rename = "BORROWER_CORRESPONDENCE:_LETTER_OF_EXPLANATION_CASHOUT")]
    BorrowerCorrespondenceLetterOfExplanationCashout,
    #[serde(rename = "BORROWER_CORRESPONDENCE:_LETTER_OF_EXPLANATION_OCCUPANCY")]
    BorrowerCorrespondenceLetterOfExplanationOccupancy,
    #[serde(rename = "BORROWER_INCOME_VERIFICATION_CONSENT_FORM")]
    BorrowerIncomeVerificationConsentForm,
    #[serde(rename = "BUSINESS_LICENSE")]
    BusinessLicense,
    #[serde(rename = "BYLAWS_OPERATING_AGREEMENT")]
    BylawsOperatingAgreement,
    #[serde(rename = "CHILD_SUPPORT_INCOME")]
    ChildSupportIncome,
    #[serde(rename = "CHILD_SUPPORT_LIABILITY")]
    ChildSupportLiability,
    #[serde(rename = "CLOSING_DISCLOSURE")]
    ClosingDisclosure,
    #[serde(rename = "COSIGNED_LIABILITY")]
    CosignedLiability,
    #[serde(rename = "CREDIT_CARD_AUTHORIZATION")]
    CreditCardAuthorization,
    #[serde(rename = "CREDIT_REPORT")]
    CreditReport,
    #[serde(rename = "DD_214")]
    Dd214,
    #[serde(rename = "DEATH_CERTIFICATE")]
    DeathCertificate,
    #[serde(rename = "DIVORCE_DECREE")]
    DivorceDecree,
    #[serde(rename = "EARNEST_MONEY_DEPOSIT")]
    EarnestMoneyDeposit,
    #[serde(rename = "EMPLOYMENT_CONTRACT")]
    EmploymentContract,
    #[serde(rename = "EMPLOYMENT_OFFER_LETTER")]
    EmploymentOfferLetter,
    #[serde(rename = "ESCROW_WAIVER")]
    EscrowWaiver,
    #[serde(rename = "FAIR_LENDING_NOTICE")]
    FairLendingNotice,
    #[serde(rename = "FINANCIAL_STATEMENT:_BALANCE_SHEET")]
    FinancialStatementBalanceSheet,
    #[serde(rename = "FINANCIAL_STATEMENT:_INCOME")]
    FinancialStatementIncome,
    #[serde(rename = "FLOOD_CERTIFICATION")]
    FloodCertification,
    #[serde(rename = "FLOOD_HAZARD_NOTICE")]
    FloodHazardNotice,
    #[serde(rename = "FOR_YOUR_PROTECTION_HOME_INSPECTION")]
    ForYourProtectionHomeInspection,
    #[serde(rename = "GENERAL_LOAN_ACKNOWLEDGMENT")]
    GeneralLoanAcknowledgment,
    #[serde(rename = "GENERATED_ASSET_STATEMENT")]
    GeneratedAssetStatement,
    #[serde(rename = "GIFT_LETTER")]
    GiftLetter,
    #[serde(rename = "GOOD_FAITH_ESTIMATE")]
    GoodFaithEstimate,
    #[serde(rename = "HOA_BILL")]
    HoaBill,
    #[serde(rename = "HOMEOWNERS_ASSOCIATION_CERTIFICATION")]
    HomeownersAssociationCertification,
    #[serde(rename = "HOME_INSPECTION_REPORT")]
    HomeInspectionReport,
    #[serde(rename = "HUD_VA_ADDENDUM")]
    HudVaAddendum,
    #[serde(rename = "IMPORTANT_NOTICE_TO_HOMEBUYER")]
    ImportantNoticeToHomebuyer,
    #[serde(rename = "INSURANCE_AGENT_CONTACT_INFO")]
    InsuranceAgentContactInfo,
    #[serde(rename = "INTENT_TO_PROCEED")]
    IntentToProceed,
    #[serde(rename = "INVOICE_UTILITY_BILL")]
    InvoiceUtilityBill,
    #[serde(rename = "IRS1041")]
    Irs1041,
    #[serde(rename = "IRS1065")]
    Irs1065,
    #[serde(rename = "IRS1099")]
    Irs1099,
    #[serde(rename = "IRS1120")]
    Irs1120,
    #[serde(rename = "IRS1120_S")]
    Irs1120S,
    #[serde(rename = "JUDGMENT")]
    Judgment,
    #[serde(rename = "K1")]
    K1,
    #[serde(rename = "LETTER_OF_EXPLANATION:_ADDRESS_VERIFICATION")]
    LetterOfExplanationAddressVerification,
    #[serde(rename = "LETTER_OF_EXPLANATION:_CREDIT_INQUIRY")]
    LetterOfExplanationCreditInquiry,
    #[serde(rename = "LETTER_OF_EXPLANATION:_DEROGATORY_CREDIT")]
    LetterOfExplanationDerogatoryCredit,
    #[serde(rename = "LETTER_OF_EXPLANATION:_JOB_GAP")]
    LetterOfExplanationJobGap,
    #[serde(rename = "LETTER_OF_EXPLANATION:_LARGE_DEPOSITS")]
    LetterOfExplanationLargeDeposits,
    #[serde(rename = "LETTER_OF_EXPLANATION_NAME_VARIATION")]
    LetterOfExplanationNameVariation,
    #[serde(rename = "LETTER_OF_EXPLANATION:SHORT_SALE")]
    LetterOfExplanationShortSale,
    #[serde(rename = "LLC_BTR_DOC")]
    LlcBtrDoc,
    #[serde(rename = "LOAN_ESTIMATE")]
    LoanEstimate,
    #[serde(rename = "MARRIAGE_CERTIFICATE")]
    MarriageCertificate,
    #[serde(rename = "MEDICAL_LICENSE_DIPLOMA")]
    MedicalLicenseDiploma,
    #[serde(rename = "MEDICAL_RESIDENCY_FELLOWSHIP")]
    MedicalResidencyFellowship,
    #[serde(rename = "MORTGAGE_STATEMENT")]
    MortgageStatement,
    #[serde(rename = "NATIONAL_IDENTIFICATION")]
    NationalIdentification,
    #[serde(rename = "NATIONAL_IDENTIFICATION:_SOCIAL_SECURITY_CARD")]
    NationalIdentificationSocialSecurityCard,
    #[serde(rename = "NATIONAL_IDENTIFICATION_NON_PERMANENT_RESIDENT")]
    NationalIdentificationNonPermanentResident,
    #[serde(rename = "NEAREST_LIVING_RELATIVE")]
    NearestLivingRelative,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "OCCUPANCY_CERTIFICATION")]
    OccupancyCertification,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "PASSPORT")]
    Passport,
    #[serde(rename = "PAYOFF_STATEMENT")]
    PayoffStatement,
    #[serde(rename = "PERMANENT_RESIDENT_IDENTIFICATION")]
    PermanentResidentIdentification,
    #[serde(rename = "PEST_INSPECTION_REPORT")]
    PestInspectionReport,
    #[serde(rename = "POOL_INSPECTION_REPORT")]
    PoolInspectionReport,
    #[serde(rename = "POWER_OF_ATTORNEY")]
    PowerOfAttorney,
    #[serde(rename = "PRE_APPROVAL_LETTER")]
    PreApprovalLetter,
    #[serde(rename = "PRE_APPROVAL_LETTER:_PREVIEW")]
    PreApprovalLetterPreview,
    #[serde(rename = "PROOF_OF_LIQUIDATION")]
    ProofOfLiquidation,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_CONDOMINIUM")]
    PropertyInsurancePolicyCondominium,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_FLOOD")]
    PropertyInsurancePolicyFlood,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_GENERAL")]
    PropertyInsurancePolicyGeneral,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_HAZARD")]
    PropertyInsurancePolicyHazard,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_HOMEOWNERS")]
    PropertyInsurancePolicyHomeowners,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_HURRICANE")]
    PropertyInsurancePolicyHurricane,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_INSECT_INFESTATION")]
    PropertyInsurancePolicyInsectInfestation,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_LEASEHOLD")]
    PropertyInsurancePolicyLeasehold,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_MASTERPOLICY_ASSN")]
    PropertyInsurancePolicyMasterpolicyAssn,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_MINE_SUBSIDENCE")]
    PropertyInsurancePolicyMineSubsidence,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_PERSONAL_PROPERTY")]
    PropertyInsurancePolicyPersonalProperty,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_STORM")]
    PropertyInsurancePolicyStorm,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_TORNADO")]
    PropertyInsurancePolicyTornado,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_VOLCANO")]
    PropertyInsurancePolicyVolcano,
    #[serde(rename = "PROPERTY_INSURANCE_POLICY:_WIND")]
    PropertyInsurancePolicyWind,
    #[serde(rename = "PROPERTY_TAX_BILL")]
    PropertyTaxBill,
    #[serde(rename = "PURCHASE_AGREEMENT")]
    PurchaseAgreement,
    #[serde(rename = "PURCHASE_AGREEMENT_ADDENDUM")]
    PurchaseAgreementAddendum,
    #[serde(rename = "RATE_LOCK_AGREEMENT")]
    RateLockAgreement,
    #[serde(rename = "RENTAL_AGREEMENT")]
    RentalAgreement,
    #[serde(rename = "RENTAL_INCOME_PROOF_OF_RECEIPT")]
    RentalIncomeProofOfReceipt,
    #[serde(rename = "RENTAL_SECURITY_DEPOSIT")]
    RentalSecurityDeposit,
    #[serde(rename = "REQUEST_FOR_COPY_OF_TAX_RETURN:_IRS4506_T")]
    RequestForCopyOfTaxReturnIrs4506T,
    #[serde(rename = "RESIDUAL_INCOME_LETTER")]
    ResidualIncomeLetter,
    #[serde(rename = "RETIREMENT_ACCOUNT_STATEMENT")]
    RetirementAccountStatement,
    #[serde(rename = "RETIREMENT_AWARD_LETTER")]
    RetirementAwardLetter,
    #[serde(rename = "RETIREMENT_LIQUIDITY_TERMS")]
    RetirementLiquidityTerms,
    #[serde(rename = "ROOF_INSPECTION_REPORT")]
    RoofInspectionReport,
    #[serde(rename = "SATISFACTION_OF_SECURITY_INSTRUMENT_LIEN_RELEASE")]
    SatisfactionOfSecurityInstrumentLienRelease,
    #[serde(rename = "SETTLEMENT_STATEMENT_HUD1")]
    SettlementStatementHud1,
    #[serde(rename = "SOCIAL_SECURITY_PROOF_OF_RECEIPT")]
    SocialSecurityProofOfReceipt,
    #[serde(rename = "STATEMENT_CREDIT_CARD_OR_INSTALLMENT_ACCT")]
    StatementCreditCardOrInstallmentAcct,
    #[serde(rename = "TAX_TRANSCRIPT")]
    TaxTranscript,
    #[serde(rename = "TRUST_AGREEMENT")]
    TrustAgreement,
    #[serde(rename = "VA_AWARD_LETTER")]
    VaAwardLetter,
    #[serde(rename = "VA_CHILD_CARE_CERTIFICATE")]
    VaChildCareCertificate,
    #[serde(rename = "VA_CERTIFICATE_OF_ELIGIBILITY")]
    VaCertificateOfEligibility,
    #[serde(rename = "VA_STATEMENT_OF_SERVICE")]
    VaStatementOfService,
    #[serde(rename = "VERIFICATION_OF_MORTGAGE")]
    VerificationOfMortgage,
    #[serde(rename = "VERIFICATION_OF_RENT")]
    VerificationOfRent,
    #[serde(rename = "VERIFICATION_OF_SSN")]
    VerificationOfSsn,
    #[serde(rename = "VISA")]
    Visa,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowUpComments(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowUpClientFollowUpReferenceId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct FollowUpDueDate(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsSummarySchema {
    #[serde(rename = "accounts")]
    pub accounts: Vec<AssetsAccountSummarySchema>,
}
impl std::fmt::Display for AssetsSummarySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsAccountSummarySchema {
    #[serde(rename = "id")]
    ///The UUID of the an Asset account in Blend's system.
    pub id: String,
    #[serde(rename = "partyIds")]
    ///Ids of parties that own this account, as stated in the application. This is not guaranteed to match account ownership data retrieved from data providers for Connected Assets.
    pub party_ids: Vec<String>,
    #[serde(rename = "institutionName")]
    ///The name of the financial institution that holds this account.
    pub institution_name: String,
    #[serde(rename = "accountType")]
    ///The type of an asset account
    pub account_type: String,
    #[serde(rename = "accountNumberLastFour")]
    ///The last four digits of the account number.
    pub account_number_last_four: String,
    #[serde(rename = "connectionStatus")]
    /**Connection Status of an Assets account
* **CONNECTED** - This account was originally connected to through a data provider. Refreshes will result in us attempting to pull updated data from our data provider and generating an updated Asset statement. We may also attempt to pull an Asset statement from the financial institution directly.
* **NOT_CONNECTED** - This account was originally not connected to. Refreshes will result in us issuing an Account Statements follow-up
* **LOCKED** - This account was originally connected to while using multi-factor authentication. Refreshes will result in us issuing an Updated Assets follow-up for the owner to re-connect.
*/
    pub connection_status: String,
    #[serde(rename = "lastRefresh")]
    ///Summary of last Assets refresh attempt for this account. This will be omitted from accounts we have not completed a refresh for. If a refresh is in progress, this will be omitted until the refresh has completed.
    pub last_refresh: Option<serde_json::Value>,
}
impl std::fmt::Display for AssetsAccountSummarySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AssetAccountType {
    #[serde(rename = "Checking")]
    Checking,
    #[serde(rename = "Brokerage")]
    Brokerage,
    #[serde(rename = "Retirement")]
    Retirement,
    #[serde(rename = "Savings")]
    Savings,
    #[serde(rename = "Trust")]
    Trust,
    #[serde(rename = "Certificate of Deposit")]
    CertificateOfDeposit,
    #[serde(rename = "Money Market")]
    MoneyMarket,
    #[serde(rename = "Annuity")]
    Annuity,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Unknown")]
    Unknown,
    #[serde(rename = "Mutual Fund")]
    MutualFund,
    #[serde(rename = "Stocks")]
    Stocks,
    #[serde(rename = "Stock Options")]
    StockOptions,
    #[serde(rename = "Bonds")]
    Bonds,
    #[serde(rename = "Bridge Loan Proceeds")]
    BridgeLoanProceeds,
    #[serde(rename = "Individual Development Account")]
    IndividualDevelopmentAccount,
    #[serde(rename = "Cash Value of Life Insurance")]
    CashValueOfLifeInsurance,
    #[serde(rename = "Chequing")]
    Chequing,
    #[serde(rename = "Stocks/Bond/Mutual")]
    StocksBondMutual,
    #[serde(rename = "RRSP")]
    Rrsp,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ConnectionStatusType {
    #[serde(rename = "CONNECTED")]
    Connected,
    #[serde(rename = "NOT_CONNECTED")]
    NotConnected,
    #[serde(rename = "LOCKED")]
    Locked,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum AccountRefreshStatusType {
    #[serde(rename = "GAS_GENERATED")]
    GasGenerated,
    #[serde(rename = "FINANCIAL_STATEMENT_PULLED")]
    FinancialStatementPulled,
    #[serde(rename = "UPDATED_ASSETS_FOLLOW_UP_ISSUED")]
    UpdatedAssetsFollowUpIssued,
    #[serde(rename = "MANUAL_UPLOAD_FOLLOW_UP_ISSUED")]
    ManualUploadFollowUpIssued,
    #[serde(rename = "ERROR")]
    Error,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxTranscriptsSchema {
    #[serde(rename = "years")]
    ///List of years associated with 4506T form
    pub years: Option<Vec<f64>>,
    #[serde(rename = "filingStatus")]
    ///Filing status
    pub filing_status: Option<String>,
    #[serde(rename = "filingAddress")]
    pub filing_address: Option<AddressSchema>,
    #[serde(rename = "filers")]
    pub filers: Option<Vec<TaxTranscriptFilerSchema>>,
}
impl std::fmt::Display for TaxTranscriptsSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct TaxTranscriptFilerSchema {
    #[serde(rename = "partyId")]
    ///The UUID of the party in Blend's system. Equivalent to borrowerId
    pub party_id: Option<String>,
    #[serde(rename = "name")]
    pub name: NameSchema,
}
impl std::fmt::Display for TaxTranscriptFilerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PackageType {
    #[serde(rename = "CLOSING_DISCLOSURE")]
    ClosingDisclosure,
    #[serde(rename = "CLOSING_PACKAGE")]
    ClosingPackage,
    #[serde(rename = "DOCUMENT_PACKAGE")]
    DocumentPackage,
    #[serde(rename = "INITIAL_LOAN_ESTIMATE")]
    InitialLoanEstimate,
    #[serde(rename = "OTHER_DISCLOSURE")]
    OtherDisclosure,
    #[serde(rename = "REVISED_CLOSING_DISCLOSURE")]
    RevisedClosingDisclosure,
    #[serde(rename = "REVISED_LOAN_ESTIMATE")]
    RevisedLoanEstimate,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackagePostSchema {
    #[serde(rename = "applicationId")]
    ///The Blend application UUID
    pub application_id: String,
    #[serde(rename = "description")]
    ///An optional description of the package
    pub description: Option<String>,
    #[serde(rename = "dueDate")]
    ///The ISO timestamp of when the package is due
    pub due_date: Option<String>,
    #[serde(rename = "envelopes")]
    ///An envelope describes the signing strategy and the recipient assignment within the package.
    pub envelopes: Vec<serde_json::Value>,
    #[serde(rename = "foreignPackageId")]
    ///External package id provided by partner
    pub foreign_package_id: Option<String>,
    #[serde(rename = "initiatedByLenderId")]
    pub initiated_by_lender_id: Option<String>,
    #[serde(rename = "name")]
    ///An optional name for the package
    pub name: Option<String>,
    #[serde(rename = "type")]
    ///Package type
    pub type_: String,
}
impl std::fmt::Display for PackagePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IframeSignEnvelopePostSchema {
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<IframesignEnvelopesRecipientPostSchema>,
}
impl std::fmt::Display for IframeSignEnvelopePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WetsignEnvelopePostSchema {
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<WetsignEnvelopesRecipientPostSchema>,
}
impl std::fmt::Display for WetsignEnvelopePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocgenEnvelopePostSchema {
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<DocgenEnvelopesRecipientPostSchema>,
    #[serde(rename = "signatureRequired")]
    pub signature_required: bool,
}
impl std::fmt::Display for DocgenEnvelopePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ESignEnvelopePostSchema {
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<ESignEnvelopesRecipientPostSchema>,
}
impl std::fmt::Display for ESignEnvelopePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewEnvelopePostSchema {
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<ReviewEnvelopeRecipientPostSchema>,
}
impl std::fmt::Display for ReviewEnvelopePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferEnvelopePostSchema {
    #[serde(rename = "providerType")]
    pub provider_type: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<OfferEnvelopeRecipientPostSchema>,
}
impl std::fmt::Display for OfferEnvelopePostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesRecipientPostSchema {
    #[serde(rename = "partyId")]
    ///The Blend party UUID
    pub party_id: String,
}
impl std::fmt::Display for EnvelopesRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecipientPartyType {
    #[serde(rename = "BORROWER")]
    Borrower,
    #[serde(rename = "LENDER")]
    Lender,
    #[serde(rename = "NON_APPLICANT_USER")]
    NonApplicantUser,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "SETTLOR")]
    Settlor,
    #[serde(rename = "SPOUSE")]
    Spouse,
    #[serde(rename = "TITLE_HOLDER")]
    TitleHolder,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RecipientStatus {
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "HAS_NOT_VIEWED")]
    HasNotViewed,
    #[serde(rename = "SIGNED")]
    Signed,
    #[serde(rename = "VIEWED")]
    Viewed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocgenEnvelopesRecipientPostSchema {
    #[serde(rename = "metadata")]
    pub metadata: DocgenMeatadataSchema,
    #[serde(rename = "partyId")]
    pub party_id: String,
}
impl std::fmt::Display for DocgenEnvelopesRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocgenMeatadataSchema {
    #[serde(rename = "docLink")]
    pub doc_link: String,
}
impl std::fmt::Display for DocgenMeatadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ESignEnvelopesRecipientPostSchema {
    #[serde(rename = "metadata")]
    pub metadata: EsignMetadataSchema,
    #[serde(rename = "partyId")]
    ///The Blend party UUID, but for lender recipients, the partyId must be set to 'LENDER'.
    pub party_id: String,
}
impl std::fmt::Display for ESignEnvelopesRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IframesignEnvelopesRecipientPostSchema {
    #[serde(rename = "metadata")]
    pub metadata: IfamesignPostMetadataSchema,
    #[serde(rename = "partyId")]
    pub party_id: String,
}
impl std::fmt::Display for IframesignEnvelopesRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WetsignEnvelopesRecipientPostSchema {
    #[serde(rename = "metadata")]
    pub metadata: WetsignPostMetadataSchema,
    #[serde(rename = "partyId")]
    pub party_id: String,
}
impl std::fmt::Display for WetsignEnvelopesRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewEnvelopeRecipientPostSchema {
    #[serde(rename = "metadata")]
    pub metadata: ReviewPostMetadataSchema,
    #[serde(rename = "partyId")]
    pub party_id: String,
}
impl std::fmt::Display for ReviewEnvelopeRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferEnvelopeRecipientPostSchema {
    #[serde(rename = "metadata")]
    pub metadata: OfferPostMetadataSchema,
    #[serde(rename = "partyId")]
    pub party_id: String,
}
impl std::fmt::Display for OfferEnvelopeRecipientPostSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopesRecipientResponseSchema {
    #[serde(rename = "declinedDate")]
    pub declined_date: Option<String>,
    #[serde(rename = "declinedIp")]
    pub declined_ip: Option<String>,
    #[serde(rename = "deliveredDate")]
    pub delivered_date: Option<String>,
    #[serde(rename = "metadata")]
    pub metadata: serde_json::Value,
    #[serde(rename = "partyId")]
    ///The Blend party UUID
    pub party_id: String,
    #[serde(rename = "partyType")]
    pub party_type: Option<String>,
    #[serde(rename = "receivedIp")]
    pub received_ip: Option<String>,
    #[serde(rename = "signedDate")]
    pub signed_date: Option<String>,
    #[serde(rename = "signedIp")]
    pub signed_ip: Option<String>,
    #[serde(rename = "status")]
    pub status: Option<String>,
}
impl std::fmt::Display for EnvelopesRecipientResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IfamesignPostMetadataSchema {
    #[serde(rename = "closingId")]
    pub closing_id: String,
    #[serde(rename = "documents")]
    pub documents: Vec<serde_json::Value>,
}
impl std::fmt::Display for IfamesignPostMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WetsignPostMetadataSchema {
    #[serde(rename = "documents")]
    ///List of Blend documents needed to be signed
    pub documents: Vec<serde_json::Value>,
}
impl std::fmt::Display for WetsignPostMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct WetsignResponseMetadataSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponseMetadataSchema {}
impl std::fmt::Display for EmptyResponseMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignResponseMetadataSchema {
    #[serde(rename = "documents")]
    pub documents: Vec<serde_json::Value>,
    #[serde(rename = "loanId")]
    pub loan_id: String,
}
impl std::fmt::Display for EsignResponseMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignMetadataSchema {
    #[serde(rename = "documents")]
    pub documents: Vec<serde_json::Value>,
}
impl std::fmt::Display for EsignMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ESignDocumentWithTabs {
    #[serde(rename = "documentId")]
    pub document_id: String,
    #[serde(rename = "eSignTemplateId")]
    pub e_sign_template_id: Option<String>,
    #[serde(rename = "signersRequired")]
    pub signers_required: Option<String>,
    #[serde(rename = "tabs")]
    pub tabs: Vec<serde_json::Value>,
}
impl std::fmt::Display for ESignDocumentWithTabs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ESignDocumentWithTabExtractionMethod {
    #[serde(rename = "documentId")]
    pub document_id: String,
    #[serde(rename = "eSignTemplateId")]
    pub e_sign_template_id: Option<String>,
    #[serde(rename = "extractTabs")]
    ///If additional information is required for tab extraction (which it is for EXPERE_ACROFORM documents), include it in extractTabs.
    pub extract_tabs: Option<Vec<serde_json::Value>>,
    #[serde(rename = "signersRequired")]
    pub signers_required: Option<String>,
    #[serde(rename = "tabExtractionMethod")]
    ///If tabs must be extracted from embedded fields, specify the extraction method to use.
    pub tab_extraction_method: String,
}
impl std::fmt::Display for ESignDocumentWithTabExtractionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignFontTabAttributesSchema {
    #[serde(rename = "bold")]
    pub bold: Option<bool>,
    #[serde(rename = "font")]
    pub font: Option<String>,
    #[serde(rename = "fontColor")]
    pub font_color: Option<String>,
    #[serde(rename = "fontSize")]
    pub font_size: Option<String>,
}
impl std::fmt::Display for EsignFontTabAttributesSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignBaseTabAttributesSchema {
    #[serde(rename = "attributes")]
    pub attributes: serde_json::Value,
}
impl std::fmt::Display for EsignBaseTabAttributesSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignSignHereSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignApproveSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignDateSignedSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignCheckboxSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ESignCheckboxGroupTabSchema {
    #[serde(rename = "attributes")]
    pub attributes: serde_json::Value,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for ESignCheckboxGroupTabSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckboxGroupOtherTabAttributesSchema {
    #[serde(rename = "checkboxes")]
    pub checkboxes: Option<Vec<CheckboxInCheckboxGroupTabSchema>>,
    #[serde(rename = "groupName")]
    pub group_name: Option<String>,
    #[serde(rename = "maximumAllowed")]
    pub maximum_allowed: Option<f64>,
    #[serde(rename = "minimumRequired")]
    pub minimum_required: Option<f64>,
    #[serde(rename = "validationRule")]
    pub validation_rule: Option<String>,
}
impl std::fmt::Display for CheckboxGroupOtherTabAttributesSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CheckboxInCheckboxGroupTabSchema {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "pageNumber")]
    pub page_number: f64,
    #[serde(rename = "selected")]
    pub selected: Option<bool>,
    #[serde(rename = "tabLabel")]
    pub tab_label: Option<String>,
    #[serde(rename = "xPosition")]
    pub x_position: f64,
    #[serde(rename = "yPosition")]
    pub y_position: f64,
}
impl std::fmt::Display for CheckboxInCheckboxGroupTabSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignTextSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignListSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignRadioGroupSchema {
    #[serde(rename = "attributes")]
    pub attributes: serde_json::Value,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for EsignRadioGroupSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignRadioSchema {
    #[serde(rename = "pageNumber")]
    pub page_number: f64,
    #[serde(rename = "selected")]
    pub selected: Option<bool>,
    #[serde(rename = "tabId")]
    pub tab_id: Option<String>,
    #[serde(rename = "value")]
    pub value: Option<String>,
    #[serde(rename = "xPosition")]
    pub x_position: f64,
    #[serde(rename = "yPosition")]
    pub y_position: f64,
}
impl std::fmt::Display for EsignRadioSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignInitialHereSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignDateSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignNoteSchema(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignExtractTabExpereAcroformSchema {
    #[serde(rename = "fieldName")]
    ///The name of the acroField from which to extract an Esign tab
    pub field_name: String,
    #[serde(rename = "required")]
    ///True if the tab is required to be completed by the recipient in Docusign, false if it is optional. Defaults to true.
    pub required: Option<bool>,
}
impl std::fmt::Display for EsignExtractTabExpereAcroformSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewPostMetadataSchema {
    #[serde(rename = "documents")]
    pub documents: Vec<serde_json::Value>,
}
impl std::fmt::Display for ReviewPostMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewResponseMetadataSchema {
    #[serde(rename = "documents")]
    pub documents: Vec<ReviewDocument>,
}
impl std::fmt::Display for ReviewResponseMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewDocument {
    #[serde(rename = "description")]
    pub description: Option<String>,
    #[serde(rename = "documentId")]
    ///The Blend UUID returned when uploading a document
    pub document_id: String,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "reviewTracking")]
    pub review_tracking: Option<ReviewTracking>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for ReviewDocument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ReviewTracking {
    #[serde(rename = "ip")]
    pub ip: String,
    #[serde(rename = "timestamp")]
    ///unix timestamp
    pub timestamp: f64,
}
impl std::fmt::Display for ReviewTracking {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OfferPostMetadataSchema {
    #[serde(rename = "termDetails")]
    pub term_details: Option<Vec<serde_json::Value>>,
    #[serde(rename = "documents")]
    ///The documents to be attached to the offer
    pub documents: Vec<serde_json::Value>,
}
impl std::fmt::Display for OfferPostMetadataSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageResponseSchema {
    #[serde(rename = "applicationId")]
    ///The Blend application UUID
    pub application_id: String,
    #[serde(rename = "createdDate")]
    pub created_date: Option<String>,
    #[serde(rename = "description")]
    ///An optional description of the package
    pub description: Option<String>,
    #[serde(rename = "dueDate")]
    ///The ISO timestamp of when the lender has indicated a package is due
    pub due_date: Option<String>,
    #[serde(rename = "envelopes")]
    ///An envelope describes the signing strategy and the recipient assignment within the package
    pub envelopes: Vec<EnvelopeResponseSchema>,
    #[serde(rename = "foreignPackageId")]
    ///External package id provided by partner
    pub foreign_package_id: Option<String>,
    #[serde(rename = "id")]
    ///The Blend package UUID
    pub id: String,
    #[serde(rename = "name")]
    ///A name for the package
    pub name: Option<String>,
    #[serde(rename = "paperedAt")]
    ///The ISO timestamp of when the lender decides to mail a package and not pursue electronic delivery
    pub papered_at: Option<String>,
    #[serde(rename = "requestedBy")]
    pub requested_by: Option<String>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    ///Package type
    pub type_: String,
}
impl std::fmt::Display for PackageResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum PackageStatus {
    #[serde(rename = "CANCELLED")]
    Cancelled,
    #[serde(rename = "COMPLETED")]
    Completed,
    #[serde(rename = "CREATED")]
    Created,
    #[serde(rename = "DECLINED")]
    Declined,
    #[serde(rename = "DELIVERED")]
    Delivered,
    #[serde(rename = "FAILED_TO_CREATE")]
    FailedToCreate,
    #[serde(rename = "RETRIEVED")]
    Retrieved,
    #[serde(rename = "SIGNED")]
    Signed,
    #[serde(rename = "VIEWED")]
    Viewed,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EnvelopeResponseSchema {
    #[serde(rename = "id")]
    ///The Blend envelope UUID
    pub id: String,
    #[serde(rename = "providerId")]
    ///A Blend provided UUID for providerType ESIGN, SSO_SAML, and DIRECT_LINK envelopes
    pub provider_id: String,
    #[serde(rename = "providerType")]
    /**Different signing strategies: * DIRECT_LINK: The recipient uses a Blend Sign related URL link that may contain an access token to login for signing.
* ESIGN: A Docusign integration.
* IFRAME_SIGN: A signing strategy used in close service.
* REVIEW: The recipient views the documents in Blend's document viewer.
* SSO_SAML: Uses SSO to authenticate the recipient before acting on disclosures.
* WETSIGN: The recipient downloads the package as a PDF and uploads signed PDFs into Blend.
* OFFER: The recipient is given option to accept/decline the package containing term details and documents.
*/
    pub provider_type: String,
    #[serde(rename = "signatureRequired")]
    pub signature_required: bool,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "recipients")]
    pub recipients: Vec<EnvelopesRecipientResponseSchema>,
}
impl std::fmt::Display for EnvelopeResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AssetsRefreshRequestSchema {
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "accountIds")]
    ///These are the accounts that a refresh will be initiated for. When an applicationId is provided, the accounts refreshed must be part of the specified application.
    pub account_ids: Vec<String>,
}
impl std::fmt::Display for AssetsRefreshRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Closing(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingBaseRecord {
    #[serde(rename = "id")]
    ///Record ID
    pub id: String,
    #[serde(rename = "createdAt")]
    ///Creation Date
    pub created_at: String,
}
impl std::fmt::Display for ClosingBaseRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClosingType {
    #[serde(rename = "RON")]
    Ron,
    #[serde(rename = "HYBRID")]
    Hybrid,
    #[serde(rename = "TRADITIONAL")]
    Traditional,
    #[serde(rename = "UNDETERMINED")]
    Undetermined,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingPartyPatchRequest {
    #[serde(rename = "email")]
    ///Email address of the closing party
    pub email: String,
    #[serde(rename = "partyType")]
    ///The closing party type
    pub party_type: Option<String>,
    #[serde(rename = "firstName")]
    ///First name of the closing party
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    ///Last name of the closing party
    pub last_name: Option<String>,
    #[serde(rename = "phoneNumber")]
    ///Phone number of the closing party
    pub phone_number: Option<String>,
    #[serde(rename = "agencyName")]
    ///Name of the closing party's agency
    pub agency_name: Option<String>,
}
impl std::fmt::Display for ClosingPartyPatchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClosingPartyType {
    #[serde(rename = "SETTLEMENT")]
    Settlement,
    #[serde(rename = "TITLE")]
    Title,
    #[serde(rename = "ATTORNEY")]
    Attorney,
    #[serde(rename = "CLOSER")]
    Closer,
    #[serde(rename = "SUPPORTING_PARTY")]
    SupportingParty,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingParty(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingAgency(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingDocumentReference(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingPackageReference {
    #[serde(rename = "packageID")]
    ///Package ID
    pub package_id: String,
    #[serde(rename = "packageType")]
    ///The type of the closing package
    pub package_type: String,
    #[serde(rename = "createdAt")]
    ///The UTC timestamp of the closing package creation. ISO format
    pub created_at: String,
    #[serde(rename = "details")]
    pub details: serde_json::Value,
}
impl std::fmt::Display for ClosingPackageReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingDocumentRecipients {
    #[serde(rename = "partyId")]
    ///Party UUID of signer.
    pub party_id: String,
    #[serde(rename = "tabExtractionMethod")]
    ///If omitted, the document is considered view-only for this party. For eSign documents where tabs are expected to be added manually, use value `MANUAL` for `tabExtractionMethod`.
    pub tab_extraction_method: Option<String>,
    #[serde(rename = "extractTabs")]
    ///If additional information is required for tab extraction (which it is for `EXPERE_ACROFORM` documents), include it in `extractTabs`.
    pub extract_tabs: Option<Vec<EsignExtractTabExpereAcroformSchema>>,
    #[serde(rename = "tabs")]
    pub tabs: Option<Vec<serde_json::Value>>,
}
impl std::fmt::Display for ClosingDocumentRecipients {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClosingSourceType {
    #[serde(rename = "LENDER")]
    Lender,
    #[serde(rename = "TITLE")]
    Title,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClosingDesignationRequest {
    #[serde(rename = "SIGN_AHEAD")]
    SignAhead,
    #[serde(rename = "DAY_OF")]
    DayOf,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "PRE_CLOSING")]
    PreClosing,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClosingDesignationResponse {
    #[serde(rename = "SIGN_AHEAD")]
    SignAhead,
    #[serde(rename = "DAY_OF")]
    DayOf,
    #[serde(rename = "NOTE")]
    Note,
    #[serde(rename = "PRE_CLOSING")]
    PreClosing,
    #[serde(rename = "DRAFT_PRE_CLOSING")]
    DraftPreClosing,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ClosingPackageTypeResponse {
    #[serde(rename = "ESIGN")]
    Esign,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum EsignClosingPackageStatusResponse {
    #[serde(rename = "COMPLETE")]
    Complete,
    #[serde(rename = "INCOMPLETE")]
    Incomplete,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EsignClosingPackageDetailsResponse {
    #[serde(rename = "closingPackageStatus")]
    ///The closing status of the esign package
    pub closing_package_status: String,
}
impl std::fmt::Display for EsignClosingPackageDetailsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingList(pub Vec<Closing>);
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingPostRequest {
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
}
impl std::fmt::Display for ClosingPostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingPatchRequest {
    #[serde(rename = "closingType")]
    ///The type of closing
    pub closing_type: Option<String>,
    #[serde(rename = "closingStart")]
    ///ISO 8601 datetime when the closing is set to start. This time will dictate when sign-ahead closing documents will be released to borrower(s) for eSign. A timezone offset **MUST** be provided in the date string to ensure sign ahead documents are released accurately for borrower(s), e.g., not a day earlier than expected.
    pub closing_start: Option<String>,
    #[serde(rename = "closingEnd")]
    ///ISO 8601 datetime when the closing is set to end. Borrower(s) will not be able to review or eSign closing documents after this time. A timezone offset **MUST** be provided in the date string to ensure closing documents expire at the correct time for borrower(s).
    pub closing_end: Option<String>,
    #[serde(rename = "scheduledCeremonyTime")]
    ///ISO 8601 datetime that represents a closing "day", which will be reflected in Settlement Agent Workspace and will be assumed to be the day when a borrower meets with their notary. If this value is not provided, it will have the same value as closingStart. This value should be before closingEnd.
    pub scheduled_ceremony_time: Option<String>,
    #[serde(rename = "timezone")]
    /**IMPORTANT - this field is only used by the browser in the webapp and does not effect the closing start time declared in this request.
Additionally, closingStart and closingEnd must include the correct timezone offset.

For example if you wish to use Central time, the timezone value would be "America/Chicago" and the closingStart or closingDate value must
include an offset of "-06:00" during standard time i.e. "2020-12-12T00:00:00-06:00" or "-05:00" during daylight savings time "2021-06-12T00:00:00-05:00"

Options include:

"America/New_York" (Eastern time)

"America/Chicago" (Central time)

"America/Denver" (Mountain time - day light savings time applicable)

"America/Phoenix" (Arizona time - day light savings not applicable)

"America/Los_Angeles" (Pacific time)

"America/Anchorage" (Alaska time)

"Pacific/Honolulu" (Hawaii time)
*/
    pub timezone: Option<String>,
    #[serde(rename = "closingParties")]
    ///List of closing parties
    pub closing_parties: Option<Vec<ClosingPartyPatchRequest>>,
    #[serde(rename = "specialInstructions")]
    ///Special closing instructions
    pub special_instructions: Option<String>,
    #[serde(rename = "eNoteSignStart")]
    ///ISO 8601 format. The time component needs to be at midnight. Optional. Send as null to clear out the existing value and use defaults.
    pub e_note_sign_start: Option<String>,
    #[serde(rename = "eNoteSignEnd")]
    ///ISO 8601 format. The time component needs to be at end of day 23:59:59. Optional. Send as null to clear out the existing value and use defaults.
    pub e_note_sign_end: Option<String>,
}
impl std::fmt::Display for ClosingPatchRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingDocumentsPutRequest {
    #[serde(rename = "documents")]
    pub documents: Vec<ClosingDocumentReferenceRequest>,
}
impl std::fmt::Display for ClosingDocumentsPutRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingDocumentReferenceRequest {
    #[serde(rename = "documentID")]
    ///Blend Document ID
    pub document_id: String,
    #[serde(rename = "sourceType")]
    ///The source type
    pub source_type: String,
    #[serde(rename = "closingDesignation")]
    ///The document closing designation accepted by the API. If `PRE_CLOSING` is provided, then a `sourceType` of `LENDER` is not allowed; it must be `TITLE` to indicate the addition of a pre-closing document by the title/settlement agent.
    pub closing_designation: Option<String>,
    #[serde(rename = "requireNotarization")]
    ///Whether the document requires notarization.
    pub require_notarization: Option<bool>,
    #[serde(rename = "signersRequired")]
    ///The signers required for this document. If provided, you must also provide `recipients`.
    pub signers_required: Option<String>,
    #[serde(rename = "recipients")]
    pub recipients: Option<Vec<ClosingDocumentRecipients>>,
}
impl std::fmt::Display for ClosingDocumentReferenceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PreClosingDocumentsPutRequest {
    #[serde(rename = "documents")]
    pub documents: Vec<PreClosingDocumentReferenceRequest>,
}
impl std::fmt::Display for PreClosingDocumentsPutRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct PreClosingDocumentReferenceRequest {
    #[serde(rename = "documentID")]
    ///Blend Document ID
    pub document_id: String,
}
impl std::fmt::Display for PreClosingDocumentReferenceRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingCreatedResponse {
    #[serde(rename = "id")]
    ///Closing ID
    pub id: String,
}
impl std::fmt::Display for ClosingCreatedResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingNotFoundError {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for ClosingNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingSendPostRequest {
    #[serde(rename = "sourceType")]
    ///The source type
    pub source_type: String,
}
impl std::fmt::Display for ClosingSendPostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingExternallyManagedENotePostRequest {
    #[serde(rename = "vaultData")]
    pub vault_data: VaultData,
}
impl std::fmt::Display for ClosingExternallyManagedENotePostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VaultData {}
impl std::fmt::Display for VaultData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EOriginalVault {
    #[serde(rename = "eOriginal")]
    pub e_original: serde_json::Value,
}
impl std::fmt::Display for EOriginalVault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ClosingENotePostRequest(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub enum BlendENoteArmLoanTypeFormId {
    #[serde(rename = "3501e")]
    BlendENoteArmLoanTypeFormId3501E,
    #[serde(rename = "3501.02e")]
    BlendENoteArmLoanTypeFormId350102E,
    #[serde(rename = "3501.10e")]
    BlendENoteArmLoanTypeFormId350110E,
    #[serde(rename = "3501.30e")]
    BlendENoteArmLoanTypeFormId350130E,
    #[serde(rename = "3501.39e")]
    BlendENoteArmLoanTypeFormId350139E,
    #[serde(rename = "3501.46e")]
    BlendENoteArmLoanTypeFormId350146E,
    #[serde(rename = "3501.47e")]
    BlendENoteArmLoanTypeFormId350147E,
    #[serde(rename = "3501.49e")]
    BlendENoteArmLoanTypeFormId350149E,
    #[serde(rename = "3501.50e")]
    BlendENoteArmLoanTypeFormId350150E,
    #[serde(rename = "3502e")]
    BlendENoteArmLoanTypeFormId3502E,
    #[serde(rename = "3502.02e")]
    BlendENoteArmLoanTypeFormId350202E,
    #[serde(rename = "3502.10e")]
    BlendENoteArmLoanTypeFormId350210E,
    #[serde(rename = "3502.30e")]
    BlendENoteArmLoanTypeFormId350230E,
    #[serde(rename = "3502.39e")]
    BlendENoteArmLoanTypeFormId350239E,
    #[serde(rename = "3502.46e")]
    BlendENoteArmLoanTypeFormId350246E,
    #[serde(rename = "3502.47e")]
    BlendENoteArmLoanTypeFormId350247E,
    #[serde(rename = "3502.49e")]
    BlendENoteArmLoanTypeFormId350249E,
    #[serde(rename = "3502.50e")]
    BlendENoteArmLoanTypeFormId350250E,
    #[serde(rename = "3504e")]
    BlendENoteArmLoanTypeFormId3504E,
    #[serde(rename = "3522e")]
    BlendENoteArmLoanTypeFormId3522E,
    #[serde(rename = "3528e")]
    BlendENoteArmLoanTypeFormId3528E,
    #[serde(rename = "3528e-ak")]
    BlendENoteArmLoanTypeFormId3528EAk,
    #[serde(rename = "3528e-fl")]
    BlendENoteArmLoanTypeFormId3528EFl,
    #[serde(rename = "3528e-nh")]
    BlendENoteArmLoanTypeFormId3528ENh,
    #[serde(rename = "3528e-pa")]
    BlendENoteArmLoanTypeFormId3528EPa,
    #[serde(rename = "3528e-tx")]
    BlendENoteArmLoanTypeFormId3528ETx,
    #[serde(rename = "3528e-va")]
    BlendENoteArmLoanTypeFormId3528EVa,
    #[serde(rename = "3528e-vt")]
    BlendENoteArmLoanTypeFormId3528EVt,
    #[serde(rename = "3528e-wi")]
    BlendENoteArmLoanTypeFormId3528EWi,
    #[serde(rename = "3528e-wv")]
    BlendENoteArmLoanTypeFormId3528EWv,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BlendENoteFixedLoanTypeFormId {
    #[serde(rename = "3200e")]
    BlendENoteFixedLoanTypeFormId3200E,
    #[serde(rename = "3200e-usda")]
    BlendENoteFixedLoanTypeFormId3200EUsda,
    #[serde(rename = "3200e-fha")]
    BlendENoteFixedLoanTypeFormId3200EFha,
    #[serde(rename = "3200e-fha-ak")]
    BlendENoteFixedLoanTypeFormId3200EFhaAk,
    #[serde(rename = "3200e-fha-fl")]
    BlendENoteFixedLoanTypeFormId3200EFhaFl,
    #[serde(rename = "3200e-fha-ia")]
    BlendENoteFixedLoanTypeFormId3200EFhaIa,
    #[serde(rename = "3200e-fha-mo")]
    BlendENoteFixedLoanTypeFormId3200EFhaMo,
    #[serde(rename = "3200e-fha-nh")]
    BlendENoteFixedLoanTypeFormId3200EFhaNh,
    #[serde(rename = "3200e-fha-pa")]
    BlendENoteFixedLoanTypeFormId3200EFhaPa,
    #[serde(rename = "3200e-fha-va")]
    BlendENoteFixedLoanTypeFormId3200EFhaVa,
    #[serde(rename = "3200e-fha-vt")]
    BlendENoteFixedLoanTypeFormId3200EFhaVt,
    #[serde(rename = "3200e-fha-wi")]
    BlendENoteFixedLoanTypeFormId3200EFhaWi,
    #[serde(rename = "3200e-fha-wv")]
    BlendENoteFixedLoanTypeFormId3200EFhaWv,
    #[serde(rename = "3200e-va")]
    BlendENoteFixedLoanTypeFormId3200EVa,
    #[serde(rename = "3200e-va2")]
    BlendENoteFixedLoanTypeFormId3200EVa2,
    #[serde(rename = "3200e-va-ak")]
    BlendENoteFixedLoanTypeFormId3200EVaAk,
    #[serde(rename = "3200e-va-ca")]
    BlendENoteFixedLoanTypeFormId3200EVaCa,
    #[serde(rename = "3200e-va-fl")]
    BlendENoteFixedLoanTypeFormId3200EVaFl,
    #[serde(rename = "3200e-va-la")]
    BlendENoteFixedLoanTypeFormId3200EVaLa,
    #[serde(rename = "3200e-va-me")]
    BlendENoteFixedLoanTypeFormId3200EVaMe,
    #[serde(rename = "3200e-va-nh")]
    BlendENoteFixedLoanTypeFormId3200EVaNh,
    #[serde(rename = "3200e-va-ny")]
    BlendENoteFixedLoanTypeFormId3200EVaNy,
    #[serde(rename = "3200e-va-pa")]
    BlendENoteFixedLoanTypeFormId3200EVaPa,
    #[serde(rename = "3200e-va-tx")]
    BlendENoteFixedLoanTypeFormId3200EVaTx,
    #[serde(rename = "3200e-va-va")]
    BlendENoteFixedLoanTypeFormId3200EVaVa,
    #[serde(rename = "3200e-va-vt")]
    BlendENoteFixedLoanTypeFormId3200EVaVt,
    #[serde(rename = "3200e-va-wi")]
    BlendENoteFixedLoanTypeFormId3200EVaWi,
    #[serde(rename = "3200e-va-wv")]
    BlendENoteFixedLoanTypeFormId3200EVaWv,
    #[serde(rename = "3200e-va-dc")]
    BlendENoteFixedLoanTypeFormId3200EVaDc,
    #[serde(rename = "3200e-in")]
    BlendENoteFixedLoanTypeFormId3200EIn,
    #[serde(rename = "3200e-md")]
    BlendENoteFixedLoanTypeFormId3200EMd,
    #[serde(rename = "3200e-tx")]
    BlendENoteFixedLoanTypeFormId3200ETx,
    #[serde(rename = "3200e-al")]
    BlendENoteFixedLoanTypeFormId3200EAl,
    #[serde(rename = "3200e-al2")]
    BlendENoteFixedLoanTypeFormId3200EAl2,
    #[serde(rename = "3202e")]
    BlendENoteFixedLoanTypeFormId3202E,
    #[serde(rename = "3210e")]
    BlendENoteFixedLoanTypeFormId3210E,
    #[serde(rename = "3220e")]
    BlendENoteFixedLoanTypeFormId3220E,
    #[serde(rename = "3230e")]
    BlendENoteFixedLoanTypeFormId3230E,
    #[serde(rename = "3233e")]
    BlendENoteFixedLoanTypeFormId3233E,
    #[serde(rename = "3239e")]
    BlendENoteFixedLoanTypeFormId3239E,
    #[serde(rename = "3244.1e")]
    BlendENoteFixedLoanTypeFormId32441E,
    #[serde(rename = "3246e")]
    BlendENoteFixedLoanTypeFormId3246E,
    #[serde(rename = "3247e")]
    BlendENoteFixedLoanTypeFormId3247E,
    #[serde(rename = "3249e")]
    BlendENoteFixedLoanTypeFormId3249E,
    #[serde(rename = "3250e")]
    BlendENoteFixedLoanTypeFormId3250E,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EOriginalENoteLoanType {
    #[serde(rename = "lateChargeMaximumAmount")]
    ///The maximum monthly late charge amount allowed.
    pub late_charge_maximum_amount: Option<f64>,
    #[serde(rename = "conformingYearType")]
    pub conforming_year_type: Option<String>,
    #[serde(rename = "registryOperator")]
    pub registry_operator: Option<String>,
}
impl std::fmt::Display for EOriginalENoteLoanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EOriginalENoteArmLoanType(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct EOriginalENoteFixedLoanType(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocMagicENoteLoanType {
    #[serde(rename = "loanAmortizationPeriodCount")]
    ///The number periods (as defined by the Loan Amortization Period Type) over which the scheduled loan payments of principal and/or interest are calculated to retire the obligation. Minimum is 0, maximum is 2147483647.
    pub loan_amortization_period_count: i64,
    #[serde(rename = "loanAmortizationPeriodType")]
    ///The duration of time used to define the period over which the loan is amortized.
    pub loan_amortization_period_type: String,
    #[serde(rename = "paymentFrequencyType")]
    ///Specifies the frequency of the mortgage payment.
    pub payment_frequency_type: String,
    #[serde(rename = "baseLoanAmount")]
    ///The base loan amount to be loaned to the borrower not including PMI, MIP, or Funding Fee.
    pub base_loan_amount: Option<f64>,
}
impl std::fmt::Display for DocMagicENoteLoanType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DocMagicENoteArmLoanType(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct DocMagicENoteFixedLoanType(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteArmLoanType(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteBaseLoan {
    #[serde(rename = "MIN")]
    ///Mortgage Identification Number. 18-digit number passed as a string whose last digit is a control digit. Required in production environment.
    pub min: Option<String>,
    #[serde(rename = "lenderLoanId")]
    ///The identifier assigned by the originating Lender to be referenced as the Loan ID/Number on all settlement documents, notes, riders, etc.
    pub lender_loan_id: Option<String>,
    #[serde(rename = "executionDate")]
    pub execution_date: String,
    #[serde(rename = "executionCity")]
    ///The name of the city.
    pub execution_city: String,
    #[serde(rename = "executionState")]
    ///The two-character representation of the US state, US Territory, Canadian Province, Military APO FPO, or Territory.
    pub execution_state: String,
    #[serde(rename = "propertyAddress")]
    ///The property address will be pulled from Blend if not specified.
    pub property_address: Option<serde_json::Value>,
    #[serde(rename = "scheduledFirstPaymentDate")]
    ///The month and year of the first scheduled mortgage payment to be made by the borrower under the terms of the mortgage. Must be in YYYY-MM format. The first day of the month is assumed.
    pub scheduled_first_payment_date: String,
    #[serde(rename = "notePayToAddress")]
    pub note_pay_to_address: serde_json::Value,
    #[serde(rename = "lienPriorityType")]
    ///Specifies the priority of the lien against the subject property.
    pub lien_priority_type: Option<String>,
    #[serde(rename = "mortgageType")]
    ///Identifies the highest level private or public sector entity under whose guidelines the mortgage is originated. The mortgage type will be pulled from Blend if not specified.
    pub mortgage_type: Option<String>,
    #[serde(rename = "loanPurposeType")]
    ///Specifies the purpose for which the loan proceeds will be used. The loan purpose type will be pulled from Blend if not specified.
    pub loan_purpose_type: Option<String>,
    #[serde(rename = "noteRatePercent")]
    pub note_rate_percent: f64,
    #[serde(rename = "originalLoanAmount")]
    ///The amount to be repaid as disclosed on the Note. Will be pulled from Blend if not specified.
    pub original_loan_amount: Option<f64>,
    #[serde(rename = "loanMaturityDate")]
    ///The date when the loan is scheduled to be paid in full as reflected on the Note.
    pub loan_maturity_date: String,
    #[serde(rename = "originalPrincipalAndInterestPaymentAmount")]
    ///The dollar amount of the principal and interest payment as stated on the Note. The principal and interest payment is usually obtained using the loan amount and interest rate to arrive at full amortization during the loan term.
    pub original_principal_and_interest_payment_amount: f64,
    #[serde(rename = "lateChargeGracePeriod")]
    ///The grace period in days for this loan before a late charge will be applied.
    pub late_charge_grace_period: i64,
    #[serde(rename = "lateChargeRate")]
    pub late_charge_rate: f64,
    #[serde(rename = "parties")]
    pub parties: BlendENoteParties,
}
impl std::fmt::Display for BlendENoteBaseLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteAddressComponentType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum BlendENoteAddressUnitDesignatorType {
    #[serde(rename = "Apartment")]
    Apartment,
    #[serde(rename = "Basement")]
    Basement,
    #[serde(rename = "Building")]
    Building,
    #[serde(rename = "Condo")]
    Condo,
    #[serde(rename = "Department")]
    Department,
    #[serde(rename = "Floor")]
    Floor,
    #[serde(rename = "Front")]
    Front,
    #[serde(rename = "Hanger")]
    Hanger,
    #[serde(rename = "Key")]
    Key,
    #[serde(rename = "Lobby")]
    Lobby,
    #[serde(rename = "Lot")]
    Lot,
    #[serde(rename = "Lower")]
    Lower,
    #[serde(rename = "Office")]
    Office,
    #[serde(rename = "Other")]
    Other,
    #[serde(rename = "Penthouse")]
    Penthouse,
    #[serde(rename = "Pier")]
    Pier,
    #[serde(rename = "Rear")]
    Rear,
    #[serde(rename = "Room")]
    Room,
    #[serde(rename = "Side")]
    Side,
    #[serde(rename = "Space")]
    Space,
    #[serde(rename = "Stop")]
    Stop,
    #[serde(rename = "Suite")]
    Suite,
    #[serde(rename = "Trailer")]
    Trailer,
    #[serde(rename = "Unit")]
    Unit,
    #[serde(rename = "Upper")]
    Upper,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteAgencyCaseIdentifierType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteAgent {
    #[serde(rename = "contactName")]
    pub contact_name: Option<String>,
    #[serde(rename = "postalAddress")]
    pub postal_address: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "unparsedName")]
    pub unparsed_name: String,
}
impl std::fmt::Display for BlendENoteAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum BlendENoteAgentType {
    #[serde(rename = "Attorney")]
    Attorney,
    #[serde(rename = "ClosingAgent")]
    ClosingAgent,
    #[serde(rename = "EscrowCompany")]
    EscrowCompany,
    #[serde(rename = "SettlementAgent")]
    SettlementAgent,
    #[serde(rename = "TitleCompany")]
    TitleCompany,
    #[serde(rename = "Other")]
    Other,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteArmLoan {
    #[serde(rename = "armLoan")]
    pub arm_loan: BlendENoteArmLoanType,
}
impl std::fmt::Display for BlendENoteArmLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteCityStateZip {
    #[serde(rename = "city")]
    ///The name of the city.
    pub city: String,
    #[serde(rename = "postalCode")]
    ///The postal code (ZIP Code in the US) for the address. ZIP Code may be either 5 or 9 digits.
    pub postal_code: String,
    #[serde(rename = "state")]
    ///The two-character representation of the US state, US Territory, Canadian Province, Military APO FPO, or Territory.
    pub state: String,
}
impl std::fmt::Display for BlendENoteCityStateZip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteCityType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ExecutionCityType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum ExecutionStateType {
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AK")]
    Ak,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CT")]
    Ct,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DC")]
    Dc,
    #[serde(rename = "FL")]
    Fl,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "HI")]
    Hi,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IA")]
    Ia,
    #[serde(rename = "KS")]
    Ks,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MI")]
    Mi,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NV")]
    Nv,
    #[serde(rename = "NH")]
    Nh,
    #[serde(rename = "NJ")]
    Nj,
    #[serde(rename = "NM")]
    Nm,
    #[serde(rename = "NY")]
    Ny,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "ND")]
    Nd,
    #[serde(rename = "OH")]
    Oh,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "RI")]
    Ri,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TX")]
    Tx,
    #[serde(rename = "UT")]
    Ut,
    #[serde(rename = "VT")]
    Vt,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "WA")]
    Wa,
    #[serde(rename = "WV")]
    Wv,
    #[serde(rename = "WI")]
    Wi,
    #[serde(rename = "WY")]
    Wy,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteParties(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteContactNameType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteCountyType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteFixedLoan {
    #[serde(rename = "fixedLoan")]
    pub fixed_loan: BlendENoteFixedLoanType,
}
impl std::fmt::Display for BlendENoteFixedLoan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteFixedLoanType(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteLender {
    #[serde(rename = "contactName")]
    pub contact_name: String,
    #[serde(rename = "nmlsId")]
    ///Nationwide Multistate Licensing System ID number. Minimum is 1, maximum is 2147483647.
    pub nmls_id: i64,
    #[serde(rename = "unparsedName")]
    pub unparsed_name: String,
}
impl std::fmt::Display for BlendENoteLender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteLoanParty {
    #[serde(rename = "nmlsId")]
    ///Nationwide Multistate Licensing System ID number. Minimum is 1, maximum is 2147483647.
    pub nmls_id: i64,
    #[serde(rename = "unparsedName")]
    pub unparsed_name: String,
    #[serde(rename = "phoneNumber")]
    ///String representation of the phone number
    pub phone_number: String,
    #[serde(rename = "email")]
    ///Party email
    pub email: String,
}
impl std::fmt::Display for BlendENoteLoanParty {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteNmlsrid(pub i64);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteParsedStreetAddress {
    #[serde(rename = "addressUnitDesignatorType")]
    pub address_unit_designator_type: Option<String>,
    #[serde(rename = "addressUnitIdentifier")]
    pub address_unit_identifier: Option<String>,
    #[serde(rename = "streetName")]
    pub street_name: String,
    #[serde(rename = "streetPostDirectionalText")]
    pub street_post_directional_text: Option<String>,
    #[serde(rename = "streetPreDirectionalText")]
    pub street_pre_directional_text: Option<String>,
    #[serde(rename = "streetPrimaryNumberText")]
    pub street_primary_number_text: Option<String>,
    #[serde(rename = "streetSuffixText")]
    pub street_suffix_text: Option<String>,
}
impl std::fmt::Display for BlendENoteParsedStreetAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteParsedFullAddress(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteParsedFullAddressWithCounty(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENotePercentage(pub f64);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENotePointWordsType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub enum BlendENoteStateType {
    #[serde(rename = "AL")]
    Al,
    #[serde(rename = "AK")]
    Ak,
    #[serde(rename = "AZ")]
    Az,
    #[serde(rename = "AR")]
    Ar,
    #[serde(rename = "CA")]
    Ca,
    #[serde(rename = "CO")]
    Co,
    #[serde(rename = "CT")]
    Ct,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "DC")]
    Dc,
    #[serde(rename = "FL")]
    Fl,
    #[serde(rename = "GA")]
    Ga,
    #[serde(rename = "HI")]
    Hi,
    #[serde(rename = "ID")]
    Id,
    #[serde(rename = "IL")]
    Il,
    #[serde(rename = "IN")]
    In,
    #[serde(rename = "IA")]
    Ia,
    #[serde(rename = "KS")]
    Ks,
    #[serde(rename = "KY")]
    Ky,
    #[serde(rename = "LA")]
    La,
    #[serde(rename = "ME")]
    Me,
    #[serde(rename = "MD")]
    Md,
    #[serde(rename = "MA")]
    Ma,
    #[serde(rename = "MI")]
    Mi,
    #[serde(rename = "MN")]
    Mn,
    #[serde(rename = "MS")]
    Ms,
    #[serde(rename = "MO")]
    Mo,
    #[serde(rename = "MT")]
    Mt,
    #[serde(rename = "NE")]
    Ne,
    #[serde(rename = "NV")]
    Nv,
    #[serde(rename = "NH")]
    Nh,
    #[serde(rename = "NJ")]
    Nj,
    #[serde(rename = "NM")]
    Nm,
    #[serde(rename = "NY")]
    Ny,
    #[serde(rename = "NC")]
    Nc,
    #[serde(rename = "ND")]
    Nd,
    #[serde(rename = "OH")]
    Oh,
    #[serde(rename = "OK")]
    Ok,
    #[serde(rename = "OR")]
    Or,
    #[serde(rename = "PA")]
    Pa,
    #[serde(rename = "RI")]
    Ri,
    #[serde(rename = "SC")]
    Sc,
    #[serde(rename = "SD")]
    Sd,
    #[serde(rename = "TN")]
    Tn,
    #[serde(rename = "TX")]
    Tx,
    #[serde(rename = "UT")]
    Ut,
    #[serde(rename = "VT")]
    Vt,
    #[serde(rename = "VA")]
    Va,
    #[serde(rename = "WA")]
    Wa,
    #[serde(rename = "WV")]
    Wv,
    #[serde(rename = "WI")]
    Wi,
    #[serde(rename = "WY")]
    Wy,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteData {}
impl std::fmt::Display for BlendENoteData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteUnparsedNameType(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteUnparsedStreetAddress {
    #[serde(rename = "addressLineText")]
    pub address_line_text: String,
    #[serde(rename = "addressAdditionalLineText")]
    pub address_additional_line_text: Option<String>,
}
impl std::fmt::Display for BlendENoteUnparsedStreetAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteUnparsedFullAddress(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct BlendENoteUnparsedFullAddressWithCounty(pub serde_json::Value);
#[derive(Debug, Serialize, Deserialize)]
pub struct GetItpResponse {
    #[serde(rename = "addedAt")]
    ///ISO 8601 timestamp when the first Intent to Proceed Workflow was added for any borrower.
    pub added_at: String,
    #[serde(rename = "givenAt")]
    ///ISO 8601 timestamp when Intent to Proceed was given on an application.
    pub given_at: Option<String>,
    #[serde(rename = "givenBy")]
    ///The partyID of the borrower who gave Intent to Proceed.
    pub given_by: Option<String>,
    #[serde(rename = "givenMethod")]
    ///The method used to provide Intent to Proceed. This is 'ELECTRONICALLY_IN_BLEND' only if it was provided by a borrower completing a borrower workflow in Blend. If it is marked completed by a lender in Blend on behalf of a borrower, or via API actions, it will be set as 'OUTSIDE_OF_BLEND'.
    pub given_method: Option<String>,
}
impl std::fmt::Display for GetItpResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ItpNotFoundError {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for ItpNotFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFieldsObject {}
impl std::fmt::Display for CustomFieldsObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CustomMetadataObject {}
impl std::fmt::Display for CustomMetadataObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CreatedByObject {
    #[serde(rename = "userId")]
    pub user_id: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
}
impl std::fmt::Display for CreatedByObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedEmployersSchema {
    #[serde(rename = "verifiedEmployers")]
    pub verified_employers: Option<Vec<VerifiedEmployersSummarySchema>>,
}
impl std::fmt::Display for VerifiedEmployersSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedEmployersSummarySchema {
    #[serde(rename = "partyId")]
    ///The UUID of the partyId.
    pub party_id: String,
    #[serde(rename = "employerDetails")]
    ///Summary of Verified Employer object per borrower on an application.
    pub employer_details: Option<Vec<VerifiedEmployerObjectSchema>>,
}
impl std::fmt::Display for VerifiedEmployersSummarySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct VerifiedEmployerObjectSchema {
    #[serde(rename = "status")]
    ///Outcome of the employer verification.
    pub status: Option<String>,
    #[serde(rename = "employerName")]
    ///Name of the verified employer.
    pub employer_name: Option<String>,
    #[serde(rename = "employerId")]
    ///The UUID of the employerId.
    pub employer_id: String,
}
impl std::fmt::Display for VerifiedEmployerObjectSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmployersReverifyRequestSchema {
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "partyId")]
    ///The UUID of the partyId.
    pub party_id: String,
    #[serde(rename = "employerIds")]
    ///These are the employers that a reverify will be initiated for.
    pub employer_ids: Vec<String>,
}
impl std::fmt::Display for EmployersReverifyRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingNameSchema {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "suffix")]
    pub suffix: Option<String>,
}
impl std::fmt::Display for ConsumerLendingNameSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingPhoneNumberSchema {
    #[serde(rename = "type")]
    ///Type of phone number
    pub type_: String,
    #[serde(rename = "number")]
    ///Phone number
    pub number: f64,
}
impl std::fmt::Display for ConsumerLendingPhoneNumberSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingIdentityEvaluationSchema {
    #[serde(rename = "externalReferenceId")]
    ///Identity evaluation external reference ID
    pub external_reference_id: Option<String>,
    #[serde(rename = "status")]
    ///Identity evaluation status
    pub status: Option<String>,
    #[serde(rename = "score")]
    ///Identity evaluation score
    pub score: Option<f64>,
    #[serde(rename = "outcome")]
    ///Identity evaluation outcome
    pub outcome: Option<String>,
    #[serde(rename = "providerMetadata")]
    ///Provider metadata
    pub provider_metadata: Option<serde_json::Value>,
    #[serde(rename = "userMetadata")]
    ///User metadata
    pub user_metadata: Option<serde_json::Value>,
}
impl std::fmt::Display for ConsumerLendingIdentityEvaluationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingAddressSchema {
    #[serde(rename = "streetAddressLine1")]
    pub street_address_line1: Option<String>,
    #[serde(rename = "streetAddressLine2")]
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: Option<String>,
    #[serde(rename = "county")]
    pub county: Option<String>,
    #[serde(rename = "state")]
    ///Two-character US state code
    pub state: Option<String>,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code
    pub zip_code: Option<String>,
}
impl std::fmt::Display for ConsumerLendingAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingResidenceSchema {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "residencyType")]
    ///Residency type
    pub residency_type: Option<String>,
    #[serde(rename = "address")]
    ///Address
    pub address: ConsumerLendingAddressSchema,
    #[serde(rename = "moveInDate")]
    ///Move in date in ISO 8601 format
    pub move_in_date: Option<String>,
    #[serde(rename = "moveOutDate")]
    ///Move out date in ISO 8601 format
    pub move_out_date: Option<String>,
    #[serde(rename = "residencyDurationYears")]
    ///Number of years in residency
    pub residency_duration_years: Option<f64>,
    #[serde(rename = "residencyDurationMonths")]
    ///Number of months (in excess of number of years) in residency
    pub residency_duration_months: Option<f64>,
    #[serde(rename = "monthlyRent")]
    ///Monthly rent
    pub monthly_rent: Option<f64>,
}
impl std::fmt::Display for ConsumerLendingResidenceSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingOwnedPropertySchema {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "usageType")]
    ///Usage type
    pub usage_type: String,
    #[serde(rename = "dispositionType")]
    ///Disposition type
    pub disposition_type: Option<String>,
    #[serde(rename = "address")]
    ///Address
    pub address: ConsumerLendingAddressSchema,
    #[serde(rename = "presentMarketValue")]
    ///Present market value
    pub present_market_value: f64,
    #[serde(rename = "monthlyMortgagePayments")]
    ///Monthly mortgage payments
    pub monthly_mortgage_payments: Option<Vec<f64>>,
    #[serde(rename = "mortgageBalance")]
    ///Mortgage balance
    pub mortgage_balance: Option<f64>,
    #[serde(rename = "grossRentalIncome")]
    ///Gross rental income
    pub gross_rental_income: Option<f64>,
}
impl std::fmt::Display for ConsumerLendingOwnedPropertySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingEmploymentOwnershipSchema {
    #[serde(rename = "ownershipType")]
    ///Ownership type
    pub ownership_type: Option<String>,
    #[serde(rename = "ownershipPercentage")]
    ///Ownership percentage
    pub ownership_percentage: Option<f64>,
}
impl std::fmt::Display for ConsumerLendingEmploymentOwnershipSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingIncomeSchema {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "yearlyIncome")]
    ///Yearly income
    pub yearly_income: Option<f64>,
    #[serde(rename = "description")]
    ///Description of income source
    pub description: Option<String>,
}
impl std::fmt::Display for ConsumerLendingIncomeSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingEmployerSchema {
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "employerSEGCode")]
    ///Employer SEG code
    pub employer_seg_code: Option<String>,
    #[serde(rename = "address")]
    ///Address
    pub address: ConsumerLendingAddressSchema,
    #[serde(rename = "phoneNumber")]
    pub phone_number: Option<f64>,
    #[serde(rename = "title")]
    pub title: Option<String>,
    #[serde(rename = "startDate")]
    ///Start date in ISO 8601 format
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    ///End date in ISO 8601 format
    pub end_date: Option<String>,
    #[serde(rename = "hoursPerWeek")]
    ///Hours per week
    pub hours_per_week: Option<f64>,
    #[serde(rename = "monthsOnJob")]
    ///Number of months on the job
    pub months_on_job: Option<f64>,
    #[serde(rename = "yearsInProfession")]
    ///Number of years in profession
    pub years_in_profession: Option<f64>,
    #[serde(rename = "ownership")]
    ///Employer ownership
    pub ownership: Option<ConsumerLendingEmploymentOwnershipSchema>,
    #[serde(rename = "incomes")]
    pub incomes: Option<Vec<ConsumerLendingIncomeSchema>>,
}
impl std::fmt::Display for ConsumerLendingEmployerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingLiabilitySchema {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "monthlyPayment")]
    ///Monthly payment
    pub monthly_payment: f64,
    #[serde(rename = "monthsLeftToPay")]
    ///Months left to pay
    pub months_left_to_pay: f64,
    #[serde(rename = "unpaidBalance")]
    ///Unpaid balance
    pub unpaid_balance: f64,
}
impl std::fmt::Display for ConsumerLendingLiabilitySchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingDeclarationSchema {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "indicator")]
    pub indicator: Option<bool>,
    #[serde(rename = "explanation")]
    pub explanation: Option<String>,
    #[serde(rename = "spouseAddress")]
    pub spouse_address: Option<PartialAddressSchema>,
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "suffixName")]
    pub suffix_name: Option<String>,
}
impl std::fmt::Display for ConsumerLendingDeclarationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingBorrowerSchema {
    #[serde(rename = "id")]
    ///Borrower ID
    pub id: String,
    #[serde(rename = "memberId")]
    ///Member ID
    pub member_id: Option<String>,
    #[serde(rename = "type")]
    ///Borrower type
    pub type_: String,
    #[serde(rename = "name")]
    pub name: ConsumerLendingNameSchema,
    #[serde(rename = "email")]
    pub email: String,
    #[serde(rename = "phoneNumbers")]
    pub phone_numbers: Vec<ConsumerLendingPhoneNumberSchema>,
    #[serde(rename = "birthDate")]
    ///Birth date in ISO 8601 format
    pub birth_date: Option<String>,
    #[serde(rename = "ssn")]
    ///Social Security Number
    pub ssn: Option<String>,
    #[serde(rename = "creditReport")]
    ///Credit report
    pub credit_report: Option<String>,
    #[serde(rename = "identityEvaluations")]
    pub identity_evaluations: Option<Vec<ConsumerLendingIdentityEvaluationSchema>>,
    #[serde(rename = "citizenshipStatus")]
    pub citizenship_status: Option<String>,
    #[serde(rename = "maritalStatus")]
    ///Marital Status
    pub marital_status: Option<String>,
    #[serde(rename = "residences")]
    pub residences: Option<Vec<ConsumerLendingResidenceSchema>>,
    #[serde(rename = "ownedProperties")]
    pub owned_properties: Option<Vec<ConsumerLendingOwnedPropertySchema>>,
    #[serde(rename = "employers")]
    pub employers: Option<Vec<ConsumerLendingEmployerSchema>>,
    #[serde(rename = "otherIncomes")]
    pub other_incomes: Option<Vec<ConsumerLendingIncomeSchema>>,
    #[serde(rename = "liabilities")]
    pub liabilities: Option<Vec<ConsumerLendingLiabilitySchema>>,
    #[serde(rename = "declarations")]
    pub declarations: Option<Vec<ConsumerLendingDeclarationSchema>>,
    #[serde(rename = "consentToLenderContactGiven")]
    ///If set to true, consent is given to lender contact
    pub consent_to_lender_contact_given: Option<bool>,
    #[serde(rename = "econsent")]
    pub econsent: Option<EconsentSchema>,
    #[serde(rename = "creditConsent")]
    pub credit_consent: Option<EconsentSchema>,
    #[serde(rename = "creditConsentSoft")]
    pub credit_consent_soft: Option<EconsentSchema>,
}
impl std::fmt::Display for ConsumerLendingBorrowerSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingProductId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingLoanIdentifiersSchema {
    #[serde(rename = "losId")]
    ///The UUID or GUID of the current resource in the LOS AFTER initial export to the LOS. For loans this may match the loan reference number depending on the LOS and if the customer has selected to use only GUIDs instead of UUIDs in their LOS settings. For all other resources, it is the static identifier of this resource in the LOS.
    pub los_id: Option<String>,
    #[serde(rename = "crmId")]
    ///The unique identifier of the application in a lender's Customer Relationship Management (CRM) system (e.g. Salesforce, Velocify, etc.). This value should not change over time and should be used to connect the application's identity between Blend and the CRM.
    pub crm_id: Option<String>,
    #[serde(rename = "referenceNumber")]
    ///A mutable identifier of the application. Not safe to use to connect the application's identity across Blend and external systems because it can and (for most implementations) will change. Default value is an incremented ID set by Blend. Other Values could be LOS GUID after export of the loan to LOS (may be the same as the losID field or different), Can be manually set to anything by lenders in the UI or programmatically via the API.
    pub reference_number: Option<String>,
}
impl std::fmt::Display for ConsumerLendingLoanIdentifiersSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ConsumerLendingLoanType {
    #[serde(rename = "AUTO")]
    Auto,
    #[serde(rename = "SPECIALTY_VEHICLE")]
    SpecialtyVehicle,
    #[serde(rename = "PERSONAL_LOAN")]
    PersonalLoan,
    #[serde(rename = "PERSONAL_LOC")]
    PersonalLoc,
    #[serde(rename = "CREDIT_CARD")]
    CreditCard,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ConsumerLendingLoanPurposeType {
    #[serde(rename = "PURCHASE")]
    Purchase,
    #[serde(rename = "REFINANCE")]
    Refinance,
    #[serde(rename = "LEASE_BUYOUT")]
    LeaseBuyout,
    #[serde(rename = "FUTURE_USE")]
    FutureUse,
    #[serde(rename = "DEBT_CONSOLIDATION")]
    DebtConsolidation,
    #[serde(rename = "HOME_IMPROVEMENT")]
    HomeImprovement,
    #[serde(rename = "MEDICAL_EXPENSES")]
    MedicalExpenses,
    #[serde(rename = "OTHER")]
    Other,
    #[serde(rename = "LIVING_EXPENSES")]
    LivingExpenses,
    #[serde(rename = "CASH_NEED")]
    CashNeed,
    #[serde(rename = "EDUCATION_EXPENSES")]
    EducationExpenses,
    #[serde(rename = "ESTABLISH_CREDIT")]
    EstablishCredit,
    #[serde(rename = "FAMILY_EVENT")]
    FamilyEvent,
    #[serde(rename = "GIFT")]
    Gift,
    #[serde(rename = "MAJOR_PURCHASES")]
    MajorPurchases,
    #[serde(rename = "MEDICAL_NURSING_CARE")]
    MedicalNursingCare,
    #[serde(rename = "TAXES_LEGAL_FEES")]
    TaxesLegalFees,
    #[serde(rename = "VACATION_TRIP")]
    VacationTrip,
    #[serde(rename = "VEHICLE_REPAIR")]
    VehicleRepair,
}
#[derive(Debug, Serialize, Deserialize)]
pub enum ConsumerLendingRefinanceType {
    #[serde(rename = "LOAN")]
    Loan,
    #[serde(rename = "CASH_OUT")]
    CashOut,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingCollateralSchema {
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "primary")]
    pub primary: Option<bool>,
    #[serde(rename = "vehicle")]
    pub vehicle: Option<ConsumerLendingVehicleSchema>,
    #[serde(rename = "account")]
    pub account: Option<ConsumerLendingAccountSchema>,
}
impl std::fmt::Display for ConsumerLendingCollateralSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingAccountSchema {
    #[serde(rename = "accountNumber")]
    pub account_number: Option<String>,
    #[serde(rename = "holdAmount")]
    pub hold_amount: Option<f64>,
}
impl std::fmt::Display for ConsumerLendingAccountSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingVehicleSchema {
    #[serde(rename = "condition")]
    pub condition: Option<String>,
    #[serde(rename = "purchaseLocation")]
    pub purchase_location: Option<String>,
    #[serde(rename = "category")]
    pub category: Option<String>,
    #[serde(rename = "type")]
    pub type_: Option<String>,
    #[serde(rename = "year")]
    pub year: Option<f64>,
    #[serde(rename = "make")]
    pub make: Option<String>,
    #[serde(rename = "model")]
    pub model: Option<String>,
    #[serde(rename = "trim")]
    pub trim: Option<String>,
    #[serde(rename = "mileage")]
    pub mileage: Option<f64>,
    #[serde(rename = "valuation")]
    ///The property's value in dollars and cents
    pub valuation: Option<f64>,
    #[serde(rename = "identificationNumber")]
    pub identification_number: Option<String>,
    #[serde(rename = "salesPrice")]
    ///Sales price for the loan
    pub sales_price: Option<f64>,
}
impl std::fmt::Display for ConsumerLendingVehicleSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingApplicationDecisionSchema {
    #[serde(rename = "updatedDate")]
    pub updated_date: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(rename = "amount")]
    pub amount: Option<f64>,
    #[serde(rename = "apr")]
    pub apr: Option<f64>,
    #[serde(rename = "monthlyPayment")]
    pub monthly_payment: Option<f64>,
    #[serde(rename = "termMonths")]
    pub term_months: Option<f64>,
    #[serde(rename = "productOffering")]
    pub product_offering: Option<String>,
    #[serde(rename = "policySnapshot")]
    pub policy_snapshot: Option<serde_json::Value>,
    #[serde(rename = "attributes")]
    pub attributes: Option<serde_json::Value>,
    #[serde(rename = "result")]
    pub result: Option<serde_json::Value>,
}
impl std::fmt::Display for ConsumerLendingApplicationDecisionSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ConsumerLendingApplicationSchema {
    #[serde(rename = "id")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "productId")]
    ///The product id for the associated loan
    pub product_id: Option<String>,
    #[serde(rename = "applicationStatus")]
    ///An enum for the status of the application.
    pub application_status: Option<String>,
    #[serde(rename = "loanIdentifiers")]
    pub loan_identifiers: Option<ConsumerLendingLoanIdentifiersSchema>,
    #[serde(rename = "loanType")]
    pub loan_type: String,
    #[serde(rename = "loanPurposeType")]
    pub loan_purpose_type: Option<String>,
    #[serde(rename = "refinanceType")]
    ///Only applicable if loanPurposeType is REFINANCE
    pub refinance_type: Option<String>,
    #[serde(rename = "borrowers")]
    pub borrowers: Vec<ConsumerLendingBorrowerSchema>,
    #[serde(rename = "loanAmount")]
    ///The amount of money (dollars and cents) for which the applicant is applying
    pub loan_amount: Option<f64>,
    #[serde(rename = "loanTermMonths")]
    ///The term of the loan in months
    pub loan_term_months: Option<f64>,
    #[serde(rename = "isFlexibleLoanTerms")]
    ///Whether the applicant is flexible in their loan amount and terms
    pub is_flexible_loan_terms: Option<bool>,
    #[serde(rename = "autopayFromChecking")]
    ///Whether the applicant wants to setup autopay from their checking account
    pub autopay_from_checking: Option<bool>,
    #[serde(rename = "lender")]
    pub lender: Option<String>,
    #[serde(rename = "collaterals")]
    pub collaterals: Option<Vec<ConsumerLendingCollateralSchema>>,
    #[serde(rename = "primaryAssignedLoanOfficer")]
    pub primary_assigned_loan_officer: Option<serde_json::Value>,
    #[serde(rename = "decision")]
    ///Application decision
    pub decision: Option<ConsumerLendingApplicationDecisionSchema>,
    #[serde(rename = "promoCode")]
    pub promo_code: Option<String>,
    #[serde(rename = "appSource")]
    ///The tag corresponding to a marketing campaign. This identifier can be used to gauge which links are most effective within certain campaigns.
    pub app_source: Option<String>,
    #[serde(rename = "inLenderNetwork")]
    ///Whether the applicant is in the lenders network
    pub in_lender_network: Option<bool>,
    #[serde(rename = "newRentalLease")]
    ///Whether the applicant has a new rental lease
    pub new_rental_lease: Option<bool>,
}
impl std::fmt::Display for ConsumerLendingApplicationSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveRonEligibilityPostRequest {
    #[serde(rename = "address")]
    ///The provided address will be validated via a 3rd party API. If the provided address is not found (i.e. invalid address), we will do our best to provide eligibility information on parts that we can find. It is possible for multiple addresses to be returned in the response if provided address is ambiguous. It is possible for a slightly different address to be returned if a correction has been made to the original address input during validation.
    pub address: RonEligibilityAddressSchema,
    #[serde(rename = "titleAgency")]
    ///The title agency involved in the real estate transaction. Blend will attempt to look up underwriters working with the provided title agency and return eligibility information on whether the underwriter(s) will insure RON in the state where the address is located. If a title agency is not provided or we're not able to find a underwriter match, information on all major underwriters will be returned.
    pub title_agency: Option<String>,
    #[serde(rename = "options")]
    ///Request configuration options.
    pub options: Option<RonEligibilityRequestOptions>,
}
impl std::fmt::Display for RetrieveRonEligibilityPostRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RonEligibilityAddressSchema {
    #[serde(rename = "streetAddressLine1")]
    ///Street address.
    pub street_address_line1: String,
    #[serde(rename = "streetAddressLine2")]
    ///Apartment / unit / suite number or other address designations.
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    ///Two-character US state code.
    pub state: String,
    #[serde(rename = "countyName")]
    pub county_name: Option<String>,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code.
    pub zip_code: String,
}
impl std::fmt::Display for RonEligibilityAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RonEligibilityRequestOptions {
    #[serde(rename = "match")]
    ///The address matching type. Defaults to 'loose' if not provided. 'loose' uses a relaxed matching algorithm during address validation. Will attempt to return a validated address match for invalid address inputs. 'strict' uses a more aggressive matching algorithm during address validation. Will not return a validated address match for invalid address inputs.
    pub match_: Option<String>,
}
impl std::fmt::Display for RonEligibilityRequestOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct RetrieveRonEligibilityResponse {
    #[serde(rename = "addressesWithEligibility")]
    pub addresses_with_eligibility: Option<Vec<AddressRonEligibilityResponse>>,
}
impl std::fmt::Display for RetrieveRonEligibilityResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct UnderwritersRonEligibilityResponseObject {
    #[serde(rename = "name")]
    ///Name of the underwriter entity
    pub name: Option<String>,
    #[serde(rename = "eligibility")]
    ///There are different components to look at to determine eligibility for a RON closing (i.e. state, county, underwriter), this describes the eligibility of a specific RON eligibility component. Possible options include: 'ELIGIBLE' - The component is eligible for a RON closing. 'INELIGIBLE' - The component is NOT eligible for a RON closing. 'UNKNOWN' - There is not enough information to determine whether the component is eligible or not.
    pub eligibility: Option<String>,
    #[serde(rename = "allowsOutOfStateNotary")]
    ///Whether the underwriter will recognize a RON for a property that will be performed by a notary who is commissioned in a state outside of where the property is located.
    pub allows_out_of_state_notary: Option<bool>,
}
impl std::fmt::Display for UnderwritersRonEligibilityResponseObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AddressRonEligibilityResponse {
    #[serde(rename = "underwriters")]
    pub underwriters: Vec<UnderwritersRonEligibilityResponseObject>,
    #[serde(rename = "countyEligibility")]
    ///There are different components to look at to determine eligibility for a RON closing (i.e. state, county, underwriter), this describes the eligibility of a specific RON eligibility component. Possible options include: 'ELIGIBLE' - The component is eligible for a RON closing. 'INELIGIBLE' - The component is NOT eligible for a RON closing. 'UNKNOWN' - There is not enough information to determine whether the component is eligible or not.
    pub county_eligibility: String,
    #[serde(rename = "stateEligibility")]
    ///There are different components to look at to determine eligibility for a RON closing (i.e. state, county, underwriter), this describes the eligibility of a specific RON eligibility component. Possible options include: 'ELIGIBLE' - The component is eligible for a RON closing. 'INELIGIBLE' - The component is NOT eligible for a RON closing. 'UNKNOWN' - There is not enough information to determine whether the component is eligible or not.
    pub state_eligibility: String,
    #[serde(rename = "streetAddressLine1")]
    ///Street address
    pub street_address_line1: String,
    #[serde(rename = "streetAddressLine2")]
    ///Apartment / unit / suite number or other address designations
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    ///Two-character US state code
    pub state: String,
    #[serde(rename = "countyName")]
    ///If the county name is provided in the request, the address will not be validated. We will directly look up the eligibility of the state and county provided in the request.
    pub county_name: String,
    #[serde(rename = "countyFIPS")]
    ///Five-digit US county code
    pub county_fips: String,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code
    pub zip_code: String,
}
impl std::fmt::Display for AddressRonEligibilityResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RonEligibility {
    #[serde(rename = "ELIGIBLE")]
    Eligible,
    #[serde(rename = "INELIGIBLE")]
    Ineligible,
    #[serde(rename = "UNKNOWN")]
    Unknown,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenRequestSchema {
    #[serde(rename = "client_id")]
    ///The ID of the user making API requests
    pub client_id: String,
    #[serde(rename = "client_secret")]
    ///The password of the user making API requests
    pub client_secret: String,
    #[serde(rename = "grant_type")]
    ///The OAuth grant type (should be client_credentials)
    pub grant_type: String,
}
impl std::fmt::Display for OAuthTokenRequestSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthTokenResponseSchema {
    #[serde(rename = "access_token")]
    ///The OAuth Token
    pub access_token: String,
    #[serde(rename = "token_type")]
    ///The type of token returned (i.e., Bearer)
    pub token_type: String,
    #[serde(rename = "expires_in")]
    ///The number of seconds until the token expires
    pub expires_in: f64,
}
impl std::fmt::Display for OAuthTokenResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthErrorSchema {
    #[serde(rename = "error")]
    ///The OAuth error code
    pub error: String,
}
impl std::fmt::Display for OAuthErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentUser {
    #[serde(rename = "isAuthenticated")]
    ///User authentication result
    pub is_authenticated: Option<bool>,
}
impl std::fmt::Display for CurrentUser {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingApplicationId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingNameSchema {
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[serde(rename = "middleName")]
    pub middle_name: Option<String>,
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[serde(rename = "suffixName")]
    pub suffix_name: Option<String>,
}
impl std::fmt::Display for LendingNameSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingPartyResponseSchema {
    #[serde(rename = "id")]
    ///The UUID of the Party in Blend's system. The static identifier that should be used to connect the party's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "email")]
    pub email: Option<String>,
    #[serde(rename = "name")]
    pub name: Option<LendingNameSchema>,
    #[serde(rename = "losPartyId")]
    ///Party ID in LOS
    pub los_party_id: Option<String>,
    #[serde(rename = "econsent")]
    pub econsent: Option<serde_json::Value>,
    #[serde(rename = "customFields")]
    pub custom_fields: Option<serde_json::Value>,
    #[serde(rename = "customMetadata")]
    pub custom_metadata: Option<serde_json::Value>,
}
impl std::fmt::Display for LendingPartyResponseSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingAddressSchema {
    #[serde(rename = "streetAddressLine1")]
    pub street_address_line1: String,
    #[serde(rename = "streetAddressLine2")]
    pub street_address_line2: Option<String>,
    #[serde(rename = "city")]
    pub city: String,
    #[serde(rename = "state")]
    ///Two-digit US state code
    pub state: String,
    #[serde(rename = "zipCode")]
    ///Five-digit US zip code
    pub zip_code: String,
    #[serde(rename = "zipCodePlusFour")]
    ///Four-digit additional US zip code numbers
    pub zip_code_plus_four: Option<String>,
    #[serde(rename = "countyName")]
    ///If the county name is provided in the request, the address will not be validated. We will directly look up the eligibility of the state and county provided in the request.
    pub county_name: Option<String>,
}
impl std::fmt::Display for LendingAddressSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingPhoneNumberSchema {
    #[serde(rename = "phoneNumber")]
    ///String representation of phone number
    pub phone_number: Option<String>,
    #[serde(rename = "type")]
    ///Type of phone number
    pub type_: Option<String>,
}
impl std::fmt::Display for LendingPhoneNumberSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyStringSchema(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingDocumentSchema {
    #[serde(rename = "id")]
    ///The UUID of the Document in Blend's system. The static identifier that should be used to connect the document's identity across Blend and external integrations.
    pub id: Option<String>,
    #[serde(rename = "name")]
    ///Document Filename
    pub name: Option<String>,
    #[serde(rename = "type")]
    ///Blend document type
    pub type_: Option<String>,
    #[serde(rename = "applicationId")]
    ///The UUID of the application in Blend's system. The static identifier that should be used to connect the application's identity across Blend and external integrations.
    pub application_id: Option<String>,
    #[serde(rename = "losType")]
    ///LOS document type (used with Blend document templating)
    pub los_type: Option<String>,
    #[serde(rename = "losTypeId")]
    ///LOS document id (used for external tracking)
    pub los_type_id: Option<String>,
    #[serde(rename = "created")]
    ///UTC Timestamp of document creation
    pub created: Option<String>,
    #[serde(rename = "partyIds")]
    ///UUIDs of the Parties associated with this document
    pub party_ids: Option<Vec<String>>,
    #[serde(rename = "downloadUrl")]
    ///URL where the Document can be Downloaded from
    pub download_url: Option<String>,
    #[serde(rename = "lastExportedAt")]
    ///UTC Timestamp of last export for document
    pub last_exported_at: Option<String>,
    #[serde(rename = "text")]
    ///Description used in UI
    pub text: Option<String>,
    #[serde(rename = "category")]
    ///Document's hierarchical category
    pub category: Option<String>,
    #[serde(rename = "signerInfo")]
    pub signer_info: Option<Vec<serde_json::Value>>,
    #[serde(rename = "customFields")]
    ///Custom fields on GET responses for supported resources
    pub custom_fields: Option<LendingCustomFields>,
    #[serde(rename = "customMetadata")]
    ///Custom metadata on GET responses for supported resources
    pub custom_metadata: Option<LendingCustomMetadata>,
    #[serde(rename = "additionalEntities")]
    ///Additional entities ids associated with the document
    pub additional_entities: Option<serde_json::Value>,
}
impl std::fmt::Display for LendingDocumentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingPostEconsentSchema {
    #[serde(rename = "status")]
    ///State of econsent
    pub status: String,
    #[serde(rename = "date")]
    ///UTC Timestamp of when Econsent was given. Optional. Null is accepted.
    pub date: Option<String>,
    #[serde(rename = "ip")]
    ///IP Address from which econsent was given. Optional. Null is accepted.
    pub ip: Option<String>,
}
impl std::fmt::Display for LendingPostEconsentSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LosPartyId(pub String);
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingSingleLosMilestoneSchema {
    #[serde(rename = "losMilestone")]
    ///ENUM describing that status of the application. The following milestones can be configured to be displayed to the Borrower on their landing page after submitting their application in Blend (SUBMITTED, PROCESSING, UNDERWRITING, CONDITIONAL APPROVAL, APPROVED, CLOSING, CLOSED, FUNDED). The other milestones are only visible to the lender application.
    pub los_milestone: String,
}
impl std::fmt::Display for LendingSingleLosMilestoneSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum LendingLosMilestoneName {
    #[serde(rename = "SUBMITTED")]
    Submitted,
    #[serde(rename = "PREQUALIFIED")]
    Prequalified,
    #[serde(rename = "PREAPPROVED")]
    Preapproved,
    #[serde(rename = "PROCESSING")]
    Processing,
    #[serde(rename = "UNDERWRITING")]
    Underwriting,
    #[serde(rename = "CONDITIONAL_APPROVAL")]
    ConditionalApproval,
    #[serde(rename = "APPROVED")]
    Approved,
    #[serde(rename = "CLOSING")]
    Closing,
    #[serde(rename = "CLOSED")]
    Closed,
    #[serde(rename = "APPRAISAL_ORDERED")]
    AppraisalOrdered,
    #[serde(rename = "APPRAISAL_RECEIVED")]
    AppraisalReceived,
    #[serde(rename = "CLEAR_TO_CLOSE")]
    ClearToClose,
    #[serde(rename = "DOCS_SENT")]
    DocsSent,
    #[serde(rename = "FUNDED")]
    Funded,
    #[serde(rename = "SET_UP")]
    SetUp,
    #[serde(rename = "DECISIONED")]
    Decisioned,
    #[serde(rename = "CONDITIONS_SUBMITTED")]
    ConditionsSubmitted,
    #[serde(rename = "APPRAISAL_APPROVED")]
    AppraisalApproved,
    #[serde(rename = "RATE_LOCKED")]
    RateLocked,
    #[serde(rename = "CLOSING_DISCLOSURE_SENT")]
    ClosingDisclosureSent,
    #[serde(rename = "DENIED")]
    Denied,
    #[serde(rename = "LOAN_ESTIMATE_SENT")]
    LoanEstimateSent,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LosMilestone {
    #[serde(rename = "name")]
    ///ENUM describing that status of the application. The following milestones can be configured to be displayed to the Borrower on their landing page after submitting their application in Blend (SUBMITTED, PROCESSING, UNDERWRITING, CONDITIONAL APPROVAL, APPROVED, CLOSING, CLOSED, FUNDED). The other milestones are only visible to the lender application.
    pub name: String,
    #[serde(rename = "lastUpdated")]
    ///UTC Timestamp of when the LOS Milestone was last updated
    pub last_updated: String,
}
impl std::fmt::Display for LosMilestone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingCustomFields {
    #[serde(rename = "fieldName")]
    pub field_name: Option<serde_json::Value>,
}
impl std::fmt::Display for LendingCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingCustomMetadata {
    #[serde(rename = "fieldName")]
    ///Custom fields on GET responses for supported resources
    pub field_name: Option<CustomFieldSchema>,
}
impl std::fmt::Display for LendingCustomMetadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingInternalServerErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for LendingInternalServerErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingInvalidRequestErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}
impl std::fmt::Display for LendingInvalidRequestErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingInvalidUpdateErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}
impl std::fmt::Display for LendingInvalidUpdateErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingNotAuthorizedErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: Option<String>,
    #[serde(rename = "trackingId")]
    pub tracking_id: Option<String>,
}
impl std::fmt::Display for LendingNotAuthorizedErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingLenderRecordSchema {
    #[serde(rename = "name")]
    ///Lender Full Name
    pub name: Option<String>,
    #[serde(rename = "firstName")]
    ///Lender First Name
    pub first_name: Option<String>,
    #[serde(rename = "lastName")]
    ///Lender Last Name
    pub last_name: Option<String>,
    #[serde(rename = "email")]
    ///Lender Email
    pub email: Option<String>,
    #[serde(rename = "permittedSolutionSubTypes")]
    ///Set of allowable solution types
    pub permitted_solution_sub_types: Option<Vec<String>>,
    #[serde(rename = "requireTwoFactorAuth")]
    ///MFA Required
    pub require_two_factor_auth: bool,
    #[serde(rename = "loginMethod")]
    ///Login Method
    pub login_method: Option<String>,
    #[serde(rename = "roleNames")]
    ///Roles for Lender
    pub role_names: Option<Vec<String>>,
    #[serde(rename = "phone")]
    pub phone: Option<serde_json::Value>,
    #[serde(rename = "nmlsId")]
    ///Nationwide Multistate Licensing System ID number
    pub nmls_id: Option<String>,
    #[serde(rename = "losUsername")]
    ///Lender's LOS Username
    pub los_username: Option<String>,
    #[serde(rename = "employeeId")]
    ///Lender's Employee Id
    pub employee_id: Option<String>,
    #[serde(rename = "branchId")]
    ///Lender's Branch Id
    pub branch_id: Option<String>,
    #[serde(rename = "id")]
    ///The UUID of the Lender User in Blend's system. The static identifier that should be used to connect the user's identity across Blend and external integrations.
    pub id: String,
    #[serde(rename = "status")]
    ///The user's status in Blend
    pub status: String,
    #[serde(rename = "creditInternalAccountIdentifiers")]
    ///List of all the active creditInternalAccountIdentifiers
    pub credit_internal_account_identifiers: Option<
        Vec<LendingCreditInternalAccountIdentifiers>,
    >,
    #[serde(rename = "licensedStates")]
    ///States Lender is licensed in
    pub licensed_states: Option<Vec<String>>,
}
impl std::fmt::Display for LendingLenderRecordSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingCreditInternalAccountIdentifiers {
    #[serde(rename = "pullType")]
    ///Pull type
    pub pull_type: String,
    #[serde(rename = "creditPullScenario")]
    ///Credit Pull Scenario
    pub credit_pull_scenario: String,
    #[serde(rename = "creditProvider")]
    ///Credit Provider
    pub credit_provider: String,
    #[serde(rename = "creditAccountIdentifier")]
    ///Credit Account Identifier
    pub credit_account_identifier: String,
}
impl std::fmt::Display for LendingCreditInternalAccountIdentifiers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct NotFoundErrorSchema {
    #[serde(rename = "error")]
    pub error: String,
    #[serde(rename = "display")]
    pub display: String,
    #[serde(rename = "trackingId")]
    pub tracking_id: String,
}
impl std::fmt::Display for NotFoundErrorSchema {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct LendingConflictErrorSchema(pub serde_json::Value);
