// TODO: 使用两个变体，一个用于 title 错误，一个用于 description 错误。
//   每个变体应该包含一个字符串，准确说明哪里出了问题。
//   你还需要更新 `Ticket::new` 的实现。
#[derive(Debug)]
enum TicketNewError {
    TitleError(String),
    DescriptionError(String),
}

// TODO: `easy_ticket` 应该在 title 无效时 panic，使用存储在
//   `TicketNewError` 枚举相关变体中的错误消息。
//   而当 description 无效时，它应该使用默认的 description：
//   "Description not provided"。
fn easy_ticket(title: String, description: String, status: Status) -> Ticket {
    match Ticket::new(title.clone(),description,status.clone()){
        Ok(ticket)=>ticket,
        Err(err)=>{
            match err{
                TicketNewError::TitleError(msg)=>panic!("{msg}"),
                TicketNewError::DescriptionError(msg)=>return Ticket::new(title,"Description not provided".to_string(),status).unwrap(),
            }
        }
    }
}

#[derive(Debug, PartialEq)]
struct Ticket {
    title: String,
    description: String,
    status: Status,
}

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress { assigned_to: String },
    Done,
}

impl Ticket {
    pub fn new(
        title: String,
        description: String,
        status: Status,
    ) -> Result<Ticket, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleError("Title cannot be empty".to_string()));
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleError("Title cannot be longer than 50 bytes".to_string()));
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionError("Description cannot be empty".to_string()));
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionError("Description cannot be longer than 500 bytes".to_string()));
        }

        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use common::{overly_long_description, overly_long_title, valid_description, valid_title};

    #[test]
    #[should_panic(expected = "Title cannot be empty")]
    fn title_cannot_be_empty() {
        easy_ticket("".into(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_empty() {
        let ticket = easy_ticket(valid_title(), "".into(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }

    #[test]
    #[should_panic(expected = "Title cannot be longer than 50 bytes")]
    fn title_cannot_be_longer_than_fifty_chars() {
        easy_ticket(overly_long_title(), valid_description(), Status::ToDo);
    }

    #[test]
    fn template_description_is_used_if_too_long() {
        let ticket = easy_ticket(valid_title(), overly_long_description(), Status::ToDo);
        assert_eq!(ticket.description, "Description not provided");
    }
}
