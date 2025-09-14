__all__ = [
    "RusmppycException",
    "DnsException",
    "ConnectException",
    "ConnectionClosedException",
    "IoException",
    "ResponseTimeoutException",
    "UnexpectedResponseException",
    "UnsupportedInterfaceVersionException",
    "PduException",
]

class RusmppycException(Exception):
    "Base class for all exceptions in the Rusmppyc library."

    ...

class DnsException(RusmppycException):
    "DNS resolution failed."

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

class ResponseTimeoutException(RusmppycException):
    "The `SMPP` operation timed out."

    ...

class UnexpectedResponseException(RusmppycException):
    "The `SMPP` operation failed with an error response from the server."

    ...

class UnsupportedInterfaceVersionException(RusmppycException):
    "The client used an interface version that is not supported by the library."

    ...

class PduException(RusmppycException):
    "The user created a invalid `SMPP` PDU."

    ...
