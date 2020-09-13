pub mod music {
    use rand::{thread_rng, Rng};
    use rppal::gpio::{Error, Gpio};
    use std::{thread, time};
    use vlc::{Instance, Media, MediaPlayer};

    pub async fn play_harold(media_name: &'static str, lights: bool) {
        thread::spawn(move || {
            let instance = Instance::new().unwrap();
            let md = Media::new_path(&instance, media_name).expect("path declaration failed");

            let mdp = MediaPlayer::new(&instance).unwrap();
            mdp.set_media(&md);
            mdp.play().unwrap();

            let mut rng = thread_rng();

            thread::sleep(time::Duration::from_millis(10));

            loop {
                if lights {
                    set_lights(rng.gen_bool(0.5), rng.gen_bool(0.5), rng.gen_bool(0.5)).unwrap();
                }
                thread::sleep(time::Duration::from_millis(200));
                if !mdp.is_playing() {
                    break;
                }
            }
            set_lights(false, false, false).unwrap();
        });
    }

    pub fn set_lights(red: bool, green: bool, blue: bool) -> rppal::gpio::Result<()> {
        const red_pin: u8 = 17;
        const green_pin: u8 = 22;
        const blue_pin: u8 = 27;
        let gpio = Gpio::new()?;

        let mut red_gpio = gpio.get(red_pin)?.into_output();
        let mut green_gpio = gpio.get(green_pin)?.into_output();
        let mut blue_gpio = gpio.get(blue_pin)?.into_output();

        if red {
            red_gpio.set_high();
        }
        if green {
            green_gpio.set_high();
        }
        if blue {
            blue_gpio.set_high();
        }
        Ok(())
    }
}
