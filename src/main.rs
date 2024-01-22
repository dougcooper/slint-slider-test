use std::{
    rc::Rc,
    sync::{Arc, RwLock},
    time::Instant,
};

use slint::{Model, ModelRc, Timer, TimerMode, VecModel};

use rand::Rng;

slint::include_modules!();

fn main() {
    let recipe = SliderTest::new().unwrap();

    let elapsed: Arc<RwLock<Option<Instant>>> = Arc::new(RwLock::new(None));
    let timer = Arc::new(Timer::default());

    let recipe_weak1 = recipe.as_weak();
    let elapsed1 = elapsed.clone();
    let timer1 = timer.clone();
    recipe.on_start_pressed(move || {
        let recipe = recipe_weak1.upgrade().unwrap();
        recipe.set_btn_enabled(!recipe.get_btn_enabled());
        let elapsed1 = elapsed1.clone();
        let recipe_weak1 = recipe.as_weak();
        timer1.start(
            TimerMode::Repeated,
            std::time::Duration::from_millis(200),
            move || {
                let elapsed1 = elapsed1.clone();
                let _ = recipe_weak1.upgrade_in_event_loop(move |recipe| {
                    let curr_values = recipe.get_slider_values();
                    let count = curr_values.row_count();
                    let mut rng = rand::thread_rng();

                    let max = recipe.get_slider_max();
                    let min = recipe.get_slider_min();

                    let r: Vec<f32> = (0..count).map(|_| rng.gen_range(min..=max)).collect();

                    let values = ModelRc::from(Rc::new(VecModel::from(r)));
                    recipe.set_slider_values(values);
                    // let elapsed1 = elapsed1.clone();
                    if let Ok(mut elapsed) = elapsed1.write() {
                        *elapsed = Some(Instant::now());
                    }
                    // println!("slider set to {}",value);
                });
                // println!("timer called");
            },
        );
        println!("on start pressed");
    });

    let timer2 = timer.clone();
    let recipe_weak2 = recipe.as_weak();
    recipe.on_stop_pressed(move || {
        timer2.stop();
        let recipe = recipe_weak2.upgrade().unwrap();
        recipe.set_btn_enabled(!recipe.get_btn_enabled());
        println!("on stop pressed");
        //TODO: compute stats and update text
    });

    recipe.show().unwrap();
    slint::run_event_loop().unwrap();
}
