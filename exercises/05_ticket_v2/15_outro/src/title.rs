// TODO: 为 `TicketTitle` 类型实现 `TryFrom<String>` 和 `TryFrom<&str>`，
//   确保 title 不为空且不超过 50 字节。
//   也要实现使测试通过所需的 trait。
#[derive(Debug,PartialEq,Clone)]
pub struct TicketTitle(String);
#[derive(Debug,thiserror::Error)]
#[error("{invalid_title}")]
pub struct TicketTitleError{
    invalid_title:String,
}

impl TryFrom<String>for TicketTitle{
    type Error = TicketTitleError;
    fn try_from(value:String)->Result<Self,Self::Error>{
        if value.is_empty(){
            return Err(TicketTitleError{
                invalid_title: "The title cannot be empty".to_string(),
            });
        }
        if value.len() > 50{
            return Err(TicketTitleError{
                invalid_title: "The title cannot be longer than 50 bytes".to_string(),
            });
        }
        Ok(Self(value))
    }
}

impl TryFrom<&str> for TicketTitle{
    type Error=TicketTitleError;
    fn try_from(value:&str)->Result<Self,Self::Error>{
        let title = value.to_string();
        Self::try_from(title)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
