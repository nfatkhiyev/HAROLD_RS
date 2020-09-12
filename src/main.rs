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

    let future_s3_link =
        requests::requests::get_s3_link(function_uid.await.unwrap(), harold_secrets);
    println!("{:?}", future_s3_link.await.unwrap());
}
