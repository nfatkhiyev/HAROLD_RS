use ldap3::result::Result;
use ldap3::{LdapConn, Scope, SearchEntry};

use secrets;

fn main() -> Result<()> {
    let mut ldap = LdapConn::new(secrets::LDAP_SERVER)?;
    let _res = ldap
        .simple_bind(secrets::LDAP_DN, secrets::LDAP_PASS)?
        .success()?;
    let (rs, rsa) = ldap
        .search(
            "cn=users,cn=accounts,dc=csh,dc=rit,dc=edu",
            Scope::Subtree,
            secrets::NATE_IBUTTON,
            vec!["uid"],
        )?
        .success()?;
    for entry in rs {
        println!("{:?}", SearchEntry::construct(entry)[0]);
    }
    println!("{}", rsa);
    Ok(ldap.unbind()?)
}
