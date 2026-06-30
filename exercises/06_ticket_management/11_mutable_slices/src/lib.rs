// TODO: 定义一个名为 `squared` 的函数，将切片中的所有 `i32` 平方。
//  切片应该被就地修改。
pub fn squared(slice:&mut[i32]){
    let n:usize=slice.len();
    for i in 0..n{
        slice[i]=slice[i]*slice[i];
    }
    //slice.iter_mut().for_each(|x|*x=*x*(*x));
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut s = vec![];
        squared(&mut s);
        assert_eq!(s, vec![]);
    }

    #[test]
    fn one() {
        let mut s = [2];
        squared(&mut s);
        assert_eq!(s, [4]);
    }

    #[test]
    fn multiple() {
        let mut s = vec![2, 4];
        squared(&mut s);
        assert_eq!(s, vec![4, 16]);
    }
}
