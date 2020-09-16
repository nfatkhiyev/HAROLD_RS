use serialport::{DataBits, FlowControl, Parity, SerialPortSettings, StopBits};
use std::io::{self, BufRead, BufReader, ErrorKind};
use std::{path::Path, thread, time};
use tokio::runtime::Runtime;

mod music;
mod requests;
mod secrets;

#[tokio::main]
async fn main() {
    //initialize harold_secrets struct
    let harold_secrets = secrets::secrets::initialized_secrets();
    //initialize settings for serial ports
    let serial_settings = serialport::SerialPortSettings {
        baud_rate: 9600,
        data_bits: DataBits::Eight,
        flow_control: FlowControl::None,
        parity: Parity::None,
        stop_bits: StopBits::One,
        timeout: time::Duration::from_secs(1),
    };

    //setup iButton reader serial port, buffer reader and clone.
    //TODO: investigate if using clone to flush the serial buffer causes scanComplete to loop
    let button_reader = serialport::open_with_settings("/dev/ttyACM0", &serial_settings).unwrap();
    let clone = button_reader.try_clone().unwrap();
    let mut buf_reader = BufReader::new(button_reader);

    //main loop
    loop {
        let mut msg = String::new();

        //match line read from iButton reader. Must be matched or main will panic due to timeout
        match buf_reader.read_line(&mut msg) {
            Ok(_) => {
                //Remove new line and end character from serial
                msg.pop();
                msg.pop();
                //match msg to make sure that ready does not cause false positive
                match msg.as_str() {
                    "ready" => (),
                    _ => {
                        //remove iButton family characters and create string used for LDAP search
                        let i_button_code = msg.split_off(2);
                        let mut i_button_complete: String = "(ibutton=*".to_string();
                        i_button_complete.push_str(i_button_code.as_str());
                        i_button_complete.push_str(")");

                        //variables for music file names
                        let scan_complete: &'static str = "scanComplete";
                        let harold_name: &'static str = "music";
                        let mut music: &'static str = scan_complete;

                        //LDAP search.
                        let function_uid =
                            requests::requests::get_uid(i_button_complete.as_str(), harold_secrets);
                        let used_function_uid = function_uid.await.unwrap();

                        //makes sure that ldap search returns something
                        match used_function_uid.as_str() {
                            "" => (),
                            _ => {
                                let future_retrieve_harold =
                                    harold_retriever(&used_function_uid, harold_secrets);

                                let scan_complete_future = music::music::play_harold(music, false);
                                //dl harold and play scanComplete.
                                //TODO: investigate dl on separate thread
                                tokio::join!(future_retrieve_harold, scan_complete_future);

                                //change music file name to harold file name
                                music = harold_name;

                                //play harold
                                let harold_future = music::music::play_harold(music, true);
                                harold_future.await;
                                //wait 2 seconds before looping
                                thread::sleep(time::Duration::from_secs(2));
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

//makes all calls to dl music and notify JumpStart
async fn harold_retriever(used_function_uid: &String, harold_secrets: secrets::secrets::Secrets) {
    let future_s3_link = requests::requests::get_s3_link(used_function_uid, harold_secrets);
    let future_jumpstart_request =
        requests::requests::update_jumpstart(used_function_uid, harold_secrets);
    //get s3 link and notify JumpStart at the same time
    let (used_future_s3_link, _second) = tokio::join!(future_s3_link, future_jumpstart_request);
    let s3_link: &String = &used_future_s3_link.unwrap();
    //dl music
    let future_music_file = requests::requests::get_music_file(s3_link);
    future_music_file.await.expect("music file failed");
}
