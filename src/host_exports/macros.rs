#[macro_export]
macro_rules! host_fn_test {
    ($guest_func:ident, $host:ident, $ptr:ident $body:block) => {
        #[::rstest::rstest]
        #[case("0.0.3")]
        #[case("0.0.4")]
        #[case("0.0.5")]
        fn $guest_func(#[case] version: &str) {
            use convert_case::Case;
            use convert_case::Casing;
            use env_logger;
            use std::env;

            env::set_var("SUBGRAPH_WASM_RUNTIME_TEST", "YES");

            env_logger::try_init().unwrap_or_default();
            let (version, wasm_path) = version_to_test_resource(version);

            let mut $host = mock_host_instance(version, &wasm_path);
            let wasm_test_func_name = format!("{}", stringify!($guest_func).to_case(Case::Camel));
            let func = $host
                .instance
                .exports
                .get_function(&wasm_test_func_name)
                .expect(&format!(
                    "No function with name `{wasm_test_func_name}` exists!",
                ));

            let result = func
                .call(&mut $host.store, &[])
                .expect("Calling function failed!");
            let $ptr = result.first().unwrap().unwrap_i32();

            $body
        }
    };
    ($guest_func:ident, $host:ident $body:block) => {
        #[::rstest::rstest]
        #[case("0.0.3")]
        #[case("0.0.4")]
        #[case("0.0.5")]
        fn $guest_func(#[case] version: &str) {
            use convert_case::Case;
            use convert_case::Casing;
            use env_logger;

            env_logger::try_init().unwrap_or_default();
            let (version, wasm_path) = version_to_test_resource(version);

            let mut $host = mock_host_instance(version, &wasm_path);
            let wasm_test_func_name = format!("{}", stringify!($guest_func).to_case(Case::Camel));
            let func = $host
                .instance
                .exports
                .get_function(&wasm_test_func_name)
                .expect(&format!(
                    "No function with name `{wasm_test_func_name}` exists!",
                ));

            let result = func
                .call(&mut $host.store, &[])
                .expect("Calling function failed!");
            assert!(result.is_empty());
            $body
        }
    };
}