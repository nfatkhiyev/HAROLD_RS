use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};
use tokio::runtime::Runtime;

mod requests;
mod secrets;

#[tokio::main]
async fn main() {
    let harold_secrets = secrets::secrets::initialized_secrets();

    let function_uid =
        requests::requests::get_uid(harold_secrets.get_nate_ibutton(), harold_secrets);
    let used_function_uid = &function_uid.await.unwrap();

    let future_s3_link = requests::requests::get_s3_link(used_function_uid, harold_secrets);

    let used_future_s3_link = &future_s3_link.await.unwrap();

    let future_music_file = requests::requests::get_music_file(used_future_s3_link);
    println!("{:?}", used_future_s3_link);
}
