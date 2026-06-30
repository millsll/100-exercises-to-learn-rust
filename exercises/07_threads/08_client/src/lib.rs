use crate::data::{Ticket, TicketDraft};
use crate::store::{TicketId, TicketStore};
use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

#[derive(Clone)]
// TODO: 充实客户端的实现。
pub struct TicketStoreClient {
    sender: Sender<Command>,
}

impl TicketStoreClient {
    // 为简单起见，可以在所有错误上直接 panic。
    pub fn insert(&self, draft: TicketDraft) -> TicketId {
        let (response_sender,response_receiver)=std::sync::mpsc::channel();
        self.sender.send(Command::Insert {
            draft,
            response_channel: response_sender,
        }).unwrap();
        response_receiver.recv().unwrap()
    }

    pub fn get(&self, id: TicketId) -> Option<Ticket> {
        let (response_sender,response_receiver)=std::sync::mpsc::channel();
        self.sender.send(Command::Get {
            id,
            response_channel: response_sender,
        }).unwrap();
        response_receiver.recv().unwrap()
    }
}

pub fn launch() -> TicketStoreClient {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    TicketStoreClient {
        sender,
    }
}

// 不再公开！这现在成为库的内部细节了。
enum Command {
    Insert {
        draft: TicketDraft,
        response_channel: Sender<TicketId>,
    },
    Get {
        id: TicketId,
        response_channel: Sender<Option<Ticket>>,
    },
}

fn server(receiver: Receiver<Command>) {
    let mut store = TicketStore::new();
    loop {
        match receiver.recv() {
            Ok(Command::Insert {
                draft,
                response_channel,
            }) => {
                let id = store.add_ticket(draft);
                let _ = response_channel.send(id);
            }
            Ok(Command::Get {
                id,
                response_channel,
            }) => {
                let ticket = store.get(id);
                let _ = response_channel.send(ticket.cloned());
            }
            Err(_) => {
                // 没有更多的发送者了，因此我们可以安全地退出
                // 并关闭服务器。
                break;
            }
        }
    }
}
