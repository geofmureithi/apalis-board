use actix::prelude::{Message, Recipient};
use actix::{fut, ActorContext, ContextFutureSpawner, WrapFuture};
use actix::{Actor, Running, StreamHandler};
use actix::{ActorFutureExt, Addr};
use actix::{AsyncContext, Handler};
use actix_web_actors::ws;
use actix_web_actors::ws::Message::Text;
use std::time::{Duration, Instant};
use uuid::Uuid;

use self::lobby::Lobby;

#[derive(Message)]
#[rtype(result = "()")]
pub struct WsMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    pub addr: Recipient<WsMessage>,
    pub task: String,
    pub self_id: Uuid,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Disconnect {
    pub id: Uuid,
    pub task: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct ClientActorMessage {
    pub id: Uuid,
    pub msg: String,
    pub task: String,
}

pub mod lobby {
    use actix::prelude::{Actor, Context, Handler, Recipient};
    use std::collections::{HashMap, HashSet};
    use uuid::Uuid;

    use super::*;

    type Socket = Recipient<WsMessage>;

    pub struct Lobby {
        sessions: HashMap<Uuid, Socket>,     //self id to self
        tasks: HashMap<String, HashSet<Uuid>>, //room id  to list of users id
    }

    impl Default for Lobby {
        fn default() -> Lobby {
            Lobby {
                sessions: HashMap::new(),
                tasks: HashMap::new(),
            }
        }
    }

    impl Lobby {
        fn send_message(&self, message: &str, id_to: &Uuid) {
            if let Some(socket_recipient) = self.sessions.get(id_to) {
                let _ = socket_recipient.do_send(WsMessage(message.to_owned()));
            } else {
                println!("attempting to send message but couldn't find user id.");
            }
        }
    }

    impl Actor for Lobby {
        type Context = Context<Self>;
    }

    impl Handler<Disconnect> for Lobby {
        type Result = ();

        fn handle(&mut self, msg: Disconnect, _: &mut Context<Self>) {
            if self.sessions.remove(&msg.id).is_some() {
                self.tasks
                    .get(&msg.task)
                    .unwrap()
                    .iter()
                    .filter(|conn_id| *conn_id.to_owned() != msg.id)
                    .for_each(|user_id| {
                        self.send_message(&format!("{} disconnected.", &msg.id), user_id)
                    });
                if let Some(lobby) = self.tasks.get_mut(&msg.task) {
                    if lobby.len() > 1 {
                        lobby.remove(&msg.id);
                    } else {
                        //only one in the lobby, remove it entirely
                        self.tasks.remove(&msg.task);
                    }
                }
            }
        }
    }

    impl Handler<Connect> for Lobby {
        type Result = ();

        fn handle(&mut self, msg: Connect, _: &mut Context<Self>) -> Self::Result {
            self.tasks
                .entry(msg.task.clone())
                .or_insert_with(HashSet::new)
                .insert(msg.self_id);

            self.tasks
                .get(&msg.task)
                .unwrap()
                .iter()
                .filter(|conn_id| *conn_id.to_owned() != msg.self_id)
                .for_each(|conn_id| {
                    self.send_message(&format!("{} just joined!", msg.self_id), conn_id)
                });

            self.sessions.insert(msg.self_id, msg.addr);

            self.send_message(&format!("your id is {}", msg.self_id), &msg.self_id);
        }
    }

    impl Handler<ClientActorMessage> for Lobby {
        type Result = ();

        fn handle(&mut self, msg: ClientActorMessage, _ctx: &mut Context<Self>) -> Self::Result {
            if msg.msg.starts_with("\\w") {
                if let Some(id_to) = msg.msg.split(' ').collect::<Vec<&str>>().get(1) {
                    self.send_message(&msg.msg, &Uuid::parse_str(id_to).unwrap());
                }
            } else {
                self.tasks
                    .get(&msg.task)
                    .unwrap()
                    .iter()
                    .for_each(|client| self.send_message(&msg.msg, client));
            }
        }
    }
}

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

pub struct WsConn {
    task: String,
    lobby_addr: Addr<Lobby>,
    hb: Instant,
    id: Uuid,
}

impl WsConn {
    pub fn new(task: String, lobby: Addr<Lobby>) -> WsConn {
        WsConn {
            id: Uuid::new_v4(),
            task,
            hb: Instant::now(),
            lobby_addr: lobby,
        }
    }
}

impl Actor for WsConn {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);

        let addr = ctx.address();
        self.lobby_addr
            .send(Connect {
                addr: addr.recipient(),
                task: self.task.clone(),
                self_id: self.id,
            })
            .into_actor(self)
            .then(|res, _, ctx| {
                match res {
                    Ok(_res) => (),
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        self.lobby_addr.do_send(Disconnect {
            id: self.id,
            task: self.task.clone(),
        });
        Running::Stop
    }
}

impl WsConn {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Disconnecting failed heartbeat");
                ctx.stop();
                return;
            }

            ctx.ping(b"hi");
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsConn {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => (),
            Ok(Text(s)) => self.lobby_addr.do_send(ClientActorMessage {
                id: self.id,
                msg: s.to_string(),
                task: self.task.clone(),
            }),
            Err(e) => std::panic::panic_any(e),
        }
    }
}

impl Handler<WsMessage> for WsConn {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}
