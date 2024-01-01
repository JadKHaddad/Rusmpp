# Rusmpp

![Tests](https://github.com/JadKHaddad/Rusmpp/actions/workflows/tests.yml/badge.svg?branch=main)

Low level SMPP library in pure rust. This is not a Client/Server implementation, but a library to build one. Baisc operations like `bind`, `unbind`, `submit_sm` and `deliver_sm` should not be difficult to implement. See examples for more details. Baisc knowledge of SMPP protocol is required.

## Semantics

- This library is designed to be as close to the SMPP protocol as possible, reads/writes pdu-bytes from/into rust structs and provides a higer level API to the user. This means that the library does not do any validation of the data, and it is up to the user to ensure that the data is semantically valid.
- Some semantics are type checked, for example the minimum and maximum length of a field is guaranteed by the type of the field. For example, the `system_id` field in the `Bind` operation is of type `COctetString<1, 16>` which means that the minimum length of the field is 1 (empty) and the maximum length is 16.
- Other semantics are checked at runtime, for example, the `command_length` field in the header is automatically calculated and set by the library, and the user does not need to set it manually. Similarly, the `command_id` field is automatically set when the user provides a `Body` enum to the `Pdu::new` function. The `command_status` and `sequence_number` fields are also validated against `GeniricNack` responses.

## Supported Operations

All operations are supported.

## Supported TLVs

All TLVs are supported.

## Example

```rust
use rusmpp::{pdus::body::bodies::bind::Bind, prelude::*};
use rusmpp_io::types::c_octet_string::COctetString;
use std::str::FromStr;
use tokio::{io::BufReader, net::TcpStream};

#[tokio::main]
async fn main() {
    let stream = TcpStream::connect("34.242.18.250:2775")
        .await
        .expect("Failed to connect");

    let (reader, mut writer) = stream.into_split();

    let mut reader = BufReader::new(reader);

    // BindTransceiver
    let pdu = Pdu::new(
        CommandStatus::EsmeRok,
        SequenceNumber::new(1),
        PduBody::BindTransceiver(Bind {
            system_id: COctetString::from_str("system_id").unwrap(),
            password: COctetString::from_str("pass").unwrap(),
            system_type: COctetString::from_str("a_type").unwrap(),
            interface_version: InterfaceVersion::Smpp5_0,
            addr_ton: Ton::Unknown,
            addr_npi: Npi::Unknown,
            address_range: COctetString::empty(),
        }),
    )
    .unwrap();

    pdu.async_io_write(&mut writer)
        .await
        .expect("Failed to write pdu bytes");

    while let Ok(pdu) = Pdu::async_io_read(&mut reader).await {
        println!("pdu: {:?}", pdu);
        if let CommandId::BindTransceiverResp = pdu.command_id() {
            println!("BindTransceiverResp received");
            break;
        }
    }
}
```

See the [examples](https://github.com/JadKHaddad/Rusmpp/tree/main/rusmpp/examples) directory for more details.

## Releases

`vec![]`
