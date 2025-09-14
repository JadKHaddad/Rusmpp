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

    The client is a handle to communicate with the `SMPP` server through a managed connection in the background
    """
    @classmethod
    async def connect(
        cls,
        host: builtins.str,
        enquire_link_interval: builtins.int = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        max_command_length: builtins.int = 4096,
        disable_interface_version_check: bool = False,
    ) -> tuple["Client", Events]:
        """
        Connects to the `SMPP` server.

        Opens and manages a connection in the background and returns a client and an event stream.

        - The client is used as a handle to communicate with the server through the managed connection.
        - The event stream is used to receive events from the server, such as incoming messages or errors.
        """
        ...
    @classmethod
    async def connected(
        cls,
        read: asyncio.StreamReader,
        write: asyncio.StreamWriter,
        enquire_link_interval: builtins.int = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        max_command_length: builtins.int = 4096,
        disable_interface_version_check: bool = False,
    ) -> tuple["Client", Events]:
        """
        Creates a client from an existing connection.

        Manages a connection in the background and returns a client and an event stream.

        - The client is used as a handle to communicate with the server through the managed connection.
        - The event stream is used to receive events from the server, such as incoming messages or errors.
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
        Sends a BindTransmitter command to the server and waits for a successful :class:`BindTransmitterResp`.
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
        Sends a BindReceiver command to the server and waits for a successful :class:`BindReceiverResp`.
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
        Sends a BindTransceiver command to the server and waits for a successful :class:`BindTransceiverResp`.
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
        Sends a SubmitSm command to the server and waits for a successful :class:`SubmitSmResp`.
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
        """
        ...
    async def unbind(self) -> None:
        """
        Sends an Unbind command to the server and waits for a successful :class:`UnbindResp`.
        """
        ...
    async def unbind_resp(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends an UnbindResp command to the server.
        """
        ...
    async def generic_nack(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None:
        """
        Sends a GenericNack command to the server.
        """
        ...
    async def close(self) -> None:
        """
        Closes the connection.

        This method completes, when the connection has registered the close request.
        The connection will stop reading from the server, stop time keeping, close the requests channel, flush pending requests and terminate.

        After calling this method, clients can no longer send requests to the server.
        """
        ...
    async def closed(self) -> None:
        """
        Completes when the connection is closed.
        """
        ...
    def is_closed(self) -> bool:
        """
        Checks if the connection is closed.

        # Note
        If the connection is not closed, this does not mean that it is active.
        The connection may be in the process of closing.

        To check if the connection is active, use :func:`is_active`.
        """
        ...
    def is_active(self) -> bool:
        """
        Checks if the connection is active.
        The connection is considered active if:
        - :func:`close` was never called.
        - The connection did not encounter an error.
        - The connection can receive requests form the client.
        # Note
        If the connection is not active, this does not mean that it is closed.
        The connection may be in the process of closing.

        To check if the connection is closed, use :func:`is_closed`.
        """
        ...
