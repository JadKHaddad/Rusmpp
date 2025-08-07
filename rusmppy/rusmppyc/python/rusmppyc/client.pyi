import builtins
import asyncio
from typing import Optional

from .rusmppyc import CommandStatus, BindTransceiverResp
from .events import Events

class Client:
    @classmethod
    async def connect(
        cls,
        host: builtins.str,
        enquire_link_interval: builtins.int = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> tuple["Client", Events]: ...
    @classmethod
    async def connected(
        cls,
        read: asyncio.StreamReader,
        write: asyncio.StreamWriter,
        enquire_link_interval: builtins.int = 5000,
        enquire_link_response_timeout: builtins.int = 2000,
        response_timeout: Optional[builtins.int] = 2000,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> tuple["Client", Events]: ...
    async def bind_transceiver(
        self,
        system_id: builtins.str,
        password: builtins.str,
        status: CommandStatus = CommandStatus.EsmeRok(),
    ) -> BindTransceiverResp: ...
    async def deliver_sm_resp(
        self,
        sequence_number: builtins.int,
        message_id: builtins.str,
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
