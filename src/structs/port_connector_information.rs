use super::*;

/// # Port Connector Information (Type 8)
///
/// The information in this structure defines the attributes of a system port connector
/// (for example, parallel, serial, keyboard, or mouse ports). The port’s type and connector information are
/// provided. One structure is present for each port provided by the system.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosPortConnectorInformation<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosPortConnectorInformation<'a> {
    const STRUCT_TYPE: u8 = 8u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosPortConnectorInformation<'a> {
    ///  Internal reference designator, that is,
    /// internal to the system enclosure
    ///
    /// EXAMPLE: "J101"
    pub fn internal_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x04)
    }

    /// Internal connector type
    pub fn internal_connector_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x05)
    }

    /// External reference designation,
    /// external to the system enclosure
    ///
    /// EXAMPLE: "COM A"
    pub fn external_reference_designator(&self) -> Option<String> {
        self.parts.get_field_string(0x06)
    }

    /// External connector type
    pub fn external_connector_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x07)
    }

    /// Describes the function of the port
    pub fn port_type(&self) -> Option<u8> {
        self.parts.get_field_byte(0x08)
    }
}

impl fmt::Debug for SMBiosPortConnectorInformation<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosPortConnectorInformation>())
            .field("header", &self.parts.header)
            .field(
                "internal_reference_designator",
                &self.internal_reference_designator(),
            )
            .field("internal_connector_type", &self.internal_connector_type())
            .field(
                "external_reference_designator",
                &self.external_reference_designator(),
            )
            .field("external_connector_type", &self.external_connector_type())
            .field("port_type", &self.port_type())
            .finish()
    }
}
