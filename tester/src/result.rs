#[cfg(test)]
mod tests {
    #[test]
    fn it_works() -> Result<(), String> {
        use crate::add::add;
        let result = add(2, 2);
        if result == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}