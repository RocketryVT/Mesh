use modular_bitfield::prelude::*;

pub struct AprsCompressedPositionReport {
    pub time: [u8; 7],         // Time in DHM or HMS format (7 bytes)
    pub symbol_table: char,    // Symbol Table Identifier (either '/' or '\') (1 byte)
    pub compressed_lat: [u8; 4], // Compressed Latitude (YYYY) (4 bytes)
    pub compressed_long: [u8; 4], // Compressed Longitude (XXXX) (4 bytes)
    pub symbol_code: char,     // Symbol Code (1 byte)
    pub compressed_altitude: [u8; 2], // Compressed Altitude (XX) (2 bytes)
    pub compressed_type: char, // Compressed Type (1 byte)
    pub comment: Comment, // Optional Comment (max 40 chars) (40 bytes)
}

pub struct Acknowledgement {
    pub id: u8,
    pub ack: bool,
}

/// Bitfield for the Comment field
/// The Comment field contains both Mesh data and Custom data from the ADS
/// The Comment field is 40 bytes long (320 bits)
#[bitfield(bits = 256)]
pub struct Comment {
    pub uid: B8, // Unique Identifier (8 bits)
    pub destination_uid: B8, // Destination Unique Identifier (8 bits)
    pub msg_id: B8, // Message ID (8 bits)
    pub hops_left : B3, // Hops Left (3 bits)
    #[bits = 2]
    pub comment_type: DeviceType, // Type (2 bits)
    #[bits = 2]
    pub msg_type: MessageType, // Message Type (2 bit)
    pub team_number: B8, // Team ID (6 bits)
    // 39 Bits for above fields
    #[bits = 128]
    pub ads: AdsCompressedPart1,
    #[bits = 80]
    pub ads2: AdsCompressedPart2,
    // 208 Bits for ADS data
    // 39 + 208 = 247 Bits
    #[skip]
    unused: B9, // Unused bits (9 bits)
}

#[derive(BitfieldSpecifier)]
pub enum DeviceType {
    Ground = 0,
    Top = 1,
    Bottom = 2,
    Mobile = 3,
}

#[derive(BitfieldSpecifier)]
pub enum MessageType {
    Ack = 0,
    Data = 1,
    Placeholder = 2,
    Custom = 3,
}

#[bitfield(bits = 416)]
pub struct AdsUncompressed {
    pub lat: B32,
    pub lon: B32,
    pub alt: B32,
    pub vel_x: B32,
    pub vel_y: B32,
    pub vel_z: B32,
    pub acc_x: B32,
    pub acc_y: B32,
    pub acc_z: B32,
    pub predicted_apogee: B32,
    pub flap_deploy_angle : B32,
    pub timestamp : B64,
}

#[bitfield(bits = 128)]
#[derive(BitfieldSpecifier)]
pub struct AdsCompressedPart1 {
    pub lat: B16,
    pub lon: B16,
    pub vel_x: B16,
    pub vel_y: B16,
    pub vel_z: B16,
    pub acc_x: B16,
    pub acc_y: B16,
    pub acc_z: B16,
}

#[bitfield(bits = 80)]
#[derive(BitfieldSpecifier)]
pub struct AdsCompressedPart2 {
    pub alt: B16,
    pub predicted_apogee: B16,
    pub flap_deploy_angle: B16,
    pub timestamp: B32,
}

// impl AprsCompressedPositionReport {
//     pub fn new(
//         time: String,
//         symbol_table: char,
//         compressed_lat: String,
//         compressed_long: String,
//         symbol_code: char,
//         compressed_altitude: String,
//         compressed_type: char,
//         comment: Option<String>,
//     ) -> Self {
//         Self {
//             time,
//             symbol_table,
//             compressed_lat,
//             compressed_long,
//             symbol_code,
//             compressed_altitude,
//             compressed_type,
//             comment,
//         }
//     }
// }

// Example usage
// fn main() {
//     let report = AprsCompressedPositionReport::new(
//         "092345z".to_string(),
//         '/',
//         "5L!!".to_string(),
//         "<*e7".to_string(),
//         '{',
//         "?!".to_string(),
//         'T',
//         Some("with APRS messaging, timestamp, radio range".to_string()),
//     );

//     println!("{:?}", report);
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aprs_compressed_position_report() {
        let report = AprsCompressedPositionReport {
            time: *b"092345z",
            symbol_table: '/',
            compressed_lat: *b"5L!!",
            compressed_long: *b"<*e7",
            symbol_code: '{',
            compressed_altitude: *b"?!",
            compressed_type: 'T',
            comment: Comment::new()
                .with_uid(1)
                .with_destination_uid(2)
                .with_msg_id(3)
                .with_hops_left(4)
                .with_comment_type(DeviceType::Ground)
                .with_msg_type(MessageType::Data)
                .with_team_number(5)
                .with_ads(AdsCompressedPart1::new()
                    .with_lat(100)
                    .with_lon(200)
                    .with_vel_x(300)
                    .with_vel_y(400)
                    .with_vel_z(500)
                    .with_acc_x(600)
                    .with_acc_y(700)
                    .with_acc_z(800))
                .with_ads2(AdsCompressedPart2::new()
                    .with_alt(900)
                    .with_predicted_apogee(1000)
                    .with_flap_deploy_angle(1100)
                    .with_timestamp(1200)),
        };

        assert_eq!(report.time, *b"092345z");
        assert_eq!(report.symbol_table, '/');
        assert_eq!(report.compressed_lat, *b"5L!!");
        assert_eq!(report.compressed_long, *b"<*e7");
        assert_eq!(report.symbol_code, '{');
        assert_eq!(report.compressed_altitude, *b"?!");
        assert_eq!(report.compressed_type, 'T');
    }
}