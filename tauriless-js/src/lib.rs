mod utils;

use js_sys::Uint8Array;
use serde::Deserialize;
use wasm_bindgen::prelude::*;
use web_sys::XmlHttpRequest;

#[wasm_bindgen(start)]
fn start() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &JsValue);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_with_desc(desc: &str, s: &JsValue);
}

#[wasm_bindgen]
pub fn encode(args: JsValue) -> Result<JsValue, JsValue> {
    let args = serde_wasm_bindgen::Deserializer::from(args);
    let Ok(value) = <pot::Value<'static> as Deserialize>::deserialize(args) else {
        return Err(JsValue::from_str("Unsupported arguments"));
    };
    let encoded = match pot::to_vec(&value) {
        Ok(encoded) => encoded,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed to encode arguments: {e:?}"
            )));
        }
    };

    let Ok(encoded) = serde_wasm_bindgen::to_value(&encoded) else {
        return Err(JsValue::from_str(
            "Failed to convert the endcoded args array to JsValue",
        ));
    };

    Ok(encoded)
}

#[wasm_bindgen]
pub fn invoke(command: &str, args: JsValue) -> Result<JsValue, JsValue> {
    let args = serde_wasm_bindgen::Deserializer::from(args);
    let Ok(value) = <pot::Value<'static> as Deserialize>::deserialize(args) else {
        return Err(JsValue::from_str("Unsupported arguments"));
    };
    let encoded = match pot::to_vec(&value) {
        Ok(encoded) => encoded,
        Err(e) => {
            return Err(JsValue::from_str(&format!(
                "Failed to encode arguments: {e:?}"
            )));
        }
    };

    let promise = js_sys::Promise::new(&mut move |resolve, reject| {
        let xhr = XmlHttpRequest::new().unwrap();
        xhr.set_response_type(web_sys::XmlHttpRequestResponseType::Arraybuffer);
        {
            let handler = wasm_bindgen::closure::Closure::once(move |event: web_sys::Event| {
                let xhr = event
                    .target()
                    .unwrap()
                    .dyn_into::<XmlHttpRequest>()
                    .unwrap();
                if xhr.status().unwrap() == 200 {
                    let response = xhr
                        .response()
                        .unwrap()
                        .dyn_into::<js_sys::ArrayBuffer>()
                        .unwrap();
                    let response = Uint8Array::new(&response);
                    let response: Vec<u8> = response.to_vec();
                    let response: pot::Value = match pot::from_slice(&response) {
                        Ok(response) => response,
                        Err(e) => {
                            reject
                                .call1(&JsValue::UNDEFINED, &JsValue::from_str(&format!("{e:?}")))
                                .unwrap();
                            return;
                        }
                    };
                    let response: JsValue = match serde_wasm_bindgen::to_value(&response) {
                        Ok(response) => response,
                        Err(e) => {
                            reject
                                .call1(&JsValue::UNDEFINED, &JsValue::from_str(&format!("{e:?}")))
                                .unwrap();
                            return;
                        }
                    };
                    resolve.call1(&JsValue::UNDEFINED, &response).unwrap();
                } else {
                    reject
                        .call1(&JsValue::UNDEFINED, &xhr.response().unwrap())
                        .unwrap();
                }
            });
            xhr.set_onload(Some(handler.as_ref().unchecked_ref()));
            handler.forget();
        }
        xhr.open_with_async(
            "POST",
            &format!("http://tauriless.localhost/{command}", command = command.replace('_', "-")),
            true,
        )
        .unwrap();
        xhr.send_with_opt_u8_array(Some(&encoded)).unwrap();
    });

    Ok(promise.into())
}
