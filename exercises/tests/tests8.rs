// tests8.rs
//
// This execrise shares `build.rs` with the previous exercise.
// You need to add some code to `build.rs` to make both this exercise and
// the previous one work.
//
// Execute `rustlings hint tests8` or use the `hint` watch subcommand for a
// hint.



fn main() {
    // 获取当前时间戳
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // 将时间戳设置为环境变量 TEST_FOO
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 检查环境变量 TEST_PASS 是否存在
    if std::env::var("TEST_PASS").is_ok() {
        // 如果存在，设置特征 pass
        println!("cargo:rustc-cfg=pass");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        #[cfg(feature = "pass")]
        return;

        panic!("no cfg set");
    }
}
