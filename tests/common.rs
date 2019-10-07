pub const ROOT_USERNAME: &str = "root";
pub const ROOT_PASSWORD: &str = "KWNngteTps7XjrNv";

pub const NORMAL_USERNAME: &str = "username";
pub const NORMAL_PASSWORD: &str = "password";

#[test]
pub fn test_setup() {
    match env_logger::Builder::from_default_env()
        .is_test(true)
        .try_init()
    {
        _ => {}
    }
}

#[macro_export]
macro_rules! test_root_and_normal {
    ($t:ident) => {
        $t(crate::common::ROOT_USERNAME, crate::common::ROOT_PASSWORD).await;
        $t(
            crate::common::NORMAL_USERNAME,
            crate::common::NORMAL_PASSWORD,
        )
        .await;
    };
}
