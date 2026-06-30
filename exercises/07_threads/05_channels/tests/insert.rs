// TODO: 当你完成本练习时，将 `ready` 中的 `move_forward` 设置为 `true`。
//  可以随时呼叫导师来验证你的解决方案！
use channels::data::TicketDraft;
use channels::{launch, Command};
use std::time::Duration;
use ticket_fields::test_helpers::{ticket_description, ticket_title};

#[test]
fn a_thread_is_spawned() {
    let sender = launch();
    std::thread::sleep(Duration::from_millis(200));

    sender
        .send(Command::Insert(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        }))
        // 如果线程已经不再运行，这将触发 panic，
        // 因为通道已经关闭。
        .expect("Did you actually spawn a thread? The channel is closed!");
}

#[test]
fn ready() {
    // 本练习中我们能自动验证的内容很少，
    // 因为服务器没有暴露任何**读取**操作。
    // 我们无法知道插入是否真的发生了，以及是否正确执行。
    let move_forward = true;

    assert!(move_forward);
}
