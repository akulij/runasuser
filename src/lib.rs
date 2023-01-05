pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_login() {
        let username = "test";
        let pass = "pass";
        let ret = runcmd_login(username, pass, &vec!["touch", "testrunfile.txt"]);
        match ret {
            Ok(_) => (),
            Err(code) => panic!("Failed test with code {}", code.to_string())
        };
    }
}
