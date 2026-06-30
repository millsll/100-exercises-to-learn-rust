use std::sync::mpsc::{Receiver, Sender};

pub mod data;
pub mod store;

pub enum Command {
    Insert(data::TicketDraft),
}

// 启动系统，生成服务器线程。
// 它返回一个 `Sender` 实例，可供一个或多个客户端
// 用来与服务器交互。
pub fn launch() -> Sender<Command> {
    let (sender, receiver) = std::sync::mpsc::channel();
    std::thread::spawn(move || server(receiver));
    sender
}

// TODO: 服务器任务**永远不应**停止。
//  进入一个循环：等待命令在通道中出现，
//  然后执行它，接着继续等待下一个命令。
pub fn server(receiver: Receiver<Command>) {
    loop {
        let command_result = receiver.recv();
        match command_result {
            Ok(command) => {
                match command {
                    Command::Insert(ticket_draft) => {
                        store::TicketStore::new().add_ticket(ticket_draft);
                    }
                }
            }
            Err(_) => {
                continue;
            }
        }
    }
}
