import builtins
import asyncio
from typing import Optional

from .rusmppyc import (
    BindReceiverResp,
    BindTransmitterResp,
    SubmitSmResp,
    CommandStatus,
    BindTransceiverResp,
    InterfaceVersion,
    Ton,
    Npi,
    DataCoding,
)
from .events import Events

__all__ = ["Client"]

class Client:
    """
    `SMPP` Client.

    The client is a handle to communicate with the `SMPP` server through a managed connection in the background.
    """

    @classmethod
    async def connect(
        cls,
        url: builtins.str,
        enquire_link_interval: Optional[builtins.int] = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        max_command_length: builtins.int = 4096,
        disable_interface_version_check: bool = False,
    ) -> tuple["Client", Events]:
        """
        Connects to an SMPP server and manages the connection in the background.

        This method opens a connection to the server, returning a client to send
        commands and an event stream to receive messages or errors. The connection
        will manage timeouts, enquire links, and other protocol details automatically.

        Parameters:
            url (str): The URL of the SMPP server to connect to. Supports schemes:
                - `smpp` for plain TCP
                - `ssmpp` or `smpps` for TLS connections
                If no port is specified, the default port 2775 is used.
            enquire_link_interval (Optional[int], default=5000): Interval in milliseconds at which
                EnquireLink commands are sent to the server. Set to `None` to disable EnquireLink.
            enquire_link_response_timeout (int, default=2000): Time in milliseconds to wait
                for a response to an EnquireLink command before considering it failed.
            response_timeout (Optional[int], default=2000): Time in milliseconds to wait
                for a response to any command sent to the server. Set to `None` to wait indefinitely.
            max_command_length (int, default=4096): Maximum length in bytes of incoming SMPP commands.
            disable_interface_version_check (bool, default=False): If `True`, disables
                checking the SMPP interface version. Useful when connecting to servers
                with a different SMPP version than v5.

        Examples:
            Connect to an SMPP server running on localhost at port 2775:

            ```python
            from rusmppyc import Client

            async def example():
                client, events = await Client.connect("smpp://localhost:2775")
            ```

            Connect to an SMPP server running on localhost at port 2775 using TLS:

            ```python
            from rusmppyc import Client

            async def example():
                client, events = await Client.connect("smpps://localhost:2775")
            ```

        Returns:
            tuple[Client, Events]: A tuple containing the client object and an event stream.

        Notes:
            - Path and query parameters in the URL are ignored silently.

        Raises:
            ConnectException: If any of the following occur:
                - The URL is invalid.
                - The URL scheme is not supported.
                - The URL does not contain a host.
                - DNS resolution fails.
                - The connection to the server fails.
                - TLS handshake fails.
        """
        ...
    @classmethod
    async def connected(
        cls,
        read: asyncio.StreamReader,
        write: asyncio.StreamWriter,
        enquire_link_interval: Optional[builtins.int] = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        max_command_length: builtins.int = 4096,
        disable_interface_version_check: bool = False,
    ) -> tuple["Client", Events]:
        """
        Creates a client from an existing asyncio connection.

        This method manages an already established connection (`read` and `write` streams), returning a client to send
        commands and an event stream to receive messages or errors. The connection
        will manage timeouts, enquire links, and other protocol details automatically.

        Parameters:
            enquire_link_interval (Optional[int], default=5000): Interval in milliseconds at which
                EnquireLink commands are sent to the server. Set to `None` to disable EnquireLink.
            enquire_link_response_timeout (int, default=2000): Time in milliseconds to wait
                for a response to an EnquireLink command before considering it failed.
            response_timeout (Optional[int], default=2000): Time in milliseconds to wait
                for a response to any command sent to the server. Set to `None` to wait indefinitely.
            max_command_length (int, default=4096): Maximum length in bytes of incoming SMPP commands.
            disable_interface_version_check (bool, default=False): If `True`, disables
                checking the SMPP interface version. Useful when connecting to servers
                with a different SMPP version than v5.

        Returns:
            tuple[Client, Events]: A tuple containing the client object and an event stream.

        Notes:
            - This method does not establish a new TCP/TLS connection; it wraps an existing one.
        """
        ...
    async def bind_transmitter(
        self,
        system_id: builtins.str = "",
        password: builtins.str = "",
        system_type: builtins.str = "",
        interface_version: InterfaceVersion = InterfaceVersion.Smpp5_0(),
        addr_ton: Ton = Ton.Unknown(),
        addr_npi: Npi = Npi.Unknown(),
        address_range: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindTransmitterResp:
        """
        Sends a BindTransmitter command to the server and waits for a successful BindTransmitterResp.

        This operation binds the client as a transmitter (ESME) to the SMSC, enabling the client
        to submit short messages but not receive them.

        Parameters:
            system_id (str, optional): The system ID used to identify the ESME to the SMSC.
            password (str, optional): The password used to authenticate the bind request.
            system_type (str, optional): The system type to identify the kind of ESME system.
            interface_version (InterfaceVersion, optional): The SMPP protocol version to use. Defaults to Smpp5_0.
            addr_ton (Ton, optional): The Type of Number for the address range. Defaults to Ton.Unknown().
            addr_npi (Npi, optional): The Numbering Plan Indicator for the address range. Defaults to Npi.Unknown().
            address_range (str, optional): The address range associated with the bind. Defaults to an empty string.
            status (CommandStatus, optional): The command status value to include in the BindTransmitter
                request. Defaults to CommandStatus.EsmeRok().

        Returns:
            BindTransmitterResp: The response returned by the server upon a successful bind.

        Raises:
            RusmppycException
        """
        ...

    async def bind_receiver(
        self,
        system_id: builtins.str = "",
        password: builtins.str = "",
        system_type: builtins.str = "",
        interface_version: InterfaceVersion = InterfaceVersion.Smpp5_0(),
        addr_ton: Ton = Ton.Unknown(),
        addr_npi: Npi = Npi.Unknown(),
        address_range: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindReceiverResp:
        """
        Sends a BindReceiver command to the server and waits for a successful BindReceiverResp.

        This operation binds the client as a receiver (ESME) to the SMSC, enabling the client
        to receive short messages but not submit them.

        Parameters:
            system_id (str, optional): The system ID used to identify the ESME to the SMSC.
            password (str, optional): The password used to authenticate the bind request.
            system_type (str, optional): The system type to identify the kind of ESME system.
            interface_version (InterfaceVersion, optional): The SMPP protocol version to use. Defaults to Smpp5_0.
            addr_ton (Ton, optional): The Type of Number for the address range. Defaults to Ton.Unknown().
            addr_npi (Npi, optional): The Numbering Plan Indicator for the address range. Defaults to Npi.Unknown().
            address_range (str, optional): The address range associated with the bind. Defaults to an empty string.
            status (CommandStatus, optional): The command status value to include in the BindReceiver
                request. Defaults to CommandStatus.EsmeRok().

        Returns:
            BindReceiverResp: The response returned by the server upon a successful bind.

        Raises:
            RusmppycException
        """
        ...
    async def bind_transceiver(
        self,
        system_id: builtins.str = "",
        password: builtins.str = "",
        system_type: builtins.str = "",
        interface_version: InterfaceVersion = InterfaceVersion.Smpp5_0(),
        addr_ton: Ton = Ton.Unknown(),
        addr_npi: Npi = Npi.Unknown(),
        address_range: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindTransceiverResp:
        """
        Sends a BindTransceiver command to the server and waits for a successful BindTransceiverResp.

        This operation binds the client as a transceiver (ESME) to the SMSC, enabling the client
        to submit and receive short messages.

        Parameters:
            system_id (str, optional): The system ID used to identify the ESME to the SMSC.
            password (str, optional): The password used to authenticate the bind request.
            system_type (str, optional): The system type to identify the kind of ESME system.
            interface_version (InterfaceVersion, optional): The SMPP protocol version to use. Defaults to Smpp5_0.
            addr_ton (Ton, optional): The Type of Number for the address range. Defaults to Ton.Unknown().
            addr_npi (Npi, optional): The Numbering Plan Indicator for the address range. Defaults to Npi.Unknown().
            address_range (str, optional): The address range associated with the bind. Defaults to an empty string.
            status (CommandStatus, optional): The command status value to include in the BindTransceiver
                request. Defaults to CommandStatus.EsmeRok().

        Returns:
            BindTransceiverResp: The response returned by the server upon a successful bind.

        Raises:
            RusmppycException
        """
        ...
    async def submit_sm(
        self,
        service_type: builtins.str = "",
        source_addr_ton: Ton = Ton.Unknown(),
        source_addr_npi: Npi = Npi.Unknown(),
        source_addr: builtins.str = "",
        dest_addr_ton: Ton = Ton.Unknown(),
        dest_addr_npi: Npi = Npi.Unknown(),
        destination_addr: builtins.str = "",
        esm_class: builtins.int = 0,
        protocol_id: builtins.int = 0,
        priority_flag: builtins.int = 0,
        schedule_delivery_time: builtins.str = "",
        validity_period: builtins.str = "",
        registered_delivery: builtins.int = 0,
        replace_if_present_flag: builtins.int = 0,
        data_coding: DataCoding = DataCoding.McSpecific(),
        sm_default_msg_id: builtins.int = 0,
        short_message: builtins.bytes = b"",
        message_payload: Optional[builtins.bytes] = None,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> SubmitSmResp:
        """
        Sends a SubmitSm command to the server and waits for a successful SubmitSmResp.

        Parameters:
            service_type (str, optional): The service type (e.g., "CMT", "WAP", or vendor-defined).
            source_addr_ton (Ton, optional): The Type of Number for the source address. Defaults to Ton.Unknown().
            source_addr_npi (Npi, optional): The Numbering Plan Indicator for the source address. Defaults to Npi.Unknown().
            source_addr (str, optional): The source address (e.g., sender ID). Defaults to an empty string.
            dest_addr_ton (Ton, optional): The Type of Number for the destination address. Defaults to Ton.Unknown().
            dest_addr_npi (Npi, optional): The Numbering Plan Indicator for the destination address. Defaults to Npi.Unknown().
            destination_addr (str, optional): The destination address (recipient phone number).
            esm_class (int, optional): The message mode and type (e.g., delivery receipt requests, datagram mode). Defaults to 0.
            protocol_id (int, optional): The protocol identifier. Defaults to 0.
            priority_flag (int, optional): The message priority level. Defaults to 0.
            schedule_delivery_time (str, optional): The scheduled delivery time in SMPP absolute or relative format. Defaults to empty.
            validity_period (str, optional): The validity period for the message in SMPP absolute or relative format. Defaults to empty.
            registered_delivery (int, optional): Controls delivery receipt and intermediate notifications. Defaults to 0.
            replace_if_present_flag (int, optional): Indicates whether to replace an existing message with the same ID. Defaults to 0.
            data_coding (DataCoding, optional): The data coding scheme to use for the message. Defaults to DataCoding.McSpecific().
            sm_default_msg_id (int, optional): The default short message ID. Defaults to 0.
            short_message (bytes, optional): The message payload (up to 254 bytes). Ignored if `message_payload` is set.
            message_payload (bytes, optional): An optional TLV parameter carrying the full message body if longer than 254 bytes.
            status (CommandStatus, optional): The command status value to include in the SubmitSm
                request. Defaults to CommandStatus.EsmeRok().

        Returns:
            SubmitSmResp: The response returned by the server upon a successful SubmitSm request.

        Raises:
            RusmppycException
        """
        ...
    async def deliver_sm_resp(
        self,
        sequence_number: builtins.int,
        message_id: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends a DeliverSmResp command to the server.

        Parameters:
            sequence_number (int): The sequence number of the corresponding DeliverSm request.
                This value must match the sequence number of the original deliver_sm PDU.
            message_id (str, optional): The message ID associated with the received message.
                Typically used when responding to delivery receipts. Defaults to an empty string.
            status (CommandStatus, optional): The command status value to include in the DeliverSmResp.
                Defaults to CommandStatus.EsmeRok().

        Returns:
            None

        Raises:
            RusmppycException
        """
        ...
    async def unbind(self) -> None:
        """
        Sends an Unbind command to the server and waits for a successful UnbindResp.

        Returns:
            None

        Raises:
            RusmppycException
        """
        ...
    async def unbind_resp(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends an UnbindResp command to the server.

        Parameters:
            sequence_number (int): The sequence number of the corresponding Unbind request.
                This value must match the sequence number of the original Unbind PDU.
            status (CommandStatus, optional): The command status value to include in the UnbindResp.
                Defaults to CommandStatus.EsmeRok().

        Returns:
            None

        Raises:
            RusmppycException.
        """
        ...
    async def generic_nack(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends a GenericNack command to the server.

        Parameters:
            sequence_number (int): The sequence number of the PDU being negatively acknowledged.
            status (CommandStatus, optional): The command status value to include in the GenericNack.
                Defaults to CommandStatus.EsmeRok().

        Returns:
            None

        Raises:
            RusmppycException.
        """
        ...
    async def close(self) -> None:
        """
        Closes the connection to the SMPP server.

        This method completes when the connection has registered the close request.
        It stops reading from the server, halts internal timers, closes the requests channel,
        flushes any pending requests, and terminates the connection.

        After calling this method, clients can no longer send requests to the server.

        Raises:
            RusmppycException.
        """
        ...
    async def closed(self) -> None:
        """
        Waits until the connection to the SMPP server is fully closed.

        This coroutine completes when the connection has been fully terminated,
        including stopping all reads, flushing pending requests, and releasing resources.
        """
        ...
    def is_closed(self) -> bool:
        """
        Checks whether the connection to the SMPP server is closed.

        Note:
            If this returns `False`, the connection is not necessarily active.
            The connection may be in the process of closing.

        Returns:
            bool: `True` if the connection is fully closed, `False` otherwise.

        See Also:
            is_active: Use this to check if the connection is currently active.
        """
        ...

    def is_active(self) -> bool:
        """
        Checks whether the connection to the SMPP server is active.

        The connection is considered active if all of the following are true:
            - `close` was never called.
            - The connection did not encounter an error.
            - The connection can receive requests from the client.

        Note:
            If this returns `False`, it does not necessarily mean the connection is closed.
            The connection may be in the process of closing.

        Returns:
            bool: `True` if the connection is active, `False` otherwise.

        See Also:
            is_closed: Use this to check if the connection is fully closed.
        """
        ...
