import builtins
import asyncio

from .rusmppyc import BindTransceiverResp
from .events import Events

class Client:
    @classmethod
    async def connect(
        cls,
        host: builtins.str,
        enquire_link_interval: builtins.int,
        enquire_link_response_timeout: builtins.int,
        response_timeout: builtins.int,
    ) -> tuple["Client", Events]: ...
    @classmethod
    async def connected(
        cls,
        read: asyncio.StreamReader,
        write: asyncio.StreamWriter,
        enquire_link_interval: builtins.int,
        enquire_link_response_timeout: builtins.int,
        response_timeout: builtins.int,
    ) -> tuple["Client", Events]: ...
    async def bind_transceiver(
        self, system_id: builtins.str, password: builtins.str
    ) -> BindTransceiverResp: ...
    async def deliver_sm_resp(
        self, sequence_number: builtins.int, message_id: builtins.str
    ) -> None: ...
    async def unbind(self) -> None: ...
    async def unbind_resp(self, sequence_number: builtins.int) -> None: ...
    async def generic_nack(self, sequence_number: builtins.int) -> None: ...
    async def close(self) -> None: ...
    async def closed(self) -> None: ...
