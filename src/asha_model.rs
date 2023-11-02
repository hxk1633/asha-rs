use std::fmt;

use byteorder::{ByteOrder, LittleEndian}; // 1.3.4

#[derive(Debug)]
enum DeviceSide {
    Left,
    Right,
}

#[derive(Debug)]
enum BinauralType {
    Monaural,
    Binaural,
}

#[derive(Debug)]
struct DeviceCapabilities {
    device_side: DeviceSide,
    binaural_type: BinauralType,
    supports_csis: bool,
    reserved: u8
}

#[derive(Debug)]
struct HiSyncID {
    manufacturer_id: u16,
    hearing_aid_set_id: u64
}

#[derive(Debug)]
struct FeatureMap {
    le_coc_supported: bool,
    reserved: u8
}

#[derive(Debug)]
pub struct ReadOnlyProperties {
    version: u8,
    device_capabilities: DeviceCapabilities,
    hi_sync_id: HiSyncID,
    feature_map: FeatureMap,
    render_delay: u16,
    reserved: u16,
    supported_codec_ids: u16
}

impl DeviceCapabilities {
    fn new(byte: &u8) -> DeviceCapabilities {
        let device_side = match (byte & 0b00000001) == 0b00000001 {
            true => DeviceSide::Right,
            false => DeviceSide::Left
        };

        let binaural_type = match (byte & 0b00000010) == 0b00000010 {
            true => BinauralType::Binaural,
            false => BinauralType::Monaural
        };

        let supports_csis = match (byte & 0b00000100) == 0b00000100 {
            true => true,
            false => false
        };

        let reserved = byte & 0b11111000;

        DeviceCapabilities {
            device_side,
            binaural_type,
            supports_csis,
            reserved
        }
    }
}

impl fmt::Display for DeviceCapabilities {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DeviceCapabilities:\n")?;
        write!(f, "    Device Side: {:?}\n", self.device_side)?;
        write!(f, "    Binaural Type: {:?}\n", self.binaural_type)?;
        write!(f, "    Supports CSIS: {}\n", self.supports_csis)?;
        write!(f, "    Reserved: {}\n", self.reserved)
    }
}

impl HiSyncID {
    fn new(byte_array: &[u8]) -> HiSyncID {
        let manufacturer_id = LittleEndian::read_u16(&byte_array[0..2]);
        let hearing_aid_set_id = LittleEndian::read_u48(&byte_array[2..8]);

        HiSyncID {
            manufacturer_id,
            hearing_aid_set_id
        }
    }
}

impl fmt::Display for HiSyncID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "HiSyncID:\n")?;
        write!(f, "    Manufacturer Id: {}\n", self.manufacturer_id)?;
        write!(f, "    Hearing Aid Set Id: {}\n", self.hearing_aid_set_id)
    }
}

impl FeatureMap {
    fn new(byte: &u8) -> FeatureMap {
        let le_coc_supported = match (byte & 0b00000001) == 0b00000001 {
            true => true,
            false => false
        };

        let reserved = byte & 0b11111110;

        FeatureMap {
            le_coc_supported,
            reserved
        }
    }
}

impl fmt::Display for FeatureMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FeatureMap:\n")?;
        write!(f, "    LE CoC Supported: {}\n", self.le_coc_supported)?;
        write!(f, "    Reserved: {}\n", self.reserved)
    }
}

impl ReadOnlyProperties {
    pub fn new(byte_array: &[u8]) -> Option<ReadOnlyProperties> {
        // for byte in byte_array.iter() {
        //     println!("{:08b}", byte);
        // }
        if byte_array.len() == 17 {
            Some(ReadOnlyProperties {
                version: byte_array[0],
                device_capabilities: DeviceCapabilities::new(&byte_array[1]),
                hi_sync_id: HiSyncID::new(&byte_array[2..10]),
                feature_map: FeatureMap::new(&byte_array[10]),
                render_delay: LittleEndian::read_u16(&byte_array[11..13]),
                reserved: LittleEndian::read_u16(&byte_array[13..15]),
                supported_codec_ids: LittleEndian::read_u16(&byte_array[15..17])
            })
        } else {
            None
        }
    } 
}

impl fmt::Display for ReadOnlyProperties {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ReadOnlyProperties:\n")?;
        write!(f, "  Version: {}\n", self.version)?;
        write!(f, "  {}\n", self.device_capabilities)?;
        write!(f, "  {}\n", self.hi_sync_id)?;
        write!(f, "  {}\n", self.feature_map)?;
        write!(f, "  Render Delay: {}\n", self.render_delay)?;
        write!(f, "  Reserved: {}\n", self.reserved)?;
        write!(f, "  Supported Codec IDs: {}\n", self.supported_codec_ids)
    }
}

