crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum TypeOfNetwork {
        #[default]
        Generic = 0,
        Gsm = 1,
        Tdma = 2,
        Cdma = 3,
        Other(u8),
    }
}

impl From<u8> for TypeOfNetwork {
    fn from(value: u8) -> Self {
        match value {
            0 => TypeOfNetwork::Generic,
            1 => TypeOfNetwork::Gsm,
            2 => TypeOfNetwork::Tdma,
            3 => TypeOfNetwork::Cdma,
            value => TypeOfNetwork::Other(value),
        }
    }
}

impl From<TypeOfNetwork> for u8 {
    fn from(value: TypeOfNetwork) -> Self {
        match value {
            TypeOfNetwork::Generic => 0,
            TypeOfNetwork::Gsm => 1,
            TypeOfNetwork::Tdma => 2,
            TypeOfNetwork::Cdma => 3,
            TypeOfNetwork::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u16)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum EncodingContentType {
        #[default]
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
        Other(u16),
    }
}

impl From<u16> for EncodingContentType {
    fn from(value: u16) -> Self {
        match value {
            0x0000 => EncodingContentType::Index,
            0x0001 => EncodingContentType::EmergencyBroadcasts,
            0x0002 => EncodingContentType::IrdbDownload,
            0x0003 => EncodingContentType::NewsFlashes,
            0x0011 => EncodingContentType::GeneralNewsLocal,
            0x0012 => EncodingContentType::GeneralNewsRegional,
            0x0013 => EncodingContentType::GeneralNewsNational,
            0x0014 => EncodingContentType::GeneralNewsInternational,
            0x0015 => EncodingContentType::BusinessFinancialNewsLocal,
            0x0016 => EncodingContentType::BusinessFinancialNewsRegional,
            0x0017 => EncodingContentType::BusinessFinancialNewsNational,
            0x0018 => EncodingContentType::BusinessFinancialNewsInternational,
            0x0019 => EncodingContentType::SportsNewsLocal,
            0x001A => EncodingContentType::SportsNewsRegional,
            0x001B => EncodingContentType::SportsNewsNational,
            0x001C => EncodingContentType::SportsNewsInternational,
            0x001D => EncodingContentType::EntertainmentNewsLocal,
            0x001E => EncodingContentType::EntertainmentNewsRegional,
            0x001F => EncodingContentType::EntertainmentNewsNational,
            0x0020 => EncodingContentType::EntertainmentNewsInternational,
            0x0021 => EncodingContentType::MedicalHealthHospitals,
            0x0022 => EncodingContentType::Doctors,
            0x0023 => EncodingContentType::Pharmacy,
            0x0030 => EncodingContentType::LocalTrafficRoadReports,
            0x0031 => EncodingContentType::LongDistanceTrafficRoadReports,
            0x0032 => EncodingContentType::Taxis,
            0x0033 => EncodingContentType::Weather,
            0x0034 => EncodingContentType::LocalAirportFlightSchedules,
            0x0035 => EncodingContentType::Restaurants,
            0x0036 => EncodingContentType::Lodgings,
            0x0037 => EncodingContentType::RetailDirectory,
            0x0038 => EncodingContentType::Advertisements,
            0x0039 => EncodingContentType::StockQuotes,
            0x0040 => EncodingContentType::EmploymentOpportunities,
            0x0041 => EncodingContentType::TechnologyNews,
            0x0070 => EncodingContentType::DistrictBaseStationInfo,
            0x0071 => EncodingContentType::NetworkInformation,
            0x0080 => EncodingContentType::OperatorServices,
            0x0081 => EncodingContentType::DirectoryEnquiriesNational,
            0x0082 => EncodingContentType::DirectoryEnquiriesInternational,
            0x0083 => EncodingContentType::CustomerCareNational,
            0x0084 => EncodingContentType::CustomerCareInternational,
            0x0085 => EncodingContentType::LocalDateTimeTimeZone,
            0x0100 => EncodingContentType::MultiCategoryServices,
            value => EncodingContentType::Other(value),
        }
    }
}

impl From<EncodingContentType> for u16 {
    fn from(value: EncodingContentType) -> Self {
        match value {
            EncodingContentType::Index => 0x0000,
            EncodingContentType::EmergencyBroadcasts => 0x0001,
            EncodingContentType::IrdbDownload => 0x0002,
            EncodingContentType::NewsFlashes => 0x0003,
            EncodingContentType::GeneralNewsLocal => 0x0011,
            EncodingContentType::GeneralNewsRegional => 0x0012,
            EncodingContentType::GeneralNewsNational => 0x0013,
            EncodingContentType::GeneralNewsInternational => 0x0014,
            EncodingContentType::BusinessFinancialNewsLocal => 0x0015,
            EncodingContentType::BusinessFinancialNewsRegional => 0x0016,
            EncodingContentType::BusinessFinancialNewsNational => 0x0017,
            EncodingContentType::BusinessFinancialNewsInternational => 0x0018,
            EncodingContentType::SportsNewsLocal => 0x0019,
            EncodingContentType::SportsNewsRegional => 0x001A,
            EncodingContentType::SportsNewsNational => 0x001B,
            EncodingContentType::SportsNewsInternational => 0x001C,
            EncodingContentType::EntertainmentNewsLocal => 0x001D,
            EncodingContentType::EntertainmentNewsRegional => 0x001E,
            EncodingContentType::EntertainmentNewsNational => 0x001F,
            EncodingContentType::EntertainmentNewsInternational => 0x0020,
            EncodingContentType::MedicalHealthHospitals => 0x0021,
            EncodingContentType::Doctors => 0x0022,
            EncodingContentType::Pharmacy => 0x0023,
            EncodingContentType::LocalTrafficRoadReports => 0x0030,
            EncodingContentType::LongDistanceTrafficRoadReports => 0x0031,
            EncodingContentType::Taxis => 0x0032,
            EncodingContentType::Weather => 0x0033,
            EncodingContentType::LocalAirportFlightSchedules => 0x0034,
            EncodingContentType::Restaurants => 0x0035,
            EncodingContentType::Lodgings => 0x0036,
            EncodingContentType::RetailDirectory => 0x0037,
            EncodingContentType::Advertisements => 0x0038,
            EncodingContentType::StockQuotes => 0x0039,
            EncodingContentType::EmploymentOpportunities => 0x0040,
            EncodingContentType::TechnologyNews => 0x0041,
            EncodingContentType::DistrictBaseStationInfo => 0x0070,
            EncodingContentType::NetworkInformation => 0x0071,
            EncodingContentType::OperatorServices => 0x0080,
            EncodingContentType::DirectoryEnquiriesNational => 0x0081,
            EncodingContentType::DirectoryEnquiriesInternational => 0x0082,
            EncodingContentType::CustomerCareNational => 0x0083,
            EncodingContentType::CustomerCareInternational => 0x0084,
            EncodingContentType::LocalDateTimeTimeZone => 0x0085,
            EncodingContentType::MultiCategoryServices => 0x0100,
            EncodingContentType::Other(value) => value,
        }
    }
}

crate::create! {
    /// Specifies the content type of the message.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct BroadcastContentType {
        pub type_of_network: TypeOfNetwork,
        pub encoding_content_type: EncodingContentType,
    }
}

impl BroadcastContentType {
    pub fn new(type_of_network: TypeOfNetwork, encoding_content_type: EncodingContentType) -> Self {
        Self {
            type_of_network,
            encoding_content_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<TypeOfNetwork>();
        crate::tests::encode_decode_test_instances::<EncodingContentType>();
        crate::tests::encode_decode_test_instances::<BroadcastContentType>();
    }
}
