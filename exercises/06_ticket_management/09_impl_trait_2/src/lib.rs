// TODO: 重新设计 `TicketStore::add_ticket` 的签名，使用泛型类型参数而不是
//  `impl Trait` 语法。

use ticket_fields::{TicketDescription, TicketTitle};

#[derive(Clone)]
pub struct TicketStore {
    tickets: Vec<Ticket>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ticket {
    pub title: TicketTitle,
    pub description: TicketDescription,
    pub status: Status,
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
        }
    }

    // 使用 `Into<Ticket>` 作为 `ticket` 的类型参数，允许该方法接受任何可以
    // 不可变地转换为 `Ticket` 的类型。
    // 这可以让方法的使用更加友好，因为它消除了调用端 `.into()` 的语法噪音。
    // 不过，它可能会降低编译器错误消息的质量。
    pub fn add_ticket<T:Into<Ticket>>(&mut self, ticket: T) {
        self.tickets.push(ticket.into());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ticket_fields::test_helpers::{ticket_description, ticket_title};

    struct TicketDraft {
        pub title: TicketTitle,
        pub description: TicketDescription,
    }

    impl From<TicketDraft> for Ticket {
        fn from(draft: TicketDraft) -> Self {
            Self {
                title: draft.title,
                description: draft.description,
                status: Status::ToDo,
            }
        }
    }

    #[test]
    fn generic_add() {
        let mut store = TicketStore::new();
        // 如果 `add_ticket` 使用 `impl Trait` 语法，这行代码将无法编译。
        store.add_ticket::<TicketDraft>(TicketDraft {
            title: ticket_title(),
            description: ticket_description(),
        });
    }
}
