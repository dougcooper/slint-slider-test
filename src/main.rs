slint::slint! {
    import { Slider , Button} from "std-widgets.slint";
    export component SliderTest inherits Window {
        width: 600px;
        height: 1024px;
        callback start-pressed <=> start_button.clicked;
        callback stop-pressed <=> stop_button.clicked;
        callback slider-changed <=> slider.changed;
        in-out property <float> slider-value: slider.value ;
        out property <float> slider-min: slider.minimum ;
        out property <float> slider-max: slider.maximum ;
        out property <float> slider-tick: 1 ;
        in property <string> label-text: text.text;
        in-out property <bool> btn-enabled: true;

        VerticalLayout {
            width: 50%;
            spacing: 5px;
            Text {
                text: "Rusty Slider Test";
                horizontal-alignment: center;
                vertical-alignment: center;
                font-size: 30px;
            }
            HorizontalLayout {
                spacing: 5px;
                start_button := Button {
                    height: 25px;
                    width: 50%;
                    text: "Start";
                    enabled: btn-enabled;
                    clicked => { btn-enabled = !btn-enabled }
                }
                stop_button := Button {
                    height: 25px;
                    text: "Stop";
                    enabled: !btn-enabled;
                    clicked => { btn-enabled = !btn-enabled }
                }
             }
            slider := Slider {
                width: 75%;
                height: 25px;
                value: 0;
                minimum: 0;
                maximum: 10;
                enabled: true;
            }
            text:=Text {
                text: "Click Start";
                wrap: word-wrap;
            }
        }
    }
}

use std::{
    sync::{Arc, RwLock},
    time::Instant,
};

use slint::{SharedString, Timer, TimerMode};

fn main() {
    let recipe = SliderTest::new();

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
            std::time::Duration::from_millis(1000),
            move || {
                let elapsed1 = elapsed1.clone();
                let _ = recipe_weak1.upgrade_in_event_loop(move |recipe| {
                    let mut value = recipe.get_slider_value();
                    let max = recipe.get_slider_max();
                    let min = recipe.get_slider_min();
                    if value == max {
                        value = min
                    } else {
                        value += recipe.get_slider_tick();
                    }
                    let elapsed1 = elapsed1.clone();
                    if let Some(mut elapsed) = elapsed1.write().ok() {
                        *elapsed = Some(Instant::now());
                    }
                    recipe.set_slider_value(value);
                    println!("slider set to {}",value);
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

    let recipe_weak2 = recipe.as_weak();
    let elapsed2 = elapsed.clone();
    let mut data = vec![];
    recipe.on_slider_changed(move |_| {
        let now = Instant::now();

        if let Some(clicked) = elapsed2.read().ok() {
            let d = now.duration_since(clicked.unwrap_or(now));
            data.push(d.as_micros());
            let recipe = recipe_weak2.upgrade().unwrap();
            recipe.set_label_text(SharedString::from(format!("{} us", d.as_micros())));
        }
        println!("slider updated. data len {}",data.len());
    });

    recipe.show();
    slint::run_event_loop()
}
