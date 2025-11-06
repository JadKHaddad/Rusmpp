use rusmpp::{
    tlvs::MessageSubmissionRequestTlvValue,
    values::{DataCoding, InterfaceVersion, Npi, Ton},
    CommandStatus,
};

use crate::{
    exception::Exception,
    generated::{
        CommandStatus as GCommandStatus, DataCoding as GDataCoding,
        InterfaceVersion as GInterfaceVersion,
        MessageSubmissionRequestTlvValue as GMessageSubmissionRequestTlvValue, Npi as GeneratedNpi,
        Ton as GeneratedTon,
    },
};

impl From<GCommandStatus> for CommandStatus {
    fn from(value: GCommandStatus) -> Self {
        match value {
            GCommandStatus::EsmeRok() => CommandStatus::EsmeRok,
            GCommandStatus::EsmeRinvmsglen() => CommandStatus::EsmeRinvmsglen,
            GCommandStatus::EsmeRinvcmdlen() => CommandStatus::EsmeRinvcmdlen,
            GCommandStatus::EsmeRinvcmdid() => CommandStatus::EsmeRinvcmdid,
            GCommandStatus::EsmeRinvbndsts() => CommandStatus::EsmeRinvbndsts,
            GCommandStatus::EsmeRalybnd() => CommandStatus::EsmeRalybnd,
            GCommandStatus::EsmeRinvprtflg() => CommandStatus::EsmeRinvprtflg,
            GCommandStatus::EsmeRinvregdlvflg() => CommandStatus::EsmeRinvregdlvflg,
            GCommandStatus::EsmeRsyserr() => CommandStatus::EsmeRsyserr,
            GCommandStatus::EsmeRinvsrcadr() => CommandStatus::EsmeRinvsrcadr,
            GCommandStatus::EsmeRinvdstadr() => CommandStatus::EsmeRinvdstadr,
            GCommandStatus::EsmeRinvmsgid() => CommandStatus::EsmeRinvmsgid,
            GCommandStatus::EsmeRbindfail() => CommandStatus::EsmeRbindfail,
            GCommandStatus::EsmeRinvpaswd() => CommandStatus::EsmeRinvpaswd,
            GCommandStatus::EsmeRinvsysid() => CommandStatus::EsmeRinvsysid,
            GCommandStatus::EsmeRcancelfail() => CommandStatus::EsmeRcancelfail,
            GCommandStatus::EsmeRreplacefail() => CommandStatus::EsmeRreplacefail,
            GCommandStatus::EsmeRmsgqful() => CommandStatus::EsmeRmsgqful,
            GCommandStatus::EsmeRinvsertyp() => CommandStatus::EsmeRinvsertyp,
            GCommandStatus::EsmeRinvnumdests() => CommandStatus::EsmeRinvnumdests,
            GCommandStatus::EsmeRinvdlname() => CommandStatus::EsmeRinvdlname,
            GCommandStatus::EsmeRinvdestflag() => CommandStatus::EsmeRinvdestflag,
            GCommandStatus::EsmeRinvsubrep() => CommandStatus::EsmeRinvsubrep,
            GCommandStatus::EsmeRinvesmclass() => CommandStatus::EsmeRinvesmclass,
            GCommandStatus::EsmeRcntsubdl() => CommandStatus::EsmeRcntsubdl,
            GCommandStatus::EsmeRsubmitfail() => CommandStatus::EsmeRsubmitfail,
            GCommandStatus::EsmeRinvsrcton() => CommandStatus::EsmeRinvsrcton,
            GCommandStatus::EsmeRinvsrcnpi() => CommandStatus::EsmeRinvsrcnpi,
            GCommandStatus::EsmeRinvdstton() => CommandStatus::EsmeRinvdstton,
            GCommandStatus::EsmeRinvdstnpi() => CommandStatus::EsmeRinvdstnpi,
            GCommandStatus::EsmeRinvsystyp() => CommandStatus::EsmeRinvsystyp,
            GCommandStatus::EsmeRinvrepflag() => CommandStatus::EsmeRinvrepflag,
            GCommandStatus::EsmeRinvnummsgs() => CommandStatus::EsmeRinvnummsgs,
            GCommandStatus::EsmeRthrottled() => CommandStatus::EsmeRthrottled,
            GCommandStatus::EsmeRinvsched() => CommandStatus::EsmeRinvsched,
            GCommandStatus::EsmeRinvexpiry() => CommandStatus::EsmeRinvexpiry,
            GCommandStatus::EsmeRinvdftmsgid() => CommandStatus::EsmeRinvdftmsgid,
            GCommandStatus::EsmeRxTAppn() => CommandStatus::EsmeRxTAppn,
            GCommandStatus::EsmeRxPAppn() => CommandStatus::EsmeRxPAppn,
            GCommandStatus::EsmeRxRAppn() => CommandStatus::EsmeRxRAppn,
            GCommandStatus::EsmeRqueryfail() => CommandStatus::EsmeRqueryfail,
            GCommandStatus::EsmeRinvtlvstream() => CommandStatus::EsmeRinvtlvstream,
            GCommandStatus::EsmeRtlvnotallwd() => CommandStatus::EsmeRtlvnotallwd,
            GCommandStatus::EsmeRinvtlvlen() => CommandStatus::EsmeRinvtlvlen,
            GCommandStatus::EsmeRmissingtlv() => CommandStatus::EsmeRmissingtlv,
            GCommandStatus::EsmeRinvtlvval() => CommandStatus::EsmeRinvtlvval,
            GCommandStatus::EsmeRdeliveryfailure() => CommandStatus::EsmeRdeliveryfailure,
            GCommandStatus::EsmeRunknownerr() => CommandStatus::EsmeRunknownerr,
            GCommandStatus::EsmeRsertypunauth() => CommandStatus::EsmeRsertypunauth,
            GCommandStatus::EsmeRprohibited() => CommandStatus::EsmeRprohibited,
            GCommandStatus::EsmeRsertypunavail() => CommandStatus::EsmeRsertypunavail,
            GCommandStatus::EsmeRsertypdenied() => CommandStatus::EsmeRsertypdenied,
            GCommandStatus::EsmeRinvdcs() => CommandStatus::EsmeRinvdcs,
            GCommandStatus::EsmeRinvsrcaddrsubunit() => CommandStatus::EsmeRinvsrcaddrsubunit,
            GCommandStatus::EsmeRinvdstaddrsubunit() => CommandStatus::EsmeRinvdstaddrsubunit,
            GCommandStatus::EsmeRinvbcastfreqint() => CommandStatus::EsmeRinvbcastfreqint,
            GCommandStatus::EsmeRinvbcastaliasName() => CommandStatus::EsmeRinvbcastaliasName,
            GCommandStatus::EsmeRinvbcastareafmt() => CommandStatus::EsmeRinvbcastareafmt,
            GCommandStatus::EsmeRinvnumbcastAreas() => CommandStatus::EsmeRinvnumbcastAreas,
            GCommandStatus::EsmeRinvbcastcnttype() => CommandStatus::EsmeRinvbcastcnttype,
            GCommandStatus::EsmeRinvbcastmsgclass() => CommandStatus::EsmeRinvbcastmsgclass,
            GCommandStatus::EsmeRbcastfail() => CommandStatus::EsmeRbcastfail,
            GCommandStatus::EsmeRbcastqueryfail() => CommandStatus::EsmeRbcastqueryfail,
            GCommandStatus::EsmeRbcastcancelfail() => CommandStatus::EsmeRbcastcancelfail,
            GCommandStatus::EsmeRinvbcastRep() => CommandStatus::EsmeRinvbcastRep,
            GCommandStatus::EsmeRinvbcastsrvgrp() => CommandStatus::EsmeRinvbcastsrvgrp,
            GCommandStatus::EsmeRinvbcastchanind() => CommandStatus::EsmeRinvbcastchanind,
            GCommandStatus::Other(value) => CommandStatus::Other(value),
        }
    }
}

impl From<GInterfaceVersion> for InterfaceVersion {
    fn from(value: GInterfaceVersion) -> Self {
        match value {
            GInterfaceVersion::Smpp3_3OrEarlier(value) => InterfaceVersion::Smpp3_3OrEarlier(value),
            GInterfaceVersion::Smpp3_4() => InterfaceVersion::Smpp3_4,
            GInterfaceVersion::Smpp5_0() => InterfaceVersion::Smpp5_0,
            GInterfaceVersion::Other(value) => InterfaceVersion::Other(value),
        }
    }
}

impl From<GeneratedTon> for Ton {
    fn from(value: GeneratedTon) -> Self {
        match value {
            GeneratedTon::Unknown() => Ton::Unknown,
            GeneratedTon::International() => Ton::International,
            GeneratedTon::National() => Ton::National,
            GeneratedTon::NetworkSpecific() => Ton::NetworkSpecific,
            GeneratedTon::SubscriberNumber() => Ton::SubscriberNumber,
            GeneratedTon::Alphanumeric() => Ton::Alphanumeric,
            GeneratedTon::Abbreviated() => Ton::Abbreviated,
            GeneratedTon::Other(value) => Ton::Other(value),
        }
    }
}

impl From<GeneratedNpi> for Npi {
    fn from(value: GeneratedNpi) -> Self {
        match value {
            GeneratedNpi::Unknown() => Npi::Unknown,
            GeneratedNpi::Isdn() => Npi::Isdn,
            GeneratedNpi::Data() => Npi::Data,
            GeneratedNpi::Telex() => Npi::Telex,
            GeneratedNpi::LandMobile() => Npi::LandMobile,
            GeneratedNpi::National() => Npi::National,
            GeneratedNpi::Private() => Npi::Private,
            GeneratedNpi::Ermes() => Npi::Ermes,
            GeneratedNpi::Internet() => Npi::Internet,
            GeneratedNpi::WapClientId() => Npi::WapClientId,
            GeneratedNpi::Other(value) => Npi::Other(value),
        }
    }
}

impl From<GDataCoding> for DataCoding {
    fn from(value: GDataCoding) -> Self {
        match value {
            GDataCoding::McSpecific() => DataCoding::McSpecific,
            GDataCoding::Ia5() => DataCoding::Ia5,
            GDataCoding::OctetUnspecified() => DataCoding::OctetUnspecified,
            GDataCoding::Latin1() => DataCoding::Latin1,
            GDataCoding::OctetUnspecified2() => DataCoding::OctetUnspecified2,
            GDataCoding::Jis() => DataCoding::Jis,
            GDataCoding::Cyrillic() => DataCoding::Cyrillic,
            GDataCoding::LatinHebrew() => DataCoding::LatinHebrew,
            GDataCoding::Ucs2() => DataCoding::Ucs2,
            GDataCoding::PictogramEncoding() => DataCoding::PictogramEncoding,
            GDataCoding::Iso2022JpMusicCodes() => DataCoding::Iso2022JpMusicCodes,
            GDataCoding::ExtendedKanjiJis() => DataCoding::ExtendedKanjiJis,
            GDataCoding::Ksc5601() => DataCoding::Ksc5601,
            GDataCoding::GsmMwiControl() => DataCoding::GsmMwiControl,
            GDataCoding::GsmMwiControl2() => DataCoding::GsmMwiControl2,
            GDataCoding::GsmMessageClassControl() => DataCoding::GsmMessageClassControl,
            GDataCoding::Other(value) => DataCoding::Other(value),
        }
    }
}

impl TryFrom<GMessageSubmissionRequestTlvValue> for MessageSubmissionRequestTlvValue {
    type Error = Exception;

    fn try_from(value: GMessageSubmissionRequestTlvValue) -> Result<Self, Self::Error> {
        match value {
            GMessageSubmissionRequestTlvValue::AlertOnMessageDelivery(value) => todo!(),
            GMessageSubmissionRequestTlvValue::BillingIdentification(value) => todo!(),
            GMessageSubmissionRequestTlvValue::CallbackNum(value) => todo!(),
            GMessageSubmissionRequestTlvValue::CallbackNumAtag(value) => todo!(),
            GMessageSubmissionRequestTlvValue::CallbackNumPresInd(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestAddrNpCountry(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestAddrNpInformation(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestAddrNpResolution(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestAddrSubunit(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestBearerType(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestNetworkId(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestNetworkType(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestNodeId(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestSubaddress(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestTelematicsId(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DestPort(value) => todo!(),
            GMessageSubmissionRequestTlvValue::DisplayTime(value) => todo!(),
            GMessageSubmissionRequestTlvValue::ItsReplyType(value) => todo!(),
            GMessageSubmissionRequestTlvValue::ItsSessionInfo(value) => todo!(),
            GMessageSubmissionRequestTlvValue::LanguageIndicator(value) => todo!(),
            GMessageSubmissionRequestTlvValue::MessagePayload(value) => todo!(),
            GMessageSubmissionRequestTlvValue::MoreMessagesToSend(value) => todo!(),
            GMessageSubmissionRequestTlvValue::MsMsgWaitFacilities(value) => todo!(),
            GMessageSubmissionRequestTlvValue::MsValidity(value) => todo!(),
            GMessageSubmissionRequestTlvValue::NumberOfMessages(value) => todo!(),
            GMessageSubmissionRequestTlvValue::PayloadType(value) => todo!(),
            GMessageSubmissionRequestTlvValue::PrivacyIndicator(value) => todo!(),
            GMessageSubmissionRequestTlvValue::QosTimeToLive(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SarMsgRefNum(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SarSegmentSeqnum(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SarTotalSegments(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SetDpf(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SmsSignal(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceAddrSubunit(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceBearerType(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceNetworkId(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceNetworkType(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceNodeId(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourcePort(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceSubaddress(value) => todo!(),
            GMessageSubmissionRequestTlvValue::SourceTelematicsId(value) => todo!(),
            GMessageSubmissionRequestTlvValue::UserMessageReference(value) => todo!(),
            GMessageSubmissionRequestTlvValue::UserResponseCode(value) => todo!(),
            GMessageSubmissionRequestTlvValue::UssdServiceOp(value) => todo!(),
            GMessageSubmissionRequestTlvValue::Other { tag, value } => todo!(),
        }
    }
}
