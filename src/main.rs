use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};
use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::{path::Path, thread, time};
use tokio::runtime::Runtime;

mod music;
mod requests;
mod secrets;

#[tokio::main]
async fn main() {
    let harold_secrets = secrets::secrets::initialized_secrets();
    let button_reader_settings = serialport::SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: time::Duration::from_secs(1),
    };

    let button_reader =
        serialport::open_with_settings("/dev/ttyACM0", &button_reader_settings).unwrap();
    let clone = button_reader.try_clone().unwrap();
    let mut buf_reader = BufReader::new(button_reader);

    loop {
        let mut msg = String::new();

        match buf_reader.read_line(&mut msg) {
            Ok(_) => {
                msg.pop();
                msg.pop();
                match msg.as_str() {
                    "ready" => (),
                    _ => {
                        let i_button_code = msg.split_off(2);
                        let mut i_button_complete: String = "(ibutton=*".to_string();
                        i_button_complete.push_str(i_button_code.as_str());
                        i_button_complete.push_str(")");

                        let scan_complete: &'static str = "scanComplete";
                        let harold_name: &'static str = "music";
                        let mut music: &'static str = scan_complete;

                        let function_uid =
                            requests::requests::get_uid(i_button_complete.as_str(), harold_secrets);
                        let used_function_uid = function_uid.await.unwrap();

                        match used_function_uid.as_str() {
                            "" => (),
                            _ => {
                                let future_retrieve_harold =
                                    harold_retriever(&used_function_uid, harold_secrets);

                                let scan_complete_future = music::music::play_harold(music, false);

                                tokio::join!(future_retrieve_harold, scan_complete_future);

                                music = harold_name;

                                let harold_future = music::music::play_harold(music, true);

                                harold_future.await;
                                thread::sleep(time::Duration::from_millis(2));
                            }
                        }
                    }
                };
            }

            Err(ref e) if e.kind() == ErrorKind::TimedOut => (),

            Err(e) => eprintln!("{:?}", e),
        };
    }
}

async fn harold_retriever(used_function_uid: &String, harold_secrets: secrets::secrets::Secrets) {
    let future_s3_link = requests::requests::get_s3_link(used_function_uid, harold_secrets);
    let future_jumpstart_request =
        requests::requests::update_jumpstart(used_function_uid, harold_secrets);
    future_jumpstart_request.await.unwrap();
    let used_future_s3_link = &future_s3_link.await.unwrap();

    let future_music_file = requests::requests::get_music_file(used_future_s3_link);
    future_music_file.await.expect("music file failed");
}
