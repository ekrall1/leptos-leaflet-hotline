use js_sys::JsString;
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

///
/// struct data type for [`Browser`]
/// contains browser details
/// see <`https://leafletjs.com/reference.html#browser`>
///
#[wasm_bindgen]
#[derive(Debug, Clone, PartialEq)]
pub struct Browser {
    /// browser is chrome
    pub chrome: bool,
    /// browser is safari
    pub safari: bool,
    /// browser runs on a mobile device
    pub mobile: bool,
    /// browser supports pointer events
    pub pointer: bool,
    /// browser supports touch or pointer events
    pub touch: bool,
    /// browser supports touch events
    pub touch_native: bool,
    /// browser on a high-resolution screen or zoom is more than 100%
    pub retina: bool,
    /// browser is running in a Mac or linux platform
    pub mac: bool,
    /// browser is running in a linux platform
    pub linux: bool,
    /// browser is edge
    pub edge: bool, // added
}

/// implement get details for browser
#[wasm_bindgen]
impl Browser {
    ///
    /// get browser details
    ///
    /// # Returns
    /// [`Browser`]
    ///
    pub fn get() -> Browser {
        let window = window().expect("Missing Window");
        let agent = window.navigator().user_agent().clone();
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
            edge: JsString::from(JsValue::from_str(&agent.unwrap()))
                .to_lower_case()
                .includes("edg", 0),
        }
    }
}
