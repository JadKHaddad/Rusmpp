import builtins
import asyncio

from .rusmppyc import Events, BindTransceiverResp

class Client:
    @classmethod
    async def connect(cls, host:builtins.str, enquire_link_interval:builtins.int, response_timeout:builtins.int) -> tuple["Client", Events]: ...
    @classmethod
    async def connected(cls, read:asyncio.StreamReader, write:asyncio.StreamWriter, enquire_link_interval:builtins.int, response_timeout:builtins.int) -> tuple["Client", Events]: ...
    async def bind_transceiver(self, system_id:builtins.str, password:builtins.str) -> BindTransceiverResp: ...
