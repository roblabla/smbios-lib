use super::*;

/// # System Configuration Options (Type 12)
///
/// This structure contains information required to configure the baseboard’s Jumpers and Switches.
///
/// Compliant with:
/// DMTF SMBIOS Reference Specification 3.4.0 (DSP0134)
/// Document Date: 2020-07-17
pub struct SMBiosSystemConfigurationOptions<'a> {
    parts: &'a SMBiosStructParts<'a>,
}

impl<'a> SMBiosStruct<'a> for SMBiosSystemConfigurationOptions<'a> {
    const STRUCT_TYPE: u8 = 12u8;

    fn new(parts: &'a SMBiosStructParts<'_>) -> Self {
        Self { parts }
    }

    fn parts(&self) -> &'a SMBiosStructParts<'a> {
        self.parts
    }
}

impl<'a> SMBiosSystemConfigurationOptions<'a> {
    /// Number of strings
    pub fn count(&self) -> Option<u8> {
        self.parts.get_field_byte(0x04)
    }

    /// Iterable collection of OEM strings
    ///
    /// EXAMPLES:
    /// "JP2: 1-2 Cache Size is 256K, 2-3 Cache Size is 512K"
    /// "SW1-1: Close to Disable On Board Video"
    pub fn configuration_strings(&self) -> &Strings {
        &self.parts.strings
    }
}

impl fmt::Debug for SMBiosSystemConfigurationOptions<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct(std::any::type_name::<SMBiosSystemConfigurationOptions>())
            .field("header", &self.parts.header)
            .field("count", &self.count())
            .field("configuration_strings", &self.configuration_strings())
            .finish()
    }
}
