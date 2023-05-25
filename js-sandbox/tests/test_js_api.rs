// Copyright (c) 2020-2023 js-sandbox contributors. Zlib license.

use js_sandbox::{js_api, JsResult, Script, exposed_func::DefaultExposedFunction};

#[js_api]
trait TripleApi {
	fn triple(&mut self, a: i32) -> JsResult<i32>;
}

#[js_api]
trait SaveLoadApi {
	fn save(&mut self, s: &str);
	fn load(&mut self) -> String;
}

#[test]
fn test_stateless() {
	let code = r#"
		function triple(a) { return 3 * a; }
	"#;

	let mut script = Script::from_string
	(code).unwrap();
	let mut api: TripleApi = script.bind_api();

	{
		let result = api.triple(5);
		assert_eq!(result.unwrap(), 15);
	}
}

#[test]
fn test_stateful() {
	let code = r#"
		let value = "10";
		console.log("Hi in JS!");
		function save(v) { value = v; }
		function load() { return value; }
	"#;

	let mut script = Script::from_string(code).unwrap();
	let mut api = script.bind_api::<SaveLoadApi>();

	{
		let loaded = api.load();
		println!("before load value {loaded}");
		api.save("secret");
		println!("after load value {loaded}");
		let loaded = api.load();

		assert_eq!(loaded.as_str(), "secret");
	}
}
