// TODO: 添加必要的 `Clone` 实现（和调用）
//  以使代码能够编译。

pub fn summary(ticket: Ticket) -> (Ticket, Summary) {
    let new_ticket=ticket.clone();
    (ticket, new_ticket.summary())
}
#[derive(Clone)]
pub struct Ticket {
    pub title: String,
    pub description: String,
    pub status: String,
}

impl Ticket {
    pub fn summary(self) -> Summary {
        Summary {
            title: self.title,
            status: self.status,
        }
    }
}

pub struct Summary {
    pub title: String,
    pub status: String,
}
