use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::{RusmppIo, RusmppIoU16, RusmppIoU8};

use crate::io::read::{AsyncIoRead, AsyncIoReadable, IoReadError};

#[repr(u8)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU8,
)]
pub enum TypeOfNetwork {
    Generic = 0,
    Gsm = 1,
    Tdma = 2,
    Cdma = 3,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for TypeOfNetwork {
    fn default() -> Self {
        TypeOfNetwork::Generic
    }
}

#[repr(u16)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU16,
)]
pub enum EncodingContentType {
    Index = 0x0000,
    EmergencyBroadcasts = 0x0001,
    IrdbDownload = 0x0002,
    NewsFlashes = 0x0003,
    GeneralNewsLocal = 0x0011,
    GeneralNewsRegional = 0x0012,
    GeneralNewsNational = 0x0013,
    GeneralNewsInternational = 0x0014,
    BusinessFinancialNewsLocal = 0x0015,
    BusinessFinancialNewsRegional = 0x0016,
    BusinessFinancialNewsNational = 0x0017,
    BusinessFinancialNewsInternational = 0x0018,
    SportsNewsLocal = 0x0019,
    SportsNewsRegional = 0x001A,
    SportsNewsNational = 0x001B,
    SportsNewsInternational = 0x001C,
    EntertainmentNewsLocal = 0x001D,
    EntertainmentNewsRegional = 0x001E,
    EntertainmentNewsNational = 0x001F,
    EntertainmentNewsInternational = 0x0020,
    MedicalHealthHospitals = 0x0021,
    Doctors = 0x0022,
    Pharmacy = 0x0023,
    LocalTrafficRoadReports = 0x0030,
    LongDistanceTrafficRoadReports = 0x0031,
    Taxis = 0x0032,
    Weather = 0x0033,
    LocalAirportFlightSchedules = 0x0034,
    Restaurants = 0x0035,
    Lodgings = 0x0036,
    RetailDirectory = 0x0037,
    Advertisements = 0x0038,
    StockQuotes = 0x0039,
    EmploymentOpportunities = 0x0040,
    TechnologyNews = 0x0041,
    DistrictBaseStationInfo = 0x0070,
    NetworkInformation = 0x0071,
    OperatorServices = 0x0080,
    DirectoryEnquiriesNational = 0x0081,
    DirectoryEnquiriesInternational = 0x0082,
    CustomerCareNational = 0x0083,
    CustomerCareInternational = 0x0084,
    LocalDateTimeTimeZone = 0x0085,
    MultiCategoryServices = 0x0100,
    #[num_enum(catch_all)]
    Other(u16),
}

#[allow(clippy::derivable_impls)]
impl Default for EncodingContentType {
    fn default() -> Self {
        EncodingContentType::Index
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct BroadcastContentType {
    pub type_of_network: TypeOfNetwork,
    pub encoding_content_type: EncodingContentType,
}

impl BroadcastContentType {
    pub fn new(type_of_network: TypeOfNetwork, encoding_content_type: EncodingContentType) -> Self {
        Self {
            type_of_network,
            encoding_content_type,
        }
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for BroadcastContentType {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        Ok(Self {
            type_of_network: TypeOfNetwork::async_io_read(buf).await?,
            encoding_content_type: EncodingContentType::async_io_read(buf).await?,
        })
    }
}
