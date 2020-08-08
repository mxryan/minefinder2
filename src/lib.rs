use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

mod board;
mod cell;
use board::*;
use cell::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

const GRID_WIDTH: usize = 16;
const GRID_HEIGHT: usize = 16;
const LEN: f64 = 20.0;
const RGBA_MAGENTA: &str = "rgba(255,0,255,1)";
const RGBA_CYAN: &str = "rgba(0,255,255,1)";
const RGBA_WHITE: &str = "rgba(255,255,255,1)";


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

    let handle_click = move |event: web_sys::MouseEvent| {
        let (x, y) = convert_coords_click_event_to_board(
            event.offset_x(),
            event.offset_y(),
        );

        let click = match event.button() {
            0 => Click::Left,
            2 => Click::Right,
            _ => return
        };

        log(&format!("x: {}, y: {}", x, y));

        if !game_running {
            board.place_mines(x as usize, y as usize);
            board.set_cells_num_bomb_neighbors();
            board.update_state(x as usize, y as usize, click);
            game_running = true;
        } else if game_running {
            board.update_state(x as usize, y as usize, click);
        }

        render_grid(&context, &board);
    };

    let click_handler = Closure::wrap(Box::new(handle_click) as Box<dyn FnMut(_)>);

    let context_menu_cb = Closure::wrap(
        Box::new(
            |event: web_sys::MouseEvent| { event.prevent_default(); }
        ) as Box<dyn FnMut(_)>
    );

    let click_handler_unchecked = click_handler.as_ref().unchecked_ref();
    canvas
        .add_event_listener_with_callback("mousedown", click_handler_unchecked)
        .expect("Failed to add the mousedown event listener");
    click_handler.forget();

    let context_menu_cb_unchecked = context_menu_cb.as_ref().unchecked_ref();
    canvas
        .add_event_listener_with_callback("contextmenu", context_menu_cb_unchecked)
        .expect("Failed to add the contextmenu event listener.");
    context_menu_cb.forget();
}

fn render_grid(context: &web_sys::CanvasRenderingContext2d, board: &Board) {
    let num_cells = board.get_total_number_of_cells();

    for i in 0..num_cells {
        let x = (i % GRID_WIDTH) as i32;
        let y = (i / GRID_WIDTH) as i32;
        let (canv_x, canv_y) = convert_coords_board_to_canvas(x, y);

        // todo: only draw grid once. can draw grid with lines instead of rects
        context.stroke_rect(canv_x, canv_y, LEN, LEN);

        let font = "14px serif";
        context.set_font(font);

        let font_x = canv_x + LEN / 2.5;
        let font_y = canv_y + LEN / 1.5;

        match board.cells[i].state {
            CellState::Hidden => {
                context.set_fill_style(&JsValue::from_str(RGBA_MAGENTA));
                context.fill_rect(canv_x, canv_y, LEN, LEN);
            }

            CellState::Flagged => {
                context.set_fill_style(&JsValue::from_str(RGBA_MAGENTA));
                context.fill_rect(canv_x, canv_y, LEN, LEN);
                context.stroke_text("F", font_x, font_y);
            }

            CellState::Revealed => {
                context.set_fill_style(&JsValue::from_str(RGBA_CYAN));
                context.fill_rect(canv_x, canv_y, LEN, LEN);

                if board.cells[i].has_mine {
                    context.stroke_text("X", font_x, font_y)
                        .expect("Failed context.stroke_text");
                } else {
                    let text = &format!("{}", board.cells[i].neighboring_mines);
                    context.stroke_text(text, font_x, font_y)
                        .expect("Failed context.stroke_text");
                }
            }
        }
    }
}

/// Converts offset_x and offset_y from click event to game board coordinates
fn convert_coords_click_event_to_board(x: i32, y: i32) -> (i32, i32) {
    (x / LEN as i32, y / LEN as i32)
}

/// Converts board coordinates to canvas coordinates (draw functions expect f64)
fn convert_coords_board_to_canvas(x: i32, y: i32) -> (f64, f64) {
    (x as f64 * LEN, y as f64 * LEN)
}

