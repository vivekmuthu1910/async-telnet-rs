pub const IAC: u8  = 255; // "Interpret As Command"
pub const DONT: u8 = 254; 
pub const DO: u8   = 253; 
pub const WONT: u8 = 252; 
pub const WILL: u8 = 251; 
pub const NULL: u8 = 0; 

pub const SE: u8  = 240;   // Subnegotiation End
pub const NOP: u8 = 241;   // No Operation
pub const DM: u8  = 242;   // Data Mark
pub const BRK: u8 = 243;   // Break
pub const IP: u8  = 244;   // Interrupt process
pub const AO: u8  = 245;   // Abort output
pub const AYT: u8 = 246;   // Are You There
pub const EC: u8  = 247;   // Erase Character
pub const EL: u8  = 248;   // Erase Line
pub const GA: u8  = 249;   // Go Ahead
pub const SB: u8 =  250;   // Subnegotiation Begin

// Telnet protocol options code (don't change)
// These ones all come from arpa/telnet.h
pub const BINARY: u8 = 0;  // 8-bit data path
pub const ECHO: u8 = 1;  // echo
pub const RCP: u8 = 2;  // prepare to reconnect
pub const SGA: u8 = 3;  // suppress go ahead
pub const NAMS: u8 = 4;  // approximate message size
pub const STATUS: u8 = 5;  // give status
pub const TM: u8 = 6;  // timing mark
pub const RCTE: u8 = 7;  // remote controlled transmission and echo
pub const NAOL: u8 = 8;  // negotiate about output line width
pub const NAOP: u8 = 9;  // negotiate about output page size
pub const NAOCRD: u8 = 10;  // negotiate about CR disposition
pub const NAOHTS: u8 = 11;  // negotiate about horizontal tabstops
pub const NAOHTD: u8 = 12;  // negotiate about horizontal tab disposition
pub const NAOFFD: u8 = 13;  // negotiate about formfeed disposition
pub const NAOVTS: u8 = 14;  // negotiate about vertical tab stops
pub const NAOVTD: u8 = 15;  // negotiate about vertical tab disposition
pub const NAOLFD: u8 = 16;  // negotiate about output LF disposition
pub const XASCII: u8 = 17;  // extended ascii character set
pub const LOGOUT: u8 = 18;  // force logout
pub const BM: u8 = 19;  // byte macro
pub const DET: u8 = 20;  // data entry terminal
pub const SUPDUP: u8 = 21;  // supdup protocol
pub const SUPDUPOUTPUT: u8 = 22;  // supdup output
pub const SNDLOC: u8 = 23;  // send location
pub const TTYPE: u8 = 24;  // terminal type
pub const EOR: u8 = 25;  // end or record
pub const TUID: u8 = 26;  // TACACS user identification
pub const OUTMRK: u8 = 27;  // output marking
pub const TTYLOC: u8 = 28;  // terminal location number
pub const VT3270REGIME: u8 = 29;  // 3270 regime
pub const X3PAD: u8 = 30;  // X.3 PAD
pub const NAWS: u8 = 31;  // window size
pub const TSPEED: u8 = 32;  // terminal speed
pub const LFLOW: u8 = 33;  // remote flow control
pub const LINEMODE: u8 = 34;  // Linemode option
pub const XDISPLOC: u8 = 35;  // X Display Location
pub const OLD_ENVIRON: u8 = 36;  // Old - Environment variables
pub const AUTHENTICATION: u8 = 37;  // Authenticate
pub const ENCRYPT: u8 = 38;  // Encryption option
pub const NEW_ENVIRON: u8 = 39;  // New - Environment variables

// the following ones come from
// http://www.iana.org/assignments/telnet-options
// Unfortunately, that document does not assign identifiers
// to all of them, so we are making them up
pub const TN3270E: u8 = 40;  // TN3270E
pub const XAUTH: u8 = 41;  // XAUTH
pub const CHARSET: u8 = 42;  // CHARSET
pub const RSP: u8 = 43;  // Telnet Remote Serial Port
pub const COM_PORT_OPTION: u8 = 44;  // Com Port Control Option
pub const SUPPRESS_LOCAL_ECHO: u8 = 45;  // Telnet Suppress Local Echo
pub const TLS: u8 = 46;  // Telnet Start TLS
pub const KERMIT: u8 = 47;  // KERMIT
pub const SEND_URL: u8 = 48;  // SEND-URL
pub const FORWARD_X: u8 = 49;  // FORWARD_X
pub const PRAGMA_LOGON: u8 = 138;  // TELOPT PRAGMA LOGON
pub const SSPI_LOGON: u8 = 139;  // TELOPT SSPI LOGON
pub const PRAGMA_HEARTBEAT: u8 = 140;  // TELOPT PRAGMA HEARTBEAT
pub const EXOPL: u8 = 255;  // Extended-Options-List
pub const NOOPT: u8 = 0; 
