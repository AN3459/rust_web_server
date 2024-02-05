// use crate::common::config::config::{POSTGRES_CONNECTION_TIMEOUT, POSTGRES_HOST};
// // use lazy_static::lazy_static;
// use std::time::Duration;


// lazy_static! {
//     pub static ref POOL: PgPool = {
//         // 创建一个异步块，通过 tokio::main 来启动异步运行时
//         let runtime = tokio::runtime::Runtime::new().unwrap();
//         runtime.block_on(async {
//             PgPoolOptions::new()
//                 .acquire_timeout(Duration::from_secs(*POSTGRES_CONNECTION_TIMEOUT))
//                 .connect(&POSTGRES_HOST)
//                 .await
//                 .expect("can't connect to database")
//         })
//     };
// }

// pub struct Pool {
// }
// impl Pool {
//     pub async fn create_pool()->PgPool{
//         PgPoolOptions::new()
//             .acquire_timeout(Duration::from_secs(*POSTGRES_CONNECTION_TIMEOUT))
//             .connect(&POSTGRES_HOST)
//             .await
//             .expect("can't connect to database")
//     }
// }
