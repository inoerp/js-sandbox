use deno_core::v8::{FunctionCallbackArguments, HandleScope, ReturnValue};

pub struct ExposedObject1 {
	pub name: String,
	pub call_back: fn(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue),
}

pub struct ExposedObject {
	pub name: String,
	pub before_get: fn(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue) -> (),
}

impl ExposedObject {
	pub fn new(
		name: String,
		call_back: fn(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue),
	) -> Self
	where
		Self: Sized,
	{
		Self { name, before_get: call_back }
	}

	pub async fn call<'a>(scope: &'a mut HandleScope<'a>, args: FunctionCallbackArguments<'a>, rv: ReturnValue<'a>) ->() {
		//(self.call_back)(scope, args, rv);
	}
}

pub trait ExposedFunction {
	fn rust_func_for_js(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue);
	// fn run_call_back(
	// 	&self,
	// 	scope: &mut HandleScope,
	// 	args: FunctionCallbackArguments,
	// 	rv: ReturnValue,
	// );
	fn name() -> String;
}

pub trait ExposedFunction2 {
	fn rust_func_for_js(
		self: &Self,
		scope: &mut HandleScope,
		args: FunctionCallbackArguments,
		rv: ReturnValue,
	);

	fn name(self: &Self) -> String;
}

pub struct DefaultExposedFunction {
	name: String,
}

impl DefaultExposedFunction {
	pub fn new(name: String) -> Self
	where
		Self: Sized,
	{
		DefaultExposedFunction { name }
	}
}

impl ExposedFunction for DefaultExposedFunction {
	fn rust_func_for_js(
		_scope: &mut HandleScope,
		_args: FunctionCallbackArguments,
		_rv: ReturnValue,
	) {
		println!("In DefaultExposedFunction rust_func_for_js");
	}

	fn name() -> String {
		return "default_func".to_string();
	}

	// fn run_call_back(
	// 	&self,
	// 	scope: &mut HandleScope,
	// 	args: FunctionCallbackArguments,
	// 	rv: ReturnValue,
	// ) {
	// 	println!("In DefaultExposedFunction run_call_back");
	// }
}

// impl ExposedFunction for DefaultExposedFunction {
// 	fn rust_func_for_js(
// 		_scope: &mut HandleScope,
// 		_args: FunctionCallbackArguments,
// 		_rv: ReturnValue,
// 	) {
// 		println!("In DefaultExposedFunction rust_func_for_js");
// 	}

// 	fn name(&self) -> String {
// 		return "default_func".to_string();
// 	}
// }

pub struct SqlSelectExposedFunction {
	name: String,
}

impl SqlSelectExposedFunction {
	pub fn new(name: String) -> Self {
		SqlSelectExposedFunction { name }
	}
}

impl ExposedFunction for SqlSelectExposedFunction {
	fn rust_func_for_js(
		_scope: &mut HandleScope,
		_args: FunctionCallbackArguments,
		_rv: ReturnValue,
	) {
		println!("In SqlSelectExposedFunction rust_func_for_js");
	}

	fn name() -> String {
		return "sqlSelect".to_string();	}

	// fn run_call_back(
	// 	&self,
	// 	scope: &mut HandleScope,
	// 	args: FunctionCallbackArguments,
	// 	rv: ReturnValue,
	// ) {
	// 	println!("In SqlSelectExposedFunction run_call_back");
	// }
}
