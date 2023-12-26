# Rusmpp

Low level smpp library in pure rust. This is not a Client/Server implementation, but a library to build one. See examples for more details.

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
- [ ] data_sm
- [ ] data_sm_resp
- [x] deliver_sm
- [ ] deliver_sm_resp
- [x] query_sm
- [x] query_sm_resp
- [ ] cancel_sm
- [ ] cancel_sm_resp
- [ ] replace_sm
- [ ] replace_sm_resp
- [x] enquire_link
- [x] enquire_link_resp
- [x] alert_notification
- [x] generic_nack
