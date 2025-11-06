use rusmpp::{
    tlvs::{MessageSubmissionRequestTlvValue, TlvTag},
    values::{DataCoding, InterfaceVersion, Npi, Ton},
    CommandStatus,
};

use crate::{
    exception::Exception,
    generated::{
        CommandStatus as GCommandStatus, DataCoding as GDataCoding,
        InterfaceVersion as GInterfaceVersion,
        MessageSubmissionRequestTlvValue as GMessageSubmissionRequestTlvValue, Npi as GeneratedNpi,
        TlvTag as GeneratedTlvTag, Ton as GeneratedTon,
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

impl From<GeneratedTon> for Ton {
    fn from(value: GeneratedTon) -> Self {
        match value {
            GeneratedTon::Unknown() => Self::Unknown,
            GeneratedTon::International() => Self::International,
            GeneratedTon::National() => Self::National,
            GeneratedTon::NetworkSpecific() => Self::NetworkSpecific,
            GeneratedTon::SubscriberNumber() => Self::SubscriberNumber,
            GeneratedTon::Alphanumeric() => Self::Alphanumeric,
            GeneratedTon::Abbreviated() => Self::Abbreviated,
            GeneratedTon::Other(value) => Self::Other(value),
        }
    }
}

impl From<GeneratedNpi> for Npi {
    fn from(value: GeneratedNpi) -> Self {
        match value {
            GeneratedNpi::Unknown() => Self::Unknown,
            GeneratedNpi::Isdn() => Self::Isdn,
            GeneratedNpi::Data() => Self::Data,
            GeneratedNpi::Telex() => Self::Telex,
            GeneratedNpi::LandMobile() => Self::LandMobile,
            GeneratedNpi::National() => Self::National,
            GeneratedNpi::Private() => Self::Private,
            GeneratedNpi::Ermes() => Self::Ermes,
            GeneratedNpi::Internet() => Self::Internet,
            GeneratedNpi::WapClientId() => Self::WapClientId,
            GeneratedNpi::Other(value) => Self::Other(value),
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

impl TryFrom<GMessageSubmissionRequestTlvValue> for MessageSubmissionRequestTlvValue {
    type Error = Exception;

    fn try_from(value: GMessageSubmissionRequestTlvValue) -> Result<Self, Self::Error> {
        use GMessageSubmissionRequestTlvValue as GValue;
        use MessageSubmissionRequestTlvValue as Value;

        let value = match value {
            GValue::AlertOnMessageDelivery(value) => todo!(),
            GValue::BillingIdentification(value) => todo!(),
            GValue::CallbackNum(value) => todo!(),
            GValue::CallbackNumAtag(value) => todo!(),
            GValue::CallbackNumPresInd(value) => todo!(),
            GValue::DestAddrNpCountry(value) => todo!(),
            GValue::DestAddrNpInformation(value) => todo!(),
            GValue::DestAddrNpResolution(value) => todo!(),
            GValue::DestAddrSubunit(value) => todo!(),
            GValue::DestBearerType(value) => todo!(),
            GValue::DestNetworkId(value) => todo!(),
            GValue::DestNetworkType(value) => todo!(),
            GValue::DestNodeId(value) => todo!(),
            GValue::DestSubaddress(value) => todo!(),
            GValue::DestTelematicsId(value) => todo!(),
            GValue::DestPort(value) => todo!(),
            GValue::DisplayTime(value) => todo!(),
            GValue::ItsReplyType(value) => todo!(),
            GValue::ItsSessionInfo(value) => todo!(),
            GValue::LanguageIndicator(value) => todo!(),
            GValue::MessagePayload(value) => todo!(),
            GValue::MoreMessagesToSend(value) => todo!(),
            GValue::MsMsgWaitFacilities(value) => todo!(),
            GValue::MsValidity(value) => todo!(),
            GValue::NumberOfMessages(value) => todo!(),
            GValue::PayloadType(value) => todo!(),
            GValue::PrivacyIndicator(value) => todo!(),
            GValue::QosTimeToLive(value) => todo!(),
            GValue::SarMsgRefNum(value) => todo!(),
            GValue::SarSegmentSeqnum(value) => todo!(),
            GValue::SarTotalSegments(value) => todo!(),
            GValue::SetDpf(value) => todo!(),
            GValue::SmsSignal(value) => todo!(),
            GValue::SourceAddrSubunit(value) => todo!(),
            GValue::SourceBearerType(value) => todo!(),
            GValue::SourceNetworkId(value) => todo!(),
            GValue::SourceNetworkType(value) => todo!(),
            GValue::SourceNodeId(value) => todo!(),
            GValue::SourcePort(value) => todo!(),
            GValue::SourceSubaddress(value) => todo!(),
            GValue::SourceTelematicsId(value) => todo!(),
            GValue::UserMessageReference(value) => todo!(),
            GValue::UserResponseCode(value) => todo!(),
            GValue::UssdServiceOp(value) => todo!(),
            GValue::Other { tag, value } => Value::Other {
                tag: tag.into(),
                value: value.into(),
            },
        };

        Ok(value)
    }
}
