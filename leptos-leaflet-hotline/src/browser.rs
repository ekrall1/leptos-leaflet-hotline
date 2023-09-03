use js_sys::*;
use std::ops::Deref;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;
use web_sys::window;

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name="Browser.chrome", js_namespace=L)]
    static CHROME: bool;

    #[wasm_bindgen(js_name="Browser.safari", js_namespace=L)]
    static SAFARI: bool;

    #[wasm_bindgen(js_name="Browser.mobile", js_namespace=L)]
    static MOBILE: bool;

    #[wasm_bindgen(js_name="Browser.pointer", js_namespace=L)]
    static POINTER: bool;

    #[wasm_bindgen(js_name="Browser.touchNative", js_namespace=L)]
    static TOUCH_NATIVE: bool;

    #[wasm_bindgen(js_name="Browser.touch", js_namespace=L)]
    static TOUCH: bool;

    #[wasm_bindgen(js_name="Browser.retina", js_namespace=L)]
    static RETINA: bool;

    #[wasm_bindgen(js_name="Browser.mac", js_namespace=L)]
    static MAC: bool;

    #[wasm_bindgen(js_name="Browser.linux", js_namespace=L)]
    static LINUX: bool;
}

#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Browser {
    pub chrome: bool,
    pub safari: bool,
    pub mobile: bool,
    pub pointer: bool,
    pub touch: bool,
    pub touch_native: bool,
    pub retina: bool,
    pub mac: bool,
    pub linux: bool,
    pub edge: bool, // added
}

fn check_edge() -> bool {
    let window = window().expect("Missing Window");
    let agent = window.navigator().user_agent().clone();
    let edge = JsString::from(JsValue::from_str(&agent.unwrap()))
        .to_lower_case()
        .includes("edg", 0);
    edge
}

#[wasm_bindgen]
impl Browser {
    pub fn default() -> Browser {
        Browser {
            chrome: *CHROME.deref(),
            safari: *SAFARI.deref(),
            mobile: *MOBILE.deref(),
            pointer: *POINTER.deref(),
            touch_native: *TOUCH_NATIVE.deref(),
            touch: *TOUCH.deref(),
            retina: *RETINA.deref(),
            mac: *MAC.deref(),
            linux: *LINUX.deref(),
            edge: check_edge(),
        }
    }
}
