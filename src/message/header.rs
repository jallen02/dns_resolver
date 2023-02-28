/// Flags are set in 2 octets (16 bits). The following are the number of bits from the right where
/// the flag is located within those 16 bits
const MESSAGE_TYPE_SHIFT: usize = 15;
const OPCODE_SHIFT: usize = 11;
const AUTHORITATIVE_SHIFT: usize = 10;
const TRUNCATED_SHIFT: usize = 9;
const RECURSION_DESIRED_SHIFT: usize = 8;
const RECURSION_AVAILABLE_SHIFT: usize = 7;
/// Reserved for future use - zero out the bits in this field
const Z_MASK: u16 = 0b1111_1111_1000_1111; 

#[repr(u16)]
#[derive(Clone, Copy)]
enum MessageType {
    Query = 0,
    Response = 1,
}

#[repr(u16)]
#[derive(Clone, Copy)]
enum Opcode {
    /// Standard query
    Query = 0,
    /// Inverse query
    IQuery = 1,
    /// Server status request
    Status = 2,
}

#[repr(u16)]
#[derive(Clone, Copy)]
enum ResponseCode {
    NoError = 0,
    FormErr = 1,
    ServFail = 2,
    NXDomain = 3,
    NotImp = 4,
    Refused = 5,
}

struct Header { /// 16 bit identifier that is echoed back by DNS server.
    /// Used for matching outstanding requests with responses.
    id: u16,
    flags: Flags
}

struct Flags {
    /// 1 bit field that specifies whether this message 
    /// is a query or a response
    message_type: MessageType,
    /// 4 bit field that specifies the kind of query in this message.
    opcode: Opcode,
    /// 1 bit field that indicates that the answer is authoritative for the domain name in
    /// question.
    /// Only valid in responses.
    authoritative: bool,
    /// 1 bit field that indicates that this message has been truncated due to length.
    truncated: bool,
    /// 1 bit field. This bit may be set in a query and
    /// is copied into the response.  If RD is set, it directs
    /// the name server to pursue the query recursively.
    recursion_desired: bool,
    /// 1 bit field. This bit is set or cleared in a response and denotes whether recursive queries
    /// are supported.
    recursion_available: bool,
    response_code: ResponseCode, 
}

impl Flags {
    pub fn get_bits(&self) -> u16 {
        let mut bits = 0u16;
        bits |= (self.message_type as u16) << MESSAGE_TYPE_SHIFT;
        bits |= (self.opcode as u16) << OPCODE_SHIFT;
        if self.authoritative {
            bits |= 1 << AUTHORITATIVE_SHIFT;
        }
        if self.truncated {
            bits |= 1 << TRUNCATED_SHIFT;
        }
        if self.recursion_desired {
            bits |= 1 << RECURSION_DESIRED_SHIFT;
        }
        if self.recursion_available {
            bits |= 1 << RECURSION_AVAILABLE_SHIFT;
        }
        bits &= Z_MASK;
        bits |= self.response_code as u16;
        bits 
    }
}

#[test]
fn get_bits() {
    let mut flags = Flags {
        message_type: MessageType::Query,
        opcode: Opcode::Query,
        authoritative: false,
        truncated: false,
        recursion_desired: false,
        recursion_available: false,
        response_code: ResponseCode::NoError,
    };
    assert_eq!(flags.get_bits(), 0b0000_0000_0000_0000);

    flags.opcode = Opcode::IQuery; 
    assert_eq!(flags.get_bits(), 0b0000_1000_0000_0000);

    flags.response_code = ResponseCode::Refused;
    assert_eq!(flags.get_bits(), 0b0000_1000_0000_0101);

    flags.authoritative = true;
    assert_eq!(flags.get_bits(), 0b0000_1100_0000_0101);

    flags.truncated = true;
    assert_eq!(flags.get_bits(), 0b0000_1110_0000_0101);

    flags.recursion_desired = true;
    assert_eq!(flags.get_bits(), 0b0000_1111_0000_0101);

    flags.recursion_available = true;
    assert_eq!(flags.get_bits(), 0b0000_1111_1000_0101);

    flags.message_type = MessageType::Response;
    assert_eq!(flags.get_bits(), 0b1000_1111_1000_0101);
}
