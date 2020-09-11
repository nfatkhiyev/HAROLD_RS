use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};

mod secrets;

fn main() -> Result<()> {
    let harold_secrets = secrets::secrets::initialized_secrets();

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
    println!("{}", rsa);
    Ok(ldap.unbind()?)
}
