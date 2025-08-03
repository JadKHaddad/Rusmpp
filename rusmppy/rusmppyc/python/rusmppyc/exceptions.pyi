class RusmppycException(Exception):
    pass

class DnsResolverError(RusmppycException):
    pass

class ConnectionError(RusmppycException):
    pass

class IOError(RusmppycException):
    pass

class EncodeError(RusmppycException):
    pass

class DecodeError(RusmppycException):
    pass

class EnquireLinkTimeoutError(RusmppycException):
    pass

class ResponseTimeoutError(RusmppycException):
    pass

class UnexpectedResponseError(RusmppycException):
    pass

class PduError(RusmppycException):
    pass

class OtherError(RusmppycException):
    pass