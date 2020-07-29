use crate::types::guild::MixedType;
use tracing::warn;
/// Internal function to handle the optional query parameters
/// that are accepted by certain API routes.
///
/// Should not be used in your own code.
#[macro_export]
macro_rules! handle_query_params {
    ($($a:expr),+) => {{
        let vec = Vec::new();
        let value;
        $(
        match $a.1 {
            MixedType::U8(v) => let value = v.unwrap(),
            MixedType::String(v) => let value = v.unwrap(),
            MixedType::AuditLogEvent(v) => let value = v.unwrap(),
            _ => { warn!("unhandled case in `handle_query_params!()`"); return vec},
        }
        if value.is_some() {
            vec.push(value.unwrap());
        }
        )+
        vec
    }}
}