//! encapsulates lib3h ghostmessage for sim2h including security challenge
use crate::error::Sim2hError;
use lib3h_protocol::{data_types::Opaque, protocol::*};
use std::convert::TryFrom;
use std::cmp::Ordering;

pub const WIRE_VERSION: u32 = 1;

#[derive(Debug, Clone, PartialEq,Eq,Serialize, Deserialize,PartialOrd)]
pub enum WireError {
    MessageWhileInLimbo,
    Other(String),
}

#[derive(Debug, Clone, PartialEq,Eq,Serialize, Deserialize,PartialOrd)]
pub struct StatusData {
    pub spaces: usize,
    pub connections: usize,
    pub redundant_count: u64,
    pub version: u32,
}

#[derive(Debug, Clone,PartialEq,Serialize, Deserialize)]
pub enum WireMessage {
    ClientToLib3hResponse(ClientToLib3hResponse),
    Lib3hToClientResponse(Lib3hToClientResponse),
    ClientToLib3h(ClientToLib3h),
    Lib3hToClient(Lib3hToClient),
    Ping,
    Pong,
    Err(WireError),
    Status,
    StatusResponse(StatusData),
}

impl Ord for WireMessage {
    fn cmp(&self, other: &Self) -> Ordering {
        match self
        {
            WireMessage::ClientToLib3hResponse(_) =>
            {
                match other
                {
                    WireMessage::ClientToLib3hResponse(_) => Ordering::Equal,
                    WireMessage::Lib3hToClientResponse(_) => Ordering::Equal,
                    _ => Ordering::Greater
                }
            },
            WireMessage::Lib3hToClientResponse(_)=>
            {
                match other
                {
                    WireMessage::ClientToLib3hResponse(_) => Ordering::Equal,
                    WireMessage::Lib3hToClientResponse(_) => Ordering::Equal,
                    _ => Ordering::Greater
                } 
            },
            _ => match other
            {
                WireMessage::ClientToLib3hResponse(_) => Ordering::Less,
                WireMessage::Lib3hToClientResponse(_) => Ordering::Less,
                _ => Ordering::Equal
            }
        }
    }
}

impl Eq for WireMessage
{

}

impl PartialOrd for WireMessage {
    fn partial_cmp(&self, other: &WireMessage) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


impl WireMessage {
    pub fn message_type(&self) -> String {
        String::from(match self {
            WireMessage::Ping => "Ping",
            WireMessage::Pong => "Pong",
            WireMessage::Status => "Status",
            WireMessage::StatusResponse(_) => "StatusResponse",
            WireMessage::ClientToLib3h(ClientToLib3h::Bootstrap(_)) => "[C>L]Bootstrap",
            WireMessage::ClientToLib3h(ClientToLib3h::FetchEntry(_)) => "[C>L]FetchEntry",
            WireMessage::ClientToLib3h(ClientToLib3h::JoinSpace(_)) => "[C>L]JoinSpace",
            WireMessage::ClientToLib3h(ClientToLib3h::LeaveSpace(_)) => "[C>L]LeaveSpace",
            WireMessage::ClientToLib3h(ClientToLib3h::PublishEntry(_)) => "[C>L]PublishEntry",
            WireMessage::ClientToLib3h(ClientToLib3h::QueryEntry(_)) => "[C>L]QueryEntry",
            WireMessage::ClientToLib3h(ClientToLib3h::SendDirectMessage(_)) => {
                "[C>L]SendDirectmessage"
            }
            WireMessage::ClientToLib3hResponse(ClientToLib3hResponse::BootstrapSuccess) => {
                "[C<L]BootsrapSuccess"
            }
            WireMessage::ClientToLib3hResponse(ClientToLib3hResponse::FetchEntryResult(_)) => {
                "[C<L]FetchEntryResult"
            }
            WireMessage::ClientToLib3hResponse(ClientToLib3hResponse::JoinSpaceResult) => {
                "[C<L]JoinSpaceResult"
            }
            WireMessage::ClientToLib3hResponse(ClientToLib3hResponse::LeaveSpaceResult) => {
                "[C<L]LeaveSpaceResult"
            }
            WireMessage::ClientToLib3hResponse(ClientToLib3hResponse::QueryEntryResult(_)) => {
                "[C<L]QueryEntryResult"
            }
            WireMessage::ClientToLib3hResponse(ClientToLib3hResponse::SendDirectMessageResult(
                _,
            )) => "[C<L]SendDirectMessageResult",
            WireMessage::Lib3hToClient(Lib3hToClient::Connected(_)) => "[L>C]Connected",
            WireMessage::Lib3hToClient(Lib3hToClient::HandleDropEntry(_)) => "[L>C]HandleDropEntry",
            WireMessage::Lib3hToClient(Lib3hToClient::HandleFetchEntry(_)) => {
                "[L>C]HandleFetchEntry"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::HandleGetAuthoringEntryList(_)) => {
                "[L>C]HandleGetAuthoringList"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::HandleGetGossipingEntryList(_)) => {
                "[L>C]HandleGetGossipingEntryList"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::HandleQueryEntry(_)) => {
                "[L>C]HandleQueryEntry"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::HandleSendDirectMessage(_)) => {
                "[L>C]HandleSendDirectMessage"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::HandleStoreEntryAspect(_)) => {
                "[L>C]HandleStoreEntryAspect"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::SendDirectMessageResult(_)) => {
                "[L>C]SendDirectMessageResult"
            }
            WireMessage::Lib3hToClient(Lib3hToClient::Unbound(_)) => "[L>C]Unbound",
            WireMessage::Lib3hToClientResponse(Lib3hToClientResponse::HandleDropEntryResult) => {
                "[L<C]HandleDropEntryResult"
            }
            WireMessage::Lib3hToClientResponse(Lib3hToClientResponse::HandleFetchEntryResult(
                _,
            )) => "[L<C]HandleFetchEntryResult",
            WireMessage::Lib3hToClientResponse(
                Lib3hToClientResponse::HandleGetAuthoringEntryListResult(_),
            ) => "[L<C]HandleGetAuthoringEntryListResult",
            WireMessage::Lib3hToClientResponse(
                Lib3hToClientResponse::HandleGetGossipingEntryListResult(_),
            ) => "[L<C]HandleGetGossipingEntryListResult",
            WireMessage::Lib3hToClientResponse(Lib3hToClientResponse::HandleQueryEntryResult(
                _,
            )) => "[L<C]HandleQueryEntryResult",
            WireMessage::Lib3hToClientResponse(
                Lib3hToClientResponse::HandleSendDirectMessageResult(_),
            ) => "[L<C]HandleSendDirectMessageResult",
            WireMessage::Lib3hToClientResponse(
                Lib3hToClientResponse::HandleStoreEntryAspectResult,
            ) => "[L<C]HandleStoreEntryAspectResult",
            WireMessage::Err(_) => "[Error] {:?}",
        })
    }
}

impl From<WireMessage> for Opaque {
    fn from(message: WireMessage) -> Opaque {
        serde_json::to_string(&message)
            .expect("wiremessage should serialize")
            .into()
    }
}

impl From<WireMessage> for String {
    fn from(message: WireMessage) -> String {
        serde_json::to_string(&message).expect("wiremessage should serialize")
    }
}

impl TryFrom<Opaque> for WireMessage {
    type Error = WireError;
    fn try_from(message: Opaque) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&String::from_utf8_lossy(&message))
            .map_err(|e| format!("{:?}", e))?)
    }
}

impl TryFrom<&Opaque> for WireMessage {
    type Error = WireError;
    fn try_from(message: &Opaque) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(&String::from_utf8_lossy(message))
            .map_err(|e| format!("{:?}", e))?)
    }
}

impl From<&str> for WireError {
    fn from(err: &str) -> Self {
        WireError::Other(format!("{:?}", err))
    }
}

impl From<String> for WireError {
    fn from(err: String) -> Self {
        WireError::Other(err)
    }
}

impl From<WireError> for Sim2hError {
    fn from(err: WireError) -> Sim2hError {
        format!("{:?}", err).into()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    pub fn test_wire_message() {
        let msg = WireMessage::Err("fake_error".into());

        let opaque_msg: Opaque = msg.clone().into();
        assert_eq!(
            "\"{\\\"Err\\\":{\\\"Other\\\":\\\"\\\\\\\"fake_error\\\\\\\"\\\"}}\"",
            format!("{}", opaque_msg)
        );
        let roundtrip_msg = WireMessage::try_from(opaque_msg).expect("deserialize should work");
        assert_eq!(roundtrip_msg, msg);
    }
}
