#[derive(Debug)]
pub struct Header {
    name: &'static [u8; 4],
    major: u8,
    minor: u8,
    revision: u8,
}

pub const HEADER: Header = Header {
    name: b"AMQP",
    major: 0,
    minor: 9,
    revision: 1,
};

pub const PORT: u16 = 5672;

pub const PROT_HEADER: [u8; 8] = [65, 77, 81, 80, 0, 0, 9, 1];

pub const FRAME_METHOD: u8 = 1;
pub const FRAME_HEADER: u8 = 2;
pub const FRAME_BODY: u8 = 3;
pub const FRAME_HEARTBEAT: u8 = 8;
pub const FRAME_MIN_SIZE: u16 = 4096;
pub const FRAME_END: u8 = 206;
pub const REPLY_SUCCESS: u8 = 200;
pub const CONTENT_TOO_LARGE: u16 = 311;
pub const NO_ROUTE: u16 = 312;
pub const NO_CONSUMERS: u16 = 313;
pub const ACCESS_REFUSED: u16 = 403;
pub const NOT_FOUND: u16 = 404;
pub const RESOURCE_LOCKED: u16 = 405;
pub const PRECONDITION_FAILED: u16 = 406;
pub const CONNECTION_FORCED: u16 = 320;
pub const INVALID_PATH: u16 = 402;
pub const FRAME_ERROR: u16 = 501;
pub const SYNTAX_ERROR: u16 = 502;
pub const COMMAND_INVALID: u16 = 503;
pub const CHANNEL_ERROR: u16 = 504;
pub const UNEXPECTED_FRAME: u16 = 505;
pub const RESOURCE_ERROR: u16 = 506;
pub const NOT_ALLOWED: u16 = 530;
pub const NOT_IMPLEMENTED: u16 = 540;
pub const INTERNAL_ERROR: u16 = 541;

pub mod connection {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Start {
        cid: Short,
        id: Short,
        pub version_major: Octet,
        pub version_minor: Octet,
        pub server_properties: Table,
        pub mechanisms: Longstr,
        pub locales: Longstr,
    }

    impl Method for Start {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "mechanisms" => Some(StrType::Long),
                "locales" => Some(StrType::Long),
                _ => None
            }
        }
    }

    impl Default for Start {
        fn default() -> Start {
            Start {
                cid: 10,
                id: 10,
                version_major: 0,
                version_minor: 9,
                server_properties: Default::default(),
                mechanisms: b"PLAIN".to_vec(),
                locales: b"en_US".to_vec(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct StartOk {
        cid: Short,
        id: Short,
        pub client_properties: Table,
        pub mechanism: Shortstr,
        pub response: Longstr,
        pub locale: Shortstr,
    }

    impl Method for StartOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "mechanism" => Some(StrType::Short),
                "response" => Some(StrType::Long),
                "locale" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for StartOk {
        fn default() -> StartOk {
            StartOk {
                cid: 10,
                id: 11,
                client_properties: Default::default(),
                mechanism: "PLAIN".to_string(),
                response: Default::default(),
                locale: "en_US".to_string(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Secure {
        cid: Short,
        id: Short,
        pub challenge: Longstr,
    }

    impl Method for Secure {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "challenge" => Some(StrType::Long),
                _ => None
            }
        }
    }

    impl Default for Secure {
        fn default() -> Secure {
            Secure {
                cid: 10,
                id: 20,
                challenge: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct SecureOk {
        cid: Short,
        id: Short,
        pub response: Longstr,
    }

    impl Method for SecureOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "response" => Some(StrType::Long),
                _ => None
            }
        }
    }

    impl Default for SecureOk {
        fn default() -> SecureOk {
            SecureOk {
                cid: 10,
                id: 21,
                response: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Tune {
        cid: Short,
        id: Short,
        pub channel_max: Short,
        pub frame_max: Long,
        pub heartbeat: Short,
    }

    impl Method for Tune {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Tune {
        fn default() -> Tune {
            Tune {
                cid: 10,
                id: 30,
                channel_max: 0,
                frame_max: 0,
                heartbeat: 0,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct TuneOk {
        cid: Short,
        id: Short,
        pub channel_max: Short,
        pub frame_max: Long,
        pub heartbeat: Short,
    }

    impl Method for TuneOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for TuneOk {
        fn default() -> TuneOk {
            TuneOk {
                cid: 10,
                id: 31,
                channel_max: 0,
                frame_max: 0,
                heartbeat: 0,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Open {
        cid: Short,
        id: Short,
        pub virtual_host: Shortstr,
        pub capabilities: Shortstr,
        pub insist: Bit,
    }

    impl Method for Open {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "virtual_host" => Some(StrType::Short),
                "capabilities" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Open {
        fn default() -> Open {
            Open {
                cid: 10,
                id: 40,
                virtual_host: "/".to_string(),
                capabilities: "".to_string(),
                insist: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct OpenOk {
        cid: Short,
        id: Short,
        pub known_hosts: Shortstr,
    }

    impl Method for OpenOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "known_hosts" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for OpenOk {
        fn default() -> OpenOk {
            OpenOk {
                cid: 10,
                id: 41,
                known_hosts: "".to_string(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Close {
        cid: Short,
        id: Short,
        pub reply_code: Short,
        pub reply_text: Shortstr,
        pub class_id: Short,
        pub method_id: Short,
    }

    impl Method for Close {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "reply_text" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Close {
        fn default() -> Close {
            Close {
                cid: 10,
                id: 50,
                reply_code: Default::default(),
                reply_text: "".to_string(),
                class_id: Default::default(),
                method_id: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct CloseOk {
        cid: Short,
        id: Short,
    }

    impl Method for CloseOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for CloseOk {
        fn default() -> CloseOk {
            CloseOk {
                cid: 10,
                id: 51,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Blocked {
        cid: Short,
        id: Short,
        pub reason: Shortstr,
    }

    impl Method for Blocked {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "reason" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Blocked {
        fn default() -> Blocked {
            Blocked {
                cid: 10,
                id: 60,
                reason: "".to_string(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Unblocked {
        cid: Short,
        id: Short,
    }

    impl Method for Unblocked {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Unblocked {
        fn default() -> Unblocked {
            Unblocked {
                cid: 10,
                id: 61,
            }
        }
    }
}

pub mod channel {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Open {
        cid: Short,
        id: Short,
        pub out_of_band: Shortstr,
    }

    impl Method for Open {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "out_of_band" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Open {
        fn default() -> Open {
            Open {
                cid: 20,
                id: 10,
                out_of_band: "".to_string(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct OpenOk {
        cid: Short,
        id: Short,
        pub channel_id: Longstr,
    }

    impl Method for OpenOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "channel_id" => Some(StrType::Long),
                _ => None
            }
        }
    }

    impl Default for OpenOk {
        fn default() -> OpenOk {
            OpenOk {
                cid: 20,
                id: 11,
                channel_id: b"".to_vec(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Flow {
        cid: Short,
        id: Short,
        pub active: Bit,
    }

    impl Method for Flow {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Flow {
        fn default() -> Flow {
            Flow {
                cid: 20,
                id: 20,
                active: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct FlowOk {
        cid: Short,
        id: Short,
        pub active: Bit,
    }

    impl Method for FlowOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for FlowOk {
        fn default() -> FlowOk {
            FlowOk {
                cid: 20,
                id: 21,
                active: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Close {
        cid: Short,
        id: Short,
        pub reply_code: Short,
        pub reply_text: Shortstr,
        pub class_id: Short,
        pub method_id: Short,
    }

    impl Method for Close {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "reply_text" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Close {
        fn default() -> Close {
            Close {
                cid: 20,
                id: 40,
                reply_code: Default::default(),
                reply_text: "".to_string(),
                class_id: Default::default(),
                method_id: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct CloseOk {
        cid: Short,
        id: Short,
    }

    impl Method for CloseOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for CloseOk {
        fn default() -> CloseOk {
            CloseOk {
                cid: 20,
                id: 41,
            }
        }
    }
}

pub mod access {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Request {
        cid: Short,
        id: Short,
        pub realm: Shortstr,
        pub exclusive: Bit,
        pub passive: Bit,
        pub active: Bit,
        pub write: Bit,
        pub read: Bit,
    }

    impl Method for Request {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "realm" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Request {
        fn default() -> Request {
            Request {
                cid: 30,
                id: 10,
                realm: "/data".to_string(),
                exclusive: false,
                passive: true,
                active: true,
                write: true,
                read: true,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct RequestOk {
        cid: Short,
        id: Short,
        pub ticket: Short,
    }

    impl Method for RequestOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for RequestOk {
        fn default() -> RequestOk {
            RequestOk {
                cid: 30,
                id: 11,
                ticket: 1,
            }
        }
    }
}

pub mod exchange {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Declare {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub exchange: Shortstr,
        pub ty: Shortstr,
        pub passive: Bit,
        pub durable: Bit,
        pub auto_delete: Bit,
        pub internal: Bit,
        pub nowait: Bit,
        pub arguments: Table,
    }

    impl Method for Declare {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "exchange" => Some(StrType::Short),
                "ty" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Declare {
        fn default() -> Declare {
            Declare {
                cid: 40,
                id: 10,
                ticket: 0,
                exchange: Default::default(),
                ty: "direct".to_string(),
                passive: false,
                durable: false,
                auto_delete: false,
                internal: false,
                nowait: false,
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct DeclareOk {
        cid: Short,
        id: Short,
    }

    impl Method for DeclareOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for DeclareOk {
        fn default() -> DeclareOk {
            DeclareOk {
                cid: 40,
                id: 11,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Delete {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub exchange: Shortstr,
        pub if_unused: Bit,
        pub nowait: Bit,
    }

    impl Method for Delete {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "exchange" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Delete {
        fn default() -> Delete {
            Delete {
                cid: 40,
                id: 20,
                ticket: 0,
                exchange: Default::default(),
                if_unused: false,
                nowait: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct DeleteOk {
        cid: Short,
        id: Short,
    }

    impl Method for DeleteOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for DeleteOk {
        fn default() -> DeleteOk {
            DeleteOk {
                cid: 40,
                id: 21,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Bind {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub destination: Shortstr,
        pub source: Shortstr,
        pub routing_key: Shortstr,
        pub nowait: Bit,
        pub arguments: Table,
    }

    impl Method for Bind {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "destination" => Some(StrType::Short),
                "source" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Bind {
        fn default() -> Bind {
            Bind {
                cid: 40,
                id: 30,
                ticket: 0,
                destination: Default::default(),
                source: Default::default(),
                routing_key: "".to_string(),
                nowait: false,
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct BindOk {
        cid: Short,
        id: Short,
    }

    impl Method for BindOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for BindOk {
        fn default() -> BindOk {
            BindOk {
                cid: 40,
                id: 31,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Unbind {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub destination: Shortstr,
        pub source: Shortstr,
        pub routing_key: Shortstr,
        pub nowait: Bit,
        pub arguments: Table,
    }

    impl Method for Unbind {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "destination" => Some(StrType::Short),
                "source" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Unbind {
        fn default() -> Unbind {
            Unbind {
                cid: 40,
                id: 40,
                ticket: 0,
                destination: Default::default(),
                source: Default::default(),
                routing_key: "".to_string(),
                nowait: false,
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct UnbindOk {
        cid: Short,
        id: Short,
    }

    impl Method for UnbindOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for UnbindOk {
        fn default() -> UnbindOk {
            UnbindOk {
                cid: 40,
                id: 51,
            }
        }
    }
}

pub mod queue {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Declare {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub passive: Bit,
        pub durable: Bit,
        pub exclusive: Bit,
        pub auto_delete: Bit,
        pub nowait: Bit,
        pub arguments: Table,
    }

    impl Method for Declare {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Declare {
        fn default() -> Declare {
            Declare {
                cid: 50,
                id: 10,
                ticket: 0,
                queue: "".to_string(),
                passive: false,
                durable: false,
                exclusive: false,
                auto_delete: false,
                nowait: false,
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct DeclareOk {
        cid: Short,
        id: Short,
        pub queue: Shortstr,
        pub message_count: Long,
        pub consumer_count: Long,
    }

    impl Method for DeclareOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for DeclareOk {
        fn default() -> DeclareOk {
            DeclareOk {
                cid: 50,
                id: 11,
                queue: Default::default(),
                message_count: Default::default(),
                consumer_count: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Bind {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub exchange: Shortstr,
        pub routing_key: Shortstr,
        pub nowait: Bit,
        pub arguments: Table,
    }

    impl Method for Bind {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                "exchange" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Bind {
        fn default() -> Bind {
            Bind {
                cid: 50,
                id: 20,
                ticket: 0,
                queue: "".to_string(),
                exchange: Default::default(),
                routing_key: "".to_string(),
                nowait: false,
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct BindOk {
        cid: Short,
        id: Short,
    }

    impl Method for BindOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for BindOk {
        fn default() -> BindOk {
            BindOk {
                cid: 50,
                id: 21,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Purge {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub nowait: Bit,
    }

    impl Method for Purge {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Purge {
        fn default() -> Purge {
            Purge {
                cid: 50,
                id: 30,
                ticket: 0,
                queue: "".to_string(),
                nowait: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct PurgeOk {
        cid: Short,
        id: Short,
        pub message_count: Long,
    }

    impl Method for PurgeOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for PurgeOk {
        fn default() -> PurgeOk {
            PurgeOk {
                cid: 50,
                id: 31,
                message_count: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Delete {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub if_unused: Bit,
        pub if_empty: Bit,
        pub nowait: Bit,
    }

    impl Method for Delete {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Delete {
        fn default() -> Delete {
            Delete {
                cid: 50,
                id: 40,
                ticket: 0,
                queue: "".to_string(),
                if_unused: false,
                if_empty: false,
                nowait: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct DeleteOk {
        cid: Short,
        id: Short,
        pub message_count: Long,
    }

    impl Method for DeleteOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for DeleteOk {
        fn default() -> DeleteOk {
            DeleteOk {
                cid: 50,
                id: 41,
                message_count: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Unbind {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub exchange: Shortstr,
        pub routing_key: Shortstr,
        pub arguments: Table,
    }

    impl Method for Unbind {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                "exchange" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Unbind {
        fn default() -> Unbind {
            Unbind {
                cid: 50,
                id: 50,
                ticket: 0,
                queue: "".to_string(),
                exchange: Default::default(),
                routing_key: "".to_string(),
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct UnbindOk {
        cid: Short,
        id: Short,
    }

    impl Method for UnbindOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for UnbindOk {
        fn default() -> UnbindOk {
            UnbindOk {
                cid: 50,
                id: 51,
            }
        }
    }
}

pub mod basic {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Qos {
        cid: Short,
        id: Short,
        pub prefetch_size: Long,
        pub prefetch_count: Short,
        pub global: Bit,
    }

    impl Method for Qos {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Qos {
        fn default() -> Qos {
            Qos {
                cid: 60,
                id: 10,
                prefetch_size: 0,
                prefetch_count: 0,
                global: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct QosOk {
        cid: Short,
        id: Short,
    }

    impl Method for QosOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for QosOk {
        fn default() -> QosOk {
            QosOk {
                cid: 60,
                id: 11,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Consume {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub consumer_tag: Shortstr,
        pub no_local: Bit,
        pub no_ack: Bit,
        pub exclusive: Bit,
        pub nowait: Bit,
        pub arguments: Table,
    }

    impl Method for Consume {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                "consumer_tag" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Consume {
        fn default() -> Consume {
            Consume {
                cid: 60,
                id: 20,
                ticket: 0,
                queue: "".to_string(),
                consumer_tag: "".to_string(),
                no_local: false,
                no_ack: false,
                exclusive: false,
                nowait: false,
                arguments: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct ConsumeOk {
        cid: Short,
        id: Short,
        pub consumer_tag: Shortstr,
    }

    impl Method for ConsumeOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "consumer_tag" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for ConsumeOk {
        fn default() -> ConsumeOk {
            ConsumeOk {
                cid: 60,
                id: 21,
                consumer_tag: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Cancel {
        cid: Short,
        id: Short,
        pub consumer_tag: Shortstr,
        pub nowait: Bit,
    }

    impl Method for Cancel {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "consumer_tag" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Cancel {
        fn default() -> Cancel {
            Cancel {
                cid: 60,
                id: 30,
                consumer_tag: Default::default(),
                nowait: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct CancelOk {
        cid: Short,
        id: Short,
        pub consumer_tag: Shortstr,
    }

    impl Method for CancelOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "consumer_tag" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for CancelOk {
        fn default() -> CancelOk {
            CancelOk {
                cid: 60,
                id: 31,
                consumer_tag: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Publish {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub exchange: Shortstr,
        pub routing_key: Shortstr,
        pub mandatory: Bit,
        pub immediate: Bit,
    }

    impl Method for Publish {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "exchange" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Publish {
        fn default() -> Publish {
            Publish {
                cid: 60,
                id: 40,
                ticket: 0,
                exchange: "".to_string(),
                routing_key: "".to_string(),
                mandatory: false,
                immediate: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Return {
        cid: Short,
        id: Short,
        pub reply_code: Short,
        pub reply_text: Shortstr,
        pub exchange: Shortstr,
        pub routing_key: Shortstr,
    }

    impl Method for Return {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "reply_text" => Some(StrType::Short),
                "exchange" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Return {
        fn default() -> Return {
            Return {
                cid: 60,
                id: 50,
                reply_code: Default::default(),
                reply_text: "".to_string(),
                exchange: Default::default(),
                routing_key: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Deliver {
        cid: Short,
        id: Short,
        pub consumer_tag: Shortstr,
        pub delivery_tag: Longlong,
        pub redelivered: Bit,
        pub exchange: Shortstr,
        pub routing_key: Shortstr,
    }

    impl Method for Deliver {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "consumer_tag" => Some(StrType::Short),
                "exchange" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Deliver {
        fn default() -> Deliver {
            Deliver {
                cid: 60,
                id: 60,
                consumer_tag: Default::default(),
                delivery_tag: Default::default(),
                redelivered: false,
                exchange: Default::default(),
                routing_key: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Get {
        cid: Short,
        id: Short,
        pub ticket: Short,
        pub queue: Shortstr,
        pub no_ack: Bit,
    }

    impl Method for Get {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "queue" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for Get {
        fn default() -> Get {
            Get {
                cid: 60,
                id: 70,
                ticket: 0,
                queue: "".to_string(),
                no_ack: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct GetOk {
        cid: Short,
        id: Short,
        pub delivery_tag: Longlong,
        pub redelivered: Bit,
        pub exchange: Shortstr,
        pub routing_key: Shortstr,
        pub message_count: Long,
    }

    impl Method for GetOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "exchange" => Some(StrType::Short),
                "routing_key" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for GetOk {
        fn default() -> GetOk {
            GetOk {
                cid: 60,
                id: 71,
                delivery_tag: Default::default(),
                redelivered: false,
                exchange: Default::default(),
                routing_key: Default::default(),
                message_count: Default::default(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct GetEmpty {
        cid: Short,
        id: Short,
        pub cluster_id: Shortstr,
    }

    impl Method for GetEmpty {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                "cluster_id" => Some(StrType::Short),
                _ => None
            }
        }
    }

    impl Default for GetEmpty {
        fn default() -> GetEmpty {
            GetEmpty {
                cid: 60,
                id: 72,
                cluster_id: "".to_string(),
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Ack {
        cid: Short,
        id: Short,
        pub delivery_tag: Longlong,
        pub multiple: Bit,
    }

    impl Method for Ack {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Ack {
        fn default() -> Ack {
            Ack {
                cid: 60,
                id: 80,
                delivery_tag: 0,
                multiple: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Reject {
        cid: Short,
        id: Short,
        pub delivery_tag: Longlong,
        pub requeue: Bit,
    }

    impl Method for Reject {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Reject {
        fn default() -> Reject {
            Reject {
                cid: 60,
                id: 90,
                delivery_tag: Default::default(),
                requeue: true,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct RecoverAsync {
        cid: Short,
        id: Short,
        pub requeue: Bit,
    }

    impl Method for RecoverAsync {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for RecoverAsync {
        fn default() -> RecoverAsync {
            RecoverAsync {
                cid: 60,
                id: 100,
                requeue: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Recover {
        cid: Short,
        id: Short,
        pub requeue: Bit,
    }

    impl Method for Recover {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Recover {
        fn default() -> Recover {
            Recover {
                cid: 60,
                id: 110,
                requeue: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct RecoverOk {
        cid: Short,
        id: Short,
    }

    impl Method for RecoverOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for RecoverOk {
        fn default() -> RecoverOk {
            RecoverOk {
                cid: 60,
                id: 111,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Nack {
        cid: Short,
        id: Short,
        pub delivery_tag: Longlong,
        pub multiple: Bit,
        pub requeue: Bit,
    }

    impl Method for Nack {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Nack {
        fn default() -> Nack {
            Nack {
                cid: 60,
                id: 120,
                delivery_tag: 0,
                multiple: false,
                requeue: true,
            }
        }
    }
}

pub mod tx {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Select {
        cid: Short,
        id: Short,
    }

    impl Method for Select {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Select {
        fn default() -> Select {
            Select {
                cid: 90,
                id: 10,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct SelectOk {
        cid: Short,
        id: Short,
    }

    impl Method for SelectOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for SelectOk {
        fn default() -> SelectOk {
            SelectOk {
                cid: 90,
                id: 11,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Commit {
        cid: Short,
        id: Short,
    }

    impl Method for Commit {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Commit {
        fn default() -> Commit {
            Commit {
                cid: 90,
                id: 20,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct CommitOk {
        cid: Short,
        id: Short,
    }

    impl Method for CommitOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for CommitOk {
        fn default() -> CommitOk {
            CommitOk {
                cid: 90,
                id: 21,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Rollback {
        cid: Short,
        id: Short,
    }

    impl Method for Rollback {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Rollback {
        fn default() -> Rollback {
            Rollback {
                cid: 90,
                id: 30,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct RollbackOk {
        cid: Short,
        id: Short,
    }

    impl Method for RollbackOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for RollbackOk {
        fn default() -> RollbackOk {
            RollbackOk {
                cid: 90,
                id: 31,
            }
        }
    }
}

pub mod confirm {
    use method::Method;
    use types::*;
    
    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct Select {
        cid: Short,
        id: Short,
        pub nowait: Bit,
    }

    impl Method for Select {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for Select {
        fn default() -> Select {
            Select {
                cid: 85,
                id: 10,
                nowait: false,
            }
        }
    }

    #[derive(Debug, PartialEq, RustcEncodable, RustcDecodable)]
    pub struct SelectOk {
        cid: Short,
        id: Short,
    }

    impl Method for SelectOk {
        fn str_type(param: &str) -> Option<StrType> {
            match param { 
                _ => None
            }
        }
    }

    impl Default for SelectOk {
        fn default() -> SelectOk {
            SelectOk {
                cid: 85,
                id: 11,
            }
        }
    }
}
