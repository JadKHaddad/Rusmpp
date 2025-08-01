#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(clippy::enum_variant_names)]

use std::collections::BTreeMap as Map;

type Bytes = Vec<u8>;

pub mod rusmpp_types {
    pub use ::rusmpp::{pdus::*, tlvs::*, values::*, Command, CommandId, CommandStatus, Pdu};
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum AddrSubunit {
    Unknown(),
    MSDisplay(),
    MobileEquipment(),
    SmartCard(),
    ExternalUnit(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum AlertOnMessageDelivery {
    UseMobileDefaultAlert(),
    UseLowPriorityAlert(),
    UseMediumPriorityAlert(),
    UseHighPriorityAlert(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum BearerType {
    Unknown(),
    Sms(),
    Csd(),
    PacketData(),
    Ussd(),
    Cdpd(),
    DataTac(),
    FlexReFlex(),
    CellBroadcast(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum BroadcastAreaFormat {
    AliasName(),
    EllipsoidArc(),
    Polygon(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    pub area: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum BroadcastAreaSuccess {
    InformationNotAvailable(),
    ZeroToHundred(u8),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum BroadcastChannelIndicator {
    Basic(),
    Extended(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum EncodingContentType {
    Index(),
    EmergencyBroadcasts(),
    IrdbDownload(),
    NewsFlashes(),
    GeneralNewsLocal(),
    GeneralNewsRegional(),
    GeneralNewsNational(),
    GeneralNewsInternational(),
    BusinessFinancialNewsLocal(),
    BusinessFinancialNewsRegional(),
    BusinessFinancialNewsNational(),
    BusinessFinancialNewsInternational(),
    SportsNewsLocal(),
    SportsNewsRegional(),
    SportsNewsNational(),
    SportsNewsInternational(),
    EntertainmentNewsLocal(),
    EntertainmentNewsRegional(),
    EntertainmentNewsNational(),
    EntertainmentNewsInternational(),
    MedicalHealthHospitals(),
    Doctors(),
    Pharmacy(),
    LocalTrafficRoadReports(),
    LongDistanceTrafficRoadReports(),
    Taxis(),
    Weather(),
    LocalAirportFlightSchedules(),
    Restaurants(),
    Lodgings(),
    RetailDirectory(),
    Advertisements(),
    StockQuotes(),
    EmploymentOpportunities(),
    TechnologyNews(),
    DistrictBaseStationInfo(),
    NetworkInformation(),
    OperatorServices(),
    DirectoryEnquiriesNational(),
    DirectoryEnquiriesInternational(),
    CustomerCareNational(),
    CustomerCareInternational(),
    LocalDateTimeTimeZone(),
    MultiCategoryServices(),
    Other(u16),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum TypeOfNetwork {
    Generic(),
    Gsm(),
    Tdma(),
    Cdma(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BroadcastContentType {
    pub type_of_network: TypeOfNetwork,
    pub encoding_content_type: EncodingContentType,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum UnitOfTime {
    AsFrequentlyAsPossible(),
    Seconds(),
    Minutes(),
    Hours(),
    Days(),
    Weeks(),
    Months(),
    Years(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum BroadcastMessageClass {
    NoClassSpecified(),
    Class1(),
    Class2(),
    Class3(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BroadcastRepNum {
    pub value: u8,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Presentation {
    PresentationAllowed(),
    PresentationRestricted(),
    NumberNotAvailable(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Screening {
    NotScreened(),
    VerifiedAndPassed(),
    VerifiedAndFailed(),
    NetworkProvided(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct CallbackNumPresInd {
    pub presentation: Presentation,
    pub screening: Screening,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum CommandStatus {
    EsmeRok(),
    EsmeRinvmsglen(),
    EsmeRinvcmdlen(),
    EsmeRinvcmdid(),
    EsmeRinvbndsts(),
    EsmeRalybnd(),
    EsmeRinvprtflg(),
    EsmeRinvregdlvflg(),
    EsmeRsyserr(),
    EsmeRinvsrcadr(),
    EsmeRinvdstadr(),
    EsmeRinvmsgid(),
    EsmeRbindfail(),
    EsmeRinvpaswd(),
    EsmeRinvsysid(),
    EsmeRcancelfail(),
    EsmeRreplacefail(),
    EsmeRmsgqful(),
    EsmeRinvsertyp(),
    EsmeRinvnumdests(),
    EsmeRinvdlname(),
    EsmeRinvdestflag(),
    EsmeRinvsubrep(),
    EsmeRinvesmclass(),
    EsmeRcntsubdl(),
    EsmeRsubmitfail(),
    EsmeRinvsrcton(),
    EsmeRinvsrcnpi(),
    EsmeRinvdstton(),
    EsmeRinvdstnpi(),
    EsmeRinvsystyp(),
    EsmeRinvrepflag(),
    EsmeRinvnummsgs(),
    EsmeRthrottled(),
    EsmeRinvsched(),
    EsmeRinvexpiry(),
    EsmeRinvdftmsgid(),
    EsmeRxTAppn(),
    EsmeRxPAppn(),
    EsmeRxRAppn(),
    EsmeRqueryfail(),
    EsmeRinvtlvstream(),
    EsmeRtlvnotallwd(),
    EsmeRinvtlvlen(),
    EsmeRmissingtlv(),
    EsmeRinvtlvval(),
    EsmeRdeliveryfailure(),
    EsmeRunknownerr(),
    EsmeRsertypunauth(),
    EsmeRprohibited(),
    EsmeRsertypunavail(),
    EsmeRsertypdenied(),
    EsmeRinvdcs(),
    EsmeRinvsrcaddrsubunit(),
    EsmeRinvdstaddrsubunit(),
    EsmeRinvbcastfreqint(),
    EsmeRinvbcastaliasName(),
    EsmeRinvbcastareafmt(),
    EsmeRinvnumbcastAreas(),
    EsmeRinvbcastcnttype(),
    EsmeRinvbcastmsgclass(),
    EsmeRbcastfail(),
    EsmeRbcastqueryfail(),
    EsmeRbcastcancelfail(),
    EsmeRinvbcastRep(),
    EsmeRinvbcastsrvgrp(),
    EsmeRinvbcastchanind(),
    Other(u32),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum CongestionState {
    Idle(),
    LowLoad(u8),
    MediumLoad(u8),
    HighLoad(u8),
    OptimumLoad(u8),
    NearingCongestion(u8),
    Congested(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DeliveryFailureReason {
    DestinationUnavailable(),
    DestinationAddressInvalid(),
    PermanentNetworkError(),
    TemporaryNetworkError(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DestAddrNpResolution {
    QueryNotPerformed(),
    QueryPerformedNumberNotPorted(),
    QueryPerformedNumberPorted(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DisplayTime {
    Temporary(),
    Default(),
    Invoke(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DpfResult {
    NotSet(),
    Set(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum InterfaceVersion {
    Smpp3_3OrEarlier(u8),
    Smpp3_4(),
    Smpp5_0(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum ItsReplyType {
    Digit(),
    Number(),
    TelephoneNo(),
    Password(),
    CharacterLine(),
    Menu(),
    Date(),
    Time(),
    Continue(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum LanguageIndicator {
    Unspecified(),
    English(),
    French(),
    Spanish(),
    German(),
    Portuguese(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct MessagePayload {
    pub value: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MessageState {
    Scheduled(),
    Enroute(),
    Delivered(),
    Expired(),
    Deleted(),
    Undeliverable(),
    Accepted(),
    Unknown(),
    Rejected(),
    Skipped(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MoreMessagesToSend {
    NoMoreMessagesToFollow(),
    MoreMessagesToFollow(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MsAvailabilityStatus {
    Available(),
    Denied(),
    Unavailable(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Indicator {
    Inactive(),
    Active(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum TypeOfMessage {
    VoicemailMessageWaiting(),
    FaxMessageWaiting(),
    ElectronicMailMessageWaiting(),
    OtherMessageWaiting(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct MsMsgWaitFacilities {
    pub indicator: Indicator,
    pub type_of_message: TypeOfMessage,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MsValidityBehavior {
    StoreIndefinitely(),
    PowerDown(),
    ValidUntilRegistrationAreaChanges(),
    DisplayOnly(),
    RelativeTimePeriod(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum UnitsOfTime {
    Seconds(),
    Minutes(),
    Hours(),
    Days(),
    Weeks(),
    Months(),
    Years(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct MsValidityInformation {
    pub units_of_time: UnitsOfTime,
    pub number_of_time_units: u16,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct MsValidity {
    pub validity_behavior: MsValidityBehavior,
    pub validity_information: Option<MsValidityInformation>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum ErrorCodeNetworkType {
    Ansi136AccessDeniedReason(),
    Is95AccessDeniedReason(),
    Gsm(),
    Ansi136CauseCode(),
    Is95CauseCode(),
    Ansi41Error(),
    SmppError(),
    MessageCenterSpecific(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct NetworkErrorCode {
    pub network_type: ErrorCodeNetworkType,
    pub error_code: u16,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum NetworkType {
    Unknown(),
    Gsm(),
    Ansi136(),
    Is95(),
    Pdc(),
    Phs(),
    IDen(),
    Amps(),
    PagingNetwork(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum NumberOfMessages {
    Allowed(u8),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum PayloadType {
    Default(),
    WcmpMessage(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum PrivacyIndicator {
    NotRestricted(),
    Restricted(),
    Confidential(),
    Secret(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum SetDpf {
    NotRequested(),
    Requested(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum SubaddressTag {
    NsapEven(),
    NsapOdd(),
    UserSpecified(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct Subaddress {
    pub tag: SubaddressTag,
    pub addr: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum TlvTag {
    DestAddrSubunit(),
    DestNetworkType(),
    DestBearerType(),
    DestTelematicsId(),
    SourceAddrSubunit(),
    SourceNetworkType(),
    SourceBearerType(),
    SourceTelematicsId(),
    QosTimeToLive(),
    PayloadType(),
    AdditionalStatusInfoText(),
    ReceiptedMessageId(),
    MsMsgWaitFacilities(),
    PrivacyIndicator(),
    SourceSubaddress(),
    DestSubaddress(),
    UserMessageReference(),
    UserResponseCode(),
    SourcePort(),
    DestPort(),
    SarMsgRefNum(),
    LanguageIndicator(),
    SarTotalSegments(),
    SarSegmentSeqnum(),
    ScInterfaceVersion(),
    CallbackNumPresInd(),
    CallbackNumAtag(),
    NumberOfMessages(),
    CallbackNum(),
    DpfResult(),
    SetDpf(),
    MsAvailabilityStatus(),
    NetworkErrorCode(),
    MessagePayload(),
    DeliveryFailureReason(),
    MoreMessagesToSend(),
    MessageState(),
    CongestionState(),
    UssdServiceOp(),
    BroadcastChannelIndicator(),
    BroadcastContentType(),
    BroadcastContentTypeInfo(),
    BroadcastMessageClass(),
    BroadcastRepNum(),
    BroadcastFrequencyInterval(),
    BroadcastAreaIdentifier(),
    BroadcastErrorStatus(),
    BroadcastAreaSuccess(),
    BroadcastEndTime(),
    BroadcastServiceGroup(),
    BillingIdentification(),
    SourceNetworkId(),
    DestNetworkId(),
    SourceNodeId(),
    DestNodeId(),
    DestAddrNpResolution(),
    DestAddrNpInformation(),
    DestAddrNpCountry(),
    DisplayTime(),
    SmsSignal(),
    MsValidity(),
    AlertOnMessageDelivery(),
    ItsReplyType(),
    ItsSessionInfo(),
    Other(u16),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct UserMessageReference {
    pub value: u16,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum UssdServiceOp {
    PssdIndication(),
    PssrIndication(),
    UssrRequest(),
    UssnRequest(),
    PssdResponse(),
    PssrResponse(),
    UssrConfirm(),
    UssnConfirm(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum TlvValue {
    AdditionalStatusInfoText(Vec<u8>),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BillingIdentification(Vec<u8>),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastContentTypeInfo(Vec<u8>),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentType(BroadcastContentType),
    BroadcastEndTime(Vec<u8>),
    BroadcastErrorStatus(CommandStatus),
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastRepNum(BroadcastRepNum),
    BroadcastServiceGroup(Vec<u8>),
    CallbackNum(Vec<u8>),
    CallbackNumAtag(Vec<u8>),
    CallbackNumPresInd(CallbackNumPresInd),
    CongestionState(CongestionState),
    DeliveryFailureReason(DeliveryFailureReason),
    DestAddrNpCountry(Vec<u8>),
    DestAddrNpInformation(Vec<u8>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestBearerType(BearerType),
    DestNetworkId(Vec<u8>),
    DestNetworkType(NetworkType),
    DestNodeId(Vec<u8>),
    DestSubaddress(Subaddress),
    DestTelematicsId(u16),
    DestPort(u16),
    DisplayTime(DisplayTime),
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload),
    MessageState(MessageState),
    MoreMessagesToSend(MoreMessagesToSend),
    MsAvailabilityStatus(MsAvailabilityStatus),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    MsValidity(MsValidity),
    NetworkErrorCode(NetworkErrorCode),
    NumberOfMessages(NumberOfMessages),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    QosTimeToLive(u32),
    ReceiptedMessageId(Vec<u8>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    ScInterfaceVersion(InterfaceVersion),
    SetDpf(SetDpf),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourceBearerType(BearerType),
    SourceNetworkId(Vec<u8>),
    SourceNetworkType(NetworkType),
    SourceNodeId(Vec<u8>),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    SourceTelematicsId(u16),
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
    Other { tag: TlvTag, value: Vec<u8> },
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Npi {
    Unknown(),
    Isdn(),
    Data(),
    Telex(),
    LandMobile(),
    National(),
    Private(),
    Ermes(),
    Internet(),
    WapClientId(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct Tlv {
    pub tag: TlvTag,
    pub value_length: u16,
    pub value: Option<TlvValue>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Ton {
    Unknown(),
    International(),
    National(),
    NetworkSpecific(),
    SubscriberNumber(),
    Alphanumeric(),
    Abbreviated(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct AlertNotification {
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub esme_addr_ton: Ton,
    pub esme_addr_npi: Npi,
    pub esme_addr: Vec<u8>,
    pub ms_availability_status: Option<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BindReceiver {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
    pub system_type: Vec<u8>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BindReceiverResp {
    pub system_id: Vec<u8>,
    pub sc_interface_version: Option<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BindTransceiver {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
    pub system_type: Vec<u8>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BindTransceiverResp {
    pub system_id: Vec<u8>,
    pub sc_interface_version: Option<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BindTransmitter {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
    pub system_type: Vec<u8>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BindTransmitterResp {
    pub system_id: Vec<u8>,
    pub sc_interface_version: Option<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DataCoding {
    McSpecific(),
    Ia5(),
    OctetUnspecified(),
    Latin1(),
    OctetUnspecified2(),
    Jis(),
    Cyrillic(),
    LatinHebrew(),
    Ucs2(),
    PictogramEncoding(),
    Iso2022JpMusicCodes(),
    ExtendedKanjiJis(),
    Ksc5601(),
    GsmMwiControl(),
    GsmMwiControl2(),
    GsmMessageClassControl(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct PriorityFlag {
    pub value: u8,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum ReplaceIfPresentFlag {
    DoNotReplace(),
    Replace(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct ServiceType {
    pub value: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BroadcastSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub message_id: Vec<u8>,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct BroadcastSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct CancelBroadcastSm {
    pub service_type: ServiceType,
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct CancelSm {
    pub service_type: ServiceType,
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum CommandId {
    BindReceiver(),
    BindTransmitter(),
    QuerySm(),
    SubmitSm(),
    DeliverSm(),
    Unbind(),
    ReplaceSm(),
    CancelSm(),
    BindTransceiver(),
    Outbind(),
    EnquireLink(),
    SubmitMulti(),
    AlertNotification(),
    DataSm(),
    BroadcastSm(),
    QueryBroadcastSm(),
    CancelBroadcastSm(),
    GenericNack(),
    BindReceiverResp(),
    BindTransmitterResp(),
    QuerySmResp(),
    SubmitSmResp(),
    DeliverSmResp(),
    UnbindResp(),
    ReplaceSmResp(),
    CancelSmResp(),
    BindTransceiverResp(),
    EnquireLinkResp(),
    SubmitMultiResp(),
    DataSmResp(),
    BroadcastSmResp(),
    QueryBroadcastSmResp(),
    CancelBroadcastSmResp(),
    Other(u32),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Ansi41Specific {
    ShortMessageContainsDeliveryAcknowledgement(),
    ShortMessageContainsUserAcknowledgment(),
    ShortMessageContainsConversationAbort(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum GsmFeatures {
    NotSelected(),
    UdhiIndicator(),
    SetReplyPath(),
    SetUdhiAndReplyPath(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MessageType {
    Default(),
    ShortMessageContainsMCDeliveryReceipt(),
    ShortMessageContainsIntermediateDeliveryNotification(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MessagingMode {
    Default(),
    Datagram(),
    Forward(),
    StoreAndForward(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct EsmClass {
    pub messaging_mode: MessagingMode,
    pub message_type: MessageType,
    pub ansi41_specific: Ansi41Specific,
    pub gsm_features: GsmFeatures,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum IntermediateNotification {
    NoIntermediaryNotificationRequested(),
    IntermediateNotificationRequested(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum MCDeliveryReceipt {
    NoMcDeliveryReceiptRequested(),
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure(),
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure(),
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum SmeOriginatedAcknowledgement {
    NoReceiptSmeAcknowledgementRequested(),
    SmeDeliveryAcknowledgementRequested(),
    SmeUserAcknowledgementRequested(),
    BothDeliveryAndUserAcknowledgmentRequested(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct RegisteredDelivery {
    pub mc_delivery_receipt: MCDeliveryReceipt,
    pub sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
    pub intermediate_notification: IntermediateNotification,
    pub other: u8,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct DataSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub esm_class: EsmClass,
    pub registered_delivery: RegisteredDelivery,
    pub data_coding: DataCoding,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct DataSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct DeliverSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct DeliverSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct Outbind {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct QueryBroadcastSm {
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub user_message_reference: Option<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct QueryBroadcastSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct QuerySm {
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct QuerySmResp {
    pub message_id: Vec<u8>,
    pub final_date: Vec<u8>,
    pub message_state: MessageState,
    pub error_code: u8,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct ReplaceSm {
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub message_payload: Option<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DestFlag {
    SmeAddress(),
    DistributionListName(),
    Other(u8),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct DistributionListName {
    pub dest_flag: DestFlag,
    pub dl_name: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct SmeAddress {
    pub dest_flag: DestFlag,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum DestAddress {
    SmeAddress(SmeAddress),
    DistributionListName(DistributionListName),
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct SubmitMulti {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub number_of_dests: u8,
    pub dest_address: Vec<DestAddress>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct UnsuccessSme {
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub error_status_code: CommandStatus,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct SubmitMultiResp {
    pub message_id: Vec<u8>,
    pub no_unsuccess: u8,
    pub unsuccess_sme: Vec<UnsuccessSme>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct SubmitSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct SubmitSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub enum Pdu {
    BindTransmitter(BindTransmitter),
    BindTransmitterResp(BindTransmitterResp),
    BindReceiver(BindReceiver),
    BindReceiverResp(BindReceiverResp),
    BindTransceiver(BindTransceiver),
    BindTransceiverResp(BindTransceiverResp),
    Outbind(Outbind),
    AlertNotification(AlertNotification),
    SubmitSm(SubmitSm),
    SubmitSmResp(SubmitSmResp),
    QuerySm(QuerySm),
    QuerySmResp(QuerySmResp),
    DeliverSm(DeliverSm),
    DeliverSmResp(DeliverSmResp),
    DataSm(DataSm),
    DataSmResp(DataSmResp),
    CancelSm(CancelSm),
    ReplaceSm(ReplaceSm),
    SubmitMulti(SubmitMulti),
    SubmitMultiResp(SubmitMultiResp),
    BroadcastSm(BroadcastSm),
    BroadcastSmResp(BroadcastSmResp),
    QueryBroadcastSm(QueryBroadcastSm),
    QueryBroadcastSmResp(QueryBroadcastSmResp),
    CancelBroadcastSm(CancelBroadcastSm),
    Unbind(),
    UnbindResp(),
    EnquireLink(),
    EnquireLinkResp(),
    GenericNack(),
    CancelSmResp(),
    ReplaceSmResp(),
    CancelBroadcastSmResp(),
    Other {
        command_id: CommandId,
        body: Vec<u8>,
    },
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass]
pub struct Command {
    pub id: CommandId,
    pub status: CommandStatus,
    pub sequence_number: u32,
    pub pdu: Option<Pdu>,
}
