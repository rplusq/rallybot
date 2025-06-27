/// Macro to create tests that run with both InMemory and Postgres storage
#[macro_export]
macro_rules! test_with_both_storages {
    ($test_name:ident, $test_fn:expr) => {
        paste::paste! {
            #[tokio::test]
            async fn [<$test_name _in_memory>]() {
                let app = helpers::TestApp::with_in_memory().await;
                $test_fn(app).await;
            }
            
            #[tokio::test]
            #[serial_test::serial]
            async fn [<$test_name _postgres>]() {
                let app = helpers::TestApp::with_postgres().await;
                let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    let rt = tokio::runtime::Runtime::new().unwrap();
                    rt.block_on(async {
                        $test_fn(app).await;
                    })
                }));
                
                // Cleanup happens in TestApp drop
                if let Err(e) = result {
                    std::panic::resume_unwind(e);
                }
            }
        }
    };
}