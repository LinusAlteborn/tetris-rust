use tetris::*;

fn main() {
    let mut user = Settings::start().unwrap().1;

    // Initierar struct för att hantera spel data, och output data.
    let mut game = GameState::new();
    let mut output = Output::new();

    // Eftersom inga block rört sig ännu kommer inte output att måla något. Vi måsta be den att måla bakgrunden genom denna metod.
    output.redraw(&game);

    // timers
    let mut respawn_timer = Instant::now().checked_sub(Duration::from_secs(999)).unwrap();
    let mut fall_timer = Instant::now();
    let mut redraw_timer = Instant::now();
    
    // fps räknare
    let mut fps = Fps::new(Duration::from_millis(1000));

    // Detta är själva loopen. Här använder vi alla våra hjälpmoduler med hjälpfunktioner och structs för att skriva spel logiken med konsis syntax.
    // 
    // Tanken med projektets struktur är att dela upp ansvaret i olika mindre delar för att lättare kunna navigera koden.
    // Om vi får problem med glitchar i utseendet kan jag till exempel vara väldigt säker på att det är någt fel i output modulen. Att lättare kunna felsöka är ett massivt plus.
    // 
    // Exakt hur vi delade upp här är inte lika viktigt som att vi delat upp projektet över huvudtaget. Tycker jag i alla fall.
    'game_loop: loop {
        // hantera input event
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

        // respawna
        if !game.alive() && respawn_timer.elapsed() > Duration::from_millis(500) {
            game.spawn();
            if let Some(_) = game.try_move(PlayerMove::Translate(0, 0)) {
                println!("game over");
                break 'game_loop;
            }
            fall_timer = Instant::now();
        }

        // automatiskt fall varje sekund
        if game.alive() && fall_timer.elapsed() > Duration::from_millis(1000) {
            if let Some(_) = game.try_move(PlayerMove::Translate(0, 1)) {
                game.kill_player();
                respawn_timer = Instant::now();
            }
            fall_timer = Instant::now();
        }

        // målar om hela output var tredje sekund för att motverka glitcher
        if redraw_timer.elapsed() > Duration::from_secs(3) {
            output.redraw(&game);
            redraw_timer = Instant::now();
        } else {
            output.draw(&game);
        }

        // sparar poäng i settings
        user.score = game.points as u32;

        // målar poängen till skärmen
        output.draw_score(format!("{points:0<5}", points=game.points));
        // visar fps i vänstra hörnet
        output.draw_fps(format!("fps {fps:.0}", fps=fps.fps));

        // updaterar bild räkningen
        fps.frame();
    }
}
