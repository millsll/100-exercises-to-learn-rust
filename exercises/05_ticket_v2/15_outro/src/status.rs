// TODO: 为 `Status` 枚举实现 `TryFrom<String>` 和 `TryFrom<&str>`。
//  解析应该是大小写不敏感的。
#[derive(Debug,PartialEq,Clone)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}
#[derive(Debug,thiserror::Error)]
#[error("{invalid_status}")]
pub struct ParseStatusError{
    invalid_status:String,
}

impl TryFrom<String>for Status{
    type Error = ParseStatusError;
    fn try_from(value:String)->Result<Self,Self::Error>{
        if value.to_lowercase() == "todo".to_string(){
            return Ok(Self::ToDo);
        }
        if value.to_lowercase() == "inprogress".to_string(){
            return Ok(Self::InProgress);
        }
        if value.to_lowercase() == "done".to_string(){
            return Ok(Self::Done);
        }
        Err(ParseStatusError{
            invalid_status:value,
        })
    }
}

impl TryFrom<&str> for Status{
    type Error = ParseStatusError;
    fn try_from(value:&str)->Result<Self,Self::Error>{
        let status = value.to_lowercase();
        if status == "todo".to_string(){
            return Ok(Self::ToDo);
        }
        if status == "inprogress".to_string(){
            return Ok(Self::InProgress);
        }
        if status == "done".to_string(){
            return Ok(Self::Done);
        }
        Err(ParseStatusError{
            invalid_status:value.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("ToDO").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_invalid() {
        let status = Status::try_from("Invalid");
        assert!(status.is_err());
    }
}
