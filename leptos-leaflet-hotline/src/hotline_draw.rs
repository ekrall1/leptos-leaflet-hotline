use js_sys::{Array, Reflect, Function, Object, Math, JsString};
use wasm_bindgen::*;
use wasm_bindgen::prelude::*;
use std::ops::DerefMut;
use leptos::*;
use leptos_leaflet::leaflet as L;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, ImageData, window};
use std::collections::HashMap;

use crate::canvas::{Canvas, CanvasOptions};


#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct MakeHotline {
    _canvas: HtmlCanvasElement,
    _ctx: CanvasRenderingContext2d,
    _height: u32,
    _width: u32,
    _weight: f64,
    _outline_width: f64,
    _min: f64,
    _max: f64,
    _data: Array,
    _val_data: Array,
    palette: Clamped<Vec<u8>>,
    current_colorstop: String
}

#[wasm_bindgen]
impl MakeHotline {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: &Canvas, positions: &Array, values: &Array) -> Self {
        
        let canvas_clone = canvas.clone();

        let mut default_palette_hm: HashMap<&str, f32> = HashMap::new();
        default_palette_hm.insert("green", 0.0);
        default_palette_hm.insert("yellow", 0.5);
        default_palette_hm.insert("red", 1.0);
        let default_palette: Object = JsValue::from_serde(&default_palette_hm).unwrap().unchecked_into();

        MakeHotline {
            _canvas: canvas_clone._container(),
            _ctx: canvas_clone._ctx(),
            _height: canvas_clone._container().height(),
            _width: canvas_clone._container().width(),
            _weight: 5.0,
            _outline_width: 1.0,
            _min: 0.0,
            _max: 0.0,
            _data: positions.clone(),
            _val_data: values.clone(),
            palette: Self::palette(&default_palette),
            current_colorstop: "".to_string(),
        }
    }

    #[wasm_bindgen]
    pub fn data(&mut self, latlngs: &Array) -> MakeHotline {
        self._data = latlngs.clone();
        self.clone()
    }

    #[wasm_bindgen]
    pub fn val_data(mut self, vals: &Array) -> MakeHotline {
        self._val_data = vals.clone();
        self
    }

    #[wasm_bindgen]
    pub fn palette(palette: &Object) -> Clamped<Vec<u8>> {
        let new_canvas_element: HtmlCanvasElement = window().unwrap().document().unwrap().create_element("canvas").unwrap().clone()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
        new_canvas_element.set_width(1);
        new_canvas_element.set_height(256);

        let new_context = new_canvas_element.get_context("2d").unwrap().unwrap().dyn_into::<CanvasRenderingContext2d>().unwrap();
        let gradient = new_context.create_linear_gradient(0.0, 0.0, 0.0, 256.0);

        let palette_entries = Object::entries(palette);

        for entry in palette_entries {
            let vals: Array = entry.unchecked_into();
            let color: String = vals.get(0).as_string().unwrap();
            let offset: f32 = vals.get(1).as_f64().unwrap() as f32;
            let _ = gradient.add_color_stop(offset, &color);
        }

        new_context.set_fill_style(&gradient);
        new_context.fill_rect(0.0, 0.0, 1.0, 256.0);
        new_context.get_image_data(0.0, 0.0, 1.0, 256.0).unwrap().data()
    }

    #[wasm_bindgen]
    pub fn draw(mut self) -> MakeHotline {
        //let ctx = self._ctx;
        self._ctx.set_global_composite_operation("source-over");
        self._ctx.set_line_cap("round");
        self._draw_hotline();
        self
    }

    #[wasm_bindgen]
    pub fn _draw_hotline(&mut self) {
        let ctx = &self._ctx;
        ctx.set_line_width(self._weight);
        let (mut i, mut j, data_length): (u32, u32, u32) = (0, 1, 2);
        let str_start: JsString = JsString::from("rgb(");
        let str_end: JsString = JsString::from(")");
        while i < data_length {
            let path: Array = self._data.clone();
            let path_length = path.length();
            // let vec1 = path.to_vec();
            // let stri = vec1[0].as_string();
            let one = path.get(0).unchecked_into();
            let entries: JsValue = Object::entries(&one).unchecked_into();
            log(&js_sys::JSON::stringify(&Object::entries(&one).get(0)).unwrap().as_string().unwrap());
            while j < path_length {
                let point_start: Object = path.get(j-1).unchecked_into();
                let point_end: Object = path.get(j).unchecked_into();
                let lat_start: Array = Object::entries(&point_start).get(1).unchecked_into();
                let lon_start: Array = Object::entries(&point_start).get(0).unchecked_into();
                let lat_end: Array = Object::entries(&point_end).get(1).unchecked_into();
                let lon_end: Array = Object::entries(&point_end).get(0).unchecked_into();
                let gradient = ctx.create_linear_gradient(lat_start.get(1).unchecked_into_f64(), lon_start.get(1).unchecked_into_f64(), lat_end.get(1).unchecked_into_f64(), lon_end.get(1).unchecked_into_f64());
                let val_start: f64 = self._val_data.get(j-1).unchecked_into_f64();
                let val_end: f64 = self._val_data.get(j).unchecked_into_f64();
                let gradient_start_rgb = self.get_rgb_for_value(val_start);
                let gradient_end_rgb = self.get_rgb_for_value(val_end);
                let gradient_start_rgb_str = gradient_start_rgb.join(",");
                let gradient_stop_rgb_str = gradient_end_rgb.join(",");
                let color_stop_str1: String = str_start.concat(&gradient_start_rgb_str).concat(&str_end).into();
                let color_stop_str2: String = str_start.concat(&gradient_stop_rgb_str).concat(&str_end).into();
                gradient.add_color_stop(0.0, &color_stop_str1);
                gradient.add_color_stop(1.0, &color_stop_str2);
                self.current_colorstop=color_stop_str2;
                ctx.set_stroke_style(&gradient);
                ctx.move_to(lat_start.get(1).unchecked_into_f64(), lon_start.get(1).unchecked_into_f64());
                ctx.line_to(lat_end.get(1).unchecked_into_f64(), lon_end.get(1).unchecked_into_f64());
                ctx.stroke();
                j = j + 1;
            }
            i = i + 1;
        }
    }

    #[wasm_bindgen]
    pub fn get_rgb_for_value(&self, value: f64) -> Array {
        let value_relative = Math::min(Math::max((value - self._min)/(self._max - self._min), 0.0), 0.999);
        let palette_index = (Math::floor(value_relative * 256.0) * 4.0) as u32;
        let arr = Array::new();
        let r: JsValue = palette_index.into();
        let g: JsValue = (palette_index+1).into();
        let b: JsValue = (palette_index+2).into();
        arr.push(&r);
        arr.push(&g);
        arr.push(&b);
        arr
    }
}

// impl Data for MakeHotline {
//     fn data(mut self: MakeHotline, latlngs: &Array) -> MakeHotline {
//         self._data = latlngs.clone();
//         self.clone()
//     }
// }


#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct HotlineRenderer {
    _canvas: Canvas,
    _hotline: MakeHotline,
    _vals: Array,
}

#[wasm_bindgen]
impl HotlineRenderer {
    #[wasm_bindgen(constructor)]
    pub fn new(positions: &Array, values: &Array) -> Self {
        let canvas_opts = CanvasOptions::default();

        let canvas = Canvas::new(&canvas_opts);
        canvas._initContainer();
        //let ctr = canvas._container();
        let poly = MakeHotline::new(&canvas, &positions, &values).draw();
        HotlineRenderer {_canvas: canvas, _hotline: poly.clone(), _vals: values.clone()}
    }

    // #[wasm_bindgen]
    // pub fn palette(palette: Object) -> {

    // }
}



// #[wasm_bindgen]
// #[derive(Debug, Clone, PartialEq)]
// pub struct HotlineDraw {
//     _canvas: HtmlCanvasElement,
//     _ctx: CanvasRenderingContext2d,
//     _height: u32,
//     _width: u32,
//     _weight: f64,
//     _outline_width: f64,
//     _outline_color: String,
//     _min: f64,
//     _max: f64,
//     _data: Array,
//     _palette: Clamped<Vec<u8>>,
// }

// #[wasm_bindgen]
// impl HotlineDraw {
//     #[wasm_bindgen(constructor)]
//     pub fn new(canvas: &HtmlCanvasElement) -> HotlineDraw {
//         //let mut opts = CanvasOptions::new();
//         //let _hotline = Canvas::new(&opts);

//         let document = web_sys::window().unwrap().document().unwrap();
//         let container = document.create_element("canvas").unwrap();
//         let c2: HtmlCanvasElement = container
//             .clone()
//             .dyn_into::<web_sys::HtmlCanvasElement>()
//             .map_err(|_| ())
//             .unwrap();
//         let ctx = c2
//             .get_context("2d")
//             .unwrap()
//             .unwrap()
//             .dyn_into::<CanvasRenderingContext2d>()
//             .unwrap();
//         let gradient = ctx.create_linear_gradient(0.0, 0.0, 0.0, 256.0);
//         c2.set_width(1);
//         c2.set_height(256);
//         gradient.add_color_stop(0.0, "green");
//         gradient.add_color_stop(0.5, "yellow");
//         gradient.add_color_stop(1.0, "red");
//         ctx.set_fill_style(&gradient);
//         ctx.fill_rect(0.0, 0.0, 1.0, 256.0);
//         let img = ctx.get_image_data(0.0, 0.0, 1.0, 256.0).unwrap().data();

//         HotlineDraw {
//             _canvas: canvas.clone(),
//             _ctx: get_rendering_context_from_canvas(canvas),
//             _height: canvas.clone().height(),
//             _width: canvas.clone().width(),
//             _weight: 5.0,
//             _outline_width: 1.0,
//             _outline_color: "black".to_string(),
//             _min: 0.0,
//             _max: 1.0,
//             _data: Array::new(),
//             _palette: img,
//         }
//     }

//     #[wasm_bindgen]
//     pub fn add(self: HotlineDraw, data: &Array) -> HotlineDraw {
//         self.h_data.push(data);
//         self
//     }

//     #[wasm_bindgen]
//     pub fn draw(self: HotlineDraw) -> HotlineDraw {
//         let ctx = &self.h_ctx;
//         ctx.set_global_composite_operation("source-over");
//         ctx.set_line_cap("round");
//         self._draw_outline();
//         self
//     }

//     #[wasm_bindgen]
//     pub fn _draw_outline(self: &HotlineDraw) -> HotlineDraw {
//         let (
//             mut i,
//             mut j,
//             mut data_length,
//             mut path,
//             mut path_length,
//             mut point_start,
//             mut point_end,
//         ): (u32, u32, u32, u32, u32, f64, f64) = (0, 1, 0, 0, 0, 0.0, 0.0);

//         while i < self.h_data.length() {
//             let path = self.h_data.get(i);
//             self.h_ctx.set_line_width(3.0);
//             while j < self.h_data.length() { //path.length {
//                 point_start = self.h_data.get(j-1).as_f64().unwrap();
//                 point_end = self.h_data.get(j).as_f64().unwrap();
//                 self.h_ctx.set_stroke_style(&JsValue::from_str(&self.h_outline_color));
//                 self.h_ctx.begin_path();
//                 self.h_ctx.move_to(0.5, point_start);
//                 self.h_ctx.line_to(300.0, point_end);
//                 self.h_ctx.stroke();
//                 j = j + 1;
//             }
//             i = i + 1;
//         }

//         self.clone()
//     }
// }
// Define a trait for methods shared between L.Canvas and Renderer
// trait CanvasLike {
//     fn _init_container(&self);
// }

// // Stub for L.Canvas
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = L)]
//     type Canvas;

//     #[wasm_bindgen(method)]
//     fn _init_container(this: &Canvas);
// }

// // Hotline class if available in your JavaScript code
// // (you'll need to define this as well)
// #[wasm_bindgen]
// extern "C" {
//     type Hotline;
// }

// // Define the Renderer class
// #[wasm_bindgen]
// pub struct Renderer {
//     _hotline: Hotline,
// }

// // Implement the CanvasLike trait for Renderer
// #[wasm_bindgen]
// impl CanvasLike for Renderer {
//     fn _init_container(&self) {
//         // Implement your custom initialization logic here
//     }
// }

// #[wasm_bindgen]
// impl Renderer {
//     #[wasm_bindgen(constructor)]
//     pub fn new() -> Self {
//         // Create a new Renderer instance
//         let renderer = Renderer {
//             _hotline: Hotline::new(&Object::new()),
//         };

//         // Call the _init_container method via the trait
//         CanvasLike::_init_container(&renderer);

//         renderer
//     }
// }
