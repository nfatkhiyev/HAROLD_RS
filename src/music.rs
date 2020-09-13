pub mod music {
    use std::{thread, time};
    use vlc::{Instance, Media, MediaPlayer};

    pub async fn play_harold(media_name: &'static str, lights: bool) {
        thread::spawn(move || {
            let instance = Instance::new().unwrap();
            let md = Media::new_path(&instance, media_name).expect("path declaration failed");

            let mdp = MediaPlayer::new(&instance).unwrap();
            mdp.set_media(&md);
            mdp.play().unwrap();

            thread::sleep(time::Duration::from_millis(10));

            while mdp.is_playing() {
                thread::sleep(time::Duration::from_millis(1));
            }
        });
    }
}
