pub mod requests {
    use crate::secrets::secrets;
    use ldap3::result;
    use ldap3::{LdapConnAsync, Scope, SearchEntry};
    use reqwest;
    use std::collections::HashMap;
    use std::fs::File;
    use std::io;

    //search LDAP for uid filtered by iButton. Returns UID in Result
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
        let uid = SearchEntry::construct(search[0].clone()).attrs["uid"][0].clone();

        ldap.unbind().await?;

        Ok(uid.to_string())
    }

    //makes call to audiophiler with UID to get s3 link. Returns s3 link in Result
    pub async fn get_s3_link(
        uid: &String,
        harold_secrets: secrets::Secrets,
    ) -> reqwest::Result<String> {
        let mut base_url: String = "https://audiophiler.csh.rit.edu/get_harold/".to_owned();
        let uid: &str = &uid;
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

        Ok(res)
    }

    //dl music file from s3 bucket
    pub async fn get_music_file(url: &String) -> reqwest::Result<()> {
        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        let res = client.get(url).send().await?;
        let mut file = File::create("music").expect("file creation failed");
        io::copy(&mut res.bytes().await?.as_ref(), &mut file).expect("copy failed");
        Ok(())
    }

    //request to update jupstart with users UID
    pub async fn update_jumpstart(
        uid: &String,
        harold_secrets: secrets::Secrets,
    ) -> reqwest::Result<()> {
        let base_url: String = "https://jumpstart.csh.rit.edu/update-harold".to_string();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            reqwest::header::AUTHORIZATION,
            reqwest::header::HeaderValue::from_static(harold_secrets.get_jumpstart_token()),
        );
        let mut params = HashMap::new();
        params.insert("file_name", uid);
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .danger_accept_invalid_certs(true)
            .build()?;
        client.post(&base_url).json(&params).send().await?;

        Ok(())
    }
}
