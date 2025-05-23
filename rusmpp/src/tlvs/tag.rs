#![warn(missing_docs)]

crate::create! {
    #[repr(u16)]
    @[skip_test]
    #[non_exhaustive]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    pub enum TlvTag {
        /// The subcomponent in the destination device for which the user data is intended.
        ///
        /// The dest_addr_subunit parameter is used to route messages when received by a mobile
        /// station, for example to a smart card in the mobile station or to an external device connected
        /// to the mobile station.
        DestAddrSubunit = 0x0005,
        /// The correct network for the destination device.
        ///
        /// The dest_network_type parameter is used to indicate a network type associated with the
        /// destination address of a message. In the case that the receiving system (e.g. MC) does not
        /// support the indicated network type, it may treat this a failure and return a response PDU
        /// reporting a failure.
        DestNetworkType = 0x0006,
        /// The correct bearer type for delivering the user data to the destination.
        ///
        /// The dest_bearer_type parameter is used to request the desired bearer for delivery of the
        /// message to the destination address. In the case that the receiving system (e.g. MC) does not
        /// support the indicated bearer type, it may treat this a failure and return a response PDU
        /// reporting a failure.
        DestBearerType = 0x0007,
        /// The telematics identifier associated with the destination
        ///
        /// This parameter defines the telematic interworking to be used by the delivering system for the
        /// destination address. This is only useful when a specific dest_bearer_type parameter has also
        /// been specified, as the value is bearer dependent. In the case that the receiving system (e.g.
        /// MC) does not support the indicated telematic interworking, it may treat this a failure and
        /// return a response PDU reporting a failure.
        DestTelematicsId = 0x0008,
        /// The subcomponent in the destination device, which created the user data
        ///
        /// The source_addr_subunit parameter is used to indicate where a message originated in the
        /// mobile station, for example a smart card in the mobile station or an external device
        /// connected to the mobile station.
        SourceAddrSubunit = 0x000D,
        /// The correct network associated with the originatingdevice.
        ///
        /// The source_network_type parameter is used to indicate the network type associated with the
        /// device that originated the message.
        SourceNetworkType = 0x000E,
        /// The correct bearer type for delivering the user data to the destination
        ///
        ///The source_bearer_type parameter indicates the wireless bearer over which the message
        /// originated.
        SourceBearerType = 0x000F,
        /// The telematics identifier associated with the source
        ///
        /// The source_telematics_id parameter indicates the type of telematics interface over which the
        /// message originated.
        SourceTelematicsId = 0x0010,
        /// Time to live as a relative time in seconds from submission.
        ///
        /// This parameter defines the number of seconds which the sender requests the MC to keep
        /// the message if undelivered before it is deemed expired. If the parameter is not present, the
        /// MC may apply a default value
        QosTimeToLive = 0x0017,
        /// defines the type of payload (e.g. WDP, WCMP, etc.).
        ///
        /// The payload_type parameter defines the higher layer PDU type contained in the message payload.
        PayloadType = 0x0019,
        /// ASCII text giving a description of the meaning of the response.
        ///
        /// The additional_status_info_text parameter gives an ASCII textual description of the meaning
        /// of a response PDU. It is to be used by an implementation to allow easy diagnosis of problems
        AdditionalStatusInfoText = 0x001D,
        /// MC message ID of message being receipted. Should be
        /// present for MC Delivery Receipts and Intermediate
        /// Notifications.
        ///
        /// The receipted_message_id parameter indicates the ID of the message being receipted in a
        /// MC Delivery Receipt. This is the opaque MC message identifier that was returned in the
        /// message_id parameter of the SMPP response PDU that acknowledged the submission of the
        /// original message.
        ReceiptedMessageId = 0x001E,
        /// This parameter controls the indication and specifies the
        /// message type (of the message associated with the MWI)
        /// at the mobile station.
        ///
        /// The ms_msg_wait_facilities parameter allows an indication to be provided to an MS that
        /// there are messages waiting for the subscriber on systems on the PLMN. The indication can
        /// be an icon on the MS screen or other MMI indication.
        ///
        /// The ms_msg_wait_facilities can also specify the type of message associated with the
        /// message waiting indication.
        MsMsgWaitFacilities = 0x0030,
        /// Indicates the level of privacy associated with the message.
        ///
        /// The privacy_indicator indicates the privacy level of the message.
        PrivacyIndicator = 0x0201,
        /// The sub-address of the message originator.
        ///
        /// The source_subaddress parameter specifies a subaddress associated with the originator of the message.
        SourceSubaddress = 0x0202,
        /// The sub-address of the message destination.
        ///
        /// The dest_subaddress parameter specifies a subaddress associated with the destination of the message.
        DestSubaddress = 0x0203,
        /// ESME assigned message reference number.
        ///
        /// A reference assigned by the originating SME to the short message. Depending on the
        /// destination network technology, this field may be passed directly to the mobile device.
        ///
        /// The user_message_reference TLV is also applicable in ancillary broadcast operations as a
        /// means of identifying a previously submitted message. In such cases, the
        /// user_message_reference can be used to substitute an actual message_id or may be used in
        /// conjunction with a message_id
        UserMessageReference = 0x0204,
        /// A user response code. The actual response codes are implementation specific.
        ///
        /// A response code set by the user in a User Acknowledgement/Reply message. The response
        /// codes are application specific.
        UserResponseCode = 0x0205,
        /// Indicates the application port number associated with the
        /// source address of the message. This parameter should be
        /// present for WAP applications.
        ///
        /// The source_port parameter is used to indicate the application port number associated with
        /// the source address of the message.
        SourcePort = 0x020A,
        /// Indicates the application port number associated with the
        /// destination address of the message. This parameter
        /// should be present for WAP applications.
        ///
        /// The dest_port parameter is used to indicate the application port number associated with the
        /// destination address of the message.
        DestPort = 0x020B,
        /// The reference number for a particular concatenated short message.
        ///
        /// The sar_msg_ref_num parameter is used to indicate the reference number for a particular
        /// concatenated short message.
        /// The concatenation related parameters are sar_msg_ref_num, sar_total_segments and
        /// sar_segment_seqnum. Where these are present the other parameters of the message
        /// should remain unchanged for each short message fragment which forms part of a mobile
        /// terminated concatenated short message, with the exception of those parameters for which it
        /// makes sense to change them (such as the user data in the short_message parameter).
        SarMsgRefNum = 0x020C,
        LanguageIndicator = 0x020D,
        SarTotalSegments = 0x020E,
        SarSegmentSeqnum = 0x020F,
        ScInterfaceVersion = 0x0210,
        CallbackNumPresInd = 0x0302,
        CallbackNumAtag = 0x0303,
        NumberOfMessages = 0x0304,
        CallbackNum = 0x0381,
        DpfResult = 0x0420,
        SetDpf = 0x0421,
        MsAvailabilityStatus = 0x0422,
        NetworkErrorCode = 0x0423,
        MessagePayload = 0x0424,
        DeliveryFailureReason = 0x0425,
        MoreMessagesToSend = 0x0426,
        MessageState = 0x0427,
        CongestionState = 0x0428,
        UssdServiceOp = 0x0501,
        BroadcastChannelIndicator = 0x0600,
        BroadcastContentType = 0x0601,
        BroadcastContentTypeInfo = 0x0602,
        BroadcastMessageClass = 0x0603,
        BroadcastRepNum = 0x0604,
        BroadcastFrequencyInterval = 0x0605,
        BroadcastAreaIdentifier = 0x0606,
        BroadcastErrorStatus = 0x0607,
        BroadcastAreaSuccess = 0x0608,
        BroadcastEndTime = 0x0609,
        BroadcastServiceGroup = 0x060A,
        BillingIdentification = 0x060B,
        SourceNetworkId = 0x060D,
        DestNetworkId = 0x060E,
        SourceNodeId = 0x060F,
        DestNodeId = 0x0610,
        DestAddrNpResolution = 0x0611,
        DestAddrNpInformation = 0x0612,
        DestAddrNpCountry = 0x0613,
        DisplayTime = 0x1201,
        SmsSignal = 0x1203,
        MsValidity = 0x1204,
        AlertOnMessageDelivery = 0x130C,
        ItsReplyType = 0x1380,
        ItsSessionInfo = 0x1383,
        Other(u16),
    }
}

impl From<u16> for TlvTag {
    fn from(value: u16) -> Self {
        match value {
            0x0005 => TlvTag::DestAddrSubunit,
            0x0006 => TlvTag::DestNetworkType,
            0x0007 => TlvTag::DestBearerType,
            0x0008 => TlvTag::DestTelematicsId,
            0x000D => TlvTag::SourceAddrSubunit,
            0x000E => TlvTag::SourceNetworkType,
            0x000F => TlvTag::SourceBearerType,
            0x0010 => TlvTag::SourceTelematicsId,
            0x0017 => TlvTag::QosTimeToLive,
            0x0019 => TlvTag::PayloadType,
            0x001D => TlvTag::AdditionalStatusInfoText,
            0x001E => TlvTag::ReceiptedMessageId,
            0x0030 => TlvTag::MsMsgWaitFacilities,
            0x0201 => TlvTag::PrivacyIndicator,
            0x0202 => TlvTag::SourceSubaddress,
            0x0203 => TlvTag::DestSubaddress,
            0x0204 => TlvTag::UserMessageReference,
            0x0205 => TlvTag::UserResponseCode,
            0x020A => TlvTag::SourcePort,
            0x020B => TlvTag::DestPort,
            0x020C => TlvTag::SarMsgRefNum,
            0x020D => TlvTag::LanguageIndicator,
            0x020E => TlvTag::SarTotalSegments,
            0x020F => TlvTag::SarSegmentSeqnum,
            0x0210 => TlvTag::ScInterfaceVersion,
            0x0302 => TlvTag::CallbackNumPresInd,
            0x0303 => TlvTag::CallbackNumAtag,
            0x0304 => TlvTag::NumberOfMessages,
            0x0381 => TlvTag::CallbackNum,
            0x0420 => TlvTag::DpfResult,
            0x0421 => TlvTag::SetDpf,
            0x0422 => TlvTag::MsAvailabilityStatus,
            0x0423 => TlvTag::NetworkErrorCode,
            0x0424 => TlvTag::MessagePayload,
            0x0425 => TlvTag::DeliveryFailureReason,
            0x0426 => TlvTag::MoreMessagesToSend,
            0x0427 => TlvTag::MessageState,
            0x0428 => TlvTag::CongestionState,
            0x0501 => TlvTag::UssdServiceOp,
            0x0600 => TlvTag::BroadcastChannelIndicator,
            0x0601 => TlvTag::BroadcastContentType,
            0x0602 => TlvTag::BroadcastContentTypeInfo,
            0x0603 => TlvTag::BroadcastMessageClass,
            0x0604 => TlvTag::BroadcastRepNum,
            0x0605 => TlvTag::BroadcastFrequencyInterval,
            0x0606 => TlvTag::BroadcastAreaIdentifier,
            0x0607 => TlvTag::BroadcastErrorStatus,
            0x0608 => TlvTag::BroadcastAreaSuccess,
            0x0609 => TlvTag::BroadcastEndTime,
            0x060A => TlvTag::BroadcastServiceGroup,
            0x060B => TlvTag::BillingIdentification,
            0x060D => TlvTag::SourceNetworkId,
            0x060E => TlvTag::DestNetworkId,
            0x060F => TlvTag::SourceNodeId,
            0x0610 => TlvTag::DestNodeId,
            0x0611 => TlvTag::DestAddrNpResolution,
            0x0612 => TlvTag::DestAddrNpInformation,
            0x0613 => TlvTag::DestAddrNpCountry,
            0x1201 => TlvTag::DisplayTime,
            0x1203 => TlvTag::SmsSignal,
            0x1204 => TlvTag::MsValidity,
            0x130C => TlvTag::AlertOnMessageDelivery,
            0x1380 => TlvTag::ItsReplyType,
            0x1383 => TlvTag::ItsSessionInfo,
            other => TlvTag::Other(other),
        }
    }
}

impl From<TlvTag> for u16 {
    fn from(value: TlvTag) -> Self {
        match value {
            TlvTag::DestAddrSubunit => 0x0005,
            TlvTag::DestNetworkType => 0x0006,
            TlvTag::DestBearerType => 0x0007,
            TlvTag::DestTelematicsId => 0x0008,
            TlvTag::SourceAddrSubunit => 0x000D,
            TlvTag::SourceNetworkType => 0x000E,
            TlvTag::SourceBearerType => 0x000F,
            TlvTag::SourceTelematicsId => 0x0010,
            TlvTag::QosTimeToLive => 0x0017,
            TlvTag::PayloadType => 0x0019,
            TlvTag::AdditionalStatusInfoText => 0x001D,
            TlvTag::ReceiptedMessageId => 0x001E,
            TlvTag::MsMsgWaitFacilities => 0x0030,
            TlvTag::PrivacyIndicator => 0x0201,
            TlvTag::SourceSubaddress => 0x0202,
            TlvTag::DestSubaddress => 0x0203,
            TlvTag::UserMessageReference => 0x0204,
            TlvTag::UserResponseCode => 0x0205,
            TlvTag::SourcePort => 0x020A,
            TlvTag::DestPort => 0x020B,
            TlvTag::SarMsgRefNum => 0x020C,
            TlvTag::LanguageIndicator => 0x020D,
            TlvTag::SarTotalSegments => 0x020E,
            TlvTag::SarSegmentSeqnum => 0x020F,
            TlvTag::ScInterfaceVersion => 0x0210,
            TlvTag::CallbackNumPresInd => 0x0302,
            TlvTag::CallbackNumAtag => 0x0303,
            TlvTag::NumberOfMessages => 0x0304,
            TlvTag::CallbackNum => 0x0381,
            TlvTag::DpfResult => 0x0420,
            TlvTag::SetDpf => 0x0421,
            TlvTag::MsAvailabilityStatus => 0x0422,
            TlvTag::NetworkErrorCode => 0x0423,
            TlvTag::MessagePayload => 0x0424,
            TlvTag::DeliveryFailureReason => 0x0425,
            TlvTag::MoreMessagesToSend => 0x0426,
            TlvTag::MessageState => 0x0427,
            TlvTag::CongestionState => 0x0428,
            TlvTag::UssdServiceOp => 0x0501,
            TlvTag::BroadcastChannelIndicator => 0x0600,
            TlvTag::BroadcastContentType => 0x0601,
            TlvTag::BroadcastContentTypeInfo => 0x0602,
            TlvTag::BroadcastMessageClass => 0x0603,
            TlvTag::BroadcastRepNum => 0x0604,
            TlvTag::BroadcastFrequencyInterval => 0x0605,
            TlvTag::BroadcastAreaIdentifier => 0x0606,
            TlvTag::BroadcastErrorStatus => 0x0607,
            TlvTag::BroadcastAreaSuccess => 0x0608,
            TlvTag::BroadcastEndTime => 0x0609,
            TlvTag::BroadcastServiceGroup => 0x060A,
            TlvTag::BillingIdentification => 0x060B,
            TlvTag::SourceNetworkId => 0x060D,
            TlvTag::DestNetworkId => 0x060E,
            TlvTag::SourceNodeId => 0x060F,
            TlvTag::DestNodeId => 0x0610,
            TlvTag::DestAddrNpResolution => 0x0611,
            TlvTag::DestAddrNpInformation => 0x0612,
            TlvTag::DestAddrNpCountry => 0x0613,
            TlvTag::DisplayTime => 0x1201,
            TlvTag::SmsSignal => 0x1203,
            TlvTag::MsValidity => 0x1204,
            TlvTag::AlertOnMessageDelivery => 0x130C,
            TlvTag::ItsReplyType => 0x1380,
            TlvTag::ItsSessionInfo => 0x1383,
            TlvTag::Other(other) => other,
        }
    }
}
