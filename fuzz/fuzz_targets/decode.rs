#![no_main]

use libfuzzer_sys::fuzz_target;
use rusmpp::{
    commands::{
        command::Command,
        pdu::{
            AlertNotification, Bind, BindResp, BroadcastSm, BroadcastSmResp, CancelBroadcastSm,
            CancelSm, DataSm, DeliverSm, Outbind, QueryBroadcastSm, QueryBroadcastSmResp, QuerySm,
            QuerySmResp, ReplaceSm, SmResp, SubmitMulti, SubmitMultiResp, SubmitSm, SubmitSmResp,
        },
    },
    ende::decode::{Decode, DecodeWithLength},
};
use std::io::Cursor;

fuzz_target!(|data: &[u8]| {
    let mut cursor = Cursor::new(data);
    let _ = Command::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = AlertNotification::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = BindResp::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = Bind::decode_from(&mut cursor);

    let mut cursor = Cursor::new(data);
    let _ = BroadcastSmResp::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = BroadcastSm::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = CancelBroadcastSm::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = CancelSm::decode_from(&mut cursor);

    let mut cursor = Cursor::new(data);
    let _ = DataSm::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = DeliverSm::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = Outbind::decode_from(&mut cursor);

    let mut cursor = Cursor::new(data);
    let _ = QueryBroadcastSmResp::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = QueryBroadcastSm::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = QuerySmResp::decode_from(&mut cursor);

    let mut cursor = Cursor::new(data);
    let _ = QuerySm::decode_from(&mut cursor);

    let mut cursor = Cursor::new(data);
    let _ = ReplaceSm::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = SmResp::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = SubmitMultiResp::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = SubmitMulti::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = SubmitSmResp::decode_from(&mut cursor, data.len());

    let mut cursor = Cursor::new(data);
    let _ = SubmitSm::decode_from(&mut cursor, data.len());
});
