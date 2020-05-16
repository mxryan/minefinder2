use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod board;
mod tile;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log (s: &str);
}


const GRID_WIDTH: i32 = 20;
const GRID_HEIGHT: i32 = 20;
const PIXELS_PER_SQUARE_SIDE: f64 = 20.0;



#[wasm_bindgen(start)]
pub fn start() {
    log("hi");
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // draw_smiley(&context);
    draw_grid(&context);

    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        log("GOT A MOUSEDOWN!!!");
        let x = event.offset_x();
        let y = event.offset_y();
        log(&format!("x: {}, y: {}", x, y));
    }) as Box<dyn FnMut(_)>);

    let res = canvas.add_event_listener_with_callback(
        // todo: wtf is going on here.. as_ref.unchecked_ref()??
        "mousedown", closure.as_ref().unchecked_ref()
    ).unwrap();

    log(&format!("{:?}",res));
    // this is technically a memory leak. forget() drops the closure without
    // invalidating it. see:
    // https://rustwasm.github.io/docs/wasm-bindgen/examples/closures.html
    closure.forget();
}


// event listener
//https://stackoverflow.com/questions/9880279/how-do-i-add-a-simple-onclick-event-handler-to-a-canvas-element


fn draw_smiley(context: &web_sys::CanvasRenderingContext2d) {

    context.begin_path();

    // Draw the outer circle.
    context
        .arc(75.0, 75.0, 50.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the mouth.
    context.move_to(110.0, 75.0);
    context.arc(75.0, 75.0, 35.0, 0.0, f64::consts::PI).unwrap();

    // Draw the left eye.
    context.move_to(65.0, 65.0);
    context
        .arc(60.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    // Draw the right eye.
    context.move_to(95.0, 65.0);
    context
        .arc(90.0, 65.0, 5.0, 0.0, f64::consts::PI * 2.0)
        .unwrap();

    context.stroke();
}

fn draw_grid(context: &web_sys::CanvasRenderingContext2d) {
    for i in 0..GRID_WIDTH {
        for j in 0..GRID_HEIGHT {
            context.stroke_rect(i as f64 * PIXELS_PER_SQUARE_SIDE,
                                j as f64 * PIXELS_PER_SQUARE_SIDE,
                                PIXELS_PER_SQUARE_SIDE,
                                PIXELS_PER_SQUARE_SIDE);

        }
    }
}
