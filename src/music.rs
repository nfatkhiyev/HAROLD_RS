pub mod music {
    use rand::{thread_rng, Rng};
    use rppal::gpio::{Error, Gpio};
    use std::{thread, time};
    use vlc::{Instance, Media, MediaPlayer};

    //plays music and flashes lights. Returns void
    //TODO: add serial LED strip to the lights sequence and double check functionality
    pub async fn play_harold(media_name: &'static str, lights: bool) {
        thread::spawn(move || {
            let instance = Instance::new().unwrap();
            let md = Media::new_path(&instance, media_name).expect("path declaration failed");

            let mdp = MediaPlayer::new(&instance).unwrap();
            mdp.set_media(&md);
            mdp.play().unwrap();

            let timer = time::Instant::now();

            let mut rng = thread_rng();

            thread::sleep(time::Duration::from_millis(10));

            loop {
                if lights {
                    set_lights(rng.gen_bool(0.5), rng.gen_bool(0.5), rng.gen_bool(0.5)).unwrap();
                }
                thread::sleep(time::Duration::from_millis(200));
                if !mdp.is_playing() || timer.elapsed().as_secs() > 30 {
                    break;
                }
            }
            set_lights(false, false, false).unwrap();
        });
    }

    //sets the state of the light bar. Returns void
    pub fn set_lights(red: bool, green: bool, blue: bool) -> rppal::gpio::Result<()> {
        const RED_PIN: u8 = 17;
        const GREEN_PIN: u8 = 22;
        const BLUE_PIN: u8 = 27;
        let gpio = Gpio::new()?;

        let mut red_gpio = gpio.get(RED_PIN)?.into_output();
        let mut green_gpio = gpio.get(GREEN_PIN)?.into_output();
        let mut blue_gpio = gpio.get(BLUE_PIN)?.into_output();

        if red {
            red_gpio.set_high();
        } else {
            red_gpio.set_low();
        }
        if green {
            green_gpio.set_high();
        } else {
            green_gpio.set_low();
        }
        if blue {
            blue_gpio.set_high();
        } else {
            blue_gpio.set_low();
        }
        Ok(())
    }
}
