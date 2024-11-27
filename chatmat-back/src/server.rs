use tonic::{transport::Server, Request, Response, Status};

pub mod chat {
    tonic::include_proto!("chat");
}

use chat::chat_server::{Chat, ChatServer};
use chat::{
    ChatGroup, ChatGroupCreate, ChatGroupCreateResult, ChatInfo, ChatMessage, MessageResult,
    MessageSend, User, UserInfo, UserRegisterReq, UserRegisterRes,
};
use prost_types::Timestamp;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{pin::Pin, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};
use uuid::Uuid;

type ChatResult<T> = Result<Response<T>, Status>;
type ChatStream = Pin<Box<dyn Stream<Item = Result<ChatMessage, Status>> + Send>>;
#[derive(Debug, Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {
    type ListenMessagesStream = ChatStream;
    async fn register_user(
        &self,
        request: Request<UserRegisterReq>,
    ) -> Result<Response<UserRegisterRes>, Status> {
        println!("Got a request: {:?}", request);
        let now = SystemTime::now();
        // Convert to a duration since UNIX_EPOCH
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");

        // Create a prost_types::Timestamp
        let timestamp = Timestamp {
            seconds: duration.as_secs() as i64,    // Seconds since epoch
            nanos: duration.subsec_nanos() as i32, // Nanoseconds part
        };

        let request = request.into_inner();
        let reply = UserRegisterRes {
            result: Some(chat::user_register_res::Result::User(User {
                id: Uuid::new_v4().to_string(),
                name: request.name,
                created_at: Some(timestamp),
                deleted_at: None,
                email: request.email,
                picture: request.picture,
            })),
        };
        Ok(Response::new(reply)) // Send back the response
    }

    async fn send_message(
        &self,
        request: Request<MessageSend>,
    ) -> Result<Response<MessageResult>, Status> {
        println!("Got a request: {:?}", request);
        // TODO: get user id from
        let request = request.into_inner();
        let reply = MessageResult {
            result: Some(chat::message_result::Result::Message(ChatMessage {
                message: request.message,
                sender: Some(User {
                    ..Default::default()
                }),
                seen_by: vec![],
                uuid: Uuid::new_v4().to_string(),
                created_at: Some(Timestamp {
                    seconds: 0,
                    nanos: 0,
                }),
                deleted_at: None,
                chat: Some(ChatGroup {
                    ..Default::default()
                }),
            })),
        };
        Ok(Response::new(reply)) // Send back the response
    }
    async fn listen_messages(
        &self,
        req: Request<UserInfo>,
    ) -> ChatResult<Self::ListenMessagesStream> {
        println!("EchoServer::server_streaming_echo");
        println!("\tclient connected from: {:?}", req.remote_addr());
        // creating infinite stream with requested message
        // TODO: get user id from req auth
        let req = req.into_inner();
        let repeat = std::iter::repeat(ChatMessage {
            message: "Hello ".to_string() + &req.id,
            uuid: Uuid::new_v4().to_string(),
            seen_by: vec![],
            chat: Some(ChatGroup {
                ..Default::default()
            }),
            created_at: Some(Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            deleted_at: None,
            sender: Some(User {
                id: "00000000-0000-0000-0000-000000000000".to_string(),
                name: "Server".to_string(),
                created_at: None,
                deleted_at: None,
                email: "".to_string(),
                picture: "".to_string(),
            }),
        });

        let mut stream = Box::pin(tokio_stream::iter(repeat).throttle(Duration::from_millis(200)));

        // spawn and channel are required if you want handle "disconnect" functionality
        // the `out_stream` will not be polled after client disconnect
        let (tx, rx) = mpsc::channel(128);
        tokio::spawn(async move {
            while let Some(item) = stream.next().await {
                match tx.send(Result::<_, Status>::Ok(item)).await {
                    Ok(_) => {
                        // item (server response) was queued to be send to client
                    }
                    Err(_item) => {
                        // output_stream was build from rx and both are dropped
                        break;
                    }
                }
            }
            println!("\tclient disconnected");
        });

        let output_stream = ReceiverStream::new(rx);
        Ok(Response::new(
            Box::pin(output_stream) as Self::ListenMessagesStream
        ))
    }
    async fn get_user(&self, request: Request<UserInfo>) -> Result<Response<User>, Status> {
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        let reply = User {
            id: request.id,
            name: "Server".to_string(),
            created_at: Some(Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            deleted_at: None,
            email: "".to_string(),
            picture: "".to_string(),
        };
        Ok(Response::new(reply)) // Send back the response
    }
    async fn create_chat_group(
        &self,
        request: Request<ChatGroupCreate>,
    ) -> Result<Response<ChatGroupCreateResult>, Status> {
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        let reply = ChatGroupCreateResult {
            result: Some(chat::chat_group_create_result::Result::ChatGroup(
                ChatGroup {
                    id: Uuid::new_v4().to_string(),
                    name: request.name,
                    created_at: Some(Timestamp {
                        seconds: 0,
                        nanos: 0,
                    }),
                    deleted_at: None,
                    members: request.members,
                    picture: request.picture,
                },
            )),
        };

        Ok(Response::new(reply)) // Send back the response
    }
    async fn get_chat_group(
        &self,
        request: Request<ChatInfo>,
    ) -> Result<Response<ChatGroup>, Status> {
        println!("Got a request: {:?}", request);
        let request = request.into_inner();
        let reply = ChatGroup {
            id: Uuid::new_v4().to_string(),
            name: "Server".to_string(),
            created_at: Some(Timestamp {
                seconds: 0,
                nanos: 0,
            }),
            deleted_at: None,
            members: vec![],
            picture: "".to_string(),
        };
        Ok(Response::new(reply)) // Send back the response
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:9090".parse()?;
    let chat = MyChat::default();

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;

    Ok(())
}
