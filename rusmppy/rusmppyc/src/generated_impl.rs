use rusmpp::CommandStatus;

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
            crate::generated::CommandStatus::Other(code) => CommandStatus::Other(code),
        }
    }
}
