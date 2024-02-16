// use num_enum::{FromPrimitive, IntoPrimitive};
// use rusmpp_macros::{RusmppIo, RusmppIoU8};

// use crate::{
//     io::{
//         length::IoLength,
//         read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
//     },
//     types::octet_string::OctetString,
// };

// #[repr(u8)]
// #[derive(
//     Debug,
//     Copy,
//     Clone,
//     PartialEq,
//     Eq,
//     Hash,
//     PartialOrd,
//     Ord,
//     IntoPrimitive,
//     FromPrimitive,
//     RusmppIoU8,
// )]
// pub enum BroadcastAreaFormat {
//     AliasName = 0x00,
//     EllipsoidArc = 0x01,
//     Polygon = 0x02,
//     #[num_enum(catch_all)]
//     Other(u8),
// }

// #[allow(clippy::derivable_impls)]
// impl Default for BroadcastAreaFormat {
//     fn default() -> Self {
//         BroadcastAreaFormat::AliasName
//     }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
// pub struct BroadcastAreaIdentifier {
//     pub format: BroadcastAreaFormat,
//     pub area: OctetString<0, 100>,
// }

// impl BroadcastAreaIdentifier {
//     pub fn new(format: BroadcastAreaFormat, area: OctetString<0, 100>) -> Self {
//         Self { format, area }
//     }
// }

// #[async_trait::async_trait]
// impl AsyncIoReadWithLength for BroadcastAreaIdentifier {
//     async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
//         let format = BroadcastAreaFormat::async_io_read(buf).await?;
//         let area = OctetString::async_io_read(buf, length - format.length() as usize).await?;

//         Ok(Self { format, area })
//     }
// }
