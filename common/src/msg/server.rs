use super::{ClientState, EcsCompPacket, EcsResPacket};
use crate::{comp, terrain::TerrainChunk, ChatType};
use vek::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestStateError {
    Denied,
    Already,
    Impossible,
    WrongMessage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMsg {
    InitialSync {
        ecs_state: sphynx::StatePackage<EcsCompPacket, EcsResPacket>,
        entity_uid: u64,
        server_info: ServerInfo,
    },
    StateAnswer(Result<ClientState, (RequestStateError, ClientState)>),
    ForceState(ClientState),
    Ping,
    Pong,
    ChatMsg {
        chat_type: ChatType,
        msg: String,
    },
    SetPlayerEntity(u64),
    EcsSync(sphynx::SyncPackage<EcsCompPacket, EcsResPacket>),
    EntityPhysics {
        entity: u64,
        pos: comp::Pos,
        vel: comp::Vel,
        ori: comp::Ori,
        action_state: comp::ActionState,
    },
    TerrainChunkUpdate {
        key: Vec2<i32>,
        chunk: Box<TerrainChunk>,
    },
    Error(ServerError),
    Disconnect,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerError {
    TooManyPlayers,
    //TODO: InvalidAlias,
}

impl ServerMsg {
    pub fn chat(message: String) -> crate::msg::server::ServerMsg {
        crate::msg::server::ServerMsg::ChatMsg {
            chat_type: ChatType::Chat,
            msg: message,
        }
    }
    pub fn tell(message: String) -> crate::msg::server::ServerMsg {
        crate::msg::server::ServerMsg::ChatMsg {
            chat_type: ChatType::Tell,
            msg: message,
        }
    }
}
