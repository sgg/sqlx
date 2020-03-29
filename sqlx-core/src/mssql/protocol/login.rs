use super::Encode;
use super::PacketHeader;
use super::PacketType;
use super::Status;
use bitflags::bitflags;

const HOSTNAME_OFFSET: usize = 0;
const USERNAME_OFFSET: usize = 4;
const PASSWORD_OFFSET: usize = 8;
const APPNAME_OFFSET: usize = 12;
const SERVERNAME_OFFSET: usize = 16;
const EXTENSION_OFFSET: usize = 20;
const CTLINTNAME_OFFSET: usize = 24;
const DATABASE_OFFSET: usize = 32;
const UTF8_SUPPORT_FEATURE: u8 = 0xA0;
const TERMINATOR: u8 = 0xFF;

// All variable-length fields in the login record are optional. This means that the length of the
// field can be specified as 0. If the length is specified as 0, then the offset MUST be ignored.
// The only exception is ibHostName, which MUST always point to the beginning of the
// variable-length data in the login record even in the case where no variable-length
// data is included.
pub struct Login<'a> {
    // Derived when encoding
    // The total length of the LOGIN7 structure.
    // pub length: u32,

    // The highest TDS version being used by the client (for example, 0x00000071 for TDS 7.1).
    // If the TDSVersion value sent by the client is greater than the value that the server
    // recognizes, the server MUST use the highest TDS version that it can use. This provides
    // a mechanism for clients to discover the server TDS by sending a standard LOGIN7 message.
    // If the TDSVersion value sent by the client is lower than the highest TDS version the
    // server recognizes, the server MUST use the TDS version sent by the client.
    // pub tds_version: u32,

    // Derived when encoding
    // The packet size being requested by the client.
    // pub packet_size: u32,

    // The version of the interface library (for example, ODBC or OLEDB) being used by the client.
    // pub client_prog_version: u32,

    // The process ID of the client application.
    // pub client_pid: u32,

    // The connection ID of the primary Server. Used when connecting to an "Always Up" backup server.
    // pub connection_id: u32,

    // pub option_flags1: OptionFlags1,
    // pub option_flags2: OptionFlags2,
    // pub type_flags: TypeFlags,

    // This field is not used and can be set to zero
    // pub client_timezone: i32,

    // The language code identifier (LCID) value for the client collation. If ClientLCID is
    // specified, the specified collation is set as the session collation. Note that the total
    // ClientLCID is 4 bytes, which implies that there is no support for SQL Sort orders.
    //
    // NOTE: The ClientLCID value is no longer used to set language parameters and is ignored.
    // pub client_lcid: u32,

    // Derived when encoding
    // The variable portion of this message. A stream of bytes in the order shown, indicates the
    // offset (from the start of the message) and length of various parameters:
    // pub offset_length: OffsetLength,
    // pub data: &'a [u8],
    pub hostname: &'a str,
    pub username: &'a str,
    pub database: &'a str,
    pub appname: &'a str,
    pub ctlintname: &'a str,

    // Before submitting a password from the client to the server, for every byte in the password
    // buffer starting with the position pointed to by ibPassword or ibChangePassword, the client
    // SHOULD first swap the four high bits with the four low bits and then do a bit-XOR with
    // 0xA5 (10100101). After reading a submitted password; for every byte in the password buffer
    // starting with the position pointed to by ibPassword or ibChangePassword, the server SHOULD
    // first do a bit-XOR with 0xA5 (10100101) and then swap the four high bits with the
    // four low bits.
    pub password: &'a str,
    // pub feature_ext: Option<FeatureOpt<'a>>,
}

impl<'a> Encode for Login<'a> {
    fn encode(&self, buf: &mut Vec<u8>) {
        /// 10 01 00 90 00 00 01 00 88 00          10 01 00 90 00 00 01 00 88 00                                       
        /// 00 00 02 00 09 72 00 10 00 00          00 00 04 00 00 74 00 10 00 00                                       
        /// 00 00 00 07 00 01 00 00 00 00          00 00 00 07 00 01 00 00 00 00                                       
        /// 00 00 E0 03 00 00 E0 01 00 00          00 00 E0 03 00 00 E0 01 00 00                                       
        /// 09 04 00 00 5E 00 08 00 6E 00          09 04 00 00 5E 00 08 00 66 00                                       
        /// 02 00 72 00 00 00 72 00 07 00          02 00 00 00 00 00 68 00 07 00                                       
        /// 80 00 00 00 80 00 00 00 80 00          00 00 00 00 00 00 00 00 6F 00                                       
        /// 04 00 88 00 00 00 88 00 00 00          04 00 00 00 00 00 00 00 00 00                                       
        /// 00 50 8B E2 B7 8F 88 00 00 00          00 00 00 00 00 00 00 00 00 00                                       
        /// 88 00 00 00 88 00 00 00 00 00          00 00 00 00 00 00 00 00 00 00                                       
        /// 00 00 73 00 6B 00 6F 00 73 00          00 00 73 00 6B 00 6F 00 73 00                                       
        /// 74 00 6F 00 76 00 31 00 73 00          74 00 6F 00 76 00 31 00 73 00                    
        /// 61 00 4F 00 53 00 51 00 4C 00          61 00 4F 00 53 00 51 00 4C 00                    
        /// 2D 00 33 00 32 00 4F 00 44 00          2D 00 33 00 32 00 4F 00 44 00                    
        /// 42 00 43 00                            42 00 43 00                             

        // Pointer to beginning of message
        let start = buf.len();
        let mut header = PacketHeader::new(PacketType::Tds7Login);
        header.status = Status::END_OF_MESSAGE;

        header.encode(buf);

        let mut offset = OffsetLength::size() + 36;

        #[rustfmt::skip]
        buf.extend_from_slice(&[
            // Placeholder for length
            0, 0, 0, 0,
            // tds_version: SQL Server 2012 | SQL Server 2014 | SQL Server 2016 | SQL Server 2017 | SQL Server 2019
            0x04, 0, 0, 0x74,
            // packet_size
            0, 0x10, 0, 0,
            // client_prog_version
            0, 0, 0, 7,
            // client_pid
            0, 1, 0, 0,
            // conneciton_id
            0, 0, 0, 0,
        ]);

        buf.push(OptionFlags1::default().bits());
        buf.push(OptionFlags2::default().bits());
        buf.push(TypeFlags::default().bits());
        buf.push(OptionFlags3::default().bits());

        #[rustfmt::skip]
        buf.extend_from_slice(&[
            // client_timezone
            0xE0, 1, 0, 0,
            // client_lcid
            9, 4, 0, 0
        ]);

        // OffsetLength struct as bytes initialized to 0's
        // Updated later when needed
        buf.extend_from_slice(&[0u8; OffsetLength::size() as usize]);

        println!("{:X?}", buf);
        println!("{}", buf.len());
        println!("{}", HOSTNAME_OFFSET + PacketHeader::size() + 36);
        println!("{}", offset);

        buf[HOSTNAME_OFFSET + PacketHeader::size() + 36
            ..HOSTNAME_OFFSET + PacketHeader::size() + 36 + 2]
            .copy_from_slice(&offset.to_le_bytes());
        buf[HOSTNAME_OFFSET + PacketHeader::size() + 36 + 2
            ..HOSTNAME_OFFSET + PacketHeader::size() + 36 + 4]
            .copy_from_slice(&(self.hostname.len()  as u16).to_le_bytes());
        for (i, &byte) in self.hostname.as_bytes().iter().enumerate() {
            buf.push(byte);
            buf.push(0);
        }
        offset += self.hostname.as_bytes().len() as u16 * 2;

        buf[USERNAME_OFFSET + PacketHeader::size() + 36
            ..USERNAME_OFFSET + PacketHeader::size() + 36 + 2]
            .copy_from_slice(&offset.to_le_bytes());
        buf[USERNAME_OFFSET + PacketHeader::size() + 36 + 2
            ..USERNAME_OFFSET + PacketHeader::size() + 36 + 4]
            .copy_from_slice(&(self.username.len()  as u16).to_le_bytes());
        for (i, &byte) in self.username.as_bytes().iter().enumerate() {
            buf.push(byte);
            buf.push(0);
        }
        offset += self.username.as_bytes().len() as u16 * 2;

        buf[APPNAME_OFFSET + PacketHeader::size() + 36
            ..APPNAME_OFFSET + PacketHeader::size() + 36 + 2]
            .copy_from_slice(&offset.to_le_bytes());
        buf[APPNAME_OFFSET + PacketHeader::size() + 36 + 2
            ..APPNAME_OFFSET + PacketHeader::size() + 36 + 4]
            .copy_from_slice(&(self.appname.len()  as u16).to_le_bytes());
        for (i, &byte) in self.appname.as_bytes().iter().enumerate() {
            buf.push(byte);
            buf.push(0);
        }
        offset += self.appname.as_bytes().len() as u16 * 2;

        buf[CTLINTNAME_OFFSET + PacketHeader::size() + 36
            ..CTLINTNAME_OFFSET + PacketHeader::size() + 36 + 2]
            .copy_from_slice(&offset.to_le_bytes());
        buf[CTLINTNAME_OFFSET + PacketHeader::size() + 36 + 2
            ..CTLINTNAME_OFFSET + PacketHeader::size() + 36 + 4]
            .copy_from_slice(&(self.ctlintname.len()  as u16).to_le_bytes());
        for (i, &byte) in self.ctlintname.as_bytes().iter().enumerate() {
            buf.push(byte);
            buf.push(0);
        }
        offset += self.appname.as_bytes().len() as u16 * 2;

        // buf[PASSWORD_OFFSET + PacketHeader::size() + 36..PASSWORD_OFFSET + PacketHeader::size() + 36 + 2]
        //     .copy_from_slice(&offset.to_be_bytes());
        // buf[PASSWORD_OFFSET + PacketHeader::size() + 36 + 2..PASSWORD_OFFSET + PacketHeader::size() + 36 + 4]
        //     .copy_from_slice(&(self.password.len() as u16 * 2).to_be_bytes());
        // offset += self.password.as_bytes().len() as u16;
        // for (i, byte) in self.password.as_bytes().iter().enumerate() {
        //     // Swap high-low bits
        //     let byte = (byte << 4) | (byte >> 4);
        //     // XOR with 0xA5
        //     let byte = byte ^ 0xA5u8;
        //     buf.push(byte);
        //     buf.push(0);
        // }

        // // Update `Extension` to point to FeatureExt
        // buf[EXTENSION_OFFSET + PacketHeader::size()..EXTENSION_OFFSET + PacketHeader::size() + 2]
        //     .copy_from_slice(&offset.to_be_bytes());
        // buf[EXTENSION_OFFSET + PacketHeader::size() + 2
        //     ..EXTENSION_OFFSET + PacketHeader::size() + 4]
        //     .copy_from_slice(&(4u16).to_be_bytes());
        // offset += 4;

        // // FeatureExt: Utf8 Support Required
        // buf.push(UTF8_SUPPORT_FEATURE);
        // buf.extend_from_slice(&1u32.to_be_bytes());
        // buf.push(1);

        // buf.push(TERMINATOR);

        // Set length field on the Login structure
        let len = ((buf.len() - start - PacketHeader::size()) as u32).to_be_bytes();
        buf[start + PacketHeader::size() + 0] = len[3];
        buf[start + PacketHeader::size() + 1] = len[2];
        buf[start + PacketHeader::size() + 2] = len[1];
        buf[start + PacketHeader::size() + 3] = len[0];

        // Set length field on the PacketHeader structure
        let len = buf.len() - start;
        buf[start + 2..start + 4].copy_from_slice(&(len as u16).to_be_bytes());

        println!("{}", hex::encode(&buf));
        dbg!(buf.len());
    }
}

bitflags! {
    pub struct OptionFlags1: u8 {
        // fByteOrder: The byte order used by client for numeric and datetime data types.
        //      - 0 = ORDER_X86
        //      - 1 = ORDER_68000
        const BYTE_ORDER = 0x01;

        // fChar: The character set used on the client.
        //      - 0 = CHARSET_ASCII
        //      - 1 = CHARSET_EBCDIC
        const CHAR = 0x02;

        // fFloat: The type of floating point representation used by the client.
        //      - 0 = FLOAT_IEEE_754
        //      - 1 = FLOAT_VAX
        //      - 2 = ND5000
        const FLOAT_LOW = 0x04;
        const FLOAT_HI = 0x08;

        // fDumpLoad: Set is dump/load or BCP capabilities are needed by the client.
        //      - 0 = DUMPLOAD_ON
        //      - 1 = DUMPLOAD_OFF
        const DUMP_LOAD = 0x10;

        // fUseDB: Set if the client requires warning messages on execution of the USE SQL
        // statement. If this flag is not set, the server MUST NOT inform the client when
        // the database changes, and therefore the client will be unaware of any accompanying
        // collation changes.
        //      - 0 = USE_DB_OFF
        //      - 1 = USE_DB_ON
        const USE_DB = 0x20;

        // fDatabase: Set if the change to initial database needs to succeed if the connection is
        // to succeed.
        //      - 0 = INIT_DB_WARN
        //      - 1 = INIT_DB_FATAL
        const DATABASE = 0x40;

        // fSetLang: Set if the client requires warning messages on execution of a language
        // change statement.
        //      - 0 = SET_LANG_OFF
        //      - 1 = SET_LANG_ON
        const SET_LANG = 0x80;
    }
}

impl Default for OptionFlags1 {
    fn default() -> Self {
        OptionFlags1::from_bits_truncate(0xE0)
    }
}

bitflags! {
    pub struct OptionFlags2: u8 {
        // fLanguage: Set if the change to initial language needs to succeed if the connect is
        // to succeed.
        //      - 0 = INIT_LANG_WARN
        //      - 1 = INIT_LANG_FATAL
        const LANGUAGE = 0x01;

        // fODBC: Set if the client is the ODBC driver. This causes the server to set
        // ANSI_DEFAULTS to ON, CURSOR_CLOSE_ON_COMMIT and IMPLICIT_TRANSACTIONS to OFF, TEXTSIZE
        // to 0x7FFFFFFF (2GB) (TDS 7.2 and earlier); TEXTSIZE to infinite
        // (introduced in TDS 7.3), and ROWCOUNT to infinite.
        //      - 0 = ODBC_OFF
        //      - 1 = ODBC_ON
        const ODBC = 0x02;

        // fTransBoundary
        const TRAN_BOUNDRY = 0x04;

        // fCacheConnect
        const CACHE_CONNECT = 0x08;

        // fUserType: The type of user connecting to the server.
        //      - 0 = USER_NORMAL --regular logins
        //      - 1 = USER_SERVER --reserved
        //      - 2 = USER_REMUSER --Distributed Query login
        //      - 3 = USER_SQLREPL --replication login
        const USER_TYPE_1 = 0x10;
        const USER_TYPE_2 = 0x20;
        const USER_TYPE_3 = 0x40;

        // fIntSecurity: The type of security required by the client.
        //      - 0 = INTEGRATED_SECURTY_OFF
        //      - 1 = INTEGRATED_SECURITY_ON
        const INT_SECURITY = 0x80;
    }
}

impl Default for OptionFlags2 {
    fn default() -> Self {
        OptionFlags2::from_bits_truncate(0x03)
    }
}

bitflags! {
    pub struct TypeFlags: u8 {
        // fSQLType: The type of SQL the client sends to the server.
        //      - 0 = SQL_DFLT
        //      - 1 = SQL_TSQL
        const SQL_TYPE_1 = 0x01;
        const SQL_TYPE_2 = 0x02;
        const SQL_TYPE_3 = 0x04;
        const SQL_TYPE_4 = 0x08;

        // fOLEDB: Set if the client is the OLEDB driver. This causes the server to set
        // ANSI_DEFAULTS to ON, CURSOR_CLOSE_ON_COMMIT and IMPLICIT_TRANSACTIONS to OFF, TEXTSIZE
        // to 0x7FFFFFFF (2GB) (TDS 7.2 and earlier); TEXTSIZE to infinite
        // (introduced in TDS 7.3), and ROWCOUNT to infinite.
        //      - 0 = OLEDB_OFF
        //      - 1 = OLEDB_ON
        const OLE_DB = 0x10;

        // fReadOnlyIntent: This bit was introduced in TDS 7.4; however, TDS 7.1, 7.2, and
        // 7.3 clients can also use this bit in LOGIN7 to specify that the application intent
        // of the connection is read-only. The server SHOULD ignore this bit if the highest
        // TDS version supported by the server is lower than TDS 7.4.
        const READ_ONLY_INTENT = 0x20;
    }
}

impl Default for TypeFlags {
    fn default() -> Self {
        TypeFlags::from_bits_truncate(0x00)
    }
}

pub struct OffsetLength {
    // ibHostname & cchHostName: The client machine name.
    pub ib_hostname: u16,
    pub cch_hostname: u16,

    // ibUserName & cchUserName: The client user ID
    pub ib_username: u16,
    pub cch_username: u16,

    // ibPassword & cchPassword: The password supplied by the client
    pub ib_password: u16,
    pub cch_password: u16,

    // ibAppName & cchAppName: The client application name.
    pub ib_appname: u16,
    pub cch_appname: u16,

    // ibServerName & cchServerName: The server name.
    pub ib_servername: u16,
    pub cch_servername: u16,

    // ibExtension & cbExtension: This points to an extension block. Introduced in TDS7.4
    // when fExtension is 1. The content pointed by ibExtension is defined as follows:
    //
    //      ibFeatureExtLong    =  DWORD
    //      Extension           =  ibFeatureExtLong
    //
    // ibFeatureExtLong provides the offset (from the start of the message) of FeatureExt block.
    // ibFeatureExtLong MUST be 0 if FeatureExt block does not exist.
    //
    // Extension block can be extended in future. The client MUST NOT send more data than needed.
    // The server SHOULD ignore any appended data that is unknown to the server.
    pub ib_extension: u16,
    pub cb_extension: u16,

    // ibCltIntName & cchCltIntName: The interface library name (ODBC or OLEDB).
    pub ib_clt_int_name: u16,
    pub cch_clt_int_name: u16,

    // ibLanguage & cchLanguage: The initial language (overrides the user ID's default language).
    pub ib_language: u16,
    pub cch_language: u16,

    // ibDatabase & cchDatabase: The initial database (overrides the user ID's default database).
    pub ib_database: u16,
    pub cch_database: u16,

    // ClientID: The unique client ID (created by using the NIC address). ClientID is the MAC
    // address of the physical network layer. It is used to identify the client that is connecting
    // to the server. This value is mainly informational, and no processing steps on the server
    // side use it.
    pub client_id: [u8; 6],

    // ibSSPI & cbSSPI: SSPI data.
    //
    // If cbSSPI < USHORT_MAX, then this length MUST be used for SSPI and cbSSPILong MUST be ignored.
    //
    // If cbSSPI == USHORT_MAX, then cbSSPILong MUST be checked.
    //
    // If cbSSPILong > 0, then that value MUST be used. If cbSSPILong ==0, then cbSSPI (USHORT_MAX)
    // MUST be used.
    pub ib_sspi: u16,
    pub cch_sspi: u16,

    // ibAtchDBFile & cchAtchDBFile: The file name for a database that is to be attached during
    // the connection process.
    //      - ibChangePassword & cchChangePassword: New password for the specified login.
    //        Introduced in TDS 7.2.
    //      - cbSSPILong: Used for large SSPI data when cbSSPI==USHORT_MAX. Introduced in TDS 7.2
    pub ib_atch_db_file: u16,
    pub cch_atch_db_file: u16,

    // The actual variable-length data portion referred to by OffsetLength
    pub ib_change_password: u16,
    pub cch_change_password: u16,
    pub cb_sspi_long: u32,
}

impl OffsetLength {
    const fn size() -> u16 {
        58
    }
}

bitflags! {
    pub struct OptionFlags3: u8 {
        // fChangePassword: Specifies whether the login request SHOULD change password.
        //      - 0 = No change request. ibChangePassword MUST be 0.
        //      - 1 = Request to change login's password.
        const CHANGE_PASSWORD = 0x01;

        // fSendYukonBinaryXML: 1 if XML data type instances are returned as binary XML.
        const USER_INSTANCE = 0x20;

        // fUserInstance: 1 if client is requesting separate process to be spawned as user instance.
        const SEND_YUKON_BINARY_XML = 0x40;

        // fUnknownCollationHandling: This bit is used by the server to determine if a client is
        // able to properly handle collations introduced after TDS 7.2. TDS 7.2 and earlier
        // clients are encouraged to use this loginpacket bit. Servers MUST ignore this bit when
        // it is sent by TDS 7.3 or 7.4 clients. See [MSDN-SQLCollation]and [MS-LCID]for the
        // complete list of collations for a database server that supports SQL and LCIDs.
        //      - 0 = The server MUST restrict the collations sent to a specific set of collations.
        //            It MAY disconnect or send an error if some other value is outside the specific
        //            collation set. The client MUST properly support all collations within the
        //            collation set.
        //      - 1 = The server MAY send any collation that fits in the storage space. The client
        //            MUST be able to both properly support collations and gracefully fail for those
        //            it does not support.
        const UNKNOWN_COLLATION_HANDLING = 0x80;

        // fExtension: Specifies whether ibExtension/cbExtension fields are used.
        //      - 0 = ibExtension/cbExtension fields are not used. The fields are treated the
        //            same as ibUnused/cchUnused.
        //      - 1 = ibExtension/cbExtension fields are used.
        const EXTENSION = 0x10;
    }
}

impl Default for OptionFlags3 {
    fn default() -> Self {
        OptionFlags3::from_bits_truncate(0x00)
    }
}

pub struct FeatureOpt<'a> {
    // The unique identifier number of a feature. The available features are described in the
    // following table.Introduced in TDS 7.4.
    pub feature_id: u8,

    // The length, in bytes, of FeatureData for the corresponding FeatureId.Introduced in TDS 7.4.
    pub feature_data_len: u32,

    // Data of the feature. Each feature defines its own data format.The data for existing
    // features are defined in the following table.Introduced in TDS 7.4.
    pub feature_data: &'a [u8],
}

// The data block that can be used to inform and/or negotiate features between client and server.
// It contains data for one or more optional features. Each feature is assigned an identifier,
// followed by data length and data. The data for each feature is defined by the feature’s own
// logic. If the server does not support the specific feature, it MUST skip the feature data and
// jump to next feature. If needed, each feature SHOULD have its own logic to detect whether the
// server accepts the feature option.
//
// Optionally, a feature can use a FEATUREEXTACKtoken to acknowledge the feature along with LOGINACK.
// The detailed acknowledge data SHOULD be defined by the feature itself.
//
// Introduced in TDS 7.4
pub struct FeatureExt<'a> {
    pub options: Vec<FeatureOpt<'a>>,
}
