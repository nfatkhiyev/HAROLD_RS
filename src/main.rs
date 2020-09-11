use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};
use tokio::runtime::Runtime;

mod requests;
mod secrets;

fn main() -> Result<()> {
    let harold_secrets = secrets::secrets::initialized_secrets();

    let function_uid =
        requests::requests::get_uid(harold_secrets.get_nate_ibutton(), harold_secrets);

    let future_s3_link = requests::requests::get_s3_link(function_uid.unwrap(), harold_secrets);
    Runtime::new().expect("asdfads").block_on(future_s3_link);

    let mut ldap = LdapConn::new(harold_secrets.get_ldap_server())?;
    let _res = ldap
        .simple_bind(harold_secrets.get_ldap_dn(), harold_secrets.get_ldap_pw())?
        .success()?;
    let (rs, rsa) = ldap
        .search(
            "cn=users,cn=accounts,dc=csh,dc=rit,dc=edu",
            Scope::Subtree,
            harold_secrets.get_nate_ibutton(),
            vec!["uid"],
        )?
        .success()?;
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry).attrs["uid"][0]);
    }
    Ok(ldap.unbind()?)
}
