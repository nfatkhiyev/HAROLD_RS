pub mod requests {
    use crate::secrets::secrets;
    use ldap3::result::Result;
    use ldap3::{LdapConn, Scope, SearchEntry};

    pub fn get_uid(ibutton: &str, harold_secrets: secrets::Secrets) -> Result<String> {
        let mut ldap = LdapConn::new(harold_secrets.get_ldap_server())?;
        let bind = ldap
            .simple_bind(harold_secrets.get_ldap_dn(), harold_secrets.get_ldap_pw())?
            .success()?;

        let (search, res) = ldap
            .search(
                "cn=users,cn=accounts,dc=csh,dc=rit,dc=edu",
                Scope::Subtree,
                ibutton,
                vec!["uid"],
            )?
            .success()?;
        let mut uid = SearchEntry::construct(search[0].clone()).attrs["uid"][0].clone();

        println!("{:?}", &uid);

        ldap.unbind()?;

        Ok(uid.to_string())
    }
    /*pub async fn get_s3_link(uid: String) -> String {

    }*/
}
