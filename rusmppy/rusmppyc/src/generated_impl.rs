use rusmpp::{
    tlvs::{MessageSubmissionRequestTlvValue, TlvTag},
    types::{COctetString, OctetString},
    values::*,
    CommandStatus,
};

use crate::{
    exception::{Exception, ValueExceptionExt},
    generated::{
        AddrSubunit as GAddrSubunit, AlertOnMessageDelivery as GAlertOnMessageDelivery,
        BearerType as GBearerType, CallbackNumPresInd as GCallbackNumPresInd,
        CommandStatus as GCommandStatus, DataCoding as GDataCoding,
        DestAddrNpResolution as GDestAddrNpResolution, DisplayTime as GDisplayTime,
        InterfaceVersion as GInterfaceVersion, ItsReplyType as GItsReplyType,
        ItsSessionInfo as GItsSessionInfo, LanguageIndicator as GLanguageIndicator,
        MessageSubmissionRequestTlvValue as GMessageSubmissionRequestTlvValue,
        NetworkType as GNetworkType, Npi as GNpi, Presentation as GPresentation,
        PrivacyIndicator as GPrivacyIndicator, Screening as GScreening, Subaddress as GSubaddress,
        SubaddressTag as GSubaddressTag, TlvTag as GeneratedTlvTag, Ton as GTon,
    },
};

impl From<GCommandStatus> for CommandStatus {
    fn from(value: GCommandStatus) -> Self {
        match value {
            GCommandStatus::EsmeRok() => Self::EsmeRok,
            GCommandStatus::EsmeRinvmsglen() => Self::EsmeRinvmsglen,
            GCommandStatus::EsmeRinvcmdlen() => Self::EsmeRinvcmdlen,
            GCommandStatus::EsmeRinvcmdid() => Self::EsmeRinvcmdid,
            GCommandStatus::EsmeRinvbndsts() => Self::EsmeRinvbndsts,
            GCommandStatus::EsmeRalybnd() => Self::EsmeRalybnd,
            GCommandStatus::EsmeRinvprtflg() => Self::EsmeRinvprtflg,
            GCommandStatus::EsmeRinvregdlvflg() => Self::EsmeRinvregdlvflg,
            GCommandStatus::EsmeRsyserr() => Self::EsmeRsyserr,
            GCommandStatus::EsmeRinvsrcadr() => Self::EsmeRinvsrcadr,
            GCommandStatus::EsmeRinvdstadr() => Self::EsmeRinvdstadr,
            GCommandStatus::EsmeRinvmsgid() => Self::EsmeRinvmsgid,
            GCommandStatus::EsmeRbindfail() => Self::EsmeRbindfail,
            GCommandStatus::EsmeRinvpaswd() => Self::EsmeRinvpaswd,
            GCommandStatus::EsmeRinvsysid() => Self::EsmeRinvsysid,
            GCommandStatus::EsmeRcancelfail() => Self::EsmeRcancelfail,
            GCommandStatus::EsmeRreplacefail() => Self::EsmeRreplacefail,
            GCommandStatus::EsmeRmsgqful() => Self::EsmeRmsgqful,
            GCommandStatus::EsmeRinvsertyp() => Self::EsmeRinvsertyp,
            GCommandStatus::EsmeRinvnumdests() => Self::EsmeRinvnumdests,
            GCommandStatus::EsmeRinvdlname() => Self::EsmeRinvdlname,
            GCommandStatus::EsmeRinvdestflag() => Self::EsmeRinvdestflag,
            GCommandStatus::EsmeRinvsubrep() => Self::EsmeRinvsubrep,
            GCommandStatus::EsmeRinvesmclass() => Self::EsmeRinvesmclass,
            GCommandStatus::EsmeRcntsubdl() => Self::EsmeRcntsubdl,
            GCommandStatus::EsmeRsubmitfail() => Self::EsmeRsubmitfail,
            GCommandStatus::EsmeRinvsrcton() => Self::EsmeRinvsrcton,
            GCommandStatus::EsmeRinvsrcnpi() => Self::EsmeRinvsrcnpi,
            GCommandStatus::EsmeRinvdstton() => Self::EsmeRinvdstton,
            GCommandStatus::EsmeRinvdstnpi() => Self::EsmeRinvdstnpi,
            GCommandStatus::EsmeRinvsystyp() => Self::EsmeRinvsystyp,
            GCommandStatus::EsmeRinvrepflag() => Self::EsmeRinvrepflag,
            GCommandStatus::EsmeRinvnummsgs() => Self::EsmeRinvnummsgs,
            GCommandStatus::EsmeRthrottled() => Self::EsmeRthrottled,
            GCommandStatus::EsmeRinvsched() => Self::EsmeRinvsched,
            GCommandStatus::EsmeRinvexpiry() => Self::EsmeRinvexpiry,
            GCommandStatus::EsmeRinvdftmsgid() => Self::EsmeRinvdftmsgid,
            GCommandStatus::EsmeRxTAppn() => Self::EsmeRxTAppn,
            GCommandStatus::EsmeRxPAppn() => Self::EsmeRxPAppn,
            GCommandStatus::EsmeRxRAppn() => Self::EsmeRxRAppn,
            GCommandStatus::EsmeRqueryfail() => Self::EsmeRqueryfail,
            GCommandStatus::EsmeRinvtlvstream() => Self::EsmeRinvtlvstream,
            GCommandStatus::EsmeRtlvnotallwd() => Self::EsmeRtlvnotallwd,
            GCommandStatus::EsmeRinvtlvlen() => Self::EsmeRinvtlvlen,
            GCommandStatus::EsmeRmissingtlv() => Self::EsmeRmissingtlv,
            GCommandStatus::EsmeRinvtlvval() => Self::EsmeRinvtlvval,
            GCommandStatus::EsmeRdeliveryfailure() => Self::EsmeRdeliveryfailure,
            GCommandStatus::EsmeRunknownerr() => Self::EsmeRunknownerr,
            GCommandStatus::EsmeRsertypunauth() => Self::EsmeRsertypunauth,
            GCommandStatus::EsmeRprohibited() => Self::EsmeRprohibited,
            GCommandStatus::EsmeRsertypunavail() => Self::EsmeRsertypunavail,
            GCommandStatus::EsmeRsertypdenied() => Self::EsmeRsertypdenied,
            GCommandStatus::EsmeRinvdcs() => Self::EsmeRinvdcs,
            GCommandStatus::EsmeRinvsrcaddrsubunit() => Self::EsmeRinvsrcaddrsubunit,
            GCommandStatus::EsmeRinvdstaddrsubunit() => Self::EsmeRinvdstaddrsubunit,
            GCommandStatus::EsmeRinvbcastfreqint() => Self::EsmeRinvbcastfreqint,
            GCommandStatus::EsmeRinvbcastaliasName() => Self::EsmeRinvbcastaliasName,
            GCommandStatus::EsmeRinvbcastareafmt() => Self::EsmeRinvbcastareafmt,
            GCommandStatus::EsmeRinvnumbcastAreas() => Self::EsmeRinvnumbcastAreas,
            GCommandStatus::EsmeRinvbcastcnttype() => Self::EsmeRinvbcastcnttype,
            GCommandStatus::EsmeRinvbcastmsgclass() => Self::EsmeRinvbcastmsgclass,
            GCommandStatus::EsmeRbcastfail() => Self::EsmeRbcastfail,
            GCommandStatus::EsmeRbcastqueryfail() => Self::EsmeRbcastqueryfail,
            GCommandStatus::EsmeRbcastcancelfail() => Self::EsmeRbcastcancelfail,
            GCommandStatus::EsmeRinvbcastRep() => Self::EsmeRinvbcastRep,
            GCommandStatus::EsmeRinvbcastsrvgrp() => Self::EsmeRinvbcastsrvgrp,
            GCommandStatus::EsmeRinvbcastchanind() => Self::EsmeRinvbcastchanind,
            GCommandStatus::Other(value) => Self::Other(value),
        }
    }
}

impl From<GeneratedTlvTag> for TlvTag {
    fn from(value: GeneratedTlvTag) -> Self {
        match value {
            GeneratedTlvTag::DestAddrSubunit() => Self::DestAddrSubunit,
            GeneratedTlvTag::DestNetworkType() => Self::DestNetworkType,
            GeneratedTlvTag::DestBearerType() => Self::DestBearerType,
            GeneratedTlvTag::DestTelematicsId() => Self::DestTelematicsId,
            GeneratedTlvTag::SourceAddrSubunit() => Self::SourceAddrSubunit,
            GeneratedTlvTag::SourceNetworkType() => Self::SourceNetworkType,
            GeneratedTlvTag::SourceBearerType() => Self::SourceBearerType,
            GeneratedTlvTag::SourceTelematicsId() => Self::SourceTelematicsId,
            GeneratedTlvTag::QosTimeToLive() => Self::QosTimeToLive,
            GeneratedTlvTag::PayloadType() => Self::PayloadType,
            GeneratedTlvTag::AdditionalStatusInfoText() => Self::AdditionalStatusInfoText,
            GeneratedTlvTag::ReceiptedMessageId() => Self::ReceiptedMessageId,
            GeneratedTlvTag::MsMsgWaitFacilities() => Self::MsMsgWaitFacilities,
            GeneratedTlvTag::PrivacyIndicator() => Self::PrivacyIndicator,
            GeneratedTlvTag::SourceSubaddress() => Self::SourceSubaddress,
            GeneratedTlvTag::DestSubaddress() => Self::DestSubaddress,
            GeneratedTlvTag::UserMessageReference() => Self::UserMessageReference,
            GeneratedTlvTag::UserResponseCode() => Self::UserResponseCode,
            GeneratedTlvTag::SourcePort() => Self::SourcePort,
            GeneratedTlvTag::DestPort() => Self::DestPort,
            GeneratedTlvTag::SarMsgRefNum() => Self::SarMsgRefNum,
            GeneratedTlvTag::LanguageIndicator() => Self::LanguageIndicator,
            GeneratedTlvTag::SarTotalSegments() => Self::SarTotalSegments,
            GeneratedTlvTag::SarSegmentSeqnum() => Self::SarSegmentSeqnum,
            GeneratedTlvTag::ScInterfaceVersion() => Self::ScInterfaceVersion,
            GeneratedTlvTag::CallbackNumPresInd() => Self::CallbackNumPresInd,
            GeneratedTlvTag::CallbackNumAtag() => Self::CallbackNumAtag,
            GeneratedTlvTag::NumberOfMessages() => Self::NumberOfMessages,
            GeneratedTlvTag::CallbackNum() => Self::CallbackNum,
            GeneratedTlvTag::DpfResult() => Self::DpfResult,
            GeneratedTlvTag::SetDpf() => Self::SetDpf,
            GeneratedTlvTag::MsAvailabilityStatus() => Self::MsAvailabilityStatus,
            GeneratedTlvTag::NetworkErrorCode() => Self::NetworkErrorCode,
            GeneratedTlvTag::MessagePayload() => Self::MessagePayload,
            GeneratedTlvTag::DeliveryFailureReason() => Self::DeliveryFailureReason,
            GeneratedTlvTag::MoreMessagesToSend() => Self::MoreMessagesToSend,
            GeneratedTlvTag::MessageState() => Self::MessageState,
            GeneratedTlvTag::CongestionState() => Self::CongestionState,
            GeneratedTlvTag::UssdServiceOp() => Self::UssdServiceOp,
            GeneratedTlvTag::BroadcastChannelIndicator() => Self::BroadcastChannelIndicator,
            GeneratedTlvTag::BroadcastContentType() => Self::BroadcastContentType,
            GeneratedTlvTag::BroadcastContentTypeInfo() => Self::BroadcastContentTypeInfo,
            GeneratedTlvTag::BroadcastMessageClass() => Self::BroadcastMessageClass,
            GeneratedTlvTag::BroadcastRepNum() => Self::BroadcastRepNum,
            GeneratedTlvTag::BroadcastFrequencyInterval() => Self::BroadcastFrequencyInterval,
            GeneratedTlvTag::BroadcastAreaIdentifier() => Self::BroadcastAreaIdentifier,
            GeneratedTlvTag::BroadcastErrorStatus() => Self::BroadcastErrorStatus,
            GeneratedTlvTag::BroadcastAreaSuccess() => Self::BroadcastAreaSuccess,
            GeneratedTlvTag::BroadcastEndTime() => Self::BroadcastEndTime,
            GeneratedTlvTag::BroadcastServiceGroup() => Self::BroadcastServiceGroup,
            GeneratedTlvTag::BillingIdentification() => Self::BillingIdentification,
            GeneratedTlvTag::SourceNetworkId() => Self::SourceNetworkId,
            GeneratedTlvTag::DestNetworkId() => Self::DestNetworkId,
            GeneratedTlvTag::SourceNodeId() => Self::SourceNodeId,
            GeneratedTlvTag::DestNodeId() => Self::DestNodeId,
            GeneratedTlvTag::DestAddrNpResolution() => Self::DestAddrNpResolution,
            GeneratedTlvTag::DestAddrNpInformation() => Self::DestAddrNpInformation,
            GeneratedTlvTag::DestAddrNpCountry() => Self::DestAddrNpCountry,
            GeneratedTlvTag::DisplayTime() => Self::DisplayTime,
            GeneratedTlvTag::SmsSignal() => Self::SmsSignal,
            GeneratedTlvTag::MsValidity() => Self::MsValidity,
            GeneratedTlvTag::AlertOnMessageDelivery() => Self::AlertOnMessageDelivery,
            GeneratedTlvTag::ItsReplyType() => Self::ItsReplyType,
            GeneratedTlvTag::ItsSessionInfo() => Self::ItsSessionInfo,
            GeneratedTlvTag::Other(tag) => Self::Other(tag),
        }
    }
}

impl From<GInterfaceVersion> for InterfaceVersion {
    fn from(value: GInterfaceVersion) -> Self {
        match value {
            GInterfaceVersion::Smpp3_3OrEarlier(value) => Self::Smpp3_3OrEarlier(value),
            GInterfaceVersion::Smpp3_4() => Self::Smpp3_4,
            GInterfaceVersion::Smpp5_0() => Self::Smpp5_0,
            GInterfaceVersion::Other(value) => Self::Other(value),
        }
    }
}

impl From<GTon> for Ton {
    fn from(value: GTon) -> Self {
        match value {
            GTon::Unknown() => Self::Unknown,
            GTon::International() => Self::International,
            GTon::National() => Self::National,
            GTon::NetworkSpecific() => Self::NetworkSpecific,
            GTon::SubscriberNumber() => Self::SubscriberNumber,
            GTon::Alphanumeric() => Self::Alphanumeric,
            GTon::Abbreviated() => Self::Abbreviated,
            GTon::Other(value) => Self::Other(value),
        }
    }
}

impl From<GNpi> for Npi {
    fn from(value: GNpi) -> Self {
        match value {
            GNpi::Unknown() => Self::Unknown,
            GNpi::Isdn() => Self::Isdn,
            GNpi::Data() => Self::Data,
            GNpi::Telex() => Self::Telex,
            GNpi::LandMobile() => Self::LandMobile,
            GNpi::National() => Self::National,
            GNpi::Private() => Self::Private,
            GNpi::Ermes() => Self::Ermes,
            GNpi::Internet() => Self::Internet,
            GNpi::WapClientId() => Self::WapClientId,
            GNpi::Other(value) => Self::Other(value),
        }
    }
}

impl From<GDataCoding> for DataCoding {
    fn from(value: GDataCoding) -> Self {
        match value {
            GDataCoding::McSpecific() => Self::McSpecific,
            GDataCoding::Ia5() => Self::Ia5,
            GDataCoding::OctetUnspecified() => Self::OctetUnspecified,
            GDataCoding::Latin1() => Self::Latin1,
            GDataCoding::OctetUnspecified2() => Self::OctetUnspecified2,
            GDataCoding::Jis() => Self::Jis,
            GDataCoding::Cyrillic() => Self::Cyrillic,
            GDataCoding::LatinHebrew() => Self::LatinHebrew,
            GDataCoding::Ucs2() => Self::Ucs2,
            GDataCoding::PictogramEncoding() => Self::PictogramEncoding,
            GDataCoding::Iso2022JpMusicCodes() => Self::Iso2022JpMusicCodes,
            GDataCoding::ExtendedKanjiJis() => Self::ExtendedKanjiJis,
            GDataCoding::Ksc5601() => Self::Ksc5601,
            GDataCoding::GsmMwiControl() => Self::GsmMwiControl,
            GDataCoding::GsmMwiControl2() => Self::GsmMwiControl2,
            GDataCoding::GsmMessageClassControl() => Self::GsmMessageClassControl,
            GDataCoding::Other(value) => Self::Other(value),
        }
    }
}

impl From<GAlertOnMessageDelivery> for AlertOnMessageDelivery {
    fn from(value: GAlertOnMessageDelivery) -> Self {
        match value {
            GAlertOnMessageDelivery::UseMobileDefaultAlert() => Self::UseMobileDefaultAlert,
            GAlertOnMessageDelivery::UseLowPriorityAlert() => Self::UseLowPriorityAlert,
            GAlertOnMessageDelivery::UseMediumPriorityAlert() => Self::UseMediumPriorityAlert,
            GAlertOnMessageDelivery::UseHighPriorityAlert() => Self::UseHighPriorityAlert,
            GAlertOnMessageDelivery::Other(value) => Self::Other(value),
        }
    }
}

impl From<GScreening> for Screening {
    fn from(value: GScreening) -> Self {
        match value {
            GScreening::NotScreened() => Self::NotScreened,
            GScreening::VerifiedAndPassed() => Self::VerifiedAndPassed,
            GScreening::VerifiedAndFailed() => Self::VerifiedAndFailed,
            GScreening::NetworkProvided() => Self::NetworkProvided,
            GScreening::Other(value) => Self::Other(value),
        }
    }
}

impl From<GPresentation> for Presentation {
    fn from(value: GPresentation) -> Self {
        match value {
            GPresentation::PresentationAllowed() => Self::PresentationAllowed,
            GPresentation::PresentationRestricted() => Self::PresentationRestricted,
            GPresentation::NumberNotAvailable() => Self::NumberNotAvailable,
            GPresentation::Other(value) => Self::Other(value),
        }
    }
}

impl From<GCallbackNumPresInd> for CallbackNumPresInd {
    fn from(value: GCallbackNumPresInd) -> Self {
        Self {
            presentation: Presentation::from(value.presentation),
            screening: Screening::from(value.screening),
        }
    }
}

impl From<GDestAddrNpResolution> for DestAddrNpResolution {
    fn from(value: GDestAddrNpResolution) -> Self {
        match value {
            GDestAddrNpResolution::QueryNotPerformed() => Self::QueryNotPerformed,
            GDestAddrNpResolution::QueryPerformedNumberNotPorted() => {
                Self::QueryPerformedNumberNotPorted
            }
            GDestAddrNpResolution::QueryPerformedNumberPorted() => Self::QueryPerformedNumberPorted,
            GDestAddrNpResolution::Other(value) => Self::Other(value),
        }
    }
}

impl From<GAddrSubunit> for AddrSubunit {
    fn from(value: GAddrSubunit) -> Self {
        match value {
            GAddrSubunit::Unknown() => Self::Unknown,
            GAddrSubunit::MSDisplay() => Self::MSDisplay,
            GAddrSubunit::MobileEquipment() => Self::MobileEquipment,
            GAddrSubunit::SmartCard() => Self::SmartCard,
            GAddrSubunit::ExternalUnit() => Self::ExternalUnit,
            GAddrSubunit::Other(value) => Self::Other(value),
        }
    }
}

impl From<GBearerType> for BearerType {
    fn from(value: GBearerType) -> Self {
        match value {
            GBearerType::Unknown() => Self::Unknown,
            GBearerType::Sms() => Self::Sms,
            GBearerType::Csd() => Self::Csd,
            GBearerType::PacketData() => Self::PacketData,
            GBearerType::Ussd() => Self::Ussd,
            GBearerType::Cdpd() => Self::Cdpd,
            GBearerType::DataTac() => Self::DataTac,
            GBearerType::FlexReFlex() => Self::FlexReFlex,
            GBearerType::CellBroadcast() => Self::CellBroadcast,
            GBearerType::Other(value) => Self::Other(value),
        }
    }
}

impl From<GNetworkType> for NetworkType {
    fn from(value: GNetworkType) -> Self {
        match value {
            GNetworkType::Unknown() => Self::Unknown,
            GNetworkType::Gsm() => Self::Gsm,
            GNetworkType::Ansi136() => Self::Ansi136,
            GNetworkType::Is95() => Self::Is95,
            GNetworkType::Pdc() => Self::Pdc,
            GNetworkType::Phs() => Self::Phs,
            GNetworkType::IDen() => Self::IDen,
            GNetworkType::Amps() => Self::Amps,
            GNetworkType::PagingNetwork() => Self::PagingNetwork,
            GNetworkType::Other(value) => Self::Other(value),
        }
    }
}

impl From<GSubaddressTag> for SubaddressTag {
    fn from(value: GSubaddressTag) -> Self {
        match value {
            GSubaddressTag::NsapEven() => Self::NsapEven,
            GSubaddressTag::NsapOdd() => Self::NsapOdd,
            GSubaddressTag::UserSpecified() => Self::UserSpecified,
            GSubaddressTag::Other(value) => Self::Other(value),
        }
    }
}

impl TryFrom<GSubaddress> for Subaddress {
    type Error = Exception;

    fn try_from(value: GSubaddress) -> Result<Self, Self::Error> {
        Ok(Self {
            tag: value.tag.into(),
            addr: OctetString::new(value.addr).map_value_err("addr")?,
        })
    }
}

impl From<GDisplayTime> for DisplayTime {
    fn from(value: GDisplayTime) -> Self {
        match value {
            GDisplayTime::Temporary() => Self::Temporary,
            GDisplayTime::Default() => Self::Default,
            GDisplayTime::Invoke() => Self::Invoke,
            GDisplayTime::Other(value) => Self::Other(value),
        }
    }
}

impl From<GItsSessionInfo> for ItsSessionInfo {
    fn from(value: GItsSessionInfo) -> Self {
        Self {
            session_number: value.session_number,
            sequence_number: value.sequence_number,
        }
    }
}

impl From<GLanguageIndicator> for LanguageIndicator {
    fn from(value: GLanguageIndicator) -> Self {
        match value {
            GLanguageIndicator::Unspecified() => Self::Unspecified,
            GLanguageIndicator::English() => Self::English,
            GLanguageIndicator::French() => Self::French,
            GLanguageIndicator::Spanish() => Self::Spanish,
            GLanguageIndicator::German() => Self::German,
            GLanguageIndicator::Portuguese() => Self::Portuguese,
            GLanguageIndicator::Other(value) => Self::Other(value),
        }
    }
}

impl From<GPrivacyIndicator> for PrivacyIndicator {
    fn from(value: GPrivacyIndicator) -> Self {
        match value {
            GPrivacyIndicator::NotRestricted() => Self::NotRestricted,
            GPrivacyIndicator::Restricted() => Self::Restricted,
            GPrivacyIndicator::Confidential() => Self::Confidential,
            GPrivacyIndicator::Secret() => Self::Secret,
            GPrivacyIndicator::Other(value) => Self::Other(value),
        }
    }
}

impl From<GItsReplyType> for ItsReplyType {
    fn from(value: GItsReplyType) -> Self {
        match value {
            GItsReplyType::Digit() => Self::Digit,
            GItsReplyType::Number() => Self::Number,
            GItsReplyType::TelephoneNo() => Self::TelephoneNo,
            GItsReplyType::Password() => Self::Password,
            GItsReplyType::CharacterLine() => Self::CharacterLine,
            GItsReplyType::Menu() => Self::Menu,
            GItsReplyType::Date() => Self::Date,
            GItsReplyType::Time() => Self::Time,
            GItsReplyType::Continue() => Self::Continue,
            GItsReplyType::Other(value) => Self::Other(value),
        }
    }
}

impl TryFrom<GMessageSubmissionRequestTlvValue> for MessageSubmissionRequestTlvValue {
    type Error = Exception;

    fn try_from(value: GMessageSubmissionRequestTlvValue) -> Result<Self, Self::Error> {
        use GMessageSubmissionRequestTlvValue as GValue;

        let value = match value {
            GValue::AlertOnMessageDelivery(value) => Self::AlertOnMessageDelivery(value.into()),
            GValue::BillingIdentification(value) => Self::BillingIdentification(
                OctetString::new(value).map_value_err("billing_identification")?,
            ),
            GValue::CallbackNum(value) => {
                Self::CallbackNum(OctetString::new(value).map_value_err("callback_num")?)
            }
            GValue::CallbackNumAtag(value) => {
                Self::CallbackNumAtag(OctetString::new(value).map_value_err("callback_num_atag")?)
            }
            GValue::CallbackNumPresInd(value) => Self::CallbackNumPresInd(value.into()),
            GValue::DestAddrNpCountry(value) => Self::DestAddrNpCountry(
                OctetString::new(value).map_value_err("dest_addr_np_country")?,
            ),
            GValue::DestAddrNpInformation(value) => Self::DestAddrNpInformation(
                OctetString::new(value).map_value_err("dest_addr_np_information")?,
            ),
            GValue::DestAddrNpResolution(value) => Self::DestAddrNpResolution(value.into()),
            GValue::DestAddrSubunit(value) => Self::DestAddrSubunit(value.into()),
            GValue::DestBearerType(value) => Self::DestBearerType(value.into()),
            GValue::DestNetworkId(value) => {
                Self::DestNetworkId(COctetString::new(value).map_value_err("dest_network_id")?)
            }
            GValue::DestNetworkType(value) => Self::DestNetworkType(value.into()),
            GValue::DestNodeId(value) => {
                Self::DestNodeId(OctetString::new(value).map_value_err("dest_node_id")?)
            }
            GValue::DestSubaddress(value) => {
                Self::DestSubaddress(value.try_into().map_value_err("dest_subaddress")?)
            }
            GValue::DestTelematicsId(value) => Self::DestTelematicsId(value),
            GValue::DestPort(value) => Self::DestPort(value),
            GValue::DisplayTime(value) => Self::DisplayTime(value.into()),
            GValue::ItsReplyType(value) => Self::ItsReplyType(value.into()),
            GValue::ItsSessionInfo(value) => Self::ItsSessionInfo(value.into()),
            GValue::LanguageIndicator(value) => Self::LanguageIndicator(value.into()),
            GValue::MessagePayload(value) => todo!(),
            GValue::MoreMessagesToSend(value) => todo!(),
            GValue::MsMsgWaitFacilities(value) => todo!(),
            GValue::MsValidity(value) => todo!(),
            GValue::NumberOfMessages(value) => todo!(),
            GValue::PayloadType(value) => todo!(),
            GValue::PrivacyIndicator(value) => todo!(),
            GValue::QosTimeToLive(value) => Self::QosTimeToLive(value),
            GValue::SarMsgRefNum(value) => Self::SarMsgRefNum(value),
            GValue::SarSegmentSeqnum(value) => Self::SarSegmentSeqnum(value),
            GValue::SarTotalSegments(value) => Self::SarTotalSegments(value),
            GValue::SetDpf(value) => todo!(),
            GValue::SmsSignal(value) => Self::SmsSignal(value),
            GValue::SourceAddrSubunit(value) => todo!(),
            GValue::SourceBearerType(value) => todo!(),
            GValue::SourceNetworkId(value) => {
                Self::SourceNetworkId(COctetString::new(value).map_value_err("source_network_id")?)
            }
            GValue::SourceNetworkType(value) => todo!(),
            GValue::SourceNodeId(value) => {
                Self::SourceNodeId(OctetString::new(value).map_value_err("source_node_id")?)
            }
            GValue::SourcePort(value) => Self::SourcePort(value),
            GValue::SourceSubaddress(value) => todo!(),
            GValue::SourceTelematicsId(value) => Self::SourceTelematicsId(value),
            GValue::UserMessageReference(value) => todo!(),
            GValue::UserResponseCode(value) => Self::UserResponseCode(value),
            GValue::UssdServiceOp(value) => todo!(),
            GValue::Other { tag, value } => Self::Other {
                tag: tag.into(),
                value: value.into(),
            },
        };

        Ok(value)
    }
}
