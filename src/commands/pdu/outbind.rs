use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::c_octet_string::COctetString,
};

/// Authentication PDU used by a Message Centre to Outbind to
/// an ESME to inform it that messages are present in the MC.
/// The PDU contains identification, and access password for the
/// ESME. If the ESME authenticates the request, it will respond
/// with a bind_receiver or bind_transceiver to begin the process
/// of binding into the MC.
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Outbind {
    /// MC identifier.
    ///
    /// Identifies the MC to the ESME.
    pub system_id: COctetString<1, 16>,
    /// The password may be used by the
    /// ESME for security reasons to
    /// authenticate the MC originating the
    /// outbind.
    pub password: COctetString<1, 9>,
}

impl Length for Outbind {
    fn length(&self) -> usize {
        self.system_id.length() + self.password.length()
    }
}

impl Encode for Outbind {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.system_id.encode_to(writer));
        tri!(self.password.encode_to(writer));

        Ok(())
    }
}

impl Decode for Outbind {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let system_id = tri!(COctetString::decode_from(reader));
        let password = tri!(COctetString::decode_from(reader));

        Ok(Self {
            system_id,
            password,
        })
    }
}
