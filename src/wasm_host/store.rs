use super::Env;
use crate::asc::base::asc_get;
use crate::asc::base::asc_new;
use crate::asc::base::AscPtr;
use crate::asc::native_types::array::Array;
use crate::asc::native_types::string::AscString;
use crate::asc::native_types::typed_map::AscEntity;
use crate::internal_messages::StoreOperationMessage;
use crate::internal_messages::StoreRequestResult;
use wasmer::FunctionEnvMut;
use wasmer::RuntimeError;

pub fn store_set(
    fenv: FunctionEnvMut<Env>,
    entity_type_ptr: AscPtr<AscString>,
    entity_id_ptr: AscPtr<AscString>,
    data_ptr: AscPtr<AscEntity>,
) -> Result<(), RuntimeError> {
    let env = fenv.data();
    let db = env.db_agent.clone().unwrap();
    let entity_id: String = asc_get(&fenv, entity_id_ptr, 0)?;
    let data = asc_get(&fenv, data_ptr, 0)?;
    let entity_type: String = asc_get(&fenv, entity_type_ptr, 0)?;

    // FIXME: Update or insert new
    let request = StoreOperationMessage::Update((entity_type, entity_id, data));
    let _result = db
        .send_store_request(request)
        .map_err(|e| RuntimeError::new(e.to_string()))?;

    Ok(())
}

pub fn store_get(
    mut fenv: FunctionEnvMut<Env>,
    entity_type_ptr: AscPtr<AscString>,
    entity_id_ptr: AscPtr<AscString>,
) -> Result<AscPtr<AscEntity>, RuntimeError> {
    let entity_type: String = asc_get(&fenv, entity_type_ptr, 0)?;
    let entity_id: String = asc_get(&fenv, entity_id_ptr, 0)?;
    let env = fenv.data();
    let db = env.db_agent.clone().unwrap();
    let request = StoreOperationMessage::Load((entity_type, entity_id));
    let result = db
        .send_store_request(request)
        .map_err(|e| RuntimeError::new(e.to_string()))?;
    match result {
        StoreRequestResult::Load(data) => {
            let asc_result = asc_new(&mut fenv, &data.into_iter().collect::<Vec<_>>())?;
            Ok(asc_result)
        }
        other => Err(RuntimeError::new(format!(
            "Load entity failed, recevied response: {:?}",
            other
        ))),
    }
}

pub fn store_remove(
    fenv: FunctionEnvMut<Env>,
    entity_type_ptr: AscPtr<AscString>,
    entity_id_ptr: AscPtr<AscString>,
) -> Result<(), RuntimeError> {
    let env = fenv.data();
    let db = env.db_agent.clone().unwrap();
    let entity_id: String = asc_get(&fenv, entity_id_ptr, 0)?;
    let entity_type: String = asc_get(&fenv, entity_type_ptr, 0)?;

    // FIXME: Update or insert new
    let request = StoreOperationMessage::Delete((entity_type, entity_id));
    let _result = db
        .send_store_request(request)
        .map_err(|e| RuntimeError::new(e.to_string()))?;

    Ok(())
}

pub fn store_get_in_block(
    fenv: FunctionEnvMut<Env>,
    entity_type_ptr: AscPtr<AscString>,
    entity_id_ptr: AscPtr<AscString>,
) -> Result<AscPtr<AscEntity>, RuntimeError> {
    let env = fenv.data();
    let db = env.db_agent.clone().unwrap();
    let entity_id: String = asc_get(&fenv, entity_id_ptr, 0)?;
    let entity_type: String = asc_get(&fenv, entity_type_ptr, 0)?;
    // TODO: impl
    Ok(AscPtr::null())
}

pub fn store_load_related(
    fenv: FunctionEnvMut<Env>,
    entity_type_ptr: AscPtr<AscString>,
    entity_id_ptr: AscPtr<AscString>,
    field_ptr: AscPtr<AscString>,
) -> Result<AscPtr<Array<AscPtr<AscEntity>>>, RuntimeError> {
    let env = fenv.data();
    let db = env.db_agent.clone().unwrap();
    let entity_id: String = asc_get(&fenv, entity_id_ptr, 0)?;
    let entity_type: String = asc_get(&fenv, entity_type_ptr, 0)?;
    let field: String = asc_get(&fenv, field_ptr, 0)?;
    // TODO: impl
    Ok(AscPtr::null())
}