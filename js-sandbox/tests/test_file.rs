#![allow(clippy::let_unit_value)]

use std::borrow::Cow;
use std::path::Path;
use std::rc::Rc;
use std::thread;

use deno_core::anyhow::Context;
use deno_core::anyhow::Error;
use deno_core::v8::HandleScope;
use deno_core::FsModuleLoader;
use deno_core::RuntimeOptions;
use deno_core::{op, Extension, JsRuntime, OpState, ZeroCopyBuf};
use js_sandbox::exposed_func::SqlSelectExposedFunction;
use serde::de::DeserializeOwned;

use std::collections::HashMap;
use std::time::{Duration, Instant};

use deno_core::{serde_v8, v8};
use js_sandbox::exposed_func::{DefaultExposedFunction, ExposedFunction, ExposedObject};
use serde::{Deserialize, Serialize};

use js_sandbox::{AnyError, JsError, Script};

use js_sandbox::run_time::dynamic;

#[derive(Serialize, Debug)]
struct JsArgs {
	text: String,
	num: i32,
}

#[derive(Deserialize, Debug, PartialEq)]
struct JsResult {
	new_text: String,
	new_num: i32,
}

#[test]
fn call_from_file() {
	let src = r#"
	mainFunc2();
	mainFunc();

	

	function triple(a) {
		console.log("triple(" + a + ")");
		console.log("type of mainFunc2 " + typeof mainFunc2);
		return 3 * a;
	  }
	  
	function extract(obj) {
		return {
			new_text: obj.text + ".",
			new_num: triple(obj.num)
		};
	}
	
	"#;
	//let mut script = Script::from_file("./js/hello.js").expect("File can not be loaded");
	let mut script2 = Script::rd_get_run_time().expect("Initialization succeeds");
	script2
		.rd_load_module("./assets/test/test.js")
		.expect("Unable to load module");

	//script2.rd_run_file("./assets/test/test.js").expect("File can not be loaded");
	// script2.rd_run_string(src);

	// let args = JsArgs {
	// 	text: "hi".to_string(),
	// 	num: 4,
	// };
	// let exp_result = JsResult {
	// 	new_text: "hi.".to_string(),
	// 	new_num: 12,
	// };

	//  let result: JsResult = script2.call("extract", (args,)).unwrap();
	//  assert_eq!(result, exp_result);
}

#[test]
fn main_test() {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	let func1 = DefaultExposedFunction::new("default_func".to_string());

	let func2 = SqlSelectExposedFunction::new("sql_select".to_string());

	// if let Err(error) = runtime.block_on(dynamic::rd_run_js2("./assets/test/test.js")) {
	// 	eprintln!("error: {}", error);
	// }
}

#[test]
fn exp_obj_test() {
	let runtime = tokio::runtime::Builder::new_current_thread()
		.enable_all()
		.build()
		.unwrap();

	let exp_obj = ExposedObject::new("default_func".to_string(), rust_func_for_js2);

	// if let Err(error) = runtime.block_on(dynamic::rd_run_js3("./assets/test/test.js", exp_obj)) {
	// 	eprintln!("error: {}", error);
	// }
}

fn rust_func_for_js2(
	_scope: &mut v8::HandleScope,
	_args: v8::FunctionCallbackArguments,
	_rv: v8::ReturnValue,
) {
	println!("In rust_func_for_js2");
}
