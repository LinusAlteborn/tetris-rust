use std::time::Duration;
use tetris::{io::input::InputEvent, *};

fn main() {
    let mut system = System::new();
    let mut fps = Fps::new(Duration::from_millis(1000));
    loop {
        match system.input.poll() {
            Some(event) => match event {
                InputEvent::Left => system.try_move(Move::Translate(Point::from_pos(-1.0, 0.0))),
                InputEvent::Right => system.try_move(Move::Translate(Point::from_pos(1.0, 0.0))),
                InputEvent::Down => system.try_move(Move::Translate(Point::from_pos(0.0, 1.0))),
                InputEvent::Rotate => system.try_move(Move::Rotate(1)),
                InputEvent::Drop => system.try_move(Move::Drop),
                InputEvent::Quit => {
                    println!("Buh, bye!");
                    std::process::exit(0);
                }
            },
            None => (),
        }

        system.check_move_timer();

        for loop_event in system.events.drain(..) {
            match loop_event {
                Gameloop_Events::Death => {
                    println!("You lose");
                    std::process::exit(0);
                }
                _ => (),
            }
        }

        system.output.update(&system.data);

        fps.frame();
    }
}
