use std::{collections::HashMap, time::Instant};

use actix::prelude::*;
use actix_web_actors::ws::{self, WebsocketContext};
use rand::{prelude::ThreadRng, Rng};
use actix::prelude::*;
use actix_broker::{BrokerSubscribe, BrokerIssue, SystemBroker, ArbiterBroker, Broker};
use actix_http::ws::ProtocolError;
use actix_web::web::{Data, Payload};
use actix_web::HttpRequest;
use actix_web::{get, HttpResponse};
#[derive(Message)]
#[rtype(result = "()")]
pub struct Message(pub String);

#[derive(Message, Clone)]
#[rtype(result = "()")]
pub struct SendMessage {
    pub name: String,
    pub id: usize,
    pub content: String,
}

#[derive(Clone, Message)]
#[rtype(result = "usize")]
pub struct JoinSession(pub Recipient<Message>);

#[derive(Clone, Message)]
#[rtype(result = "()")]
pub struct RemoveSession(pub usize);

pub struct WebSocketSession {
    id: usize,
    name: String,
    heart_beat: Instant,
}
impl Actor for WebSocketSession {
    type Context = WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let join_session = JoinSession(ctx.address().recipient());
        WebSocketServer::from_registry()
            .send(join_session)
            .into_actor(self)
            .then(|id, act, _ctx| {
                if let Ok(id) = id {
                    act.id = id;
                }

                fut::ready(())
            })
            .wait(ctx);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let remove_session = RemoveSession(self.id);
        self.issue_system_async(remove_session);
    }
}

impl WebSocketSession {
    pub fn new() -> Self {
        println!("new session");
        Self {
            id: 0,
            name: "Main".to_owned(),
            heart_beat: Instant::now(),
        }
    }

    pub fn send_msg(&self, msg: String) {
        let msg = SendMessage {
            name: self.name.clone(),
            id: self.id,
            content: msg,
        };

        self.issue_system_async(msg);
    }
}

impl StreamHandler<Result<ws::Message, ProtocolError>> for WebSocketSession {
    fn handle(&mut self, item: Result<ws::Message, ProtocolError>, ctx: &mut Self::Context) {
        match item {
            Ok(ws::Message::Ping(msg)) => {
                self.heart_beat = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_msg)) => {
                self.heart_beat = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                self.send_msg(text);
            }
            Ok(ws::Message::Binary(_)) => println!("Unexpected binary"),
            _ => ctx.stop(),
        }
    }
}
impl Handler<Message> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
#[derive(Default)]
pub struct WebSocketServer {
    sessions: HashMap<usize, Recipient<Message>>,
    rng: ThreadRng,
}

impl WebSocketServer {
    pub fn add_session(&mut self, client: Recipient<Message>) -> usize {
        let id: usize = self.rng.gen();

        self.sessions.insert(id, client);

        id
    }
    pub fn send_message(&mut self, message: String) {
        for (_id, recipient) in &self.sessions {
            recipient
                .do_send(Message(message.to_owned()))
                .expect("Could not send message to the client.");
        }
    }
    pub fn remove_session(&mut self, session_id: usize) {
        self.sessions.remove(&session_id);
    }
}

impl Actor for WebSocketServer {
    type Context = Context<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<SendMessage>(ctx);
    }
    fn stopped(&mut self, ctx: &mut Self::Context) {
        self.subscribe_system_async::<RemoveSession>(ctx);
    }
}

impl Handler<JoinSession> for WebSocketServer {
    type Result = MessageResult<JoinSession>;

    fn handle(&mut self, msg: JoinSession, _ctx: &mut Self::Context) -> Self::Result {
        let JoinSession(client) = msg;

        let id = self.add_session(client);

        MessageResult(id)
    }
}
impl Handler<SendMessage> for WebSocketServer {
    type Result = ();
    fn handle(&mut self, msg: SendMessage, _ctx: &mut Self::Context) {
        self.send_message(msg.content);
    }
}
impl Handler<RemoveSession> for WebSocketServer {
    type Result = MessageResult<RemoveSession>;
    fn handle(&mut self, msg: RemoveSession, _ctx: &mut Self::Context) -> Self::Result {
        let RemoveSession(id) = msg;
        self.remove_session(id);
        MessageResult(())
    }
}
impl SystemService for WebSocketServer {}
impl Supervised for WebSocketServer {}