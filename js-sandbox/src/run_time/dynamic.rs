// use deno_core::anyhow::Context;
// use deno_core::anyhow::Error;
// use deno_core::error::AnyError;
// use deno_core::v8;
// use deno_core::FsModuleLoader;
// use deno_core::JsRuntime;
// use deno_core::RuntimeOptions;
// use std::rc::Rc;
// use tokio;

// use crate::exposed_func::DefaultExposedFunction;
// use crate::exposed_func::ExposedFunction;
// use crate::exposed_func::ExposedObject;
// use crate::exposed_func::SqlSelectExposedFunction;


// pub async fn rd_run_js3(file_path: &str, exp_obj: ExposedObject) -> Result<(), AnyError> {
// 	let main_module = deno_core::resolve_path(
// 		file_path,
// 		&std::env::current_dir().context("Unable to get CWD")?,
// 	)?;

// 	let mut js_runtime: JsRuntime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
// 		module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
// 		..Default::default()
// 	});

// 	//expose functions

// 	{
// 		let mut scope = js_runtime.handle_scope();
// 		let context = scope.get_current_context();
// 		let global = context.global(&mut scope);
// 		let scope = &mut v8::ContextScope::new(&mut scope, context);

// 		let my_func_key = v8::String::new(scope, &exp_obj.name).unwrap();
// 		let my_func_templ = v8::FunctionTemplate::new(
// 			scope,
// 			|scope: &mut v8::HandleScope,
// 			 args: v8::FunctionCallbackArguments,
// 			 rv: v8::ReturnValue| { exp_obj.call(scope, args, rv) },
// 		);
// 		let my_func_val = my_func_templ.get_function(scope).unwrap();
// 		global.set(scope, my_func_key.into(), my_func_val.into());
// 	}

// 	//js_runtime = Self::add_exposed_func2::<DefaultExposedFunction>(js_runtime);

// 	let mod_id = js_runtime.load_main_module(&main_module, None).await?;
// 	let result = js_runtime.mod_evaluate(mod_id);
// 	js_runtime.run_event_loop(false).await?;
// 	result.await?;
// 	Ok(())
// }




// pub async fn rd_run_js2(file_path: &str) -> Result<(), AnyError> {
// 	let main_module = deno_core::resolve_path(
// 		file_path,
// 		&std::env::current_dir().context("Unable to get CWD")?,
// 	)?;

// 	let mut js_runtime: JsRuntime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
// 		module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
// 		..Default::default()
// 	});

// 	//expose functions

// 	{
// 		let mut scope = js_runtime.handle_scope();
// 		let context = scope.get_current_context();
// 		let global = context.global(&mut scope);
// 		let scope = &mut v8::ContextScope::new(&mut scope, context);

// 		add_call_back(scope, global, "default_func", |scope: &mut v8::HandleScope,
// 			 args: v8::FunctionCallbackArguments,
// 			 rv: v8::ReturnValue| { DefaultExposedFunction::rust_func_for_js(scope, args, rv) });

// 		let my_func_key = v8::String::new(scope, "sqlSelect").unwrap();
// 		let my_func_templ = v8::FunctionTemplate::new(
// 			scope,
// 			|scope: &mut v8::HandleScope,
// 			 args: v8::FunctionCallbackArguments,
// 			 rv: v8::ReturnValue| { SqlSelectExposedFunction::rust_func_for_js(scope, args, rv) },
// 		);
// 		let my_func_val = my_func_templ.get_function(scope).unwrap();
// 		global.set(scope, my_func_key.into(), my_func_val.into());
// 	}

// 	//js_runtime = Self::add_exposed_func2::<DefaultExposedFunction>(js_runtime);

// 	let mod_id = js_runtime.load_main_module(&main_module, None).await?;
// 	let result = js_runtime.mod_evaluate(mod_id);
// 	js_runtime.run_event_loop(false).await?;
// 	result.await?;
// 	Ok(())
// }

// fn add_call_back(
// 	scope: &mut v8::ContextScope<v8::HandleScope>,
// 	global: v8::Local<v8::Object>,
// 	fun_name: &str,
// 	call_back: fn(&mut v8::HandleScope, v8::FunctionCallbackArguments, v8::ReturnValue),
// ) {
// 	let my_func_key = v8::String::new(scope, "default_func").unwrap();
// 	let my_func_templ = v8::FunctionTemplate::new(
// 		scope,
// 		|scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, rv: v8::ReturnValue| {
// 			call_back(scope, args, rv)
// 		},
// 	);
// 	let my_func_val = my_func_templ.get_function(scope).unwrap();
// 	global.set(scope, my_func_key.into(), my_func_val.into());
// }

// pub async fn rd_run_js<A>(file_path: &str, exp_func: A) -> Result<(), AnyError>
// where
// 	A: ExposedFunction,
// {
// 	let main_module = deno_core::resolve_path(
// 		file_path,
// 		&std::env::current_dir().context("Unable to get CWD")?,
// 	)?;

// 	let mut js_runtime: JsRuntime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
// 		module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
// 		..Default::default()
// 	});

// 	//expose functions

// 	{
// 		let mut scope = js_runtime.handle_scope();
// 		let context = scope.get_current_context();
// 		let global = context.global(&mut scope);
// 		let scope = &mut v8::ContextScope::new(&mut scope, context);

// 		let my_func_key = v8::String::new(scope, "default_func").unwrap();
// 		let my_func_templ = v8::FunctionTemplate::new(
// 			scope,
// 			|scope: &mut v8::HandleScope,
// 			 args: v8::FunctionCallbackArguments,
// 			 rv: v8::ReturnValue| { A::rust_func_for_js(scope, args, rv) },
// 		);
// 		let my_func_val = my_func_templ.get_function(scope).unwrap();
// 		global.set(scope, my_func_key.into(), my_func_val.into());

// 		let my_func_key = v8::String::new(scope, "sqlSelect").unwrap();
// 		let my_func_templ = v8::FunctionTemplate::new(
// 			scope,
// 			|scope: &mut v8::HandleScope,
// 			 args: v8::FunctionCallbackArguments,
// 			 rv: v8::ReturnValue| { SqlSelectExposedFunction::rust_func_for_js(scope, args, rv) },
// 		);
// 		let my_func_val = my_func_templ.get_function(scope).unwrap();
// 		global.set(scope, my_func_key.into(), my_func_val.into());
// 	}

// 	//js_runtime = Self::add_exposed_func2::<DefaultExposedFunction>(js_runtime);

// 	let mod_id = js_runtime.load_main_module(&main_module, None).await?;
// 	let result = js_runtime.mod_evaluate(mod_id);
// 	js_runtime.run_event_loop(false).await?;
// 	result.await?;
// 	Ok(())
// }

// // pub async fn rd_run_js2<A>(file_path: &str) -> Result<(), AnyError>
// // where
// // 	A: ExposedFunction,
// // {
// // 	let main_module = deno_core::resolve_path(
// // 		file_path,
// // 		&std::env::current_dir().context("Unable to get CWD")?,
// // 	)?;

// // 	let mut js_runtime: JsRuntime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
// // 		module_loader: Some(Rc::new(deno_core::FsModuleLoader)),
// // 		..Default::default()
// // 	});

// // 	{
// // 		let mut scope = js_runtime.handle_scope();
// // 		let context = scope.get_current_context();
// // 		let global = context.global(&mut scope);
// // 		let scope = &mut v8::ContextScope::new(&mut scope, context);

// // 		let my_func_key = v8::String::new(scope, A::name().as_str()).unwrap();
// // 		println!("my_func_key is {:?}", my_func_key);

// // 		let my_func_templ = v8::FunctionTemplate::new(
// // 			scope,
// // 			|scope: &mut v8::HandleScope,
// // 			 args: v8::FunctionCallbackArguments,
// // 			 rv: v8::ReturnValue| { A::rust_func_for_js(scope, args, rv) },
// // 		);
// // 		let my_func_val = my_func_templ.get_function(scope).unwrap();
// // 		global.set(scope, my_func_key.into(), my_func_val.into());
// // 	}

// // 	let mod_id = js_runtime.load_main_module(&main_module, None).await?;
// // 	let result = js_runtime.mod_evaluate(mod_id);
// // 	js_runtime.run_event_loop(false).await?;
// // 	result.await?;
// // 	Ok(())
// // }

// pub fn init() -> Result<(), Error> {
// 	let args: Vec<String> = std::env::args().collect();
// 	if args.len() < 2 {
// 		println!("Usage: target/examples/debug/fs_module_loader <path_to_module>");
// 		std::process::exit(1);
// 	}
// 	let main_url = &args[1];
// 	println!("Run {main_url}");

// 	let mut js_runtime = JsRuntime::new(RuntimeOptions {
// 		module_loader: Some(Rc::new(FsModuleLoader)),
// 		..Default::default()
// 	});

// 	let runtime = tokio::runtime::Builder::new_current_thread()
// 		.enable_all()
// 		.build()?;

// 	let main_module = deno_core::resolve_path(
// 		main_url,
// 		&std::env::current_dir().context("Unable to get CWD")?,
// 	)?;

// 	let future = async move {
// 		let mod_id = js_runtime.load_main_module(&main_module, None).await?;
// 		let result = js_runtime.mod_evaluate(mod_id);
// 		js_runtime.run_event_loop(false).await?;
// 		result.await?
// 	};
// 	runtime.block_on(future)
// }
