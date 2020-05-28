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


const GRID_WIDTH: usize = 16;
const GRID_HEIGHT: usize = 16;
const PIXELS_PER_SQUARE_SIDE: f64 = 20.0;
// todo: test blend mode of canvas by using alpha less than 1 and seeing how
//  colors mix. Is it necessary to clear_rect before each fill_rect?
const RGBA_MAGENTA: &str = "rgba(255,0,255,1)";
const RGBA_CYAN: &str = "rgba(0,255,255,1)";


fn get_canvas() -> web_sys::HtmlCanvasElement {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas
}

fn get_2d_context(canvas: &web_sys::HtmlCanvasElement)
                  -> web_sys::CanvasRenderingContext2d {
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    context
}


#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    let canvas = get_canvas();
    let context = get_2d_context(&canvas);
    let mut game_running = false;
    let mut board = Board::new(40, GRID_WIDTH, GRID_HEIGHT);


    render_grid(&context, &board);

    let cloz = move |event: web_sys::MouseEvent| {
        let x = event.offset_x();
        let y = event.offset_y();

        log(&format!("x: {}, y: {}", x, y));

        if !game_running {
            board.place_mines();
            board.set_cells_num_bomb_neighbors();
            game_running = true;
            log("mines placed");
        } else {

            let click = match event.button() {
                0 => Click::Left,
                2 => Click::Right,
                _ => return
            };

            // convert the offset_x and offset_y to board coords
            let (a, b) = convert_coords(x, y);
            board.update_state(a as usize, b as usize, click);
            log("game is running - updateBoardState()");
        }

        render_grid(&context, &board);
    };

    let click_handler = Closure::wrap(Box::new(cloz) as Box<dyn FnMut(_)>);

    let context_menu_cb = Closure::wrap(
        Box::new(|event: web_sys::MouseEvent| {
            event.prevent_default();
        }) as Box<dyn FnMut(_)>);

    canvas.add_event_listener_with_callback(
        "mousedown", click_handler.as_ref().unchecked_ref(),
    ).expect("Failed to add the mousedown event listener");
    click_handler.forget();

    canvas.add_event_listener_with_callback(
        "contextmenu", context_menu_cb.as_ref().unchecked_ref(),
    ).expect("Failed to add the contextmenu event listener.");
    context_menu_cb.forget();
}

fn render_grid(context: &web_sys::CanvasRenderingContext2d, board: &Board) {
    let num_cells = board.get_total_number_of_cells();
    for i in 0..num_cells {
        let x = i % GRID_WIDTH;
        let y = i / GRID_WIDTH;
        let fill_style = JsValue::from_str(match board.cells[i].state {
            TileState::Revealed => RGBA_CYAN,
            _ => RGBA_MAGENTA
        });
        if fill_style == RGBA_CYAN {
            log("YOOOO ITS CYAN");
        }
        context.set_fill_style(&fill_style);
        context.fill_rect(x as f64 * PIXELS_PER_SQUARE_SIDE,
                          y as f64 * PIXELS_PER_SQUARE_SIDE,
                          PIXELS_PER_SQUARE_SIDE,
                          PIXELS_PER_SQUARE_SIDE,
        );
        // todo: this only needs to be called first time the board is drawn
        context.stroke_rect(x as f64 * PIXELS_PER_SQUARE_SIDE,
                            y as f64 * PIXELS_PER_SQUARE_SIDE,
                            PIXELS_PER_SQUARE_SIDE,
                            PIXELS_PER_SQUARE_SIDE,
        );
    }
}

fn convert_coords(x: i32, y:i32) -> (i32, i32) {
    (x / PIXELS_PER_SQUARE_SIDE as i32, y / PIXELS_PER_SQUARE_SIDE as i32)
}


