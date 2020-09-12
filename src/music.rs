pub mod music {
    use std::{thread, time};
    use vlc::{Instance, Media, MediaPlayer};

    pub async unsafe fn play_harold(media_name: &str, player_state_ptr: *mut bool) {
        let instance = Instance::new().unwrap();
        let md = Media::new_path(&instance, media_name).expect("path declaration failed");

        let mdp = MediaPlayer::new(&instance).unwrap();
        mdp.set_media(&md);

        mdp.play().unwrap();

        thread::sleep(time::Duration::from_millis(10));

        while mdp.is_playing() {
            *player_state_ptr = mdp.is_playing();
            thread::sleep(time::Duration::from_millis(1));
        }
    }
}
