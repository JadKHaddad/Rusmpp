# Rusmpp

Low level SMPP library in pure rust. This is not a Client/Server implementation, but a library to build one. Baisc operations like `bind`, `unbind`, `submit_sm` and `deliver_sm` should not be difficult to implement. See examples for more details. Baisc knowledge of SMPP protocol is required.

## Semantics

- This library is designed to be as close to the SMPP protocol as possible, reads/writes pdu-bytes from/into rust structs and provides a higer level API to the user. This means that the library does not do any validation of the data, and it is up to the user to ensure that the data is semantically valid.
- Some semantics are type checked, for example the minimum and maximum length of a field is guaranteed by the type of the field. For example, the `system_id` field in the `Bind` operation is of type `COctetString<1, 16>` which means that the minimum length of the field is 1 (empty) and the maximum length is 16.
- Other semantics are checked at runtime, for example, the `command_length` field in the header is automatically calculated and set by the library, and the user does not need to set it manually. Similarly, the `command_id` field is automatically set when the user provides a `Body` enum to the `Pdu::new` function. The `command_status` and `sequence_number` fields are also validated against `GeniricNack` responses.

## Supported Operations

- [x] bind_transmitter
- [x] bind_transmitter_resp
- [x] bind_receiver
- [x] bind_receiver_resp
- [x] bind_transceiver
- [x] bind_transceiver_resp
- [x] outbind
- [x] unbind
- [x] unbind_resp
- [x] submit_sm
- [x] submit_sm_resp
- [ ] submit_sm_multi
- [ ] submit_sm_multi_resp
- [x] data_sm
- [x] data_sm_resp
- [x] deliver_sm
- [x] deliver_sm_resp
- [x] query_sm
- [x] query_sm_resp
- [x] cancel_sm
- [x] cancel_sm_resp
- [x] replace_sm
- [x] replace_sm_resp
- [x] enquire_link
- [x] enquire_link_resp
- [x] alert_notification
- [x] generic_nack
- [ ] broadcast_sm
- [ ] broadcast_sm_resp
- [ ] query_broadcast_sm
- [ ] query_broadcast_sm_resp
- [ ] cancel_broadcast_sm
- [ ] cancel_broadcast_sm_resp

## Supported TLVs

All TLVs are supported.

## Releases

`vec![]`
