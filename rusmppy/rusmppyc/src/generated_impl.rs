use rusmpp::{
    values::{DataCoding, InterfaceVersion, Npi, Ton},
    CommandStatus,
};

// TODO: can we implement this in the generated.rs instead?

impl From<crate::generated::CommandStatus> for CommandStatus {
    fn from(value: crate::generated::CommandStatus) -> Self {
        match value {
            crate::generated::CommandStatus::EsmeRok() => CommandStatus::EsmeRok,
            crate::generated::CommandStatus::EsmeRinvmsglen() => CommandStatus::EsmeRinvmsglen,
            crate::generated::CommandStatus::EsmeRinvcmdlen() => CommandStatus::EsmeRinvcmdlen,
            crate::generated::CommandStatus::EsmeRinvcmdid() => CommandStatus::EsmeRinvcmdid,
            crate::generated::CommandStatus::EsmeRinvbndsts() => CommandStatus::EsmeRinvbndsts,
            crate::generated::CommandStatus::EsmeRalybnd() => CommandStatus::EsmeRalybnd,
            crate::generated::CommandStatus::EsmeRinvprtflg() => CommandStatus::EsmeRinvprtflg,
            crate::generated::CommandStatus::EsmeRinvregdlvflg() => {
                CommandStatus::EsmeRinvregdlvflg
            }
            crate::generated::CommandStatus::EsmeRsyserr() => CommandStatus::EsmeRsyserr,
            crate::generated::CommandStatus::EsmeRinvsrcadr() => CommandStatus::EsmeRinvsrcadr,
            crate::generated::CommandStatus::EsmeRinvdstadr() => CommandStatus::EsmeRinvdstadr,
            crate::generated::CommandStatus::EsmeRinvmsgid() => CommandStatus::EsmeRinvmsgid,
            crate::generated::CommandStatus::EsmeRbindfail() => CommandStatus::EsmeRbindfail,
            crate::generated::CommandStatus::EsmeRinvpaswd() => CommandStatus::EsmeRinvpaswd,
            crate::generated::CommandStatus::EsmeRinvsysid() => CommandStatus::EsmeRinvsysid,
            crate::generated::CommandStatus::EsmeRcancelfail() => CommandStatus::EsmeRcancelfail,
            crate::generated::CommandStatus::EsmeRreplacefail() => CommandStatus::EsmeRreplacefail,
            crate::generated::CommandStatus::EsmeRmsgqful() => CommandStatus::EsmeRmsgqful,
            crate::generated::CommandStatus::EsmeRinvsertyp() => CommandStatus::EsmeRinvsertyp,
            crate::generated::CommandStatus::EsmeRinvnumdests() => CommandStatus::EsmeRinvnumdests,
            crate::generated::CommandStatus::EsmeRinvdlname() => CommandStatus::EsmeRinvdlname,
            crate::generated::CommandStatus::EsmeRinvdestflag() => CommandStatus::EsmeRinvdestflag,
            crate::generated::CommandStatus::EsmeRinvsubrep() => CommandStatus::EsmeRinvsubrep,
            crate::generated::CommandStatus::EsmeRinvesmclass() => CommandStatus::EsmeRinvesmclass,
            crate::generated::CommandStatus::EsmeRcntsubdl() => CommandStatus::EsmeRcntsubdl,
            crate::generated::CommandStatus::EsmeRsubmitfail() => CommandStatus::EsmeRsubmitfail,
            crate::generated::CommandStatus::EsmeRinvsrcton() => CommandStatus::EsmeRinvsrcton,
            crate::generated::CommandStatus::EsmeRinvsrcnpi() => CommandStatus::EsmeRinvsrcnpi,
            crate::generated::CommandStatus::EsmeRinvdstton() => CommandStatus::EsmeRinvdstton,
            crate::generated::CommandStatus::EsmeRinvdstnpi() => CommandStatus::EsmeRinvdstnpi,
            crate::generated::CommandStatus::EsmeRinvsystyp() => CommandStatus::EsmeRinvsystyp,
            crate::generated::CommandStatus::EsmeRinvrepflag() => CommandStatus::EsmeRinvrepflag,
            crate::generated::CommandStatus::EsmeRinvnummsgs() => CommandStatus::EsmeRinvnummsgs,
            crate::generated::CommandStatus::EsmeRthrottled() => CommandStatus::EsmeRthrottled,
            crate::generated::CommandStatus::EsmeRinvsched() => CommandStatus::EsmeRinvsched,
            crate::generated::CommandStatus::EsmeRinvexpiry() => CommandStatus::EsmeRinvexpiry,
            crate::generated::CommandStatus::EsmeRinvdftmsgid() => CommandStatus::EsmeRinvdftmsgid,
            crate::generated::CommandStatus::EsmeRxTAppn() => CommandStatus::EsmeRxTAppn,
            crate::generated::CommandStatus::EsmeRxPAppn() => CommandStatus::EsmeRxPAppn,
            crate::generated::CommandStatus::EsmeRxRAppn() => CommandStatus::EsmeRxRAppn,
            crate::generated::CommandStatus::EsmeRqueryfail() => CommandStatus::EsmeRqueryfail,
            crate::generated::CommandStatus::EsmeRinvtlvstream() => {
                CommandStatus::EsmeRinvtlvstream
            }
            crate::generated::CommandStatus::EsmeRtlvnotallwd() => CommandStatus::EsmeRtlvnotallwd,
            crate::generated::CommandStatus::EsmeRinvtlvlen() => CommandStatus::EsmeRinvtlvlen,
            crate::generated::CommandStatus::EsmeRmissingtlv() => CommandStatus::EsmeRmissingtlv,
            crate::generated::CommandStatus::EsmeRinvtlvval() => CommandStatus::EsmeRinvtlvval,
            crate::generated::CommandStatus::EsmeRdeliveryfailure() => {
                CommandStatus::EsmeRdeliveryfailure
            }
            crate::generated::CommandStatus::EsmeRunknownerr() => CommandStatus::EsmeRunknownerr,
            crate::generated::CommandStatus::EsmeRsertypunauth() => {
                CommandStatus::EsmeRsertypunauth
            }
            crate::generated::CommandStatus::EsmeRprohibited() => CommandStatus::EsmeRprohibited,
            crate::generated::CommandStatus::EsmeRsertypunavail() => {
                CommandStatus::EsmeRsertypunavail
            }
            crate::generated::CommandStatus::EsmeRsertypdenied() => {
                CommandStatus::EsmeRsertypdenied
            }
            crate::generated::CommandStatus::EsmeRinvdcs() => CommandStatus::EsmeRinvdcs,
            crate::generated::CommandStatus::EsmeRinvsrcaddrsubunit() => {
                CommandStatus::EsmeRinvsrcaddrsubunit
            }
            crate::generated::CommandStatus::EsmeRinvdstaddrsubunit() => {
                CommandStatus::EsmeRinvdstaddrsubunit
            }
            crate::generated::CommandStatus::EsmeRinvbcastfreqint() => {
                CommandStatus::EsmeRinvbcastfreqint
            }
            crate::generated::CommandStatus::EsmeRinvbcastaliasName() => {
                CommandStatus::EsmeRinvbcastaliasName
            }
            crate::generated::CommandStatus::EsmeRinvbcastareafmt() => {
                CommandStatus::EsmeRinvbcastareafmt
            }
            crate::generated::CommandStatus::EsmeRinvnumbcastAreas() => {
                CommandStatus::EsmeRinvnumbcastAreas
            }
            crate::generated::CommandStatus::EsmeRinvbcastcnttype() => {
                CommandStatus::EsmeRinvbcastcnttype
            }
            crate::generated::CommandStatus::EsmeRinvbcastmsgclass() => {
                CommandStatus::EsmeRinvbcastmsgclass
            }
            crate::generated::CommandStatus::EsmeRbcastfail() => CommandStatus::EsmeRbcastfail,
            crate::generated::CommandStatus::EsmeRbcastqueryfail() => {
                CommandStatus::EsmeRbcastqueryfail
            }
            crate::generated::CommandStatus::EsmeRbcastcancelfail() => {
                CommandStatus::EsmeRbcastcancelfail
            }
            crate::generated::CommandStatus::EsmeRinvbcastRep() => CommandStatus::EsmeRinvbcastRep,
            crate::generated::CommandStatus::EsmeRinvbcastsrvgrp() => {
                CommandStatus::EsmeRinvbcastsrvgrp
            }
            crate::generated::CommandStatus::EsmeRinvbcastchanind() => {
                CommandStatus::EsmeRinvbcastchanind
            }
            crate::generated::CommandStatus::Other(value) => CommandStatus::Other(value),
        }
    }
}

impl From<crate::generated::InterfaceVersion> for InterfaceVersion {
    fn from(value: crate::generated::InterfaceVersion) -> Self {
        match value {
            crate::generated::InterfaceVersion::Smpp3_3OrEarlier(value) => {
                InterfaceVersion::Smpp3_3OrEarlier(value)
            }
            crate::generated::InterfaceVersion::Smpp3_4() => InterfaceVersion::Smpp3_4,
            crate::generated::InterfaceVersion::Smpp5_0() => InterfaceVersion::Smpp5_0,
            crate::generated::InterfaceVersion::Other(value) => InterfaceVersion::Other(value),
        }
    }
}

impl From<crate::generated::Ton> for Ton {
    fn from(value: crate::generated::Ton) -> Self {
        match value {
            crate::generated::Ton::Unknown() => Ton::Unknown,
            crate::generated::Ton::International() => Ton::International,
            crate::generated::Ton::National() => Ton::National,
            crate::generated::Ton::NetworkSpecific() => Ton::NetworkSpecific,
            crate::generated::Ton::SubscriberNumber() => Ton::SubscriberNumber,
            crate::generated::Ton::Alphanumeric() => Ton::Alphanumeric,
            crate::generated::Ton::Abbreviated() => Ton::Abbreviated,
            crate::generated::Ton::Other(value) => Ton::Other(value),
        }
    }
}

impl From<crate::generated::Npi> for Npi {
    fn from(value: crate::generated::Npi) -> Self {
        match value {
            crate::generated::Npi::Unknown() => Npi::Unknown,
            crate::generated::Npi::Isdn() => Npi::Isdn,
            crate::generated::Npi::Data() => Npi::Data,
            crate::generated::Npi::Telex() => Npi::Telex,
            crate::generated::Npi::LandMobile() => Npi::LandMobile,
            crate::generated::Npi::National() => Npi::National,
            crate::generated::Npi::Private() => Npi::Private,
            crate::generated::Npi::Ermes() => Npi::Ermes,
            crate::generated::Npi::Internet() => Npi::Internet,
            crate::generated::Npi::WapClientId() => Npi::WapClientId,
            crate::generated::Npi::Other(value) => Npi::Other(value),
        }
    }
}

impl From<crate::generated::DataCoding> for DataCoding {
    fn from(value: crate::generated::DataCoding) -> Self {
        match value {
            crate::generated::DataCoding::McSpecific() => DataCoding::McSpecific,
            crate::generated::DataCoding::Ia5() => DataCoding::Ia5,
            crate::generated::DataCoding::OctetUnspecified() => DataCoding::OctetUnspecified,
            crate::generated::DataCoding::Latin1() => DataCoding::Latin1,
            crate::generated::DataCoding::OctetUnspecified2() => DataCoding::OctetUnspecified2,
            crate::generated::DataCoding::Jis() => DataCoding::Jis,
            crate::generated::DataCoding::Cyrillic() => DataCoding::Cyrillic,
            crate::generated::DataCoding::LatinHebrew() => DataCoding::LatinHebrew,
            crate::generated::DataCoding::Ucs2() => DataCoding::Ucs2,
            crate::generated::DataCoding::PictogramEncoding() => DataCoding::PictogramEncoding,
            crate::generated::DataCoding::Iso2022JpMusicCodes() => DataCoding::Iso2022JpMusicCodes,
            crate::generated::DataCoding::ExtendedKanjiJis() => DataCoding::ExtendedKanjiJis,
            crate::generated::DataCoding::Ksc5601() => DataCoding::Ksc5601,
            crate::generated::DataCoding::GsmMwiControl() => DataCoding::GsmMwiControl,
            crate::generated::DataCoding::GsmMwiControl2() => DataCoding::GsmMwiControl2,
            crate::generated::DataCoding::GsmMessageClassControl() => {
                DataCoding::GsmMessageClassControl
            }
            crate::generated::DataCoding::Other(value) => DataCoding::Other(value),
        }
    }
}
