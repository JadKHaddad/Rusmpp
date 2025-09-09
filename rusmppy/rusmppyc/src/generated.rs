#![allow(clippy::enum_variant_names)]
#![allow(clippy::useless_conversion)]

pub mod rusmpp_types {
    pub use ::rusmpp::{pdus::*, tlvs::*, values::*, Command, CommandId, CommandStatus, Pdu};
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum AddrSubunit {
    Unknown(),
    MSDisplay(),
    MobileEquipment(),
    SmartCard(),
    ExternalUnit(),
    Other(u8),
}

impl From<rusmpp_types::AddrSubunit> for AddrSubunit {
    fn from(value: rusmpp_types::AddrSubunit) -> Self {
        match value {
            rusmpp_types::AddrSubunit::Unknown => AddrSubunit::Unknown(),
            rusmpp_types::AddrSubunit::MSDisplay => AddrSubunit::MSDisplay(),
            rusmpp_types::AddrSubunit::MobileEquipment => AddrSubunit::MobileEquipment(),
            rusmpp_types::AddrSubunit::SmartCard => AddrSubunit::SmartCard(),
            rusmpp_types::AddrSubunit::ExternalUnit => AddrSubunit::ExternalUnit(),
            rusmpp_types::AddrSubunit::Other(inner) => AddrSubunit::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl AddrSubunit {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum AlertOnMessageDelivery {
    UseMobileDefaultAlert(),
    UseLowPriorityAlert(),
    UseMediumPriorityAlert(),
    UseHighPriorityAlert(),
    Other(u8),
}

impl From<rusmpp_types::AlertOnMessageDelivery> for AlertOnMessageDelivery {
    fn from(value: rusmpp_types::AlertOnMessageDelivery) -> Self {
        match value {
            rusmpp_types::AlertOnMessageDelivery::UseMobileDefaultAlert => {
                AlertOnMessageDelivery::UseMobileDefaultAlert()
            }
            rusmpp_types::AlertOnMessageDelivery::UseLowPriorityAlert => {
                AlertOnMessageDelivery::UseLowPriorityAlert()
            }
            rusmpp_types::AlertOnMessageDelivery::UseMediumPriorityAlert => {
                AlertOnMessageDelivery::UseMediumPriorityAlert()
            }
            rusmpp_types::AlertOnMessageDelivery::UseHighPriorityAlert => {
                AlertOnMessageDelivery::UseHighPriorityAlert()
            }
            rusmpp_types::AlertOnMessageDelivery::Other(inner) => {
                AlertOnMessageDelivery::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl AlertOnMessageDelivery {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum BearerType {
    Unknown(),
    Sms(),
    Csd(),
    PacketData(),
    Ussd(),
    Cdpd(),
    DataTac(),
    FlexReFlex(),
    CellBroadcast(),
    Other(u8),
}

impl From<rusmpp_types::BearerType> for BearerType {
    fn from(value: rusmpp_types::BearerType) -> Self {
        match value {
            rusmpp_types::BearerType::Unknown => BearerType::Unknown(),
            rusmpp_types::BearerType::Sms => BearerType::Sms(),
            rusmpp_types::BearerType::Csd => BearerType::Csd(),
            rusmpp_types::BearerType::PacketData => BearerType::PacketData(),
            rusmpp_types::BearerType::Ussd => BearerType::Ussd(),
            rusmpp_types::BearerType::Cdpd => BearerType::Cdpd(),
            rusmpp_types::BearerType::DataTac => BearerType::DataTac(),
            rusmpp_types::BearerType::FlexReFlex => BearerType::FlexReFlex(),
            rusmpp_types::BearerType::CellBroadcast => BearerType::CellBroadcast(),
            rusmpp_types::BearerType::Other(inner) => BearerType::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BearerType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum BroadcastAreaFormat {
    AliasName(),
    EllipsoidArc(),
    Polygon(),
    Other(u8),
}

impl From<rusmpp_types::BroadcastAreaFormat> for BroadcastAreaFormat {
    fn from(value: rusmpp_types::BroadcastAreaFormat) -> Self {
        match value {
            rusmpp_types::BroadcastAreaFormat::AliasName => BroadcastAreaFormat::AliasName(),
            rusmpp_types::BroadcastAreaFormat::EllipsoidArc => BroadcastAreaFormat::EllipsoidArc(),
            rusmpp_types::BroadcastAreaFormat::Polygon => BroadcastAreaFormat::Polygon(),
            rusmpp_types::BroadcastAreaFormat::Other(inner) => {
                BroadcastAreaFormat::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastAreaFormat {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BroadcastAreaIdentifier {
    pub format: BroadcastAreaFormat,
    pub area: Vec<u8>,
}

impl From<rusmpp_types::BroadcastAreaIdentifier> for BroadcastAreaIdentifier {
    fn from(value: rusmpp_types::BroadcastAreaIdentifier) -> Self {
        let value = value.into_parts();
        Self {
            format: value.format.into(),
            area: value.area.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastAreaIdentifier {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum BroadcastAreaSuccess {
    InformationNotAvailable(),
    ZeroToHundred(u8),
    Other(u8),
}

impl From<rusmpp_types::BroadcastAreaSuccess> for BroadcastAreaSuccess {
    fn from(value: rusmpp_types::BroadcastAreaSuccess) -> Self {
        match value {
            rusmpp_types::BroadcastAreaSuccess::InformationNotAvailable => {
                BroadcastAreaSuccess::InformationNotAvailable()
            }
            rusmpp_types::BroadcastAreaSuccess::ZeroToHundred(inner) => {
                BroadcastAreaSuccess::ZeroToHundred(inner.into())
            }
            rusmpp_types::BroadcastAreaSuccess::Other(inner) => {
                BroadcastAreaSuccess::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastAreaSuccess {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum BroadcastChannelIndicator {
    Basic(),
    Extended(),
    Other(u8),
}

impl From<rusmpp_types::BroadcastChannelIndicator> for BroadcastChannelIndicator {
    fn from(value: rusmpp_types::BroadcastChannelIndicator) -> Self {
        match value {
            rusmpp_types::BroadcastChannelIndicator::Basic => BroadcastChannelIndicator::Basic(),
            rusmpp_types::BroadcastChannelIndicator::Extended => {
                BroadcastChannelIndicator::Extended()
            }
            rusmpp_types::BroadcastChannelIndicator::Other(inner) => {
                BroadcastChannelIndicator::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastChannelIndicator {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum EncodingContentType {
    Index(),
    EmergencyBroadcasts(),
    IrdbDownload(),
    NewsFlashes(),
    GeneralNewsLocal(),
    GeneralNewsRegional(),
    GeneralNewsNational(),
    GeneralNewsInternational(),
    BusinessFinancialNewsLocal(),
    BusinessFinancialNewsRegional(),
    BusinessFinancialNewsNational(),
    BusinessFinancialNewsInternational(),
    SportsNewsLocal(),
    SportsNewsRegional(),
    SportsNewsNational(),
    SportsNewsInternational(),
    EntertainmentNewsLocal(),
    EntertainmentNewsRegional(),
    EntertainmentNewsNational(),
    EntertainmentNewsInternational(),
    MedicalHealthHospitals(),
    Doctors(),
    Pharmacy(),
    LocalTrafficRoadReports(),
    LongDistanceTrafficRoadReports(),
    Taxis(),
    Weather(),
    LocalAirportFlightSchedules(),
    Restaurants(),
    Lodgings(),
    RetailDirectory(),
    Advertisements(),
    StockQuotes(),
    EmploymentOpportunities(),
    TechnologyNews(),
    DistrictBaseStationInfo(),
    NetworkInformation(),
    OperatorServices(),
    DirectoryEnquiriesNational(),
    DirectoryEnquiriesInternational(),
    CustomerCareNational(),
    CustomerCareInternational(),
    LocalDateTimeTimeZone(),
    MultiCategoryServices(),
    Other(u16),
}

impl From<rusmpp_types::EncodingContentType> for EncodingContentType {
    fn from(value: rusmpp_types::EncodingContentType) -> Self {
        match value {
            rusmpp_types::EncodingContentType::Index => EncodingContentType::Index(),
            rusmpp_types::EncodingContentType::EmergencyBroadcasts => {
                EncodingContentType::EmergencyBroadcasts()
            }
            rusmpp_types::EncodingContentType::IrdbDownload => EncodingContentType::IrdbDownload(),
            rusmpp_types::EncodingContentType::NewsFlashes => EncodingContentType::NewsFlashes(),
            rusmpp_types::EncodingContentType::GeneralNewsLocal => {
                EncodingContentType::GeneralNewsLocal()
            }
            rusmpp_types::EncodingContentType::GeneralNewsRegional => {
                EncodingContentType::GeneralNewsRegional()
            }
            rusmpp_types::EncodingContentType::GeneralNewsNational => {
                EncodingContentType::GeneralNewsNational()
            }
            rusmpp_types::EncodingContentType::GeneralNewsInternational => {
                EncodingContentType::GeneralNewsInternational()
            }
            rusmpp_types::EncodingContentType::BusinessFinancialNewsLocal => {
                EncodingContentType::BusinessFinancialNewsLocal()
            }
            rusmpp_types::EncodingContentType::BusinessFinancialNewsRegional => {
                EncodingContentType::BusinessFinancialNewsRegional()
            }
            rusmpp_types::EncodingContentType::BusinessFinancialNewsNational => {
                EncodingContentType::BusinessFinancialNewsNational()
            }
            rusmpp_types::EncodingContentType::BusinessFinancialNewsInternational => {
                EncodingContentType::BusinessFinancialNewsInternational()
            }
            rusmpp_types::EncodingContentType::SportsNewsLocal => {
                EncodingContentType::SportsNewsLocal()
            }
            rusmpp_types::EncodingContentType::SportsNewsRegional => {
                EncodingContentType::SportsNewsRegional()
            }
            rusmpp_types::EncodingContentType::SportsNewsNational => {
                EncodingContentType::SportsNewsNational()
            }
            rusmpp_types::EncodingContentType::SportsNewsInternational => {
                EncodingContentType::SportsNewsInternational()
            }
            rusmpp_types::EncodingContentType::EntertainmentNewsLocal => {
                EncodingContentType::EntertainmentNewsLocal()
            }
            rusmpp_types::EncodingContentType::EntertainmentNewsRegional => {
                EncodingContentType::EntertainmentNewsRegional()
            }
            rusmpp_types::EncodingContentType::EntertainmentNewsNational => {
                EncodingContentType::EntertainmentNewsNational()
            }
            rusmpp_types::EncodingContentType::EntertainmentNewsInternational => {
                EncodingContentType::EntertainmentNewsInternational()
            }
            rusmpp_types::EncodingContentType::MedicalHealthHospitals => {
                EncodingContentType::MedicalHealthHospitals()
            }
            rusmpp_types::EncodingContentType::Doctors => EncodingContentType::Doctors(),
            rusmpp_types::EncodingContentType::Pharmacy => EncodingContentType::Pharmacy(),
            rusmpp_types::EncodingContentType::LocalTrafficRoadReports => {
                EncodingContentType::LocalTrafficRoadReports()
            }
            rusmpp_types::EncodingContentType::LongDistanceTrafficRoadReports => {
                EncodingContentType::LongDistanceTrafficRoadReports()
            }
            rusmpp_types::EncodingContentType::Taxis => EncodingContentType::Taxis(),
            rusmpp_types::EncodingContentType::Weather => EncodingContentType::Weather(),
            rusmpp_types::EncodingContentType::LocalAirportFlightSchedules => {
                EncodingContentType::LocalAirportFlightSchedules()
            }
            rusmpp_types::EncodingContentType::Restaurants => EncodingContentType::Restaurants(),
            rusmpp_types::EncodingContentType::Lodgings => EncodingContentType::Lodgings(),
            rusmpp_types::EncodingContentType::RetailDirectory => {
                EncodingContentType::RetailDirectory()
            }
            rusmpp_types::EncodingContentType::Advertisements => {
                EncodingContentType::Advertisements()
            }
            rusmpp_types::EncodingContentType::StockQuotes => EncodingContentType::StockQuotes(),
            rusmpp_types::EncodingContentType::EmploymentOpportunities => {
                EncodingContentType::EmploymentOpportunities()
            }
            rusmpp_types::EncodingContentType::TechnologyNews => {
                EncodingContentType::TechnologyNews()
            }
            rusmpp_types::EncodingContentType::DistrictBaseStationInfo => {
                EncodingContentType::DistrictBaseStationInfo()
            }
            rusmpp_types::EncodingContentType::NetworkInformation => {
                EncodingContentType::NetworkInformation()
            }
            rusmpp_types::EncodingContentType::OperatorServices => {
                EncodingContentType::OperatorServices()
            }
            rusmpp_types::EncodingContentType::DirectoryEnquiriesNational => {
                EncodingContentType::DirectoryEnquiriesNational()
            }
            rusmpp_types::EncodingContentType::DirectoryEnquiriesInternational => {
                EncodingContentType::DirectoryEnquiriesInternational()
            }
            rusmpp_types::EncodingContentType::CustomerCareNational => {
                EncodingContentType::CustomerCareNational()
            }
            rusmpp_types::EncodingContentType::CustomerCareInternational => {
                EncodingContentType::CustomerCareInternational()
            }
            rusmpp_types::EncodingContentType::LocalDateTimeTimeZone => {
                EncodingContentType::LocalDateTimeTimeZone()
            }
            rusmpp_types::EncodingContentType::MultiCategoryServices => {
                EncodingContentType::MultiCategoryServices()
            }
            rusmpp_types::EncodingContentType::Other(inner) => {
                EncodingContentType::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl EncodingContentType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum TypeOfNetwork {
    Generic(),
    Gsm(),
    Tdma(),
    Cdma(),
    Other(u8),
}

impl From<rusmpp_types::TypeOfNetwork> for TypeOfNetwork {
    fn from(value: rusmpp_types::TypeOfNetwork) -> Self {
        match value {
            rusmpp_types::TypeOfNetwork::Generic => TypeOfNetwork::Generic(),
            rusmpp_types::TypeOfNetwork::Gsm => TypeOfNetwork::Gsm(),
            rusmpp_types::TypeOfNetwork::Tdma => TypeOfNetwork::Tdma(),
            rusmpp_types::TypeOfNetwork::Cdma => TypeOfNetwork::Cdma(),
            rusmpp_types::TypeOfNetwork::Other(inner) => TypeOfNetwork::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl TypeOfNetwork {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BroadcastContentType {
    pub type_of_network: TypeOfNetwork,
    pub encoding_content_type: EncodingContentType,
}

impl From<rusmpp_types::BroadcastContentType> for BroadcastContentType {
    fn from(value: rusmpp_types::BroadcastContentType) -> Self {
        let value = value.into_parts();
        Self {
            type_of_network: value.type_of_network.into(),
            encoding_content_type: value.encoding_content_type.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastContentType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum UnitOfTime {
    AsFrequentlyAsPossible(),
    Seconds(),
    Minutes(),
    Hours(),
    Days(),
    Weeks(),
    Months(),
    Years(),
    Other(u8),
}

impl From<rusmpp_types::UnitOfTime> for UnitOfTime {
    fn from(value: rusmpp_types::UnitOfTime) -> Self {
        match value {
            rusmpp_types::UnitOfTime::AsFrequentlyAsPossible => {
                UnitOfTime::AsFrequentlyAsPossible()
            }
            rusmpp_types::UnitOfTime::Seconds => UnitOfTime::Seconds(),
            rusmpp_types::UnitOfTime::Minutes => UnitOfTime::Minutes(),
            rusmpp_types::UnitOfTime::Hours => UnitOfTime::Hours(),
            rusmpp_types::UnitOfTime::Days => UnitOfTime::Days(),
            rusmpp_types::UnitOfTime::Weeks => UnitOfTime::Weeks(),
            rusmpp_types::UnitOfTime::Months => UnitOfTime::Months(),
            rusmpp_types::UnitOfTime::Years => UnitOfTime::Years(),
            rusmpp_types::UnitOfTime::Other(inner) => UnitOfTime::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl UnitOfTime {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BroadcastFrequencyInterval {
    pub unit: UnitOfTime,
    pub value: u16,
}

impl From<rusmpp_types::BroadcastFrequencyInterval> for BroadcastFrequencyInterval {
    fn from(value: rusmpp_types::BroadcastFrequencyInterval) -> Self {
        let value = value.into_parts();
        Self {
            unit: value.unit.into(),
            value: value.value.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastFrequencyInterval {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum BroadcastMessageClass {
    NoClassSpecified(),
    Class1(),
    Class2(),
    Class3(),
    Other(u8),
}

impl From<rusmpp_types::BroadcastMessageClass> for BroadcastMessageClass {
    fn from(value: rusmpp_types::BroadcastMessageClass) -> Self {
        match value {
            rusmpp_types::BroadcastMessageClass::NoClassSpecified => {
                BroadcastMessageClass::NoClassSpecified()
            }
            rusmpp_types::BroadcastMessageClass::Class1 => BroadcastMessageClass::Class1(),
            rusmpp_types::BroadcastMessageClass::Class2 => BroadcastMessageClass::Class2(),
            rusmpp_types::BroadcastMessageClass::Class3 => BroadcastMessageClass::Class3(),
            rusmpp_types::BroadcastMessageClass::Other(inner) => {
                BroadcastMessageClass::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastMessageClass {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BroadcastRepNum {
    pub value: u8,
}

impl From<rusmpp_types::BroadcastRepNum> for BroadcastRepNum {
    fn from(value: rusmpp_types::BroadcastRepNum) -> Self {
        let value = value.into_parts();
        Self {
            value: value.value.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastRepNum {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Presentation {
    PresentationAllowed(),
    PresentationRestricted(),
    NumberNotAvailable(),
    Other(u8),
}

impl From<rusmpp_types::Presentation> for Presentation {
    fn from(value: rusmpp_types::Presentation) -> Self {
        match value {
            rusmpp_types::Presentation::PresentationAllowed => Presentation::PresentationAllowed(),
            rusmpp_types::Presentation::PresentationRestricted => {
                Presentation::PresentationRestricted()
            }
            rusmpp_types::Presentation::NumberNotAvailable => Presentation::NumberNotAvailable(),
            rusmpp_types::Presentation::Other(inner) => Presentation::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Presentation {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Screening {
    NotScreened(),
    VerifiedAndPassed(),
    VerifiedAndFailed(),
    NetworkProvided(),
    Other(u8),
}

impl From<rusmpp_types::Screening> for Screening {
    fn from(value: rusmpp_types::Screening) -> Self {
        match value {
            rusmpp_types::Screening::NotScreened => Screening::NotScreened(),
            rusmpp_types::Screening::VerifiedAndPassed => Screening::VerifiedAndPassed(),
            rusmpp_types::Screening::VerifiedAndFailed => Screening::VerifiedAndFailed(),
            rusmpp_types::Screening::NetworkProvided => Screening::NetworkProvided(),
            rusmpp_types::Screening::Other(inner) => Screening::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Screening {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct CallbackNumPresInd {
    pub presentation: Presentation,
    pub screening: Screening,
}

impl From<rusmpp_types::CallbackNumPresInd> for CallbackNumPresInd {
    fn from(value: rusmpp_types::CallbackNumPresInd) -> Self {
        let value = value.into_parts();
        Self {
            presentation: value.presentation.into(),
            screening: value.screening.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl CallbackNumPresInd {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum CommandStatus {
    EsmeRok(),
    EsmeRinvmsglen(),
    EsmeRinvcmdlen(),
    EsmeRinvcmdid(),
    EsmeRinvbndsts(),
    EsmeRalybnd(),
    EsmeRinvprtflg(),
    EsmeRinvregdlvflg(),
    EsmeRsyserr(),
    EsmeRinvsrcadr(),
    EsmeRinvdstadr(),
    EsmeRinvmsgid(),
    EsmeRbindfail(),
    EsmeRinvpaswd(),
    EsmeRinvsysid(),
    EsmeRcancelfail(),
    EsmeRreplacefail(),
    EsmeRmsgqful(),
    EsmeRinvsertyp(),
    EsmeRinvnumdests(),
    EsmeRinvdlname(),
    EsmeRinvdestflag(),
    EsmeRinvsubrep(),
    EsmeRinvesmclass(),
    EsmeRcntsubdl(),
    EsmeRsubmitfail(),
    EsmeRinvsrcton(),
    EsmeRinvsrcnpi(),
    EsmeRinvdstton(),
    EsmeRinvdstnpi(),
    EsmeRinvsystyp(),
    EsmeRinvrepflag(),
    EsmeRinvnummsgs(),
    EsmeRthrottled(),
    EsmeRinvsched(),
    EsmeRinvexpiry(),
    EsmeRinvdftmsgid(),
    EsmeRxTAppn(),
    EsmeRxPAppn(),
    EsmeRxRAppn(),
    EsmeRqueryfail(),
    EsmeRinvtlvstream(),
    EsmeRtlvnotallwd(),
    EsmeRinvtlvlen(),
    EsmeRmissingtlv(),
    EsmeRinvtlvval(),
    EsmeRdeliveryfailure(),
    EsmeRunknownerr(),
    EsmeRsertypunauth(),
    EsmeRprohibited(),
    EsmeRsertypunavail(),
    EsmeRsertypdenied(),
    EsmeRinvdcs(),
    EsmeRinvsrcaddrsubunit(),
    EsmeRinvdstaddrsubunit(),
    EsmeRinvbcastfreqint(),
    EsmeRinvbcastaliasName(),
    EsmeRinvbcastareafmt(),
    EsmeRinvnumbcastAreas(),
    EsmeRinvbcastcnttype(),
    EsmeRinvbcastmsgclass(),
    EsmeRbcastfail(),
    EsmeRbcastqueryfail(),
    EsmeRbcastcancelfail(),
    EsmeRinvbcastRep(),
    EsmeRinvbcastsrvgrp(),
    EsmeRinvbcastchanind(),
    Other(u32),
}

impl From<rusmpp_types::CommandStatus> for CommandStatus {
    fn from(value: rusmpp_types::CommandStatus) -> Self {
        match value {
            rusmpp_types::CommandStatus::EsmeRok => CommandStatus::EsmeRok(),
            rusmpp_types::CommandStatus::EsmeRinvmsglen => CommandStatus::EsmeRinvmsglen(),
            rusmpp_types::CommandStatus::EsmeRinvcmdlen => CommandStatus::EsmeRinvcmdlen(),
            rusmpp_types::CommandStatus::EsmeRinvcmdid => CommandStatus::EsmeRinvcmdid(),
            rusmpp_types::CommandStatus::EsmeRinvbndsts => CommandStatus::EsmeRinvbndsts(),
            rusmpp_types::CommandStatus::EsmeRalybnd => CommandStatus::EsmeRalybnd(),
            rusmpp_types::CommandStatus::EsmeRinvprtflg => CommandStatus::EsmeRinvprtflg(),
            rusmpp_types::CommandStatus::EsmeRinvregdlvflg => CommandStatus::EsmeRinvregdlvflg(),
            rusmpp_types::CommandStatus::EsmeRsyserr => CommandStatus::EsmeRsyserr(),
            rusmpp_types::CommandStatus::EsmeRinvsrcadr => CommandStatus::EsmeRinvsrcadr(),
            rusmpp_types::CommandStatus::EsmeRinvdstadr => CommandStatus::EsmeRinvdstadr(),
            rusmpp_types::CommandStatus::EsmeRinvmsgid => CommandStatus::EsmeRinvmsgid(),
            rusmpp_types::CommandStatus::EsmeRbindfail => CommandStatus::EsmeRbindfail(),
            rusmpp_types::CommandStatus::EsmeRinvpaswd => CommandStatus::EsmeRinvpaswd(),
            rusmpp_types::CommandStatus::EsmeRinvsysid => CommandStatus::EsmeRinvsysid(),
            rusmpp_types::CommandStatus::EsmeRcancelfail => CommandStatus::EsmeRcancelfail(),
            rusmpp_types::CommandStatus::EsmeRreplacefail => CommandStatus::EsmeRreplacefail(),
            rusmpp_types::CommandStatus::EsmeRmsgqful => CommandStatus::EsmeRmsgqful(),
            rusmpp_types::CommandStatus::EsmeRinvsertyp => CommandStatus::EsmeRinvsertyp(),
            rusmpp_types::CommandStatus::EsmeRinvnumdests => CommandStatus::EsmeRinvnumdests(),
            rusmpp_types::CommandStatus::EsmeRinvdlname => CommandStatus::EsmeRinvdlname(),
            rusmpp_types::CommandStatus::EsmeRinvdestflag => CommandStatus::EsmeRinvdestflag(),
            rusmpp_types::CommandStatus::EsmeRinvsubrep => CommandStatus::EsmeRinvsubrep(),
            rusmpp_types::CommandStatus::EsmeRinvesmclass => CommandStatus::EsmeRinvesmclass(),
            rusmpp_types::CommandStatus::EsmeRcntsubdl => CommandStatus::EsmeRcntsubdl(),
            rusmpp_types::CommandStatus::EsmeRsubmitfail => CommandStatus::EsmeRsubmitfail(),
            rusmpp_types::CommandStatus::EsmeRinvsrcton => CommandStatus::EsmeRinvsrcton(),
            rusmpp_types::CommandStatus::EsmeRinvsrcnpi => CommandStatus::EsmeRinvsrcnpi(),
            rusmpp_types::CommandStatus::EsmeRinvdstton => CommandStatus::EsmeRinvdstton(),
            rusmpp_types::CommandStatus::EsmeRinvdstnpi => CommandStatus::EsmeRinvdstnpi(),
            rusmpp_types::CommandStatus::EsmeRinvsystyp => CommandStatus::EsmeRinvsystyp(),
            rusmpp_types::CommandStatus::EsmeRinvrepflag => CommandStatus::EsmeRinvrepflag(),
            rusmpp_types::CommandStatus::EsmeRinvnummsgs => CommandStatus::EsmeRinvnummsgs(),
            rusmpp_types::CommandStatus::EsmeRthrottled => CommandStatus::EsmeRthrottled(),
            rusmpp_types::CommandStatus::EsmeRinvsched => CommandStatus::EsmeRinvsched(),
            rusmpp_types::CommandStatus::EsmeRinvexpiry => CommandStatus::EsmeRinvexpiry(),
            rusmpp_types::CommandStatus::EsmeRinvdftmsgid => CommandStatus::EsmeRinvdftmsgid(),
            rusmpp_types::CommandStatus::EsmeRxTAppn => CommandStatus::EsmeRxTAppn(),
            rusmpp_types::CommandStatus::EsmeRxPAppn => CommandStatus::EsmeRxPAppn(),
            rusmpp_types::CommandStatus::EsmeRxRAppn => CommandStatus::EsmeRxRAppn(),
            rusmpp_types::CommandStatus::EsmeRqueryfail => CommandStatus::EsmeRqueryfail(),
            rusmpp_types::CommandStatus::EsmeRinvtlvstream => CommandStatus::EsmeRinvtlvstream(),
            rusmpp_types::CommandStatus::EsmeRtlvnotallwd => CommandStatus::EsmeRtlvnotallwd(),
            rusmpp_types::CommandStatus::EsmeRinvtlvlen => CommandStatus::EsmeRinvtlvlen(),
            rusmpp_types::CommandStatus::EsmeRmissingtlv => CommandStatus::EsmeRmissingtlv(),
            rusmpp_types::CommandStatus::EsmeRinvtlvval => CommandStatus::EsmeRinvtlvval(),
            rusmpp_types::CommandStatus::EsmeRdeliveryfailure => {
                CommandStatus::EsmeRdeliveryfailure()
            }
            rusmpp_types::CommandStatus::EsmeRunknownerr => CommandStatus::EsmeRunknownerr(),
            rusmpp_types::CommandStatus::EsmeRsertypunauth => CommandStatus::EsmeRsertypunauth(),
            rusmpp_types::CommandStatus::EsmeRprohibited => CommandStatus::EsmeRprohibited(),
            rusmpp_types::CommandStatus::EsmeRsertypunavail => CommandStatus::EsmeRsertypunavail(),
            rusmpp_types::CommandStatus::EsmeRsertypdenied => CommandStatus::EsmeRsertypdenied(),
            rusmpp_types::CommandStatus::EsmeRinvdcs => CommandStatus::EsmeRinvdcs(),
            rusmpp_types::CommandStatus::EsmeRinvsrcaddrsubunit => {
                CommandStatus::EsmeRinvsrcaddrsubunit()
            }
            rusmpp_types::CommandStatus::EsmeRinvdstaddrsubunit => {
                CommandStatus::EsmeRinvdstaddrsubunit()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastfreqint => {
                CommandStatus::EsmeRinvbcastfreqint()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastaliasName => {
                CommandStatus::EsmeRinvbcastaliasName()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastareafmt => {
                CommandStatus::EsmeRinvbcastareafmt()
            }
            rusmpp_types::CommandStatus::EsmeRinvnumbcastAreas => {
                CommandStatus::EsmeRinvnumbcastAreas()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastcnttype => {
                CommandStatus::EsmeRinvbcastcnttype()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastmsgclass => {
                CommandStatus::EsmeRinvbcastmsgclass()
            }
            rusmpp_types::CommandStatus::EsmeRbcastfail => CommandStatus::EsmeRbcastfail(),
            rusmpp_types::CommandStatus::EsmeRbcastqueryfail => {
                CommandStatus::EsmeRbcastqueryfail()
            }
            rusmpp_types::CommandStatus::EsmeRbcastcancelfail => {
                CommandStatus::EsmeRbcastcancelfail()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastRep => CommandStatus::EsmeRinvbcastRep(),
            rusmpp_types::CommandStatus::EsmeRinvbcastsrvgrp => {
                CommandStatus::EsmeRinvbcastsrvgrp()
            }
            rusmpp_types::CommandStatus::EsmeRinvbcastchanind => {
                CommandStatus::EsmeRinvbcastchanind()
            }
            rusmpp_types::CommandStatus::Other(inner) => CommandStatus::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl CommandStatus {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum CongestionState {
    Idle(),
    LowLoad(u8),
    MediumLoad(u8),
    HighLoad(u8),
    OptimumLoad(u8),
    NearingCongestion(u8),
    Congested(),
    Other(u8),
}

impl From<rusmpp_types::CongestionState> for CongestionState {
    fn from(value: rusmpp_types::CongestionState) -> Self {
        match value {
            rusmpp_types::CongestionState::Idle => CongestionState::Idle(),
            rusmpp_types::CongestionState::LowLoad(inner) => CongestionState::LowLoad(inner.into()),
            rusmpp_types::CongestionState::MediumLoad(inner) => {
                CongestionState::MediumLoad(inner.into())
            }
            rusmpp_types::CongestionState::HighLoad(inner) => {
                CongestionState::HighLoad(inner.into())
            }
            rusmpp_types::CongestionState::OptimumLoad(inner) => {
                CongestionState::OptimumLoad(inner.into())
            }
            rusmpp_types::CongestionState::NearingCongestion(inner) => {
                CongestionState::NearingCongestion(inner.into())
            }
            rusmpp_types::CongestionState::Congested => CongestionState::Congested(),
            rusmpp_types::CongestionState::Other(inner) => CongestionState::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl CongestionState {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DeliveryFailureReason {
    DestinationUnavailable(),
    DestinationAddressInvalid(),
    PermanentNetworkError(),
    TemporaryNetworkError(),
    Other(u8),
}

impl From<rusmpp_types::DeliveryFailureReason> for DeliveryFailureReason {
    fn from(value: rusmpp_types::DeliveryFailureReason) -> Self {
        match value {
            rusmpp_types::DeliveryFailureReason::DestinationUnavailable => {
                DeliveryFailureReason::DestinationUnavailable()
            }
            rusmpp_types::DeliveryFailureReason::DestinationAddressInvalid => {
                DeliveryFailureReason::DestinationAddressInvalid()
            }
            rusmpp_types::DeliveryFailureReason::PermanentNetworkError => {
                DeliveryFailureReason::PermanentNetworkError()
            }
            rusmpp_types::DeliveryFailureReason::TemporaryNetworkError => {
                DeliveryFailureReason::TemporaryNetworkError()
            }
            rusmpp_types::DeliveryFailureReason::Other(inner) => {
                DeliveryFailureReason::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DeliveryFailureReason {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DestAddrNpResolution {
    QueryNotPerformed(),
    QueryPerformedNumberNotPorted(),
    QueryPerformedNumberPorted(),
    Other(u8),
}

impl From<rusmpp_types::DestAddrNpResolution> for DestAddrNpResolution {
    fn from(value: rusmpp_types::DestAddrNpResolution) -> Self {
        match value {
            rusmpp_types::DestAddrNpResolution::QueryNotPerformed => {
                DestAddrNpResolution::QueryNotPerformed()
            }
            rusmpp_types::DestAddrNpResolution::QueryPerformedNumberNotPorted => {
                DestAddrNpResolution::QueryPerformedNumberNotPorted()
            }
            rusmpp_types::DestAddrNpResolution::QueryPerformedNumberPorted => {
                DestAddrNpResolution::QueryPerformedNumberPorted()
            }
            rusmpp_types::DestAddrNpResolution::Other(inner) => {
                DestAddrNpResolution::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DestAddrNpResolution {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DisplayTime {
    Temporary(),
    Default(),
    Invoke(),
    Other(u8),
}

impl From<rusmpp_types::DisplayTime> for DisplayTime {
    fn from(value: rusmpp_types::DisplayTime) -> Self {
        match value {
            rusmpp_types::DisplayTime::Temporary => DisplayTime::Temporary(),
            rusmpp_types::DisplayTime::Default => DisplayTime::Default(),
            rusmpp_types::DisplayTime::Invoke => DisplayTime::Invoke(),
            rusmpp_types::DisplayTime::Other(inner) => DisplayTime::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DisplayTime {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DpfResult {
    NotSet(),
    Set(),
    Other(u8),
}

impl From<rusmpp_types::DpfResult> for DpfResult {
    fn from(value: rusmpp_types::DpfResult) -> Self {
        match value {
            rusmpp_types::DpfResult::NotSet => DpfResult::NotSet(),
            rusmpp_types::DpfResult::Set => DpfResult::Set(),
            rusmpp_types::DpfResult::Other(inner) => DpfResult::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DpfResult {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum InterfaceVersion {
    Smpp3_3OrEarlier(u8),
    Smpp3_4(),
    Smpp5_0(),
    Other(u8),
}

impl From<rusmpp_types::InterfaceVersion> for InterfaceVersion {
    fn from(value: rusmpp_types::InterfaceVersion) -> Self {
        match value {
            rusmpp_types::InterfaceVersion::Smpp3_3OrEarlier(inner) => {
                InterfaceVersion::Smpp3_3OrEarlier(inner.into())
            }
            rusmpp_types::InterfaceVersion::Smpp3_4 => InterfaceVersion::Smpp3_4(),
            rusmpp_types::InterfaceVersion::Smpp5_0 => InterfaceVersion::Smpp5_0(),
            rusmpp_types::InterfaceVersion::Other(inner) => InterfaceVersion::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl InterfaceVersion {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum ItsReplyType {
    Digit(),
    Number(),
    TelephoneNo(),
    Password(),
    CharacterLine(),
    Menu(),
    Date(),
    Time(),
    Continue(),
    Other(u8),
}

impl From<rusmpp_types::ItsReplyType> for ItsReplyType {
    fn from(value: rusmpp_types::ItsReplyType) -> Self {
        match value {
            rusmpp_types::ItsReplyType::Digit => ItsReplyType::Digit(),
            rusmpp_types::ItsReplyType::Number => ItsReplyType::Number(),
            rusmpp_types::ItsReplyType::TelephoneNo => ItsReplyType::TelephoneNo(),
            rusmpp_types::ItsReplyType::Password => ItsReplyType::Password(),
            rusmpp_types::ItsReplyType::CharacterLine => ItsReplyType::CharacterLine(),
            rusmpp_types::ItsReplyType::Menu => ItsReplyType::Menu(),
            rusmpp_types::ItsReplyType::Date => ItsReplyType::Date(),
            rusmpp_types::ItsReplyType::Time => ItsReplyType::Time(),
            rusmpp_types::ItsReplyType::Continue => ItsReplyType::Continue(),
            rusmpp_types::ItsReplyType::Other(inner) => ItsReplyType::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl ItsReplyType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct ItsSessionInfo {
    pub session_number: u8,
    pub sequence_number: u8,
}

impl From<rusmpp_types::ItsSessionInfo> for ItsSessionInfo {
    fn from(value: rusmpp_types::ItsSessionInfo) -> Self {
        let value = value.into_parts();
        Self {
            session_number: value.session_number.into(),
            sequence_number: value.sequence_number.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl ItsSessionInfo {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum LanguageIndicator {
    Unspecified(),
    English(),
    French(),
    Spanish(),
    German(),
    Portuguese(),
    Other(u8),
}

impl From<rusmpp_types::LanguageIndicator> for LanguageIndicator {
    fn from(value: rusmpp_types::LanguageIndicator) -> Self {
        match value {
            rusmpp_types::LanguageIndicator::Unspecified => LanguageIndicator::Unspecified(),
            rusmpp_types::LanguageIndicator::English => LanguageIndicator::English(),
            rusmpp_types::LanguageIndicator::French => LanguageIndicator::French(),
            rusmpp_types::LanguageIndicator::Spanish => LanguageIndicator::Spanish(),
            rusmpp_types::LanguageIndicator::German => LanguageIndicator::German(),
            rusmpp_types::LanguageIndicator::Portuguese => LanguageIndicator::Portuguese(),
            rusmpp_types::LanguageIndicator::Other(inner) => LanguageIndicator::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl LanguageIndicator {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct MessagePayload {
    pub value: Vec<u8>,
}

impl From<rusmpp_types::MessagePayload> for MessagePayload {
    fn from(value: rusmpp_types::MessagePayload) -> Self {
        let value = value.into_parts();
        Self {
            value: value.value.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MessagePayload {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MessageState {
    Scheduled(),
    Enroute(),
    Delivered(),
    Expired(),
    Deleted(),
    Undeliverable(),
    Accepted(),
    Unknown(),
    Rejected(),
    Skipped(),
    Other(u8),
}

impl From<rusmpp_types::MessageState> for MessageState {
    fn from(value: rusmpp_types::MessageState) -> Self {
        match value {
            rusmpp_types::MessageState::Scheduled => MessageState::Scheduled(),
            rusmpp_types::MessageState::Enroute => MessageState::Enroute(),
            rusmpp_types::MessageState::Delivered => MessageState::Delivered(),
            rusmpp_types::MessageState::Expired => MessageState::Expired(),
            rusmpp_types::MessageState::Deleted => MessageState::Deleted(),
            rusmpp_types::MessageState::Undeliverable => MessageState::Undeliverable(),
            rusmpp_types::MessageState::Accepted => MessageState::Accepted(),
            rusmpp_types::MessageState::Unknown => MessageState::Unknown(),
            rusmpp_types::MessageState::Rejected => MessageState::Rejected(),
            rusmpp_types::MessageState::Skipped => MessageState::Skipped(),
            rusmpp_types::MessageState::Other(inner) => MessageState::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MessageState {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MoreMessagesToSend {
    NoMoreMessagesToFollow(),
    MoreMessagesToFollow(),
    Other(u8),
}

impl From<rusmpp_types::MoreMessagesToSend> for MoreMessagesToSend {
    fn from(value: rusmpp_types::MoreMessagesToSend) -> Self {
        match value {
            rusmpp_types::MoreMessagesToSend::NoMoreMessagesToFollow => {
                MoreMessagesToSend::NoMoreMessagesToFollow()
            }
            rusmpp_types::MoreMessagesToSend::MoreMessagesToFollow => {
                MoreMessagesToSend::MoreMessagesToFollow()
            }
            rusmpp_types::MoreMessagesToSend::Other(inner) => {
                MoreMessagesToSend::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MoreMessagesToSend {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MsAvailabilityStatus {
    Available(),
    Denied(),
    Unavailable(),
    Other(u8),
}

impl From<rusmpp_types::MsAvailabilityStatus> for MsAvailabilityStatus {
    fn from(value: rusmpp_types::MsAvailabilityStatus) -> Self {
        match value {
            rusmpp_types::MsAvailabilityStatus::Available => MsAvailabilityStatus::Available(),
            rusmpp_types::MsAvailabilityStatus::Denied => MsAvailabilityStatus::Denied(),
            rusmpp_types::MsAvailabilityStatus::Unavailable => MsAvailabilityStatus::Unavailable(),
            rusmpp_types::MsAvailabilityStatus::Other(inner) => {
                MsAvailabilityStatus::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MsAvailabilityStatus {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Indicator {
    Inactive(),
    Active(),
    Other(u8),
}

impl From<rusmpp_types::Indicator> for Indicator {
    fn from(value: rusmpp_types::Indicator) -> Self {
        match value {
            rusmpp_types::Indicator::Inactive => Indicator::Inactive(),
            rusmpp_types::Indicator::Active => Indicator::Active(),
            rusmpp_types::Indicator::Other(inner) => Indicator::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Indicator {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum TypeOfMessage {
    VoicemailMessageWaiting(),
    FaxMessageWaiting(),
    ElectronicMailMessageWaiting(),
    OtherMessageWaiting(),
    Other(u8),
}

impl From<rusmpp_types::TypeOfMessage> for TypeOfMessage {
    fn from(value: rusmpp_types::TypeOfMessage) -> Self {
        match value {
            rusmpp_types::TypeOfMessage::VoicemailMessageWaiting => {
                TypeOfMessage::VoicemailMessageWaiting()
            }
            rusmpp_types::TypeOfMessage::FaxMessageWaiting => TypeOfMessage::FaxMessageWaiting(),
            rusmpp_types::TypeOfMessage::ElectronicMailMessageWaiting => {
                TypeOfMessage::ElectronicMailMessageWaiting()
            }
            rusmpp_types::TypeOfMessage::OtherMessageWaiting => {
                TypeOfMessage::OtherMessageWaiting()
            }
            rusmpp_types::TypeOfMessage::Other(inner) => TypeOfMessage::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl TypeOfMessage {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct MsMsgWaitFacilities {
    pub indicator: Indicator,
    pub type_of_message: TypeOfMessage,
}

impl From<rusmpp_types::MsMsgWaitFacilities> for MsMsgWaitFacilities {
    fn from(value: rusmpp_types::MsMsgWaitFacilities) -> Self {
        let value = value.into_parts();
        Self {
            indicator: value.indicator.into(),
            type_of_message: value.type_of_message.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MsMsgWaitFacilities {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MsValidityBehavior {
    StoreIndefinitely(),
    PowerDown(),
    ValidUntilRegistrationAreaChanges(),
    DisplayOnly(),
    RelativeTimePeriod(),
    Other(u8),
}

impl From<rusmpp_types::MsValidityBehavior> for MsValidityBehavior {
    fn from(value: rusmpp_types::MsValidityBehavior) -> Self {
        match value {
            rusmpp_types::MsValidityBehavior::StoreIndefinitely => {
                MsValidityBehavior::StoreIndefinitely()
            }
            rusmpp_types::MsValidityBehavior::PowerDown => MsValidityBehavior::PowerDown(),
            rusmpp_types::MsValidityBehavior::ValidUntilRegistrationAreaChanges => {
                MsValidityBehavior::ValidUntilRegistrationAreaChanges()
            }
            rusmpp_types::MsValidityBehavior::DisplayOnly => MsValidityBehavior::DisplayOnly(),
            rusmpp_types::MsValidityBehavior::RelativeTimePeriod => {
                MsValidityBehavior::RelativeTimePeriod()
            }
            rusmpp_types::MsValidityBehavior::Other(inner) => {
                MsValidityBehavior::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MsValidityBehavior {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum UnitsOfTime {
    Seconds(),
    Minutes(),
    Hours(),
    Days(),
    Weeks(),
    Months(),
    Years(),
    Other(u8),
}

impl From<rusmpp_types::UnitsOfTime> for UnitsOfTime {
    fn from(value: rusmpp_types::UnitsOfTime) -> Self {
        match value {
            rusmpp_types::UnitsOfTime::Seconds => UnitsOfTime::Seconds(),
            rusmpp_types::UnitsOfTime::Minutes => UnitsOfTime::Minutes(),
            rusmpp_types::UnitsOfTime::Hours => UnitsOfTime::Hours(),
            rusmpp_types::UnitsOfTime::Days => UnitsOfTime::Days(),
            rusmpp_types::UnitsOfTime::Weeks => UnitsOfTime::Weeks(),
            rusmpp_types::UnitsOfTime::Months => UnitsOfTime::Months(),
            rusmpp_types::UnitsOfTime::Years => UnitsOfTime::Years(),
            rusmpp_types::UnitsOfTime::Other(inner) => UnitsOfTime::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl UnitsOfTime {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct MsValidityInformation {
    pub units_of_time: UnitsOfTime,
    pub number_of_time_units: u16,
}

impl From<rusmpp_types::MsValidityInformation> for MsValidityInformation {
    fn from(value: rusmpp_types::MsValidityInformation) -> Self {
        let value = value.into_parts();
        Self {
            units_of_time: value.units_of_time.into(),
            number_of_time_units: value.number_of_time_units.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MsValidityInformation {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct MsValidity {
    pub validity_behavior: MsValidityBehavior,
    pub validity_information: Option<MsValidityInformation>,
}

impl From<rusmpp_types::MsValidity> for MsValidity {
    fn from(value: rusmpp_types::MsValidity) -> Self {
        let value = value.into_parts();
        Self {
            validity_behavior: value.validity_behavior.into(),
            validity_information: value.validity_information.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MsValidity {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum ErrorCodeNetworkType {
    Ansi136AccessDeniedReason(),
    Is95AccessDeniedReason(),
    Gsm(),
    Ansi136CauseCode(),
    Is95CauseCode(),
    Ansi41Error(),
    SmppError(),
    MessageCenterSpecific(),
    Other(u8),
}

impl From<rusmpp_types::ErrorCodeNetworkType> for ErrorCodeNetworkType {
    fn from(value: rusmpp_types::ErrorCodeNetworkType) -> Self {
        match value {
            rusmpp_types::ErrorCodeNetworkType::Ansi136AccessDeniedReason => {
                ErrorCodeNetworkType::Ansi136AccessDeniedReason()
            }
            rusmpp_types::ErrorCodeNetworkType::Is95AccessDeniedReason => {
                ErrorCodeNetworkType::Is95AccessDeniedReason()
            }
            rusmpp_types::ErrorCodeNetworkType::Gsm => ErrorCodeNetworkType::Gsm(),
            rusmpp_types::ErrorCodeNetworkType::Ansi136CauseCode => {
                ErrorCodeNetworkType::Ansi136CauseCode()
            }
            rusmpp_types::ErrorCodeNetworkType::Is95CauseCode => {
                ErrorCodeNetworkType::Is95CauseCode()
            }
            rusmpp_types::ErrorCodeNetworkType::Ansi41Error => ErrorCodeNetworkType::Ansi41Error(),
            rusmpp_types::ErrorCodeNetworkType::SmppError => ErrorCodeNetworkType::SmppError(),
            rusmpp_types::ErrorCodeNetworkType::MessageCenterSpecific => {
                ErrorCodeNetworkType::MessageCenterSpecific()
            }
            rusmpp_types::ErrorCodeNetworkType::Other(inner) => {
                ErrorCodeNetworkType::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl ErrorCodeNetworkType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct NetworkErrorCode {
    pub network_type: ErrorCodeNetworkType,
    pub error_code: u16,
}

impl From<rusmpp_types::NetworkErrorCode> for NetworkErrorCode {
    fn from(value: rusmpp_types::NetworkErrorCode) -> Self {
        let value = value.into_parts();
        Self {
            network_type: value.network_type.into(),
            error_code: value.error_code.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl NetworkErrorCode {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum NetworkType {
    Unknown(),
    Gsm(),
    Ansi136(),
    Is95(),
    Pdc(),
    Phs(),
    IDen(),
    Amps(),
    PagingNetwork(),
    Other(u8),
}

impl From<rusmpp_types::NetworkType> for NetworkType {
    fn from(value: rusmpp_types::NetworkType) -> Self {
        match value {
            rusmpp_types::NetworkType::Unknown => NetworkType::Unknown(),
            rusmpp_types::NetworkType::Gsm => NetworkType::Gsm(),
            rusmpp_types::NetworkType::Ansi136 => NetworkType::Ansi136(),
            rusmpp_types::NetworkType::Is95 => NetworkType::Is95(),
            rusmpp_types::NetworkType::Pdc => NetworkType::Pdc(),
            rusmpp_types::NetworkType::Phs => NetworkType::Phs(),
            rusmpp_types::NetworkType::IDen => NetworkType::IDen(),
            rusmpp_types::NetworkType::Amps => NetworkType::Amps(),
            rusmpp_types::NetworkType::PagingNetwork => NetworkType::PagingNetwork(),
            rusmpp_types::NetworkType::Other(inner) => NetworkType::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl NetworkType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum NumberOfMessages {
    Allowed(u8),
    Other(u8),
}

impl From<rusmpp_types::NumberOfMessages> for NumberOfMessages {
    fn from(value: rusmpp_types::NumberOfMessages) -> Self {
        match value {
            rusmpp_types::NumberOfMessages::Allowed(inner) => {
                NumberOfMessages::Allowed(inner.into())
            }
            rusmpp_types::NumberOfMessages::Other(inner) => NumberOfMessages::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl NumberOfMessages {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum PayloadType {
    Default(),
    WcmpMessage(),
    Other(u8),
}

impl From<rusmpp_types::PayloadType> for PayloadType {
    fn from(value: rusmpp_types::PayloadType) -> Self {
        match value {
            rusmpp_types::PayloadType::Default => PayloadType::Default(),
            rusmpp_types::PayloadType::WcmpMessage => PayloadType::WcmpMessage(),
            rusmpp_types::PayloadType::Other(inner) => PayloadType::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl PayloadType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum PrivacyIndicator {
    NotRestricted(),
    Restricted(),
    Confidential(),
    Secret(),
    Other(u8),
}

impl From<rusmpp_types::PrivacyIndicator> for PrivacyIndicator {
    fn from(value: rusmpp_types::PrivacyIndicator) -> Self {
        match value {
            rusmpp_types::PrivacyIndicator::NotRestricted => PrivacyIndicator::NotRestricted(),
            rusmpp_types::PrivacyIndicator::Restricted => PrivacyIndicator::Restricted(),
            rusmpp_types::PrivacyIndicator::Confidential => PrivacyIndicator::Confidential(),
            rusmpp_types::PrivacyIndicator::Secret => PrivacyIndicator::Secret(),
            rusmpp_types::PrivacyIndicator::Other(inner) => PrivacyIndicator::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl PrivacyIndicator {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum SetDpf {
    NotRequested(),
    Requested(),
    Other(u8),
}

impl From<rusmpp_types::SetDpf> for SetDpf {
    fn from(value: rusmpp_types::SetDpf) -> Self {
        match value {
            rusmpp_types::SetDpf::NotRequested => SetDpf::NotRequested(),
            rusmpp_types::SetDpf::Requested => SetDpf::Requested(),
            rusmpp_types::SetDpf::Other(inner) => SetDpf::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SetDpf {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum SubaddressTag {
    NsapEven(),
    NsapOdd(),
    UserSpecified(),
    Other(u8),
}

impl From<rusmpp_types::SubaddressTag> for SubaddressTag {
    fn from(value: rusmpp_types::SubaddressTag) -> Self {
        match value {
            rusmpp_types::SubaddressTag::NsapEven => SubaddressTag::NsapEven(),
            rusmpp_types::SubaddressTag::NsapOdd => SubaddressTag::NsapOdd(),
            rusmpp_types::SubaddressTag::UserSpecified => SubaddressTag::UserSpecified(),
            rusmpp_types::SubaddressTag::Other(inner) => SubaddressTag::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SubaddressTag {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Subaddress {
    pub tag: SubaddressTag,
    pub addr: Vec<u8>,
}

impl From<rusmpp_types::Subaddress> for Subaddress {
    fn from(value: rusmpp_types::Subaddress) -> Self {
        let value = value.into_parts();
        Self {
            tag: value.tag.into(),
            addr: value.addr.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Subaddress {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum TlvTag {
    DestAddrSubunit(),
    DestNetworkType(),
    DestBearerType(),
    DestTelematicsId(),
    SourceAddrSubunit(),
    SourceNetworkType(),
    SourceBearerType(),
    SourceTelematicsId(),
    QosTimeToLive(),
    PayloadType(),
    AdditionalStatusInfoText(),
    ReceiptedMessageId(),
    MsMsgWaitFacilities(),
    PrivacyIndicator(),
    SourceSubaddress(),
    DestSubaddress(),
    UserMessageReference(),
    UserResponseCode(),
    SourcePort(),
    DestPort(),
    SarMsgRefNum(),
    LanguageIndicator(),
    SarTotalSegments(),
    SarSegmentSeqnum(),
    ScInterfaceVersion(),
    CallbackNumPresInd(),
    CallbackNumAtag(),
    NumberOfMessages(),
    CallbackNum(),
    DpfResult(),
    SetDpf(),
    MsAvailabilityStatus(),
    NetworkErrorCode(),
    MessagePayload(),
    DeliveryFailureReason(),
    MoreMessagesToSend(),
    MessageState(),
    CongestionState(),
    UssdServiceOp(),
    BroadcastChannelIndicator(),
    BroadcastContentType(),
    BroadcastContentTypeInfo(),
    BroadcastMessageClass(),
    BroadcastRepNum(),
    BroadcastFrequencyInterval(),
    BroadcastAreaIdentifier(),
    BroadcastErrorStatus(),
    BroadcastAreaSuccess(),
    BroadcastEndTime(),
    BroadcastServiceGroup(),
    BillingIdentification(),
    SourceNetworkId(),
    DestNetworkId(),
    SourceNodeId(),
    DestNodeId(),
    DestAddrNpResolution(),
    DestAddrNpInformation(),
    DestAddrNpCountry(),
    DisplayTime(),
    SmsSignal(),
    MsValidity(),
    AlertOnMessageDelivery(),
    ItsReplyType(),
    ItsSessionInfo(),
    Other(u16),
}

impl From<rusmpp_types::TlvTag> for TlvTag {
    fn from(value: rusmpp_types::TlvTag) -> Self {
        match value {
            rusmpp_types::TlvTag::DestAddrSubunit => TlvTag::DestAddrSubunit(),
            rusmpp_types::TlvTag::DestNetworkType => TlvTag::DestNetworkType(),
            rusmpp_types::TlvTag::DestBearerType => TlvTag::DestBearerType(),
            rusmpp_types::TlvTag::DestTelematicsId => TlvTag::DestTelematicsId(),
            rusmpp_types::TlvTag::SourceAddrSubunit => TlvTag::SourceAddrSubunit(),
            rusmpp_types::TlvTag::SourceNetworkType => TlvTag::SourceNetworkType(),
            rusmpp_types::TlvTag::SourceBearerType => TlvTag::SourceBearerType(),
            rusmpp_types::TlvTag::SourceTelematicsId => TlvTag::SourceTelematicsId(),
            rusmpp_types::TlvTag::QosTimeToLive => TlvTag::QosTimeToLive(),
            rusmpp_types::TlvTag::PayloadType => TlvTag::PayloadType(),
            rusmpp_types::TlvTag::AdditionalStatusInfoText => TlvTag::AdditionalStatusInfoText(),
            rusmpp_types::TlvTag::ReceiptedMessageId => TlvTag::ReceiptedMessageId(),
            rusmpp_types::TlvTag::MsMsgWaitFacilities => TlvTag::MsMsgWaitFacilities(),
            rusmpp_types::TlvTag::PrivacyIndicator => TlvTag::PrivacyIndicator(),
            rusmpp_types::TlvTag::SourceSubaddress => TlvTag::SourceSubaddress(),
            rusmpp_types::TlvTag::DestSubaddress => TlvTag::DestSubaddress(),
            rusmpp_types::TlvTag::UserMessageReference => TlvTag::UserMessageReference(),
            rusmpp_types::TlvTag::UserResponseCode => TlvTag::UserResponseCode(),
            rusmpp_types::TlvTag::SourcePort => TlvTag::SourcePort(),
            rusmpp_types::TlvTag::DestPort => TlvTag::DestPort(),
            rusmpp_types::TlvTag::SarMsgRefNum => TlvTag::SarMsgRefNum(),
            rusmpp_types::TlvTag::LanguageIndicator => TlvTag::LanguageIndicator(),
            rusmpp_types::TlvTag::SarTotalSegments => TlvTag::SarTotalSegments(),
            rusmpp_types::TlvTag::SarSegmentSeqnum => TlvTag::SarSegmentSeqnum(),
            rusmpp_types::TlvTag::ScInterfaceVersion => TlvTag::ScInterfaceVersion(),
            rusmpp_types::TlvTag::CallbackNumPresInd => TlvTag::CallbackNumPresInd(),
            rusmpp_types::TlvTag::CallbackNumAtag => TlvTag::CallbackNumAtag(),
            rusmpp_types::TlvTag::NumberOfMessages => TlvTag::NumberOfMessages(),
            rusmpp_types::TlvTag::CallbackNum => TlvTag::CallbackNum(),
            rusmpp_types::TlvTag::DpfResult => TlvTag::DpfResult(),
            rusmpp_types::TlvTag::SetDpf => TlvTag::SetDpf(),
            rusmpp_types::TlvTag::MsAvailabilityStatus => TlvTag::MsAvailabilityStatus(),
            rusmpp_types::TlvTag::NetworkErrorCode => TlvTag::NetworkErrorCode(),
            rusmpp_types::TlvTag::MessagePayload => TlvTag::MessagePayload(),
            rusmpp_types::TlvTag::DeliveryFailureReason => TlvTag::DeliveryFailureReason(),
            rusmpp_types::TlvTag::MoreMessagesToSend => TlvTag::MoreMessagesToSend(),
            rusmpp_types::TlvTag::MessageState => TlvTag::MessageState(),
            rusmpp_types::TlvTag::CongestionState => TlvTag::CongestionState(),
            rusmpp_types::TlvTag::UssdServiceOp => TlvTag::UssdServiceOp(),
            rusmpp_types::TlvTag::BroadcastChannelIndicator => TlvTag::BroadcastChannelIndicator(),
            rusmpp_types::TlvTag::BroadcastContentType => TlvTag::BroadcastContentType(),
            rusmpp_types::TlvTag::BroadcastContentTypeInfo => TlvTag::BroadcastContentTypeInfo(),
            rusmpp_types::TlvTag::BroadcastMessageClass => TlvTag::BroadcastMessageClass(),
            rusmpp_types::TlvTag::BroadcastRepNum => TlvTag::BroadcastRepNum(),
            rusmpp_types::TlvTag::BroadcastFrequencyInterval => {
                TlvTag::BroadcastFrequencyInterval()
            }
            rusmpp_types::TlvTag::BroadcastAreaIdentifier => TlvTag::BroadcastAreaIdentifier(),
            rusmpp_types::TlvTag::BroadcastErrorStatus => TlvTag::BroadcastErrorStatus(),
            rusmpp_types::TlvTag::BroadcastAreaSuccess => TlvTag::BroadcastAreaSuccess(),
            rusmpp_types::TlvTag::BroadcastEndTime => TlvTag::BroadcastEndTime(),
            rusmpp_types::TlvTag::BroadcastServiceGroup => TlvTag::BroadcastServiceGroup(),
            rusmpp_types::TlvTag::BillingIdentification => TlvTag::BillingIdentification(),
            rusmpp_types::TlvTag::SourceNetworkId => TlvTag::SourceNetworkId(),
            rusmpp_types::TlvTag::DestNetworkId => TlvTag::DestNetworkId(),
            rusmpp_types::TlvTag::SourceNodeId => TlvTag::SourceNodeId(),
            rusmpp_types::TlvTag::DestNodeId => TlvTag::DestNodeId(),
            rusmpp_types::TlvTag::DestAddrNpResolution => TlvTag::DestAddrNpResolution(),
            rusmpp_types::TlvTag::DestAddrNpInformation => TlvTag::DestAddrNpInformation(),
            rusmpp_types::TlvTag::DestAddrNpCountry => TlvTag::DestAddrNpCountry(),
            rusmpp_types::TlvTag::DisplayTime => TlvTag::DisplayTime(),
            rusmpp_types::TlvTag::SmsSignal => TlvTag::SmsSignal(),
            rusmpp_types::TlvTag::MsValidity => TlvTag::MsValidity(),
            rusmpp_types::TlvTag::AlertOnMessageDelivery => TlvTag::AlertOnMessageDelivery(),
            rusmpp_types::TlvTag::ItsReplyType => TlvTag::ItsReplyType(),
            rusmpp_types::TlvTag::ItsSessionInfo => TlvTag::ItsSessionInfo(),
            rusmpp_types::TlvTag::Other(inner) => TlvTag::Other(inner.into()),
            _ => panic!("Unexpected variant in Rusmpp type TlvTag"),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl TlvTag {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct UserMessageReference {
    pub value: u16,
}

impl From<rusmpp_types::UserMessageReference> for UserMessageReference {
    fn from(value: rusmpp_types::UserMessageReference) -> Self {
        let value = value.into_parts();
        Self {
            value: value.value.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl UserMessageReference {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum UssdServiceOp {
    PssdIndication(),
    PssrIndication(),
    UssrRequest(),
    UssnRequest(),
    PssdResponse(),
    PssrResponse(),
    UssrConfirm(),
    UssnConfirm(),
    Other(u8),
}

impl From<rusmpp_types::UssdServiceOp> for UssdServiceOp {
    fn from(value: rusmpp_types::UssdServiceOp) -> Self {
        match value {
            rusmpp_types::UssdServiceOp::PssdIndication => UssdServiceOp::PssdIndication(),
            rusmpp_types::UssdServiceOp::PssrIndication => UssdServiceOp::PssrIndication(),
            rusmpp_types::UssdServiceOp::UssrRequest => UssdServiceOp::UssrRequest(),
            rusmpp_types::UssdServiceOp::UssnRequest => UssdServiceOp::UssnRequest(),
            rusmpp_types::UssdServiceOp::PssdResponse => UssdServiceOp::PssdResponse(),
            rusmpp_types::UssdServiceOp::PssrResponse => UssdServiceOp::PssrResponse(),
            rusmpp_types::UssdServiceOp::UssrConfirm => UssdServiceOp::UssrConfirm(),
            rusmpp_types::UssdServiceOp::UssnConfirm => UssdServiceOp::UssnConfirm(),
            rusmpp_types::UssdServiceOp::Other(inner) => UssdServiceOp::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl UssdServiceOp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum TlvValue {
    AdditionalStatusInfoText(Vec<u8>),
    AlertOnMessageDelivery(AlertOnMessageDelivery),
    BillingIdentification(Vec<u8>),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastContentTypeInfo(Vec<u8>),
    BroadcastChannelIndicator(BroadcastChannelIndicator),
    BroadcastContentType(BroadcastContentType),
    BroadcastEndTime(Vec<u8>),
    BroadcastErrorStatus(CommandStatus),
    BroadcastFrequencyInterval(BroadcastFrequencyInterval),
    BroadcastMessageClass(BroadcastMessageClass),
    BroadcastRepNum(BroadcastRepNum),
    BroadcastServiceGroup(Vec<u8>),
    CallbackNum(Vec<u8>),
    CallbackNumAtag(Vec<u8>),
    CallbackNumPresInd(CallbackNumPresInd),
    CongestionState(CongestionState),
    DeliveryFailureReason(DeliveryFailureReason),
    DestAddrNpCountry(Vec<u8>),
    DestAddrNpInformation(Vec<u8>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestBearerType(BearerType),
    DestNetworkId(Vec<u8>),
    DestNetworkType(NetworkType),
    DestNodeId(Vec<u8>),
    DestSubaddress(Subaddress),
    DestTelematicsId(u16),
    DestPort(u16),
    DisplayTime(DisplayTime),
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload),
    MessageState(MessageState),
    MoreMessagesToSend(MoreMessagesToSend),
    MsAvailabilityStatus(MsAvailabilityStatus),
    MsMsgWaitFacilities(MsMsgWaitFacilities),
    MsValidity(MsValidity),
    NetworkErrorCode(NetworkErrorCode),
    NumberOfMessages(NumberOfMessages),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    QosTimeToLive(u32),
    ReceiptedMessageId(Vec<u8>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    ScInterfaceVersion(InterfaceVersion),
    SetDpf(SetDpf),
    SmsSignal(u16),
    SourceAddrSubunit(AddrSubunit),
    SourceBearerType(BearerType),
    SourceNetworkId(Vec<u8>),
    SourceNetworkType(NetworkType),
    SourceNodeId(Vec<u8>),
    SourcePort(u16),
    SourceSubaddress(Subaddress),
    SourceTelematicsId(u16),
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
    Other { tag: TlvTag, value: Vec<u8> },
}

impl From<rusmpp_types::TlvValue> for TlvValue {
    fn from(value: rusmpp_types::TlvValue) -> Self {
        match value {
            rusmpp_types::TlvValue::AdditionalStatusInfoText(inner) => {
                TlvValue::AdditionalStatusInfoText(inner.into())
            }
            rusmpp_types::TlvValue::AlertOnMessageDelivery(inner) => {
                TlvValue::AlertOnMessageDelivery(inner.into())
            }
            rusmpp_types::TlvValue::BillingIdentification(inner) => {
                TlvValue::BillingIdentification(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastAreaIdentifier(inner) => {
                TlvValue::BroadcastAreaIdentifier(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastAreaSuccess(inner) => {
                TlvValue::BroadcastAreaSuccess(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastContentTypeInfo(inner) => {
                TlvValue::BroadcastContentTypeInfo(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastChannelIndicator(inner) => {
                TlvValue::BroadcastChannelIndicator(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastContentType(inner) => {
                TlvValue::BroadcastContentType(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastEndTime(inner) => {
                TlvValue::BroadcastEndTime(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastErrorStatus(inner) => {
                TlvValue::BroadcastErrorStatus(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastFrequencyInterval(inner) => {
                TlvValue::BroadcastFrequencyInterval(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastMessageClass(inner) => {
                TlvValue::BroadcastMessageClass(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastRepNum(inner) => {
                TlvValue::BroadcastRepNum(inner.into())
            }
            rusmpp_types::TlvValue::BroadcastServiceGroup(inner) => {
                TlvValue::BroadcastServiceGroup(inner.into())
            }
            rusmpp_types::TlvValue::CallbackNum(inner) => TlvValue::CallbackNum(inner.into()),
            rusmpp_types::TlvValue::CallbackNumAtag(inner) => {
                TlvValue::CallbackNumAtag(inner.into())
            }
            rusmpp_types::TlvValue::CallbackNumPresInd(inner) => {
                TlvValue::CallbackNumPresInd(inner.into())
            }
            rusmpp_types::TlvValue::CongestionState(inner) => {
                TlvValue::CongestionState(inner.into())
            }
            rusmpp_types::TlvValue::DeliveryFailureReason(inner) => {
                TlvValue::DeliveryFailureReason(inner.into())
            }
            rusmpp_types::TlvValue::DestAddrNpCountry(inner) => {
                TlvValue::DestAddrNpCountry(inner.into())
            }
            rusmpp_types::TlvValue::DestAddrNpInformation(inner) => {
                TlvValue::DestAddrNpInformation(inner.into())
            }
            rusmpp_types::TlvValue::DestAddrNpResolution(inner) => {
                TlvValue::DestAddrNpResolution(inner.into())
            }
            rusmpp_types::TlvValue::DestAddrSubunit(inner) => {
                TlvValue::DestAddrSubunit(inner.into())
            }
            rusmpp_types::TlvValue::DestBearerType(inner) => TlvValue::DestBearerType(inner.into()),
            rusmpp_types::TlvValue::DestNetworkId(inner) => TlvValue::DestNetworkId(inner.into()),
            rusmpp_types::TlvValue::DestNetworkType(inner) => {
                TlvValue::DestNetworkType(inner.into())
            }
            rusmpp_types::TlvValue::DestNodeId(inner) => TlvValue::DestNodeId(inner.into()),
            rusmpp_types::TlvValue::DestSubaddress(inner) => TlvValue::DestSubaddress(inner.into()),
            rusmpp_types::TlvValue::DestTelematicsId(inner) => {
                TlvValue::DestTelematicsId(inner.into())
            }
            rusmpp_types::TlvValue::DestPort(inner) => TlvValue::DestPort(inner.into()),
            rusmpp_types::TlvValue::DisplayTime(inner) => TlvValue::DisplayTime(inner.into()),
            rusmpp_types::TlvValue::DpfResult(inner) => TlvValue::DpfResult(inner.into()),
            rusmpp_types::TlvValue::ItsReplyType(inner) => TlvValue::ItsReplyType(inner.into()),
            rusmpp_types::TlvValue::ItsSessionInfo(inner) => TlvValue::ItsSessionInfo(inner.into()),
            rusmpp_types::TlvValue::LanguageIndicator(inner) => {
                TlvValue::LanguageIndicator(inner.into())
            }
            rusmpp_types::TlvValue::MessagePayload(inner) => TlvValue::MessagePayload(inner.into()),
            rusmpp_types::TlvValue::MessageState(inner) => TlvValue::MessageState(inner.into()),
            rusmpp_types::TlvValue::MoreMessagesToSend(inner) => {
                TlvValue::MoreMessagesToSend(inner.into())
            }
            rusmpp_types::TlvValue::MsAvailabilityStatus(inner) => {
                TlvValue::MsAvailabilityStatus(inner.into())
            }
            rusmpp_types::TlvValue::MsMsgWaitFacilities(inner) => {
                TlvValue::MsMsgWaitFacilities(inner.into())
            }
            rusmpp_types::TlvValue::MsValidity(inner) => TlvValue::MsValidity(inner.into()),
            rusmpp_types::TlvValue::NetworkErrorCode(inner) => {
                TlvValue::NetworkErrorCode(inner.into())
            }
            rusmpp_types::TlvValue::NumberOfMessages(inner) => {
                TlvValue::NumberOfMessages(inner.into())
            }
            rusmpp_types::TlvValue::PayloadType(inner) => TlvValue::PayloadType(inner.into()),
            rusmpp_types::TlvValue::PrivacyIndicator(inner) => {
                TlvValue::PrivacyIndicator(inner.into())
            }
            rusmpp_types::TlvValue::QosTimeToLive(inner) => TlvValue::QosTimeToLive(inner.into()),
            rusmpp_types::TlvValue::ReceiptedMessageId(inner) => {
                TlvValue::ReceiptedMessageId(inner.into())
            }
            rusmpp_types::TlvValue::SarMsgRefNum(inner) => TlvValue::SarMsgRefNum(inner.into()),
            rusmpp_types::TlvValue::SarSegmentSeqnum(inner) => {
                TlvValue::SarSegmentSeqnum(inner.into())
            }
            rusmpp_types::TlvValue::SarTotalSegments(inner) => {
                TlvValue::SarTotalSegments(inner.into())
            }
            rusmpp_types::TlvValue::ScInterfaceVersion(inner) => {
                TlvValue::ScInterfaceVersion(inner.into())
            }
            rusmpp_types::TlvValue::SetDpf(inner) => TlvValue::SetDpf(inner.into()),
            rusmpp_types::TlvValue::SmsSignal(inner) => TlvValue::SmsSignal(inner.into()),
            rusmpp_types::TlvValue::SourceAddrSubunit(inner) => {
                TlvValue::SourceAddrSubunit(inner.into())
            }
            rusmpp_types::TlvValue::SourceBearerType(inner) => {
                TlvValue::SourceBearerType(inner.into())
            }
            rusmpp_types::TlvValue::SourceNetworkId(inner) => {
                TlvValue::SourceNetworkId(inner.into())
            }
            rusmpp_types::TlvValue::SourceNetworkType(inner) => {
                TlvValue::SourceNetworkType(inner.into())
            }
            rusmpp_types::TlvValue::SourceNodeId(inner) => TlvValue::SourceNodeId(inner.into()),
            rusmpp_types::TlvValue::SourcePort(inner) => TlvValue::SourcePort(inner.into()),
            rusmpp_types::TlvValue::SourceSubaddress(inner) => {
                TlvValue::SourceSubaddress(inner.into())
            }
            rusmpp_types::TlvValue::SourceTelematicsId(inner) => {
                TlvValue::SourceTelematicsId(inner.into())
            }
            rusmpp_types::TlvValue::UserMessageReference(inner) => {
                TlvValue::UserMessageReference(inner.into())
            }
            rusmpp_types::TlvValue::UserResponseCode(inner) => {
                TlvValue::UserResponseCode(inner.into())
            }
            rusmpp_types::TlvValue::UssdServiceOp(inner) => TlvValue::UssdServiceOp(inner.into()),
            rusmpp_types::TlvValue::Other { tag, value } => TlvValue::Other {
                tag: tag.into(),
                value: value.into(),
            },
            _ => panic!("Unexpected variant in Rusmpp type TlvValue"),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl TlvValue {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Npi {
    Unknown(),
    Isdn(),
    Data(),
    Telex(),
    LandMobile(),
    National(),
    Private(),
    Ermes(),
    Internet(),
    WapClientId(),
    Other(u8),
}

impl From<rusmpp_types::Npi> for Npi {
    fn from(value: rusmpp_types::Npi) -> Self {
        match value {
            rusmpp_types::Npi::Unknown => Npi::Unknown(),
            rusmpp_types::Npi::Isdn => Npi::Isdn(),
            rusmpp_types::Npi::Data => Npi::Data(),
            rusmpp_types::Npi::Telex => Npi::Telex(),
            rusmpp_types::Npi::LandMobile => Npi::LandMobile(),
            rusmpp_types::Npi::National => Npi::National(),
            rusmpp_types::Npi::Private => Npi::Private(),
            rusmpp_types::Npi::Ermes => Npi::Ermes(),
            rusmpp_types::Npi::Internet => Npi::Internet(),
            rusmpp_types::Npi::WapClientId => Npi::WapClientId(),
            rusmpp_types::Npi::Other(inner) => Npi::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Npi {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Tlv {
    pub tag: TlvTag,
    pub value_length: u16,
    pub value: Option<TlvValue>,
}

impl From<rusmpp_types::Tlv> for Tlv {
    fn from(value: rusmpp_types::Tlv) -> Self {
        let value = value.into_parts();
        Self {
            tag: value.tag.into(),
            value_length: value.value_length.into(),
            value: value.value.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Tlv {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Ton {
    Unknown(),
    International(),
    National(),
    NetworkSpecific(),
    SubscriberNumber(),
    Alphanumeric(),
    Abbreviated(),
    Other(u8),
}

impl From<rusmpp_types::Ton> for Ton {
    fn from(value: rusmpp_types::Ton) -> Self {
        match value {
            rusmpp_types::Ton::Unknown => Ton::Unknown(),
            rusmpp_types::Ton::International => Ton::International(),
            rusmpp_types::Ton::National => Ton::National(),
            rusmpp_types::Ton::NetworkSpecific => Ton::NetworkSpecific(),
            rusmpp_types::Ton::SubscriberNumber => Ton::SubscriberNumber(),
            rusmpp_types::Ton::Alphanumeric => Ton::Alphanumeric(),
            rusmpp_types::Ton::Abbreviated => Ton::Abbreviated(),
            rusmpp_types::Ton::Other(inner) => Ton::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Ton {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct AlertNotification {
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub esme_addr_ton: Ton,
    pub esme_addr_npi: Npi,
    pub esme_addr: Vec<u8>,
    pub ms_availability_status: Option<Tlv>,
}

impl From<rusmpp_types::AlertNotification> for AlertNotification {
    fn from(value: rusmpp_types::AlertNotification) -> Self {
        let value = value.into_parts();
        Self {
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            esme_addr_ton: value.esme_addr_ton.into(),
            esme_addr_npi: value.esme_addr_npi.into(),
            esme_addr: value.esme_addr.into(),
            ms_availability_status: value.ms_availability_status.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl AlertNotification {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BindReceiver {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
    pub system_type: Vec<u8>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: Vec<u8>,
}

impl From<rusmpp_types::BindReceiver> for BindReceiver {
    fn from(value: rusmpp_types::BindReceiver) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            password: value.password.into(),
            system_type: value.system_type.into(),
            interface_version: value.interface_version.into(),
            addr_ton: value.addr_ton.into(),
            addr_npi: value.addr_npi.into(),
            address_range: value.address_range.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BindReceiver {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BindReceiverResp {
    pub system_id: Vec<u8>,
    pub sc_interface_version: Option<Tlv>,
}

impl From<rusmpp_types::BindReceiverResp> for BindReceiverResp {
    fn from(value: rusmpp_types::BindReceiverResp) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            sc_interface_version: value.sc_interface_version.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BindReceiverResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BindTransceiver {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
    pub system_type: Vec<u8>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: Vec<u8>,
}

impl From<rusmpp_types::BindTransceiver> for BindTransceiver {
    fn from(value: rusmpp_types::BindTransceiver) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            password: value.password.into(),
            system_type: value.system_type.into(),
            interface_version: value.interface_version.into(),
            addr_ton: value.addr_ton.into(),
            addr_npi: value.addr_npi.into(),
            address_range: value.address_range.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BindTransceiver {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BindTransceiverResp {
    pub system_id: Vec<u8>,
    pub sc_interface_version: Option<Tlv>,
}

impl From<rusmpp_types::BindTransceiverResp> for BindTransceiverResp {
    fn from(value: rusmpp_types::BindTransceiverResp) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            sc_interface_version: value.sc_interface_version.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BindTransceiverResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BindTransmitter {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
    pub system_type: Vec<u8>,
    pub interface_version: InterfaceVersion,
    pub addr_ton: Ton,
    pub addr_npi: Npi,
    pub address_range: Vec<u8>,
}

impl From<rusmpp_types::BindTransmitter> for BindTransmitter {
    fn from(value: rusmpp_types::BindTransmitter) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            password: value.password.into(),
            system_type: value.system_type.into(),
            interface_version: value.interface_version.into(),
            addr_ton: value.addr_ton.into(),
            addr_npi: value.addr_npi.into(),
            address_range: value.address_range.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BindTransmitter {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BindTransmitterResp {
    pub system_id: Vec<u8>,
    pub sc_interface_version: Option<Tlv>,
}

impl From<rusmpp_types::BindTransmitterResp> for BindTransmitterResp {
    fn from(value: rusmpp_types::BindTransmitterResp) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            sc_interface_version: value.sc_interface_version.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BindTransmitterResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DataCoding {
    McSpecific(),
    Ia5(),
    OctetUnspecified(),
    Latin1(),
    OctetUnspecified2(),
    Jis(),
    Cyrillic(),
    LatinHebrew(),
    Ucs2(),
    PictogramEncoding(),
    Iso2022JpMusicCodes(),
    ExtendedKanjiJis(),
    Ksc5601(),
    GsmMwiControl(),
    GsmMwiControl2(),
    GsmMessageClassControl(),
    Other(u8),
}

impl From<rusmpp_types::DataCoding> for DataCoding {
    fn from(value: rusmpp_types::DataCoding) -> Self {
        match value {
            rusmpp_types::DataCoding::McSpecific => DataCoding::McSpecific(),
            rusmpp_types::DataCoding::Ia5 => DataCoding::Ia5(),
            rusmpp_types::DataCoding::OctetUnspecified => DataCoding::OctetUnspecified(),
            rusmpp_types::DataCoding::Latin1 => DataCoding::Latin1(),
            rusmpp_types::DataCoding::OctetUnspecified2 => DataCoding::OctetUnspecified2(),
            rusmpp_types::DataCoding::Jis => DataCoding::Jis(),
            rusmpp_types::DataCoding::Cyrillic => DataCoding::Cyrillic(),
            rusmpp_types::DataCoding::LatinHebrew => DataCoding::LatinHebrew(),
            rusmpp_types::DataCoding::Ucs2 => DataCoding::Ucs2(),
            rusmpp_types::DataCoding::PictogramEncoding => DataCoding::PictogramEncoding(),
            rusmpp_types::DataCoding::Iso2022JpMusicCodes => DataCoding::Iso2022JpMusicCodes(),
            rusmpp_types::DataCoding::ExtendedKanjiJis => DataCoding::ExtendedKanjiJis(),
            rusmpp_types::DataCoding::Ksc5601 => DataCoding::Ksc5601(),
            rusmpp_types::DataCoding::GsmMwiControl => DataCoding::GsmMwiControl(),
            rusmpp_types::DataCoding::GsmMwiControl2 => DataCoding::GsmMwiControl2(),
            rusmpp_types::DataCoding::GsmMessageClassControl => {
                DataCoding::GsmMessageClassControl()
            }
            rusmpp_types::DataCoding::Other(inner) => DataCoding::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DataCoding {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct PriorityFlag {
    pub value: u8,
}

impl From<rusmpp_types::PriorityFlag> for PriorityFlag {
    fn from(value: rusmpp_types::PriorityFlag) -> Self {
        let value = value.into_parts();
        Self {
            value: value.value.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl PriorityFlag {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum ReplaceIfPresentFlag {
    DoNotReplace(),
    Replace(),
    Other(u8),
}

impl From<rusmpp_types::ReplaceIfPresentFlag> for ReplaceIfPresentFlag {
    fn from(value: rusmpp_types::ReplaceIfPresentFlag) -> Self {
        match value {
            rusmpp_types::ReplaceIfPresentFlag::DoNotReplace => {
                ReplaceIfPresentFlag::DoNotReplace()
            }
            rusmpp_types::ReplaceIfPresentFlag::Replace => ReplaceIfPresentFlag::Replace(),
            rusmpp_types::ReplaceIfPresentFlag::Other(inner) => {
                ReplaceIfPresentFlag::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl ReplaceIfPresentFlag {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct ServiceType {
    pub value: Vec<u8>,
}

impl From<rusmpp_types::ServiceType> for ServiceType {
    fn from(value: rusmpp_types::ServiceType) -> Self {
        let value = value.into_parts();
        Self {
            value: value.value.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl ServiceType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BroadcastSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub message_id: Vec<u8>,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::BroadcastSm> for BroadcastSm {
    fn from(value: rusmpp_types::BroadcastSm) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            message_id: value.message_id.into(),
            priority_flag: value.priority_flag.into(),
            schedule_delivery_time: value.schedule_delivery_time.into(),
            validity_period: value.validity_period.into(),
            replace_if_present_flag: value.replace_if_present_flag.into(),
            data_coding: value.data_coding.into(),
            sm_default_msg_id: value.sm_default_msg_id.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct BroadcastSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::BroadcastSmResp> for BroadcastSmResp {
    fn from(value: rusmpp_types::BroadcastSmResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl BroadcastSmResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct CancelBroadcastSm {
    pub service_type: ServiceType,
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::CancelBroadcastSm> for CancelBroadcastSm {
    fn from(value: rusmpp_types::CancelBroadcastSm) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            message_id: value.message_id.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl CancelBroadcastSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct CancelSm {
    pub service_type: ServiceType,
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
}

impl From<rusmpp_types::CancelSm> for CancelSm {
    fn from(value: rusmpp_types::CancelSm) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            message_id: value.message_id.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            dest_addr_ton: value.dest_addr_ton.into(),
            dest_addr_npi: value.dest_addr_npi.into(),
            destination_addr: value.destination_addr.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl CancelSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum CommandId {
    BindReceiver(),
    BindTransmitter(),
    QuerySm(),
    SubmitSm(),
    DeliverSm(),
    Unbind(),
    ReplaceSm(),
    CancelSm(),
    BindTransceiver(),
    Outbind(),
    EnquireLink(),
    SubmitMulti(),
    AlertNotification(),
    DataSm(),
    BroadcastSm(),
    QueryBroadcastSm(),
    CancelBroadcastSm(),
    GenericNack(),
    BindReceiverResp(),
    BindTransmitterResp(),
    QuerySmResp(),
    SubmitSmResp(),
    DeliverSmResp(),
    UnbindResp(),
    ReplaceSmResp(),
    CancelSmResp(),
    BindTransceiverResp(),
    EnquireLinkResp(),
    SubmitMultiResp(),
    DataSmResp(),
    BroadcastSmResp(),
    QueryBroadcastSmResp(),
    CancelBroadcastSmResp(),
    Other(u32),
}

impl From<rusmpp_types::CommandId> for CommandId {
    fn from(value: rusmpp_types::CommandId) -> Self {
        match value {
            rusmpp_types::CommandId::BindReceiver => CommandId::BindReceiver(),
            rusmpp_types::CommandId::BindTransmitter => CommandId::BindTransmitter(),
            rusmpp_types::CommandId::QuerySm => CommandId::QuerySm(),
            rusmpp_types::CommandId::SubmitSm => CommandId::SubmitSm(),
            rusmpp_types::CommandId::DeliverSm => CommandId::DeliverSm(),
            rusmpp_types::CommandId::Unbind => CommandId::Unbind(),
            rusmpp_types::CommandId::ReplaceSm => CommandId::ReplaceSm(),
            rusmpp_types::CommandId::CancelSm => CommandId::CancelSm(),
            rusmpp_types::CommandId::BindTransceiver => CommandId::BindTransceiver(),
            rusmpp_types::CommandId::Outbind => CommandId::Outbind(),
            rusmpp_types::CommandId::EnquireLink => CommandId::EnquireLink(),
            rusmpp_types::CommandId::SubmitMulti => CommandId::SubmitMulti(),
            rusmpp_types::CommandId::AlertNotification => CommandId::AlertNotification(),
            rusmpp_types::CommandId::DataSm => CommandId::DataSm(),
            rusmpp_types::CommandId::BroadcastSm => CommandId::BroadcastSm(),
            rusmpp_types::CommandId::QueryBroadcastSm => CommandId::QueryBroadcastSm(),
            rusmpp_types::CommandId::CancelBroadcastSm => CommandId::CancelBroadcastSm(),
            rusmpp_types::CommandId::GenericNack => CommandId::GenericNack(),
            rusmpp_types::CommandId::BindReceiverResp => CommandId::BindReceiverResp(),
            rusmpp_types::CommandId::BindTransmitterResp => CommandId::BindTransmitterResp(),
            rusmpp_types::CommandId::QuerySmResp => CommandId::QuerySmResp(),
            rusmpp_types::CommandId::SubmitSmResp => CommandId::SubmitSmResp(),
            rusmpp_types::CommandId::DeliverSmResp => CommandId::DeliverSmResp(),
            rusmpp_types::CommandId::UnbindResp => CommandId::UnbindResp(),
            rusmpp_types::CommandId::ReplaceSmResp => CommandId::ReplaceSmResp(),
            rusmpp_types::CommandId::CancelSmResp => CommandId::CancelSmResp(),
            rusmpp_types::CommandId::BindTransceiverResp => CommandId::BindTransceiverResp(),
            rusmpp_types::CommandId::EnquireLinkResp => CommandId::EnquireLinkResp(),
            rusmpp_types::CommandId::SubmitMultiResp => CommandId::SubmitMultiResp(),
            rusmpp_types::CommandId::DataSmResp => CommandId::DataSmResp(),
            rusmpp_types::CommandId::BroadcastSmResp => CommandId::BroadcastSmResp(),
            rusmpp_types::CommandId::QueryBroadcastSmResp => CommandId::QueryBroadcastSmResp(),
            rusmpp_types::CommandId::CancelBroadcastSmResp => CommandId::CancelBroadcastSmResp(),
            rusmpp_types::CommandId::Other(inner) => CommandId::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl CommandId {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Ansi41Specific {
    ShortMessageContainsDeliveryAcknowledgement(),
    ShortMessageContainsUserAcknowledgment(),
    ShortMessageContainsConversationAbort(),
    Other(u8),
}

impl From<rusmpp_types::Ansi41Specific> for Ansi41Specific {
    fn from(value: rusmpp_types::Ansi41Specific) -> Self {
        match value {
            rusmpp_types::Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement => {
                Ansi41Specific::ShortMessageContainsDeliveryAcknowledgement()
            }
            rusmpp_types::Ansi41Specific::ShortMessageContainsUserAcknowledgment => {
                Ansi41Specific::ShortMessageContainsUserAcknowledgment()
            }
            rusmpp_types::Ansi41Specific::ShortMessageContainsConversationAbort => {
                Ansi41Specific::ShortMessageContainsConversationAbort()
            }
            rusmpp_types::Ansi41Specific::Other(inner) => Ansi41Specific::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Ansi41Specific {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum GsmFeatures {
    NotSelected(),
    UdhiIndicator(),
    SetReplyPath(),
    SetUdhiAndReplyPath(),
    Other(u8),
}

impl From<rusmpp_types::GsmFeatures> for GsmFeatures {
    fn from(value: rusmpp_types::GsmFeatures) -> Self {
        match value {
            rusmpp_types::GsmFeatures::NotSelected => GsmFeatures::NotSelected(),
            rusmpp_types::GsmFeatures::UdhiIndicator => GsmFeatures::UdhiIndicator(),
            rusmpp_types::GsmFeatures::SetReplyPath => GsmFeatures::SetReplyPath(),
            rusmpp_types::GsmFeatures::SetUdhiAndReplyPath => GsmFeatures::SetUdhiAndReplyPath(),
            rusmpp_types::GsmFeatures::Other(inner) => GsmFeatures::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl GsmFeatures {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MessageType {
    Default(),
    ShortMessageContainsMCDeliveryReceipt(),
    ShortMessageContainsIntermediateDeliveryNotification(),
    Other(u8),
}

impl From<rusmpp_types::MessageType> for MessageType {
    fn from(value: rusmpp_types::MessageType) -> Self {
        match value {
            rusmpp_types::MessageType::Default => MessageType::Default(),
            rusmpp_types::MessageType::ShortMessageContainsMCDeliveryReceipt => {
                MessageType::ShortMessageContainsMCDeliveryReceipt()
            }
            rusmpp_types::MessageType::ShortMessageContainsIntermediateDeliveryNotification => {
                MessageType::ShortMessageContainsIntermediateDeliveryNotification()
            }
            rusmpp_types::MessageType::Other(inner) => MessageType::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MessageType {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MessagingMode {
    Default(),
    Datagram(),
    Forward(),
    StoreAndForward(),
    Other(u8),
}

impl From<rusmpp_types::MessagingMode> for MessagingMode {
    fn from(value: rusmpp_types::MessagingMode) -> Self {
        match value {
            rusmpp_types::MessagingMode::Default => MessagingMode::Default(),
            rusmpp_types::MessagingMode::Datagram => MessagingMode::Datagram(),
            rusmpp_types::MessagingMode::Forward => MessagingMode::Forward(),
            rusmpp_types::MessagingMode::StoreAndForward => MessagingMode::StoreAndForward(),
            rusmpp_types::MessagingMode::Other(inner) => MessagingMode::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MessagingMode {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct EsmClass {
    pub messaging_mode: MessagingMode,
    pub message_type: MessageType,
    pub ansi41_specific: Ansi41Specific,
    pub gsm_features: GsmFeatures,
}

impl From<rusmpp_types::EsmClass> for EsmClass {
    fn from(value: rusmpp_types::EsmClass) -> Self {
        let value = value.into_parts();
        Self {
            messaging_mode: value.messaging_mode.into(),
            message_type: value.message_type.into(),
            ansi41_specific: value.ansi41_specific.into(),
            gsm_features: value.gsm_features.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl EsmClass {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum IntermediateNotification {
    NoIntermediaryNotificationRequested(),
    IntermediateNotificationRequested(),
    Other(u8),
}

impl From<rusmpp_types::IntermediateNotification> for IntermediateNotification {
    fn from(value: rusmpp_types::IntermediateNotification) -> Self {
        match value {
            rusmpp_types::IntermediateNotification::NoIntermediaryNotificationRequested => {
                IntermediateNotification::NoIntermediaryNotificationRequested()
            }
            rusmpp_types::IntermediateNotification::IntermediateNotificationRequested => {
                IntermediateNotification::IntermediateNotificationRequested()
            }
            rusmpp_types::IntermediateNotification::Other(inner) => {
                IntermediateNotification::Other(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl IntermediateNotification {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum MCDeliveryReceipt {
    NoMcDeliveryReceiptRequested(),
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure(),
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure(),
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess(),
    Other(u8),
}

impl From<rusmpp_types::MCDeliveryReceipt> for MCDeliveryReceipt {
    fn from(value: rusmpp_types::MCDeliveryReceipt) -> Self {
        match value {
            rusmpp_types::MCDeliveryReceipt::NoMcDeliveryReceiptRequested => MCDeliveryReceipt::NoMcDeliveryReceiptRequested(),
            rusmpp_types::MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure => MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure(),
            rusmpp_types::MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure => MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure(),
            rusmpp_types::MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess => MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess(),
            rusmpp_types::MCDeliveryReceipt::Other(inner) => MCDeliveryReceipt::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl MCDeliveryReceipt {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum SmeOriginatedAcknowledgement {
    NoReceiptSmeAcknowledgementRequested(),
    SmeDeliveryAcknowledgementRequested(),
    SmeUserAcknowledgementRequested(),
    BothDeliveryAndUserAcknowledgmentRequested(),
    Other(u8),
}

impl From<rusmpp_types::SmeOriginatedAcknowledgement> for SmeOriginatedAcknowledgement {
    fn from(value: rusmpp_types::SmeOriginatedAcknowledgement) -> Self {
        match value {
            rusmpp_types::SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested => SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested(),
            rusmpp_types::SmeOriginatedAcknowledgement::SmeDeliveryAcknowledgementRequested => SmeOriginatedAcknowledgement::SmeDeliveryAcknowledgementRequested(),
            rusmpp_types::SmeOriginatedAcknowledgement::SmeUserAcknowledgementRequested => SmeOriginatedAcknowledgement::SmeUserAcknowledgementRequested(),
            rusmpp_types::SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested => SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested(),
            rusmpp_types::SmeOriginatedAcknowledgement::Other(inner) => SmeOriginatedAcknowledgement::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SmeOriginatedAcknowledgement {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct RegisteredDelivery {
    pub mc_delivery_receipt: MCDeliveryReceipt,
    pub sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
    pub intermediate_notification: IntermediateNotification,
    pub other: u8,
}

impl From<rusmpp_types::RegisteredDelivery> for RegisteredDelivery {
    fn from(value: rusmpp_types::RegisteredDelivery) -> Self {
        let value = value.into_parts();
        Self {
            mc_delivery_receipt: value.mc_delivery_receipt.into(),
            sme_originated_acknowledgement: value.sme_originated_acknowledgement.into(),
            intermediate_notification: value.intermediate_notification.into(),
            other: value.other.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl RegisteredDelivery {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct DataSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub esm_class: EsmClass,
    pub registered_delivery: RegisteredDelivery,
    pub data_coding: DataCoding,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::DataSm> for DataSm {
    fn from(value: rusmpp_types::DataSm) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            dest_addr_ton: value.dest_addr_ton.into(),
            dest_addr_npi: value.dest_addr_npi.into(),
            destination_addr: value.destination_addr.into(),
            esm_class: value.esm_class.into(),
            registered_delivery: value.registered_delivery.into(),
            data_coding: value.data_coding.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DataSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct DataSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::DataSmResp> for DataSmResp {
    fn from(value: rusmpp_types::DataSmResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DataSmResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct DeliverSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::DeliverSm> for DeliverSm {
    fn from(value: rusmpp_types::DeliverSm) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            dest_addr_ton: value.dest_addr_ton.into(),
            dest_addr_npi: value.dest_addr_npi.into(),
            destination_addr: value.destination_addr.into(),
            esm_class: value.esm_class.into(),
            protocol_id: value.protocol_id.into(),
            priority_flag: value.priority_flag.into(),
            schedule_delivery_time: value.schedule_delivery_time.into(),
            validity_period: value.validity_period.into(),
            registered_delivery: value.registered_delivery.into(),
            replace_if_present_flag: value.replace_if_present_flag.into(),
            data_coding: value.data_coding.into(),
            sm_default_msg_id: value.sm_default_msg_id.into(),
            sm_length: value.sm_length.into(),
            short_message: value.short_message.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DeliverSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct DeliverSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::DeliverSmResp> for DeliverSmResp {
    fn from(value: rusmpp_types::DeliverSmResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DeliverSmResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Outbind {
    pub system_id: Vec<u8>,
    pub password: Vec<u8>,
}

impl From<rusmpp_types::Outbind> for Outbind {
    fn from(value: rusmpp_types::Outbind) -> Self {
        let value = value.into_parts();
        Self {
            system_id: value.system_id.into(),
            password: value.password.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Outbind {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct QueryBroadcastSm {
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub user_message_reference: Option<Tlv>,
}

impl From<rusmpp_types::QueryBroadcastSm> for QueryBroadcastSm {
    fn from(value: rusmpp_types::QueryBroadcastSm) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            user_message_reference: value.user_message_reference.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl QueryBroadcastSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct QueryBroadcastSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::QueryBroadcastSmResp> for QueryBroadcastSmResp {
    fn from(value: rusmpp_types::QueryBroadcastSmResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl QueryBroadcastSmResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct QuerySm {
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
}

impl From<rusmpp_types::QuerySm> for QuerySm {
    fn from(value: rusmpp_types::QuerySm) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl QuerySm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct QuerySmResp {
    pub message_id: Vec<u8>,
    pub final_date: Vec<u8>,
    pub message_state: MessageState,
    pub error_code: u8,
}

impl From<rusmpp_types::QuerySmResp> for QuerySmResp {
    fn from(value: rusmpp_types::QuerySmResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            final_date: value.final_date.into(),
            message_state: value.message_state.into(),
            error_code: value.error_code.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl QuerySmResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct ReplaceSm {
    pub message_id: Vec<u8>,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub message_payload: Option<Tlv>,
}

impl From<rusmpp_types::ReplaceSm> for ReplaceSm {
    fn from(value: rusmpp_types::ReplaceSm) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            schedule_delivery_time: value.schedule_delivery_time.into(),
            validity_period: value.validity_period.into(),
            registered_delivery: value.registered_delivery.into(),
            sm_default_msg_id: value.sm_default_msg_id.into(),
            sm_length: value.sm_length.into(),
            short_message: value.short_message.into(),
            message_payload: value.message_payload.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl ReplaceSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DestFlag {
    SmeAddress(),
    DistributionListName(),
    Other(u8),
}

impl From<rusmpp_types::DestFlag> for DestFlag {
    fn from(value: rusmpp_types::DestFlag) -> Self {
        match value {
            rusmpp_types::DestFlag::SmeAddress => DestFlag::SmeAddress(),
            rusmpp_types::DestFlag::DistributionListName => DestFlag::DistributionListName(),
            rusmpp_types::DestFlag::Other(inner) => DestFlag::Other(inner.into()),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DestFlag {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct DistributionListName {
    pub dest_flag: DestFlag,
    pub dl_name: Vec<u8>,
}

impl From<rusmpp_types::DistributionListName> for DistributionListName {
    fn from(value: rusmpp_types::DistributionListName) -> Self {
        let value = value.into_parts();
        Self {
            dest_flag: value.dest_flag.into(),
            dl_name: value.dl_name.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DistributionListName {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct SmeAddress {
    pub dest_flag: DestFlag,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
}

impl From<rusmpp_types::SmeAddress> for SmeAddress {
    fn from(value: rusmpp_types::SmeAddress) -> Self {
        let value = value.into_parts();
        Self {
            dest_flag: value.dest_flag.into(),
            dest_addr_ton: value.dest_addr_ton.into(),
            dest_addr_npi: value.dest_addr_npi.into(),
            destination_addr: value.destination_addr.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SmeAddress {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum DestAddress {
    SmeAddress(SmeAddress),
    DistributionListName(DistributionListName),
}

impl From<rusmpp_types::DestAddress> for DestAddress {
    fn from(value: rusmpp_types::DestAddress) -> Self {
        match value {
            rusmpp_types::DestAddress::SmeAddress(inner) => DestAddress::SmeAddress(inner.into()),
            rusmpp_types::DestAddress::DistributionListName(inner) => {
                DestAddress::DistributionListName(inner.into())
            }
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl DestAddress {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct SubmitMulti {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub number_of_dests: u8,
    pub dest_address: Vec<DestAddress>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::SubmitMulti> for SubmitMulti {
    fn from(value: rusmpp_types::SubmitMulti) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            number_of_dests: value.number_of_dests.into(),
            dest_address: value.dest_address.into_iter().map(Into::into).collect(),
            esm_class: value.esm_class.into(),
            protocol_id: value.protocol_id.into(),
            priority_flag: value.priority_flag.into(),
            schedule_delivery_time: value.schedule_delivery_time.into(),
            validity_period: value.validity_period.into(),
            registered_delivery: value.registered_delivery.into(),
            replace_if_present_flag: value.replace_if_present_flag.into(),
            data_coding: value.data_coding.into(),
            sm_default_msg_id: value.sm_default_msg_id.into(),
            sm_length: value.sm_length.into(),
            short_message: value.short_message.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SubmitMulti {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct UnsuccessSme {
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub error_status_code: CommandStatus,
}

impl From<rusmpp_types::UnsuccessSme> for UnsuccessSme {
    fn from(value: rusmpp_types::UnsuccessSme) -> Self {
        let value = value.into_parts();
        Self {
            dest_addr_ton: value.dest_addr_ton.into(),
            dest_addr_npi: value.dest_addr_npi.into(),
            destination_addr: value.destination_addr.into(),
            error_status_code: value.error_status_code.into(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl UnsuccessSme {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct SubmitMultiResp {
    pub message_id: Vec<u8>,
    pub no_unsuccess: u8,
    pub unsuccess_sme: Vec<UnsuccessSme>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::SubmitMultiResp> for SubmitMultiResp {
    fn from(value: rusmpp_types::SubmitMultiResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            no_unsuccess: value.no_unsuccess.into(),
            unsuccess_sme: value.unsuccess_sme.into_iter().map(Into::into).collect(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SubmitMultiResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct SubmitSm {
    pub service_type: ServiceType,
    pub source_addr_ton: Ton,
    pub source_addr_npi: Npi,
    pub source_addr: Vec<u8>,
    pub dest_addr_ton: Ton,
    pub dest_addr_npi: Npi,
    pub destination_addr: Vec<u8>,
    pub esm_class: EsmClass,
    pub protocol_id: u8,
    pub priority_flag: PriorityFlag,
    pub schedule_delivery_time: Vec<u8>,
    pub validity_period: Vec<u8>,
    pub registered_delivery: RegisteredDelivery,
    pub replace_if_present_flag: ReplaceIfPresentFlag,
    pub data_coding: DataCoding,
    pub sm_default_msg_id: u8,
    pub sm_length: u8,
    pub short_message: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::SubmitSm> for SubmitSm {
    fn from(value: rusmpp_types::SubmitSm) -> Self {
        let value = value.into_parts();
        Self {
            service_type: value.service_type.into(),
            source_addr_ton: value.source_addr_ton.into(),
            source_addr_npi: value.source_addr_npi.into(),
            source_addr: value.source_addr.into(),
            dest_addr_ton: value.dest_addr_ton.into(),
            dest_addr_npi: value.dest_addr_npi.into(),
            destination_addr: value.destination_addr.into(),
            esm_class: value.esm_class.into(),
            protocol_id: value.protocol_id.into(),
            priority_flag: value.priority_flag.into(),
            schedule_delivery_time: value.schedule_delivery_time.into(),
            validity_period: value.validity_period.into(),
            registered_delivery: value.registered_delivery.into(),
            replace_if_present_flag: value.replace_if_present_flag.into(),
            data_coding: value.data_coding.into(),
            sm_default_msg_id: value.sm_default_msg_id.into(),
            sm_length: value.sm_length.into(),
            short_message: value.short_message.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SubmitSm {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct SubmitSmResp {
    pub message_id: Vec<u8>,
    pub tlvs: Vec<Tlv>,
}

impl From<rusmpp_types::SubmitSmResp> for SubmitSmResp {
    fn from(value: rusmpp_types::SubmitSmResp) -> Self {
        let value = value.into_parts();
        Self {
            message_id: value.message_id.into(),
            tlvs: value.tlvs.into_iter().map(Into::into).collect(),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl SubmitSmResp {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass_complex_enum]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub enum Pdu {
    BindTransmitter(BindTransmitter),
    BindTransmitterResp(BindTransmitterResp),
    BindReceiver(BindReceiver),
    BindReceiverResp(BindReceiverResp),
    BindTransceiver(BindTransceiver),
    BindTransceiverResp(BindTransceiverResp),
    Outbind(Outbind),
    AlertNotification(AlertNotification),
    SubmitSm(SubmitSm),
    SubmitSmResp(SubmitSmResp),
    QuerySm(QuerySm),
    QuerySmResp(QuerySmResp),
    DeliverSm(DeliverSm),
    DeliverSmResp(DeliverSmResp),
    DataSm(DataSm),
    DataSmResp(DataSmResp),
    CancelSm(CancelSm),
    ReplaceSm(ReplaceSm),
    SubmitMulti(SubmitMulti),
    SubmitMultiResp(SubmitMultiResp),
    BroadcastSm(BroadcastSm),
    BroadcastSmResp(BroadcastSmResp),
    QueryBroadcastSm(QueryBroadcastSm),
    QueryBroadcastSmResp(QueryBroadcastSmResp),
    CancelBroadcastSm(CancelBroadcastSm),
    Unbind(),
    UnbindResp(),
    EnquireLink(),
    EnquireLinkResp(),
    GenericNack(),
    CancelSmResp(),
    ReplaceSmResp(),
    CancelBroadcastSmResp(),
    Other {
        command_id: CommandId,
        body: Vec<u8>,
    },
}

impl From<rusmpp_types::Pdu> for Pdu {
    fn from(value: rusmpp_types::Pdu) -> Self {
        match value {
            rusmpp_types::Pdu::BindTransmitter(inner) => Pdu::BindTransmitter(inner.into()),
            rusmpp_types::Pdu::BindTransmitterResp(inner) => Pdu::BindTransmitterResp(inner.into()),
            rusmpp_types::Pdu::BindReceiver(inner) => Pdu::BindReceiver(inner.into()),
            rusmpp_types::Pdu::BindReceiverResp(inner) => Pdu::BindReceiverResp(inner.into()),
            rusmpp_types::Pdu::BindTransceiver(inner) => Pdu::BindTransceiver(inner.into()),
            rusmpp_types::Pdu::BindTransceiverResp(inner) => Pdu::BindTransceiverResp(inner.into()),
            rusmpp_types::Pdu::Outbind(inner) => Pdu::Outbind(inner.into()),
            rusmpp_types::Pdu::AlertNotification(inner) => Pdu::AlertNotification(inner.into()),
            rusmpp_types::Pdu::SubmitSm(inner) => Pdu::SubmitSm(inner.into()),
            rusmpp_types::Pdu::SubmitSmResp(inner) => Pdu::SubmitSmResp(inner.into()),
            rusmpp_types::Pdu::QuerySm(inner) => Pdu::QuerySm(inner.into()),
            rusmpp_types::Pdu::QuerySmResp(inner) => Pdu::QuerySmResp(inner.into()),
            rusmpp_types::Pdu::DeliverSm(inner) => Pdu::DeliverSm(inner.into()),
            rusmpp_types::Pdu::DeliverSmResp(inner) => Pdu::DeliverSmResp(inner.into()),
            rusmpp_types::Pdu::DataSm(inner) => Pdu::DataSm(inner.into()),
            rusmpp_types::Pdu::DataSmResp(inner) => Pdu::DataSmResp(inner.into()),
            rusmpp_types::Pdu::CancelSm(inner) => Pdu::CancelSm(inner.into()),
            rusmpp_types::Pdu::ReplaceSm(inner) => Pdu::ReplaceSm(inner.into()),
            rusmpp_types::Pdu::SubmitMulti(inner) => Pdu::SubmitMulti(inner.into()),
            rusmpp_types::Pdu::SubmitMultiResp(inner) => Pdu::SubmitMultiResp(inner.into()),
            rusmpp_types::Pdu::BroadcastSm(inner) => Pdu::BroadcastSm(inner.into()),
            rusmpp_types::Pdu::BroadcastSmResp(inner) => Pdu::BroadcastSmResp(inner.into()),
            rusmpp_types::Pdu::QueryBroadcastSm(inner) => Pdu::QueryBroadcastSm(inner.into()),
            rusmpp_types::Pdu::QueryBroadcastSmResp(inner) => {
                Pdu::QueryBroadcastSmResp(inner.into())
            }
            rusmpp_types::Pdu::CancelBroadcastSm(inner) => Pdu::CancelBroadcastSm(inner.into()),
            rusmpp_types::Pdu::Unbind => Pdu::Unbind(),
            rusmpp_types::Pdu::UnbindResp => Pdu::UnbindResp(),
            rusmpp_types::Pdu::EnquireLink => Pdu::EnquireLink(),
            rusmpp_types::Pdu::EnquireLinkResp => Pdu::EnquireLinkResp(),
            rusmpp_types::Pdu::GenericNack => Pdu::GenericNack(),
            rusmpp_types::Pdu::CancelSmResp => Pdu::CancelSmResp(),
            rusmpp_types::Pdu::ReplaceSmResp => Pdu::ReplaceSmResp(),
            rusmpp_types::Pdu::CancelBroadcastSmResp => Pdu::CancelBroadcastSmResp(),
            rusmpp_types::Pdu::Other { command_id, body } => Pdu::Other {
                command_id: command_id.into(),
                body: body.into(),
            },
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Pdu {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

#[::pyo3_stub_gen_derive::gen_stub_pyclass]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
#[::pyo3::pyclass(get_all, set_all)]
pub struct Command {
    pub id: CommandId,
    pub status: CommandStatus,
    pub sequence_number: u32,
    pub pdu: Option<Pdu>,
}

impl From<rusmpp_types::Command> for Command {
    fn from(value: rusmpp_types::Command) -> Self {
        let value = value.into_parts();
        Self {
            id: value.id.into(),
            status: value.status.into(),
            sequence_number: value.sequence_number.into(),
            pdu: value.pdu.map(Into::into),
        }
    }
}

#[::pyo3::pymethods]
#[::pyo3_stub_gen_derive::gen_stub_pymethods]
impl Command {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

pub fn add_classes(m: &::pyo3::Bound<'_, ::pyo3::prelude::PyModule>) -> ::pyo3::PyResult<()> {
    use ::pyo3::types::PyModuleMethods;
    m.add_class::<AddrSubunit>()?;
    m.add_class::<AlertNotification>()?;
    m.add_class::<AlertOnMessageDelivery>()?;
    m.add_class::<Ansi41Specific>()?;
    m.add_class::<BearerType>()?;
    m.add_class::<BindReceiver>()?;
    m.add_class::<BindReceiverResp>()?;
    m.add_class::<BindTransceiver>()?;
    m.add_class::<BindTransceiverResp>()?;
    m.add_class::<BindTransmitter>()?;
    m.add_class::<BindTransmitterResp>()?;
    m.add_class::<BroadcastAreaFormat>()?;
    m.add_class::<BroadcastAreaIdentifier>()?;
    m.add_class::<BroadcastAreaSuccess>()?;
    m.add_class::<BroadcastChannelIndicator>()?;
    m.add_class::<BroadcastContentType>()?;
    m.add_class::<BroadcastFrequencyInterval>()?;
    m.add_class::<BroadcastMessageClass>()?;
    m.add_class::<BroadcastRepNum>()?;
    m.add_class::<BroadcastSm>()?;
    m.add_class::<BroadcastSmResp>()?;
    m.add_class::<CallbackNumPresInd>()?;
    m.add_class::<CancelBroadcastSm>()?;
    m.add_class::<CancelSm>()?;
    m.add_class::<Command>()?;
    m.add_class::<CommandId>()?;
    m.add_class::<CommandStatus>()?;
    m.add_class::<CongestionState>()?;
    m.add_class::<DataCoding>()?;
    m.add_class::<DataSm>()?;
    m.add_class::<DataSmResp>()?;
    m.add_class::<DeliverSm>()?;
    m.add_class::<DeliverSmResp>()?;
    m.add_class::<DeliveryFailureReason>()?;
    m.add_class::<DestAddrNpResolution>()?;
    m.add_class::<DestAddress>()?;
    m.add_class::<DestFlag>()?;
    m.add_class::<DisplayTime>()?;
    m.add_class::<DistributionListName>()?;
    m.add_class::<DpfResult>()?;
    m.add_class::<EncodingContentType>()?;
    m.add_class::<ErrorCodeNetworkType>()?;
    m.add_class::<EsmClass>()?;
    m.add_class::<GsmFeatures>()?;
    m.add_class::<Indicator>()?;
    m.add_class::<InterfaceVersion>()?;
    m.add_class::<IntermediateNotification>()?;
    m.add_class::<ItsReplyType>()?;
    m.add_class::<ItsSessionInfo>()?;
    m.add_class::<LanguageIndicator>()?;
    m.add_class::<MCDeliveryReceipt>()?;
    m.add_class::<MessagePayload>()?;
    m.add_class::<MessageState>()?;
    m.add_class::<MessageType>()?;
    m.add_class::<MessagingMode>()?;
    m.add_class::<MoreMessagesToSend>()?;
    m.add_class::<MsAvailabilityStatus>()?;
    m.add_class::<MsMsgWaitFacilities>()?;
    m.add_class::<MsValidity>()?;
    m.add_class::<MsValidityBehavior>()?;
    m.add_class::<MsValidityInformation>()?;
    m.add_class::<NetworkErrorCode>()?;
    m.add_class::<NetworkType>()?;
    m.add_class::<Npi>()?;
    m.add_class::<NumberOfMessages>()?;
    m.add_class::<Outbind>()?;
    m.add_class::<PayloadType>()?;
    m.add_class::<Pdu>()?;
    m.add_class::<Presentation>()?;
    m.add_class::<PriorityFlag>()?;
    m.add_class::<PrivacyIndicator>()?;
    m.add_class::<QueryBroadcastSm>()?;
    m.add_class::<QueryBroadcastSmResp>()?;
    m.add_class::<QuerySm>()?;
    m.add_class::<QuerySmResp>()?;
    m.add_class::<RegisteredDelivery>()?;
    m.add_class::<ReplaceIfPresentFlag>()?;
    m.add_class::<ReplaceSm>()?;
    m.add_class::<Screening>()?;
    m.add_class::<ServiceType>()?;
    m.add_class::<SetDpf>()?;
    m.add_class::<SmeAddress>()?;
    m.add_class::<SmeOriginatedAcknowledgement>()?;
    m.add_class::<Subaddress>()?;
    m.add_class::<SubaddressTag>()?;
    m.add_class::<SubmitMulti>()?;
    m.add_class::<SubmitMultiResp>()?;
    m.add_class::<SubmitSm>()?;
    m.add_class::<SubmitSmResp>()?;
    m.add_class::<Tlv>()?;
    m.add_class::<TlvTag>()?;
    m.add_class::<TlvValue>()?;
    m.add_class::<Ton>()?;
    m.add_class::<TypeOfMessage>()?;
    m.add_class::<TypeOfNetwork>()?;
    m.add_class::<UnitOfTime>()?;
    m.add_class::<UnitsOfTime>()?;
    m.add_class::<UnsuccessSme>()?;
    m.add_class::<UserMessageReference>()?;
    m.add_class::<UssdServiceOp>()?;
    Ok(())
}
