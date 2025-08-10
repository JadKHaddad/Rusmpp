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
)
from .events import Events

class Client:
    @classmethod
    async def connect(
        cls,
        host: builtins.str,
        enquire_link_interval: builtins.int = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        max_command_length: builtins.int = 4096,
        disable_interface_version_check: bool = False,
    ) -> tuple["Client", Events]: ...
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
    ) -> tuple["Client", Events]: ...
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
    ) -> BindTransmitterResp: ...
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
    ) -> BindReceiverResp: ...
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
    ) -> BindTransceiverResp: ...
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
        data_coding: builtins.int = 0,
        sm_default_msg_id: builtins.int = 0,
        short_message: builtins.str = "",
        message_payload: Optional[builtins.str] = None,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> SubmitSmResp: ...
    async def deliver_sm_resp(
        self,
        sequence_number: builtins.int,
        message_id: builtins.str = "",
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None: ...
    async def unbind(self) -> None: ...
    async def unbind_resp(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None: ...
    async def generic_nack(
        self,
        sequence_number: builtins.int,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> None: ...
    async def close(self) -> None: ...
    async def closed(self) -> None: ...
    def is_closed(self) -> bool: ...
    def is_active(self) -> bool: ...
