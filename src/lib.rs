use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod board;

use board::*;

mod tile;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}


// todo: these should live on the board
const GRID_WIDTH: i32 = 16;
const GRID_HEIGHT: i32 = 8;
// todo: move this to a 'render_info' struct?
const PIXELS_PER_SQUARE_SIDE: f64 = 20.0;


#[wasm_bindgen(start)]
pub fn start() {
    log("hi");
    console_error_panic_hook::set_once();
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

    let mut board = Board::new(40, GRID_WIDTH, GRID_HEIGHT);
    render_grid(&context, &board);


    let closure = Closure::wrap(Box::new(move |event: web_sys::MouseEvent| {
        let x = event.offset_x();
        let y = event.offset_y();
        log(&format!("x: {}, y: {}", x, y));
        log(&format!("{:?}", &board));
    }) as Box<dyn FnMut(_)>);

    let res = canvas.add_event_listener_with_callback(
        // todo: wtf is going on here.. as_ref.unchecked_ref()??
        "mousedown", closure.as_ref().unchecked_ref(),
    ).unwrap();

    log(&format!("{:?}", res));
    log(&format!("By the way 5 / 2 = {}", 5i32 / 2i32));
    closure.forget();
}

fn render_grid(context: &web_sys::CanvasRenderingContext2d, board: &Board) {
    let num_cells = board.get_total_number_of_cells();
    for i in 0..num_cells {
        let x = i % GRID_WIDTH;
        let y = i / GRID_WIDTH;
        context.stroke_rect(x as f64 * PIXELS_PER_SQUARE_SIDE,
                            y as f64 * PIXELS_PER_SQUARE_SIDE,
                            PIXELS_PER_SQUARE_SIDE,
                            PIXELS_PER_SQUARE_SIDE
        );
    }
}
