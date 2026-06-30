use crate::status::{Status, ParseStatusError};

// 我们在早期的练习中见过如何声明模块，但没有见过如何将它们提取到单独的文件中。
// 现在让我们来修复这个问题！
//
// 在最简单的情况下，当被提取的模块是一个单独的文件时，只需创建一个与模块同名的文件，
// 并将模块内容移动到那里即可。
// 模块文件应该放在声明该模块的文件的同一目录中。
// 在这个例子中，是 `src/lib.rs`，因此 `status.rs` 应该放在 `src` 目录中。
mod status;

// TODO: 为 `TicketNewError` 添加一个新的错误变体，用于处理 status 字符串无效的情况。
//   当在该变体的错误上调用 `source` 时，它应该返回 `ParseStatusError` 而不是 `None`。

#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("Title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("Title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("Description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("Description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error("{0}")]
    InvalidStatus(#[from]ParseStatusError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Ticket {
    title: String,
    description: String,
    status: Status,
}

impl Ticket {
    pub fn new(title: String, description: String, status: String) -> Result<Self, TicketNewError> {
        if title.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if title.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        if description.is_empty() {
            return Err(TicketNewError::DescriptionCannotBeEmpty);
        }
        if description.len() > 500 {
            return Err(TicketNewError::DescriptionTooLong);
        }

        // TODO: 将 status 字符串解析为 `Status` 枚举。
        let status =Status::try_from(status)?;
        Ok(Ticket {
            title,
            description,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use common::{valid_description, valid_title};
    use std::error::Error;

    use super::*;

    #[test]
    fn invalid_status() {
        let err = Ticket::new(valid_title(), valid_description(), "invalid".into()).unwrap_err();
        assert_eq!(
            err.to_string(),
            "`invalid` is not a valid status. Use one of: ToDo, InProgress, Done"
        );
        assert!(err.source().is_some());
    }
}
