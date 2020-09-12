pub mod requests {
    use crate::secrets::secrets;
    use ldap3::result;
    use ldap3::{LdapConnAsync, Scope, SearchEntry};
    use reqwest;
    use std::collections::HashMap;

    pub async fn get_uid(
        ibutton: &str,
        harold_secrets: secrets::Secrets,
    ) -> result::Result<String> {
        let (conn, mut ldap) = LdapConnAsync::new(harold_secrets.get_ldap_server()).await?;
        ldap3::drive!(conn);
        let bind = ldap
            .simple_bind(harold_secrets.get_ldap_dn(), harold_secrets.get_ldap_pw())
            .await?
            .success()?;

        let (search, res) = ldap
            .search(
                "cn=users,cn=accounts,dc=csh,dc=rit,dc=edu",
                Scope::Subtree,
                ibutton,
                vec!["uid"],
            )
            .await?
            .success()?;
        let mut uid = SearchEntry::construct(search[0].clone()).attrs["uid"][0].clone();

        println!("{:?}", &uid);

        ldap.unbind().await?;

        Ok(uid.to_string())
    }

    pub async fn get_s3_link(
        uid: String,
        harold_secrets: secrets::Secrets,
    ) -> reqwest::Result<String> {
        let mut base_url: String = "https://audiophiler.csh.rit.edu/get_harold/".to_owned();
        let mut uid: &str = &uid;
        base_url.push_str(uid);

        let mut params = HashMap::new();
        params.insert("auth_key", harold_secrets.get_audiophiler_secret());

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let res = client
            .post(&base_url)
            .json(&params)
            .send()
            .await?
            .text()
            .await?;

        println!("{:?}", res);
        Ok(res)
    }
    /*pub async fn get_s3_link(uid: String) -> String {

    }*/
}
