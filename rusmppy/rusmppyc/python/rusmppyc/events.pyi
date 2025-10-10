from collections.abc import Awaitable
from .rusmppyc import Event

__all__ = ["Events"]

class Events:
    r"""
    An async stream of Events.

    This class represents a stream of events that can be iterated over asynchronously using `async for`.
    """

    def __aiter__(self) -> Events: ...
    def __anext__(self) -> Awaitable[Event]: ...
