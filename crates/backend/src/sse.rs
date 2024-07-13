use std::{
    pin::Pin,
    sync::Mutex,
    task::{Context, Poll},
    time::Duration,
};

use actix_web::{
    rt::time,
    web::{Bytes, Data},
};
use futures::{
    channel::mpsc::{channel, Receiver, Sender},
    Stream, StreamExt,
};

#[derive(Debug)]
pub struct Broadcaster {
    clients: Vec<Sender<Bytes>>,
}

impl Broadcaster {
    pub fn create() -> Data<Mutex<Self>> {
        // Data ≃ Arc
        let me = Data::new(Mutex::new(Broadcaster::new()));

        // ping clients every 10 seconds to see if they are alive
        Broadcaster::spawn_ping(me.clone());

        me
    }

    pub fn new() -> Self {
        Broadcaster {
            clients: Vec::new(),
        }
    }

    fn spawn_ping(me: Data<Mutex<Self>>) {
        let mut interval = time::interval(Duration::from_millis(500));
        let task = async move {
            loop {
                interval.tick().await;
                let _ = me.lock().map(|mut res| res.remove_stale_clients());
            }
        };
        tokio::spawn(task);
    }

    fn remove_stale_clients(&mut self) {
        let mut ok_clients = Vec::new();
        for client in self.clients.iter() {
            let result = client.clone().try_send(Bytes::from("data: ping\n\n"));

            if let Ok(()) = result {
                ok_clients.push(client.clone());
            }
        }
        self.clients = ok_clients;
    }

    pub fn new_client(&mut self) -> Client {
        let (tx, rx) = channel(100);

        tx.clone()
            .try_send(Bytes::from("data: connected\n\n"))
            .unwrap();

        self.clients.push(tx);
        Client(rx)
    }

    pub fn send(&self, msg: &str) {
        let msg = unescape::unescape(msg).unwrap();
        let msg = Bytes::from(["data: ", &msg, "\n\n"].concat());

        for client in self.clients.iter().filter(|client| !client.is_closed()) {
            client.clone().try_send(msg.clone()).unwrap();
        }
    }
}

// wrap Receiver in own type, with correct error type
pub struct Client(Receiver<Bytes>);

impl Stream for Client {
    type Item = Result<Bytes, actix_web::Error>;

    fn poll_next(mut self: Pin<&mut Client>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.poll_next_unpin(cx).map(|c| Ok(c).transpose())
    }
}
