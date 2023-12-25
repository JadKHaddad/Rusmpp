use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

use super::{
    tlv_tag::{MessageSubmissionRequestTLVTag, MessageSubmissionResponseTLVTag, TLVTag},
    tlv_value::{MessageSubmissionRequestTLVValue, MessageSubmissionResponseTLVValue, TLVValue},
};

/// A Tagged Length Value Field is a special composite field
/// that comprises of three parts:
/// • A 2-octet Integer (Tag)
/// The tag identifies the parameter.
///
/// • A 2-octet Integer (Length)
/// The length field indicates the length of the value
/// field in octets. Note that this length does not
/// include the length of the tag and length fields.
///
/// • An Octet String (Value)
/// The value field contains the actual data for the
/// TLV field.
///
/// The Tag identifies the parameter. The Length indicates
/// the size of the Value field in octets.
///
/// An example of a TLV is the dest_bearer_type. Its Tag is
/// 0x0007 and has a value size of 1 octet. The value 0x04
/// indicates USSD as a bearer type. In its encoded form,
/// this TLV would appear as follows:
///
/// 0x0007000104
///
/// The first 2 octets 0x0007 identifies the Tag
/// dest_bearer_type. The next two octets 0x0001 indicate
/// the 1-octet length of the value field. The value field 0x04
/// indicates USSD.
///
/// There are two types of NULL encoding for a TLV. The
/// first is a TLV that may not carry a value part. An example
/// of such a TLV is alert_on_message_delivery. This TLV is
/// typically used as an indicator only, i.e. its function is
/// driven by its very presence in the PDU. No data is
/// typically present. However it may carry up to 1 octet of
/// data if required.
///
/// Here are two examples of how this TLV can be encoded,
/// the first example carries a value, the second example
/// does not:
///
/// Tag=0x130C
/// Length=0x0001
/// Value=0x01
///
/// Encoded Format: 0x130C000101
///
/// Tag=0x130C
/// Length=0x0000
/// Value=NULL
///
/// Encoded Format: 0x130C0000
///
/// Note: Only the Tag and Length are encoded. No NULL
/// octets are specified for the zero length Value field.
///
/// If the TLV itself is not required, then it is not encoded at
/// all. The very absence of the TLV from the PDU is the
/// means by which we set the values to NULL.
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TLV {
    tag: TLVTag,
    value_length: u16,
    value: Option<TLVValue>,
}

impl TLV {
    pub fn new(value: TLVValue) -> Self {
        let tag = value.tlv_tag();
        let value_length = value.length() as u16;

        Self {
            tag,
            value_length,
            value: Some(value),
        }
    }

    pub fn new_without_value(tag: TLVTag) -> Self {
        // TODO: Maybe check the tag!
        Self {
            tag,
            value_length: 0,
            value: None,
        }
    }

    pub fn tag(&self) -> &TLVTag {
        &self.tag
    }

    pub fn value_length(&self) -> u16 {
        self.value_length
    }

    pub fn value(&self) -> Option<&TLVValue> {
        self.value.as_ref()
    }

    pub fn into_value(self) -> Option<TLVValue> {
        self.value
    }
}

impl IoLength for TLV {
    fn length(&self) -> usize {
        self.tag.length() + self.value_length.length() + self.value.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for TLV {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.tag.async_io_write(buf).await?;
        self.value_length.async_io_write(buf).await?;
        self.value.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for TLV {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        let tag = TLVTag::async_io_read(buf).await?;
        let value_length = u16::async_io_read(buf).await?;

        let value = if value_length > 0 {
            TLVValue::async_io_read(tag, buf, value_length as usize).await?
        } else {
            None
        };

        Ok(Self {
            tag,
            value_length,
            value,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionRequestTLV {
    tlv: TLV,
}

impl MessageSubmissionRequestTLV {
    pub fn new(value: MessageSubmissionRequestTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn new_without_value(tag: MessageSubmissionRequestTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageSubmissionRequestTLV> for TLV {
    fn from(tlv: MessageSubmissionRequestTLV) -> Self {
        tlv.tlv
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct MessageSubmissionResponseTLV {
    tlv: TLV,
}

impl MessageSubmissionResponseTLV {
    pub fn new(value: MessageSubmissionResponseTLVValue) -> Self {
        let tlv = TLV::new(value.into());

        Self { tlv }
    }

    pub fn new_without_value(tag: MessageSubmissionResponseTLVTag) -> Self {
        let tlv = TLV::new_without_value(tag.into());

        Self { tlv }
    }
}

impl From<MessageSubmissionResponseTLV> for TLV {
    fn from(tlv: MessageSubmissionResponseTLV) -> Self {
        tlv.tlv
    }
}
