mod ticket {
    pub struct Ticket {
        title: String,
        description: String,
        status: String,
    }

    impl Ticket {
        pub fn new(title: String, description: String, status: String) -> Ticket {
            if title.is_empty() {
                panic!("Title cannot be empty");
            }
            if title.len() > 50 {
                panic!("Title cannot be longer than 50 bytes");
            }
            if description.is_empty() {
                panic!("Description cannot be empty");
            }
            if description.len() > 500 {
                panic!("Description cannot be longer than 500 bytes");
            }
            if status != "To-Do" && status != "In Progress" && status != "Done" {
                panic!("Only `To-Do`, `In Progress`, and `Done` statuses are allowed");
            }

            Ticket {
                title,
                description,
                status,
            }
        }
    }
}

// TODO: **例外情况下**，你将同时修改 `ticket` 模块和 `tests` 模块
//  这个练习中。
#[cfg(test)]
mod tests {
    // TODO: 在父模块中添加必要的 `pub` 修饰符，以消除关于下方 use 语句的
    //  编译器错误。
    use super::ticket::Ticket;

    // 但要小心！我们不希望在你修改可见性以使 use 语句编译之后，
    // 这个函数仍然能够编译！
    // 一旦你确认它确实无法编译，就将其注释掉。
    fn should_not_be_possible() {
        let ticket = Ticket::new("A title".into(), "A description".into(), "To-Do".into());

        // 当你尝试运行这个练习时，应该会看到以下错误：
        //
        // error[E0616]: field `description` of struct `Ticket` is private
        //    |
        //    |              assert_eq!(ticket.description, "A description");
        //    |                         ^^^^^^^^^^^^^^^^^^
        //
        // TODO: 一旦你确认下面的代码无法编译，
        //   就将其注释掉以继续下一个练习！
        //assert_eq!(ticket.description, "A description");
    }

    fn encapsulation_cannot_be_violated() {
        // 这也应该是不可能的，会遇到与上面类似的错误。
        // （只有在你注释掉前一个测试中的错误行之后，
        // 才会抛出编译错误——下一个编译阶段！）
        //
        // 这证明了 `Ticket::new` 现在是获取 `Ticket` 实例的唯一方式。
        // 不可能用非法的 title 或 description 创建 ticket！
        //
        // TODO: 一旦你确认下面的代码无法编译，
        //   就将其注释掉以继续下一个练习！
        //let ticket = Ticket {
        //    title: "A title".into(),
        //    description: "A description".into(),
        //    status: "To-Do".into(),
        //};
    }
}
