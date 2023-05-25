use deno_core::v8::{HandleScope, FunctionCallbackArguments, ReturnValue};

pub struct ExposedObject {
	pub name: String,
	pub call_back:
		fn(scope: &mut HandleScope, args: FunctionCallbackArguments, rv: ReturnValue),
}

impl ExposedObject {
	pub fn new(
		name: String,
		call_back: fn(
			scope: &mut HandleScope,
			args: FunctionCallbackArguments,
			rv: ReturnValue,
		),
	) -> Self {
		Self { name, call_back }
	}

	pub fn call(
		&self,
		scope: &mut HandleScope,
		args: FunctionCallbackArguments,
		rv: ReturnValue,
	) {
		(self.call_back)(scope, args, rv);
	}
}


pub trait ExposedFunction {
	fn rust_func_for_js(
		scope: &mut HandleScope,
		args: FunctionCallbackArguments,
		rv: ReturnValue,
	);

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

pub struct DefaultExposedFunction {}
impl ExposedFunction for DefaultExposedFunction {
	fn rust_func_for_js(
		_scope: &mut HandleScope,
		_args: FunctionCallbackArguments,
		_rv: ReturnValue,
	) {
	}

	fn name() -> String {
		return "default_func".to_string();
	}
}
