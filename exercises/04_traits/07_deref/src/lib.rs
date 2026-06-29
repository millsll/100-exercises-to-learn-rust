// TODO: 当 `title` 和 `description` 通过其访问器方法返回时，
//   它们应该被规范化——即去除前导和尾随的空白字符。
//   Rust 标准库中有一个方法可以帮助你完成这个任务，但你不会
//   在 `String` 的文档中找到它。
//   你能找出它在哪里定义以及如何使用吗？

pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

impl Ticket {
    pub fn title(&self) -> &str {
        self.title.trim()
    }

    pub fn description(&self) -> &str {
        self.description.trim()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalization() {
        let ticket = Ticket {
            title: "   A title ".to_string(),
            description: " A description   ".to_string(),
            status: "To-Do".to_string(),
        };

        assert_eq!("A title", ticket.title());
        assert_eq!("A description", ticket.description());
    }
}
