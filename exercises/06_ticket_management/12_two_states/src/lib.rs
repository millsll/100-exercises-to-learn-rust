// TODO: 更新 `add_ticket` 的签名：它应该接收一个 `TicketDraft` 作为输入，
//  并返回一个 `TicketId` 作为输出。
//  每个 ticket 都应该有一个由 `TicketStore` 生成的唯一 id。
//  如有需要，可以自由修改 `TicketStore` 的字段。
//
// 你还需要添加一个 `get` 方法，它接收一个 `TicketId` 作为输入，
// 并返回一个 `Option<&Ticket>`。

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
    counter: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TicketId(u64);

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub id: TicketId,
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TicketDraft {
    pub title: TicketTitle,
    pub description: TicketDescription,
}

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

impl TicketStore {
    pub fn new() -> Self {
        Self {
            tickets: Vec::new(),
            counter:0,
        }
    }

    pub fn add_ticket(&mut self, ticket_draft: TicketDraft)->TicketId {
        self.counter+=1;
        let ticket_id=TicketId(self.counter);
        let ticket=Ticket{
            id:ticket_id.clone(),
            title:ticket_draft.title,
            description:ticket_draft.description,
            status:Status::ToDo,
        };
        self.tickets.push(ticket);
        ticket_id
    }

    pub fn get(&self, id: TicketId) -> Option<&Ticket> {
        self.tickets.iter().find(|&t| t.id == id)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Status, TicketDraft, TicketStore};
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    #[test]
    fn works() {
        let mut store = TicketStore::new();

        let draft1 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id1 = store.add_ticket(draft1.clone());
        let ticket1 = store.get(id1).unwrap();
        assert_eq!(draft1.title, ticket1.title);
        assert_eq!(draft1.description, ticket1.description);
        assert_eq!(ticket1.status, Status::ToDo);

        let draft2 = TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        };
        let id2 = store.add_ticket(draft2);
        let ticket2 = store.get(id2).unwrap();

        assert_ne!(id1, id2);
    }
}
