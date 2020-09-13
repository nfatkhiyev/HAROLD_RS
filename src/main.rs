use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};
use rppal::uart::{Parity, Queue, Uart};
use std::{path::Path, time};
use tokio::runtime::Runtime;

mod music;
mod requests;
mod secrets;

#[tokio::main]
async fn main() {
    let usb_path = Path::new("/dev/ttyACM0");

    let mut uart =
        Uart::with_path(usb_path, 9_600, Parity::None, 8, 1).expect("uart creation failed");

    loop {
        uart.set_read_mode(6, time::Duration::from_millis(500))
            .expect("set read mode error");

        //let mut buffer = [0u8; 14];

        //if uart.read(&mut buffer).unwrap() > 6 {
        //println!("{:?}", buffer);
        //uart.flush(Queue::Both).expect("uart flush failed");
        let scan_complete: &'static str = "scanComplete";
        let harold_name: &'static str = "music";
        let mut music: &'static str = scan_complete;

        let harold_secrets = secrets::secrets::initialized_secrets();

        let future_retrieve_harold = harold_retriever(harold_secrets);

        let scan_complete_future = music::music::play_harold(music, false);

        tokio::join!(future_retrieve_harold, scan_complete_future);

        music = harold_name;

        let harold_future = music::music::play_harold(music, true);

        harold_future.await;
        //}
    }
}

async fn harold_retriever(harold_secrets: secrets::secrets::Secrets) {
    let function_uid =
        requests::requests::get_uid(harold_secrets.get_nate_ibutton(), harold_secrets);
    let used_function_uid = &function_uid.await.unwrap();

    let future_s3_link = requests::requests::get_s3_link(used_function_uid, harold_secrets);

    let used_future_s3_link = &future_s3_link.await.unwrap();

    let future_music_file = requests::requests::get_music_file(used_future_s3_link);
    future_music_file.await.expect("music file failed");
}
