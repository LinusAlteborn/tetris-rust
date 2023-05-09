use tetris::{io::{input::*, output::*}, *};

fn main() {
    let mut user = Settings::start().unwrap().1;

    let mut game = GameState::new();
    let mut output = Output::new();

    output.redraw(&game);

    let mut respawn_timer = Instant::now().checked_sub(Duration::from_secs(999)).unwrap();
    let mut fall_timer = Instant::now();
    let mut redraw_timer = Instant::now();
    
    let mut fps = Fps::new(Duration::from_millis(1000));

    'game_loop: loop {
        if let Some(input) = input() {
            match input {
                InputEvent::Left => {
                    game.try_move(PlayerMove::Translate(-1, 0));
                    ()
                },
                InputEvent::Right => {
                    game.try_move(PlayerMove::Translate(1, 0));
                    ()
                },
                InputEvent::Rotate => {
                    game.try_move(PlayerMove::Rotate(1));
                    ()
                },
                InputEvent::Drop => {
                    for _ in 0..ROWS {
                        game.try_move(PlayerMove::Translate(0, 1));
                    }
                    game.kill_player();
                    respawn_timer = Instant::now();
                },
                InputEvent::Down => {
                    if let Some(_) = game.try_move(PlayerMove::Translate(0, 1)) {
                        game.kill_player();
                        respawn_timer = Instant::now();
                    }
                    fall_timer = Instant::now();
                },
                InputEvent::Quit => {
                    println!("Buh, Bye!");
                    break 'game_loop;
                }
            }
        }

        if !game.alive() && respawn_timer.elapsed() > Duration::from_millis(500) {
            game.spawn();
            fall_timer = Instant::now();
        }

        if game.alive() && fall_timer.elapsed() > Duration::from_millis(1000) {
            if let Some(_) = game.try_move(PlayerMove::Translate(0, 1)) {
                game.kill_player();
                respawn_timer = Instant::now();
            }
            fall_timer = Instant::now();
        }

        if redraw_timer.elapsed() > Duration::from_secs(3) {
            output.redraw(&game);
            redraw_timer = Instant::now();
        } else {
            output.draw(&game);
        }

        user.score = game.points as u32;

        output.draw_score(format!("{points:0<5}", points=game.points));
        output.draw_fps(format!("fps {fps:.0}", fps=fps.fps));

        fps.frame();
    }
}
