use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};
use tokio::runtime::Runtime;

mod music;
mod requests;
mod secrets;

#[tokio::main]
async fn main() {
    let mut player_status: bool = false;
    let player_status_ptr: *mut bool = &mut player_status;

    let scan_complete: &str = "scanComplete";
    let harold_name: &str = "music";
    let mut music: &str = scan_complete;

    let harold_secrets = secrets::secrets::initialized_secrets();

    let future_retrieve_harold = harold_retriever(harold_secrets);

    unsafe {
        let music_future = music::music::play_harold(music, player_status_ptr);

        tokio::join!(future_retrieve_harold, music_future);
    }

    music = harold_name;
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
