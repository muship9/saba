use alloc::{
    str,
    string::{String, ToString},
    vec::Vec,
};

#[derive(Debug, Clone, PartialEq)]

pub struct Url {
    url: String,
    host: String,
    port: String,
    path: String,
    searchpart: String,
}

impl Url {
    pub fn host(&self) -> String {
        self.host.clone()
    }
    pub fn port(&self) -> String {
        self.port.clone()
    }
    pub fn path(&self) -> String {
        self.path.clone()
    }
    pub fn searchpart(&self) -> String {
        self.searchpart.clone()
    }

    // URL 構造体のインスタンスを作成するコンストラクタ
    pub fn new(url: String) -> Self {
        Self {
            url,
            host: "".to_string(),
            port: "".to_string(),
            path: "".to_string(),
            searchpart: "".to_string(),
        }
    }

    // http プロトコルのみ対応のため、URL に指定文字が含まれるかチェック
    fn is_http(&mut self) -> bool {
        if self.url.contains("http://") {
            return true;
        }
        return false;
    }

    // ホストの取得
    // 最初の/ ~ port の間にあるものをホストとしてみなす
    fn exrract_host(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][..index].to_string()
        } else {
            url_parts[0].to_string()
        }
    }

    // ポート番号の取得
    fn expect_port(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if let Some(index) = url_parts[0].find(':') {
            url_parts[0][index + 1..].to_string()
        } else {
            "80".to_string()
        }
    }

    // パスの取得
    // ホスト以降 ~ をパスとしてみなし、? があればそれ以前を対象とする
    fn extract_path(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();

        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();
        path_and_searchpart[0].to_string()
    }

    // クエリパラメータの対応
    fn extract_searchpart(&self) -> String {
        let url_parts: Vec<&str> = self
            .url
            .trim_start_matches("http://")
            .splitn(2, "/")
            .collect();
        if url_parts.len() < 2 {
            return "".to_string();
        }

        let path_and_searchpart: Vec<&str> = url_parts[1].splitn(2, "?").collect();

        if path_and_searchpart.len() < 2 {
            "".to_string()
        } else {
            path_and_searchpart[1].to_string()
        }
    }
    pub fn parse(&mut self) -> Result<Self, String> {
        if !self.is_http() {
            return Err("Only HTTP schema supported.".to_string());
        }
        self.host = self.exrract_host();
        self.port = self.expect_port();
        self.path = self.extract_path();
        self.searchpart = self.extract_searchpart();

        Ok(self.clone())
    }
}
