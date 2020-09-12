pub mod music {
    use std::thread;
    use std::time;
    use vlc::{Instance, Media, MediaPlayer};

    pub async fn play_harold() {
        let instance = Instance::new().unwrap();
        let md = Media::new_path(&instance, "music").expect("path declaration failed");
        let duration = md.duration().expect("duration failed");

        println!("duration of harold is: {:?}", duration);

        let mdp = MediaPlayer::new(&instance).unwrap();
        mdp.set_media(&md);

        mdp.play().unwrap();
        thread::sleep(time::Duration::from_millis(duration as u64));
    }
}
