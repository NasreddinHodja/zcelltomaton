use nannou::prelude::*;
use nannou::winit::event::MouseButton;

mod automaton;
use automaton::Automaton;

mod board;
use board::Board;

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const BOARD_ROWS: usize = 64;
const BOARD_COLS: usize = 64;

const STATE_COLORS: [Srgb<u8>; 2] = [BLACK, PLUM];

struct Model {
    board: Board<BOARD_ROWS, BOARD_COLS>,
    next_board: Board<BOARD_ROWS, BOARD_COLS>,
    automaton: Automaton,
    playing: bool,
}

fn model(app: &App) -> Model {
    app.new_window()
        .size_pixels(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .build()
        .unwrap();

    let board: Board<BOARD_ROWS, BOARD_COLS> = Board::new();
    let next_board: Board<BOARD_ROWS, BOARD_COLS> = Board::new();
    let automaton = Automaton::bb();
    let playing = false;

    Model {
        board,
        next_board,
        automaton,
        playing,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if model.playing {
        step(model)
    }
}

fn handle_mousepress(button: MouseButton, app: &App, model: &mut Model) {
    let win = app.window_rect();
    let cell_w = win.w() / BOARD_COLS as f32;
    let cell_h = win.h() / BOARD_ROWS as f32;
    let window_w = app.window_rect().w();
    let window_h = app.window_rect().h();
    let pos = app.mouse.position();
    let pos = model
        .board
        .position_to_rc(pos.x, pos.y, cell_h, cell_w, window_w, window_h);
    let mut state = 0;
    match button {
        MouseButton::Left => state = 1,
        MouseButton::Right => state = 0,
        _ => {}
    }
    model.board.set_cell(pos.x as usize, pos.y as usize, state);
}

fn step(model: &mut Model) {
    model
        .board
        .compute_next(&model.automaton, &mut model.next_board);
    (model.board, model.next_board) = (model.next_board.clone(), model.board.clone())
}

fn event(app: &App, model: &mut Model, event: Event) {
    match event {
        Event::WindowEvent {
            id: _,
            simple: Some(event),
        } => match event {
            MousePressed(MouseButton::Left) => {
                handle_mousepress(MouseButton::Left, &app, model);
            }
            MousePressed(MouseButton::Right) => {
                handle_mousepress(MouseButton::Right, &app, model);
            }
            KeyPressed(Key::Space) => step(model),
            KeyPressed(Key::P) => {
                model.playing = !model.playing;
            }
            _ => {}
        },
        // Event::DeviceEvent(_, _) => todo!(),
        // Event::Update(_) => todo!(),
        // Event::Suspended => todo!(),
        // Event::Resumed => todo!(),
        _ => {}
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(STATE_COLORS[0]);

    let win = app.window_rect();
    let cell_w = win.w() / BOARD_COLS as f32;
    let cell_h = win.h() / BOARD_ROWS as f32;

    for r in 0..BOARD_ROWS {
        for c in 0..BOARD_COLS {
            let x = -(win.w() / 2.0) + cell_w * c as f32 + cell_w / 2.0;
            let y = win.h() / 2.0 - cell_h * r as f32 - cell_h / 2.0;
            let cell = model.board.get_cell(r, c);
            let state = match model.automaton.get_state(cell) {
                Some(cell) => cell,
                None => panic!("No state found for cell {cell}"),
            };
            draw.rect().x_y(x, y).w_h(cell_w, cell_h).color(state.color);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).event(event).run();
}
