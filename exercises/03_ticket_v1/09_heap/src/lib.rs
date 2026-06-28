pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: 根据你在本节学到的知识，将 `todo!()` 替换为
//  相应类型的正确**栈大小**。
#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 8+8+8);
    }

    #[test]
    fn ticket_size() {
        // 这是一个棘手的问题！
        // 这次"直觉"答案恰好是正确答案，
        // 但一般来说，结构体的内存布局是一个更复杂的话题。
        // 如果你感兴趣，可以查看 Rust Reference 的"Type layout"章节
        // https://doc.rust-lang.org/reference/type-layout.html 获取更多信息。
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
