#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        // 测试
        let res = defang_i_paddr(String::from("1.1.1.1"));
        println!("{}", res);
        // 实现
        fn defang_i_paddr(address: String) -> String {
            let mut ans = String::from("");
            for &c in address.as_bytes().iter() {
                if c == ('.' as u8) {
                    ans.push_str("[.]")
                } else {
                    ans.push(c as char)
                }
            }
            ans
        }
    }
}