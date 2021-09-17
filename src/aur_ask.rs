pub mod aur_use {
    use reqwest;
    use serde::Deserialize;    

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    #[repr(C)]
    pub struct AurResponse {
        pub version: i16,
        pub r#type: String,
        pub resultcount: i32,
    }
    #[no_mangle] // &str
    pub async extern "C" fn does_pkg_exist(pkg_name: &'static String) -> Result<AurResponse, reqwest::Error> {
        let resp: AurResponse = reqwest::get(pkg_name)
        .await?
        .json::<AurResponse>()
        .await?;
        Ok(resp)
    }
}

