mod asc;
mod big_decimal;
mod bigint;
mod log;
mod types_conversion;

use semver::Version;
use wasmer::Memory;
use wasmer::TypedFunction;

#[derive(Clone)]
pub struct Env {
    pub memory: Option<Memory>,
    pub memory_allocate: Option<TypedFunction<i32, i32>>,
    pub api_version: Version,
    pub id_of_type: Option<TypedFunction<u32, u32>>,
    pub arena_start_ptr: i32,
    pub arena_free_size: i32,
}

#[cfg(test)]
mod test {
    use super::asc::test::UnitTestHost;
    use super::big_decimal;
    use super::bigint;
    use super::log as host_log;
    use super::types_conversion;
    use super::Env;
    use crate::global;
    use crate::store;
    use log;
    use semver::Version;
    use std::env;
    use wasmer::imports;
    use wasmer::Function;
    use wasmer::FunctionEnv;
    use wasmer::Instance;
    use wasmer::Module;
    use wasmer::Store;

    pub fn create_mock_host_instance(
        wasm_path: &str,
    ) -> Result<UnitTestHost, Box<dyn std::error::Error>> {
        let wasm_bytes = std::fs::read(wasm_path)?;
        let mut store = Store::default();

        let module = Module::new(&store, wasm_bytes)?;
        let api_version = Version::parse(
            env::var("RUNTIME_API_VERSION")
                .unwrap_or("0.0.5".to_string())
                .as_str(),
        )
        .unwrap();

        log::warn!("Init WASM Instance with api-version={api_version}");

        let env = FunctionEnv::new(
            &mut store,
            Env {
                memory: None,
                memory_allocate: None,
                id_of_type: None,
                api_version: api_version.clone(),
                arena_start_ptr: 0,
                arena_free_size: 0,
            },
        );

        // Global
        let abort = Function::new(&mut store, global::ABORT_TYPE, global::abort);

        // Conversion functions

        // Store functions
        let store_set = Function::new(
            &mut store,
            store::STORE_SET_TYPE,
            // TODO: fix implementation
            store::store_set,
        );

        let store_get = Function::new(
            &mut store,
            store::STORE_GET_TYPE,
            // TODO: fix implementation
            store::store_get,
        );

        let import_object = imports! {
            "env" => {
                "abort" => abort,
            },
            "conversion" => {
                "typeConversion.bytesToString" => Function::new_typed_with_env(&mut store, &env, types_conversion::bytes_to_string),
                "typeConversion.bytesToHex" => Function::new_typed_with_env(&mut store, &env, types_conversion::bytes_to_hex),
                "typeConversion.bigIntToString" => Function::new_typed_with_env(&mut store, &env, types_conversion::big_int_to_string),
                "typeConversion.bigIntToHex" => Function::new_typed_with_env(&mut store, &env, types_conversion::big_int_to_hex),
                "typeConversion.stringToH160" => Function::new_typed_with_env(&mut store, &env, types_conversion::string_to_h160),
                "typeConversion.bytesToBase58" => Function::new_typed_with_env(&mut store, &env, types_conversion::bytes_to_base58),
            },
            "numbers" => {
                "bigInt.plus" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_plus),
                "bigInt.minus" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_minus),
                "bigInt.times" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_times),
                "bigInt.dividedBy" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_divided_by),
                "bigInt.dividedByDecimal" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_divided_by_decimal),
                "bigInt.pow" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_pow),
                "bigInt.mod" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_mod),
                "bigInt.fromString" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_from_string),
                "bigInt.bitOr" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_bit_or),
                "bigInt.bitAnd" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_bit_and),
                "bigInt.leftShift" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_left_shift),
                "bigInt.rightShift" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_right_shift),
                //Big Decimal
                "bigDecimal.fromString" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_from_string),
                "bigDecimal.toString" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_to_string),
                "bigDecimal.plus" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_plus),
                "bigDecimal.minus" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_minus),
                "bigDecimal.times" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_times),
                "bigDecimal.dividedBy" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_divided_by),
                "bigDecimal.equals" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_equals),
            },
            "index" => {
                "store.set" => store_set,
                "store.get" => store_get,
                "log.log" => Function::new_typed_with_env(&mut store, &env, host_log::log_log),
                "bigInt.plus" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_plus),
                "bigInt.minus" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_minus),
                "bigInt.minus" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_minus),
                "bigInt.times" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_times),
                "bigInt.dividedBy" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_divided_by),
                "bigInt.dividedByDecimal" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_divided_by_decimal),
                "bigInt.pow" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_pow),
                "bigInt.mod" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_mod),
                "bigInt.fromString" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_from_string),
                "bigInt.bitOr" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_bit_or),
                "bigInt.bitAnd" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_bit_and),
                "bigInt.leftShift" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_left_shift),
                "bigInt.rightShift" => Function::new_typed_with_env(&mut store, &env, bigint::big_int_right_shift),
                //Big Decimal
                "bigDecimal.fromString" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_from_string),
                "bigDecimal.toString" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_to_string),
                "bigDecimal.plus" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_plus),
                "bigDecimal.minus" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_minus),
                "bigDecimal.times" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_times),
                "bigDecimal.dividedBy" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_divided_by),
                "bigDecimal.equals" => Function::new_typed_with_env(&mut store, &env, big_decimal::big_decimal_equals),
            }
        };
        // Running cargo-run will immediately tell which functions are missing

        let instance = Instance::new(&mut store, &module, &import_object)?;

        // Bind guest memory ref & __alloc to env
        let mut env_mut = env.into_mut(&mut store);
        let (data_mut, mut store_mut) = env_mut.data_and_store_mut();

        data_mut.memory = Some(instance.exports.get_memory("memory")?.clone());
        data_mut.memory_allocate = match api_version.clone() {
            version if version <= Version::new(0, 0, 4) => instance
                .exports
                .get_typed_function(&store_mut, "memory.allocate")
                .ok(),
            _ => instance
                .exports
                .get_typed_function(&store_mut, "allocate")
                .ok(),
        };

        if data_mut.memory_allocate.is_none() {
            log::warn!("MemoryAllocate function is not available in host-exports");
        }

        data_mut.id_of_type = match api_version.clone() {
            version if version <= Version::new(0, 0, 4) => None,
            _ => instance
                .exports
                .get_typed_function(&store_mut, "id_of_type")
                .ok(),
        };

        if data_mut.id_of_type.is_none() {
            log::warn!("id_of_type function is not available in host-exports");
        }

        match data_mut.api_version.clone() {
            version if version <= Version::new(0, 0, 4) => {}
            _ => {
                log::warn!("Try calling `_start` if possible");
                instance
                    .exports
                    .get_function("_start")
                    .map(|f| {
                        log::info!("Calling `_start`");
                        f.call(&mut store_mut, &[]).unwrap();
                    })
                    .ok();
            }
        }

        let memory = instance.exports.get_memory("memory")?.clone();
        let id_of_type = data_mut.id_of_type.clone().unwrap();

        Ok(UnitTestHost {
            store,
            instance,
            api_version,
            memory,
            id_of_type,
        })
    }
}
