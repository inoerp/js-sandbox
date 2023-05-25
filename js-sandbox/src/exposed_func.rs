use deno_core::v8;

pub struct ExposedObject {
	pub name: String,
	pub call_back:
		fn(scope: &mut v8::HandleScope, args: v8::FunctionCallbackArguments, rv: v8::ReturnValue),
}

impl ExposedObject {
	pub fn new(
		name: String,
		call_back: fn(
			scope: &mut v8::HandleScope,
			args: v8::FunctionCallbackArguments,
			rv: v8::ReturnValue,
		),
	) -> Self {
		Self { name, call_back }
	}

	pub fn call(
		&self,
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		rv: v8::ReturnValue,
	) {
		(self.call_back)(scope, args, rv);
	}
}

pub trait ExposedFunction {
	fn rust_func_for_js(
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		rv: v8::ReturnValue,
	);

	fn name() -> String;
}

pub trait ExposedFunction2 {
	fn rust_func_for_js(
		self: &Self,
		scope: &mut v8::HandleScope,
		args: v8::FunctionCallbackArguments,
		rv: v8::ReturnValue,
	);

	fn name(self: &Self) -> String;
}

pub struct DefaultExposedFunction {}
impl ExposedFunction for DefaultExposedFunction {
	fn rust_func_for_js(
		_scope: &mut deno_core::v8::HandleScope,
		_args: deno_core::v8::FunctionCallbackArguments,
		_rv: deno_core::v8::ReturnValue,
	) {
	}

	fn name() -> String {
		return "default_func".to_string();
	}
}
