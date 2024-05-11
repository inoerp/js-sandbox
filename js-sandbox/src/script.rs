// Copyright (c) 2020-2023 js-sandbox contributors. Zlib license.

use std::borrow::Cow;
use std::path::Path;
use std::rc::Rc;
use std::{thread, time::Duration};

use deno_core::v8::{FunctionCallbackArguments, HandleScope, ReturnValue};
use deno_core::{futures, op, v8, Extension, JsBuffer, JsRuntime, OpState};
use serde::de::DeserializeOwned;

use crate::exposed_func::{
	DefaultExposedFunction, ExposedFunction, ExposedObject, SqlSelectExposedFunction, ExposedObject1,
};
use crate::{AnyError, CallArgs, JsError, JsValue};

use deno_core::anyhow::Context;
use deno_core::anyhow::Error;
use deno_core::FsModuleLoader;
use deno_core::RuntimeOptions;

use std::cell::RefCell;

pub trait JsApi<'a> {
	/// Generate an API from a script
	fn from_script(script: &'a mut Script) -> Self
	where
		Self: Sized;
}

/// Represents a single JavaScript file that can be executed.
///
/// The code can be loaded from a file or from a string in memory.
/// A typical usage pattern is to load a file with one or more JS function definitions, and then call those functions from Rust.
pub struct Script {
	runtime: JsRuntime,
	last_rid: u32,
	timeout: Option<Duration>,
}

impl Script {
	const DEFAULT_FILENAME: &'static str = "sandboxed.js";

	// ----------------------------------------------------------------------------------------------------------------------------------------------
	// Constructors and builders

	/// Initialize a script with the given JavaScript source code.
	///
	/// Returns a new object on success, and an error in case of syntax or initialization error with the code.
	pub fn from_string(js_code: &str) -> Result<Self, JsError> {
		// console.log() is not available by default -- add the most basic version with single argument (and no warn/info/... variants)
		let all_code =
			"const console = { log: function(expr) { Deno.core.print(expr + '\\n', false); } };"
				.to_string() + js_code;

		Self::create_script(all_code)
	}

	pub fn rd_get_run_time() -> Result<Self, JsError> {
		Self::rd_create_run_time()
	}

	// pub fn rd_get_run_time2(file_path: &str) -> Result<Self, AnyError> {
	// 	Self::rd_create_run_time2(file_path)
	// }

	pub fn rd_run_string(&mut self, js_code: &str) -> Result<v8::Global<v8::Value>, JsError> {
		// console.log() is not available by default -- add the most basic version with single argument (and no warn/info/... variants)
		let all_code =
			"const console = { log: function(expr) { Deno.core.print(expr + '\\n', false); } };"
				.to_string() + js_code;

		self.rd_run_script(all_code)
	}

	pub fn rd_run_file(
		&mut self,
		file: impl AsRef<Path>,
	) -> Result<v8::Global<v8::Value>, JsError> {
		match std::fs::read_to_string(file) {
			Ok(js_code) => self.rd_run_script(js_code),
			Err(e) => Err(JsError::Runtime(AnyError::from(e))),
		}
	}

	/// Initialize a script by loading it from a .js file.
	///
	/// To load a file at compile time, you can use [`Self::from_string()`] in combination with the [`include_str!`] macro.
	/// At the moment, a script is limited to a single file, and you will need to do bundling yourself (e.g. with `esbuild`).
	///
	/// Returns a new object on success. Fails if the file cannot be opened or in case of syntax or initialization error with the code.
	pub fn from_file(file: impl AsRef<Path>) -> Result<Self, JsError> {
		// let filename = file
		// 	.as_ref()
		// 	.file_name()
		// 	.and_then(|s| s.to_str())
		// 	.unwrap_or(Self::DEFAULT_FILENAME)
		// 	.to_owned();

		match std::fs::read_to_string(file) {
			Ok(js_code) => Self::create_script(js_code),
			Err(e) => Err(JsError::Runtime(AnyError::from(e))),
		}
	}

	/// Equips this script with a timeout, meaning that any function call is aborted after the specified duration.
	///
	/// This requires creating a separate thread for each function call, which tracks time and pulls the plug
	/// if the JS function does not return in time. Use this for untrusted 3rd-party code, not if you know that
	/// your functions always return.
	///
	/// Panics with invalid timeouts or if this script already has a timeout set.
	pub fn with_timeout(mut self, timeout: Duration) -> Self {
		assert!(self.timeout.is_none());
		assert!(timeout > Duration::ZERO);

		self.timeout = Some(timeout);
		self
	}

	// ----------------------------------------------------------------------------------------------------------------------------------------------
	// Call API

	/// Invokes a JavaScript function.
	///
	/// Blocks on asynchronous functions until completion.
	///
	/// `args_tuple` needs to be a tuple.
	///
	/// Each tuple element is converted to JSON (using serde_json) and passed as a distinct argument to the JS function.
	pub fn call<A, R>(&mut self, fn_name: &str, args_tuple: A) -> Result<R, JsError>
	where
		A: CallArgs,
		R: DeserializeOwned,
	{
		let json_args = args_tuple.into_arg_string()?;
		let json_result = self.call_impl(fn_name, json_args)?;
		let result: R = serde_json::from_value(json_result)?;

		Ok(result)
	}

	pub fn bind_api<'a, A>(&'a mut self) -> A
	where
		A: JsApi<'a>,
	{
		A::from_script(self)
	}

	pub(crate) fn call_json(&mut self, fn_name: &str, args: &JsValue) -> Result<JsValue, JsError> {
		self.call_impl(fn_name, args.to_string())
	}

	fn call_impl(&mut self, fn_name: &str, json_args: String) -> Result<JsValue, JsError> {
		// Note: ops() is required to initialize internal state
		// Wrap everything in scoped block

		// 'undefined' will cause JSON serialization error, so it needs to be treated as null
		let js_code = format!(
			"(async () => {{
				let __rust_result = {fn_name}.constructor.name === 'AsyncFunction'
					? await {fn_name}({json_args})
					: {fn_name}({json_args});

				if (typeof __rust_result === 'undefined')
					__rust_result = null;

				Deno.core.ops.op_return(__rust_result);
			}})()"
		);

		if let Some(timeout) = self.timeout {
			let handle = self.runtime.v8_isolate().thread_safe_handle();

			thread::spawn(move || {
				thread::sleep(timeout);
				handle.terminate_execution();
			});
		}

		// syncing ops is required cause they sometimes change while preparing the engine
		// self.runtime.sync_ops_cache();

		// TODO use strongly typed JsError here (downcast)
		self.runtime
			.execute_script(Self::DEFAULT_FILENAME, js_code.into())?;
		deno_core::futures::executor::block_on(self.runtime.run_event_loop(false))?;

		let state_rc = self.runtime.op_state();
		let mut state = state_rc.borrow_mut();
		let table = &mut state.resource_table;

		// Get resource, and free slot (no longer needed)
		let entry: Rc<ResultResource> = table
			.take(self.last_rid)
			.expect("Resource entry must be present");
		let extracted =
			Rc::try_unwrap(entry).expect("Rc must hold single strong ref to resource entry");
		self.last_rid += 1;

		Ok(extracted.json_value)
	}

	fn rd_create_run_time() -> Result<Self, JsError> {
		let ext = Extension::builder("script")
			.ops(vec![(op_return::decl())])
			.build();

		let js_runtime = JsRuntime::new(deno_core::RuntimeOptions {
			module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
			extensions: vec![ext],
			..Default::default()
		});

		Ok(Script {
			runtime: js_runtime,
			last_rid: 0,
			timeout: None,
		})
	}

	fn rd_run_script(&mut self, js_code: String) -> Result<v8::Global<v8::Value>, JsError> {
		let value: v8::Global<v8::Value> = self
			.runtime
			.execute_script(Self::DEFAULT_FILENAME, js_code.into())?;

		Ok(value)
	}

	pub fn rd_load_module(&mut self, main_url: &str) -> Result<(), Error> {
		println!("Run {main_url}");

		let runtime = tokio::runtime::Builder::new_current_thread()
			.enable_all()
			.build()?;

		let main_module = deno_core::resolve_path(
			main_url,
			&std::env::current_dir().context("Unable to get CWD")?,
		)?;

		let future = async move {
			let mod_id = self.runtime.load_main_module(&main_module, None).await?;
			let result = self.runtime.mod_evaluate(mod_id);
			self.runtime.run_event_loop(false).await?;
			result.await?
		};
		runtime.block_on(future)
	}

	fn create_script(js_code: String) -> Result<Self, JsError> {
		let ext = Extension::builder("script")
			.ops(vec![(op_return::decl())])
			.build();

		let mut isolate = JsRuntime::new(deno_core::RuntimeOptions {
			module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
			extensions: vec![ext],
			..Default::default()
		});

		// We cannot provide a dynamic filename because execute_script() requires a &'static str
		isolate.execute_script(Self::DEFAULT_FILENAME, js_code.into())?;

		Ok(Script {
			runtime: isolate,
			last_rid: 0,
			timeout: None,
		})
	}

	// pub fn add_exposed_object(&mut self, obj:ExposedObject)
	// {
	// 	let mut scope = self.runtime.handle_scope();
	// 	let context = scope.get_current_context();
	// 	let global = context.global(&mut scope);
	// 	let scope = &mut v8::ContextScope::new(&mut scope, context);

	// 	let my_func_key = v8::String::new(scope, &obj.name).unwrap();
	// 	println!("my_func_key is ${:?}", my_func_key);

	// 	let my_func_templ = v8::FunctionTemplate::new(
	// 		scope,
	// 		|scope: &mut v8::HandleScope,
	// 		 args: v8::FunctionCallbackArguments,
	// 		 rv: v8::ReturnValue| { obj.call(scope, args, rv) },
	// 	);
	// 	let my_func_val = my_func_templ.get_function(scope).unwrap();
	// 	global.set(scope, my_func_key.into(), my_func_val.into());
	// }

	pub async fn rd_run_js(file_path: &str) -> Result<(), AnyError> {
		let main_module = deno_core::resolve_path(
			file_path,
			&std::env::current_dir().context("Unable to get CWD")?,
		)?;

		let mut js_runtime: JsRuntime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
			module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
			..Default::default()
		});

		//js_runtime = Self::add_exposed_func2::<DefaultExposedFunction>(js_runtime);

		let mod_id = js_runtime.load_main_module(&main_module, None).await?;
		let result = js_runtime.mod_evaluate(mod_id);
		js_runtime.run_event_loop(false).await?;
		result.await?;
		Ok(())
	}

	// pub fn add_exposed_func2<A>(runtime: JsRuntime) -> JsRuntime
	// where
	// 	A: ExposedFunction,
	// {
	// 	let mut scope = runtime.handle_scope();
	// 	let context = scope.get_current_context();
	// 	let global = context.global(&mut scope);
	// 	let scope = &mut v8::ContextScope::new(&mut scope, context);

	// 	let my_func_key = v8::String::new(scope, &A::name()).unwrap();
	// 	println!("my_func_key is {:?}", my_func_key);

	// 	let runtime_clone = Rc::clone(runtime);
	// 	let my_func_templ = v8::FunctionTemplate::new(
	// 		scope,
	// 		move |scope: &mut v8::HandleScope,
	// 			  args: v8::FunctionCallbackArguments,
	// 			  rv: v8::ReturnValue| {
	// 			A::rust_func_for_js(scope, args, rv);
	// 		},
	// 	);
	// 	let my_func_val = my_func_templ.get_function(scope).unwrap();
	// 	global.set(scope, my_func_key.into(), my_func_val.into());
	// 	runtime_clone
	// }

	pub fn add_exposed_func<A>(&mut self)
	where
		A: ExposedFunction,
	{
		let mut scope = self.runtime.handle_scope();
		let context = scope.get_current_context();
		let global = context.global(&mut scope);
		let scope = &mut v8::ContextScope::new(&mut scope, context);

		let my_func_key = v8::String::new(scope, A::name().as_str()).unwrap();
		println!("my_func_key is ${:?}", my_func_key);

		let my_func_templ = v8::FunctionTemplate::new(
			scope,
			|scope: &mut v8::HandleScope,
			 args: v8::FunctionCallbackArguments,
			 rv: v8::ReturnValue| { A::rust_func_for_js(scope, args, rv) },
		);
		let my_func_val = my_func_templ.get_function(scope).unwrap();
		global.set(scope, my_func_key.into(), my_func_val.into());
	}

	// pub async fn add_exposed_func2<A>(&mut self, b: ExposedObject)
	// where
	// 	A: ExposedFunction,
	// {
	// 	let mut scope = self.runtime.handle_scope();
	// 	let context = scope.get_current_context();
	// 	let global = context.global(&mut scope);
	// 	let scope = &mut v8::ContextScope::new(&mut scope, context);

	// 	let my_func_key = v8::String::new(scope, &b.name.as_str()).unwrap();
	// 	println!("my_func_key is ${:?}", my_func_key);

	// 	let my_func_templ = v8::FunctionTemplate::new(
	// 		scope,
	// 		|inner_scope: &mut v8::HandleScope,
	// 		 inner_args: v8::FunctionCallbackArguments,
	// 		 inner_rv: v8::ReturnValue| {             tokio::task::spawn_blocking(move || {
    //             futures::executor::block_on(ExposedObject::call(inner_scope, inner_args, inner_rv))
    //         }); },
	// 	);
	// 	let my_func_val = my_func_templ.get_function(scope).unwrap();
	// 	global.set(scope, my_func_key.into(), my_func_val.into());
	// }

	// pub fn add_async_func(
	// 	&mut self,
	// 	fn_name: &str,
	// 	callback: fn(&mut HandleScope, FunctionCallbackArguments, ReturnValue),
	// ) {
	// 	let mut scope = self.runtime.handle_scope();
	// 	let context = scope.get_current_context();
	// 	let global = context.global(&mut scope);
	// 	let scope = &mut v8::ContextScope::new(&mut scope, context);

	// 	let my_func_key = v8::String::new(scope, fn_name).unwrap();
	// 	println!("my_func_key is ${:?}", my_func_key);

	// 	let my_func_templ = v8::FunctionTemplate::new(
	// 		scope,
	// 		|scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue| {
	// 			callback(scope, args, rv)
	// 		},
	// 	);
	// 	let my_func_val = my_func_templ.get_function(scope).unwrap();
	// 	global.set(scope, my_func_key.into(), my_func_val.into());
	// }


}

#[derive(Debug)]
struct ResultResource {
	json_value: JsValue,
}

// Type that is stored inside Deno's resource table
impl deno_core::Resource for ResultResource {
	fn name(&self) -> Cow<str> {
		"__rust_Result".into()
	}
}

#[op]
fn op_return(
	state: &mut OpState,
	args: JsValue,
	_buf: Option<JsBuffer>,
) -> Result<JsValue, deno_core::error::AnyError> {
	let entry = ResultResource { json_value: args };
	let resource_table = &mut state.resource_table;
	let _rid = resource_table.add(entry);
	Ok(serde_json::Value::Null)
}
