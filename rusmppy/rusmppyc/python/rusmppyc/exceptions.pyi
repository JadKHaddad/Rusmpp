__all__ = [
    "RusmppycException",
    "ConnectException",
    "ConnectionClosedException",
    "IoException",
    "EncodeException",
    "DecodeException",
    "ResponseTimeoutException",
    "UnexpectedResponseException",
    "UnsupportedInterfaceVersionException",
    "ValueException",
]

class RusmppycException(Exception):
    "Base class for all exceptions in the Rusmppyc library."

    ...

class ConnectException(RusmppycException):
    "Connection to `SMPP` server failed."

    ...

class ConnectionClosedException(RusmppycException):
    "Connection to the `SMPP` server is closed."

    ...

class IoException(RusmppycException):
    "IO error occurred."

    ...

class EncodeException(RusmppycException):
    "Failed to encode `SMPP` PDU."

    ...

class DecodeException(RusmppycException):
    "Failed to decode `SMPP` PDU."

    ...

class ResponseTimeoutException(RusmppycException):
    "The `SMPP` operation timed out."

    ...

class UnexpectedResponseException(RusmppycException):
    "The `SMPP` operation failed with an error response from the server."

    ...

class UnsupportedInterfaceVersionException(RusmppycException):
    "The client used an interface version that is not supported by the library."

    ...

class ValueException(RusmppycException):
    "The client created an invalid `SMPP` value."

    ...
