//! `SMPP` values.

// pub mod parts {
//     pub use super::broadcast_area_identifier::BroadcastAreaIdentifierParts;
//     pub use super::broadcast_content_type::BroadcastContentTypeParts;
//     pub use super::broadcast_frequency_interval::BroadcastFrequencyIntervalParts;
//     pub use super::broadcast_rep_num::BroadcastRepNumParts;
//     pub use super::callback_num_pres_ind::CallbackNumPresIndParts;
//     pub use super::dest_address::{DistributionListNameParts, SmeAddressParts};
//     pub use super::esm_class::EsmClassParts;
//     pub use super::its_session_info::ItsSessionInfoParts;
//     pub use super::message_payload::MessagePayloadParts;
//     pub use super::ms_msg_wait_facilities::MsMsgWaitFacilitiesParts;
//     pub use super::ms_validity::{MsValidityInformationParts, MsValidityParts};
//     pub use super::network_error_code::NetworkErrorCodeParts;
//     pub use super::priority_flag::PriorityFlagParts;
//     pub use super::registered_delivery::RegisteredDeliveryParts;
//     pub use super::service_type::ServiceTypeParts;
//     pub use super::sub_address::SubaddressParts;
//     pub use super::unsuccess_sme::UnsuccessSmeParts;
//     pub use super::user_message_reference::UserMessageReferenceParts;
// }

pub mod addr_subunit;

pub mod alert_on_msg_delivery;

pub mod bearer_type;

pub mod broadcast_area_identifier;

pub mod broadcast_area_success;

pub mod broadcast_channel_indicator;

pub mod broadcast_content_type;

pub mod broadcast_frequency_interval;

pub mod broadcast_message_class;

pub mod broadcast_rep_num;

pub mod callback_num_pres_ind;

pub mod congestion_state;

pub mod data_coding;

pub mod delivery_failure_reason;

pub mod dest_addr_np_resolution;

pub mod dest_address;

pub mod display_time;

pub mod dpf_result;

pub mod esm_class;

pub mod interface_version;

pub mod its_reply_type;

pub mod its_session_info;

pub mod language_indicator;

pub mod message_state;

pub mod more_messages_to_send;

pub mod ms_availability_status;

pub mod ms_msg_wait_facilities;

pub mod ms_validity;

pub mod network_error_code;

pub mod network_type;

pub mod npi;

pub mod number_of_messages;

pub mod payload_type;

pub mod priority_flag;

pub mod privacy_indicator;

pub mod registered_delivery;

pub mod replace_if_present_flag;

pub mod service_type;

pub mod set_dpf;

pub mod sub_address;

pub mod ton;

pub mod unsuccess_sme;

pub mod ussd_service_op;

pub mod user_message_reference;

pub mod message_payload;
