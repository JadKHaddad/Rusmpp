# Rusmpp

Speak SMPP like english.

```rust
// Build commands. Omitted values will be set to default.
let submit_sm_command = Command::builder()
        .command_status(CommandStatus::EsmeRok)
        .sequence_number(2)
        .pdu(
            SubmitSm::builder()
                .serivce_type(ServiceType::default())
                .source_addr_ton(Ton::Unknown)
                .source_addr_npi(Npi::Unknown)
                .esm_class(EsmClass::default())
                .registered_delivery(RegisteredDelivery::request_all())
                .short_message(
                    OctetString::from_str("Hi, I am a short message.")?
                )
                .build()
                .into_submit_sm(),
        )
        .build();

// Send commands.
framed_writer
    .send(submit_sm_command)
    .await?;

// Wait for responses.
while let Some(Ok(command)) = framed_reader.next().await {
    if let CommandId::SubmitSmResp = command.command_id() {
        break;
    }
}
```
