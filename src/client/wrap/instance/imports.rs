// This file is heavily modified from: rust-client\packages\wasm\src\runtime\imports.rs
use super::State;

pub fn create(
    memory: wasmer::Memory,
    store: &mut wasmer::Store,
    env: &wasmer::FunctionEnv<State>,
) -> wasmer::Imports {
    wasmer::imports! {
        "wrap" => {
            "__wrap_invoke_args" => wasmer::Function::new_typed_with_env(store, env, wrap_invoke_args),
            "__wrap_invoke_result" => wasmer::Function::new_typed_with_env(store, env, wrap_invoke_result),
            "__wrap_invoke_error" => wasmer::Function::new_typed_with_env(store, env, wrap_invoke_error),
            "__wrap_abort" => wasmer::Function::new_typed_with_env(store, env, wrap_abort),
            "__wrap_subinvoke" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke),
            "__wrap_subinvoke_result_len" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_result_len),
            "__wrap_subinvoke_result" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_result),
            "__wrap_subinvoke_error_len" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_error_len),
            "__wrap_subinvoke_error" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_error),
            "__wrap_subinvokeImplementation" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_implementation),
            "__wrap_subinvokeImplementation_result_len" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_implementation_result_len),
            "__wrap_subinvokeImplementation_result" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_implementation_result),
            "__wrap_subinvokeImplementation_error_len" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_implementation_error_len),
            "__wrap_subinvokeImplementation_error" => wasmer::Function::new_typed_with_env(store, env, wrap_subinvoke_implementation_error),
            "__wrap_getImplementations" => wasmer::Function::new_typed_with_env(store, env, wrap_get_implementations),
            "__wrap_getImplementations_result" => wasmer::Function::new_typed_with_env(store, env, wrap_get_implementations_result),
            "__wrap_getImplementations_result_len" => wasmer::Function::new_typed_with_env(store, env, wrap_get_implementations_result_len),
            "__wrap_load_env" => wasmer::Function::new_typed_with_env(store, env, wrap_load_env),
            "__wrap_debug_log" => wasmer::Function::new_typed_with_env(store, env, wrap_debug_log),
        },
        "env" => {
            "memory" => memory,
        }
    }
}

fn wrap_invoke_args(context: Context, method_ptr: i32, args_ptr: i32) -> Result<()> {
    let data = context.data();

    if data.method.is_empty() {
        return Err(error("__wrap_invoke_args: method is not set"));
    }
    if data.args.is_empty() {
        return Err(error("__wrap_invoke_args: args is not set"));
    }

    let memory_view = data.memory.view(&context);
    memory_view.write(method_ptr as u64, &data.method)?;
    memory_view.write(args_ptr as u64, &data.args)?;

    Ok(())
}

fn wrap_invoke_result(mut context: Context, offset: i32, length: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    let memory_view = data.memory.view(&store);
    let mut buffer: Vec<u8> = empty_buffer(length);
    memory_view.read(offset as u64, &mut buffer)?;
    data.invoke = Some(Ok(buffer));
    Ok(())
}

fn wrap_invoke_error(mut context: Context, offset: i32, length: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    let memory_view = data.memory.view(&store);
    let message = string_from_memory(&memory_view, length, offset, "__wrap_invoke_error")?;

    data.invoke = Some(Err(message));
    Ok(())
}

fn wrap_abort(
    mut context: Context,
    msg_offset: i32,
    msg_length: i32,
    file_offset: i32,
    file_length: i32,
    line: i32,
    column: i32,
) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    let memory_view = data.memory.view(&store);
    let msg = string_from_memory(&memory_view, msg_length, msg_offset, "__wrap_abort")?;
    let file = string_from_memory(&memory_view, file_length, file_offset, "__wrap_abort")?;

    Err(error(&format!(
        "Abort: {}\nFile: {}\nLocation: [{}, {}]",
        msg, file, line, column
    )))
}

fn wrap_subinvoke(
    mut context: Context,
    uri_ptr: i32,
    uri_len: i32,
    method_ptr: i32,
    method_len: i32,
    args_ptr: i32,
    args_len: i32,
) -> Result<i32> {
    let (data, store) = context.data_and_store_mut();

    let memory_view = data.memory.view(&store);
    let mut uri_buffer: Vec<u8> = empty_buffer(uri_len);
    let mut method_buffer: Vec<u8> = empty_buffer(method_len);
    let mut args_buffer: Vec<u8> = empty_buffer(args_len);

    memory_view.read(uri_ptr as u64, &mut uri_buffer)?;
    memory_view.read(method_ptr as u64, &mut method_buffer)?;
    memory_view.read(args_ptr as u64, &mut args_buffer)?;

    // TODO: Handle the subinvoke logic
    unimplemented!("wrap_subinvoke")
}

fn wrap_subinvoke_result_len(context: Context) -> Result<i32> {
    let data = context.data();

    match &data.subinvoke {
        Some(Ok(result)) => Ok(result.len() as _),
        _ => Err(error(
            "wrap_subinvoke_result_len: No subinvoke result available",
        )),
    }
}

fn wrap_subinvoke_result(mut context: Context, pointer: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    match &data.subinvoke {
        Some(Ok(result)) => {
            data.memory.view(&store).write(pointer as u64, result)?;
            Ok(())
        }
        _ => Err(error(
            "wrap_subinvoke_result: No subinvoke result available",
        )),
    }
}

fn wrap_subinvoke_error_len(context: Context) -> Result<i32> {
    let data = context.data();

    match &data.subinvoke {
        Some(Err(e)) => Ok(e.len() as _),
        _ => Err(error(
            "wrap_subinvoke_error_len: No subinvoke error available",
        )),
    }
}

fn wrap_subinvoke_error(mut context: Context, pointer: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    match &data.subinvoke {
        Some(Err(e)) => {
            data.memory
                .view(&store)
                .write(pointer as u64, e.as_bytes())?;
            Ok(())
        }
        _ => {
            return Err(error("wrap_subinvoke_error: No subinvoke error available"));
        }
    }
}

fn wrap_subinvoke_implementation(
    mut context: Context,
    interface_ptr: i32,
    interface_len: i32,
    impl_uri_ptr: i32,
    impl_uri_len: i32,
    method_ptr: i32,
    method_len: i32,
    args_ptr: i32,
    args_len: i32,
) -> Result<i32> {
    let (data, store) = context.data_and_store_mut();
    let memory_view = data.memory.view(&store);

    let _interface = string_from_memory(
        &memory_view,
        interface_len,
        interface_ptr,
        "wrap_subinvoke_implementation",
    )?;
    let _impl_uri = string_from_memory(
        &memory_view,
        impl_uri_len,
        impl_uri_ptr,
        "wrap_subinvoke_implementation",
    )?;
    let _method = string_from_memory(
        &memory_view,
        method_len,
        method_ptr,
        "wrap_subinvoke_implementation",
    )?;
    let _args = string_from_memory(
        &memory_view,
        args_len,
        args_ptr,
        "wrap_subinvoke_implementation",
    )?;

    // TODO: Handle the subinvoke implementation logic
    unimplemented!("wrap_subinvoke_implementation")
}

fn wrap_subinvoke_implementation_result_len(context: Context) -> Result<i32> {
    let data = context.data();

    match &data.subinvoke_implementation {
        Some(Ok(result)) => Ok(result.len() as _),
        _ => Err(error(
            "wrap_subinvoke_implementation_result_len: No subinvoke implementation result available",
        )),
    }
}

fn wrap_subinvoke_implementation_result(mut context: Context, pointer: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    match &data.subinvoke_implementation {
        Some(Ok(result)) => {
            data.memory.view(&store).write(pointer as u64, result)?;
            Ok(())
        }
        _ => Err(error(
            "wrap_subinvoke_implementation_result: No subinvoke implementation result available",
        )),
    }
}

fn wrap_subinvoke_implementation_error_len(context: Context) -> Result<i32> {
    let data = context.data();

    match &data.subinvoke_implementation {
        Some(Err(error)) => Ok(error.len() as _),
        _ => Err(error(
            "wrap_subinvoke_implementation_error_len: No subinvoke implementation error available",
        )),
    }
}

fn wrap_subinvoke_implementation_error(mut context: Context, pointer: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    match &data.subinvoke_implementation {
        Some(Err(error)) => {
            data.memory
                .view(&store)
                .write(pointer as u64, error.as_bytes())?;
            Ok(())
        }
        _ => Err(error(
            "wrap_subinvoke_implementation_error: No subinvoke implementation error available",
        )),
    }
}

fn wrap_get_implementations(mut context: Context, pointer: i32, length: i32) -> Result<i32> {
    let (data, store) = context.data_and_store_mut();
    let memory_view = data.memory.view(&store);

    let _uri = string_from_memory(&memory_view, length, pointer, "wrap_get_implementations")?;

    // TODO: Handle the logic to get implementations
    unimplemented!("wrap_get_implementations")
}

fn wrap_get_implementations_result_len(context: Context) -> Result<i32> {
    let data = context.data();

    match &data.get_implementations_result {
        Some(result) => Ok(result.len() as _),
        None => Err(error(
            "wrap_get_implementations_result_len: No get implementations result available",
        )),
    }
}

fn wrap_get_implementations_result(mut context: Context, pointer: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    match &data.get_implementations_result {
        Some(result) => {
            data.memory.view(&store).write(pointer as u64, result)?;
            Ok(())
        }
        None => Err(error(
            "wrap_get_implementations_result: No get implementations result available",
        )),
    }
}

fn wrap_load_env(mut context: Context, pointer: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();

    data.memory.view(&store).write(pointer as u64, &data.env)?;
    Ok(())
}

fn wrap_debug_log(mut context: Context, msg_offset: i32, msg_length: i32) -> Result<()> {
    let (data, store) = context.data_and_store_mut();
    let memory_view = data.memory.view(&store);

    let msg = string_from_memory(&memory_view, msg_length, msg_offset, "wrap_debug_log")?;
    println!("Debug log: {}", msg);

    Ok(())
}

type Context<'a> = wasmer::FunctionEnvMut<'a, State>;
type Result<T> = std::result::Result<T, wasmer::RuntimeError>;

#[inline]
fn error(msg: &str) -> wasmer::RuntimeError {
    wasmer::RuntimeError::new(msg)
}

fn string_from_memory(
    memory_view: &wasmer::MemoryView<'_>,
    length: i32,
    offset: i32,
    import_name: &str,
) -> Result<String> {
    let mut buffer = empty_buffer(length);
    memory_view.read(offset as u64, &mut buffer)?;
    String::from_utf8(buffer).map_err(|_| error(&format!("{}: invalid string", import_name)))
}

fn empty_buffer(size: i32) -> Vec<u8> {
    let mut empty: Vec<u8> = Vec::with_capacity(size as usize);
    unsafe {
        empty.set_len(size as usize);
    }
    empty
}
