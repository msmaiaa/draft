pub struct LoggerServer<F>
where
    F: Fn() -> Send + Clone + 'static,
{
    addr: String,
    host: String,
    url: Option<String>,
    token: String,
    threads: usize,
    factory: F,
    #[cfg(feature = "tls-server")]
    cert_and_key: Option<CertAndKey>,
}

impl<F> LoggerServer<F>
where
    F: Fn() -> Send + Clone + 'static,
{
    pub fn new(addr: String, token: String, host: String, factory: F) -> Self {
        Self {
            addr,
            host,
            url: None,
            threads: 1,
            factory,
            token,
            #[cfg(feature = "tls-server")]
            cert_and_key: None,
        }
    }

    pub fn set_workers(mut self, num: usize) -> Self {
        self.threads = num;
        self
    }

    pub fn set_url(mut self, url: String) -> Self {
        self.url = Some(url);
        self
    }

    pub fn url(&self) -> &str {
        self.url.as_ref().map_or(self.token.as_str(), |url| url)
    }

    pub fn full_url(&self) -> String {
        format!("{}/{}", self.host, self.url())
    }

    #[cfg(feature = "tls-server")]
    pub fn set_certificate_and_key(mut self, cert_and_key: CertAndKey, self_signed: bool) -> Self {
        self.cert_and_key = Some(cert_and_key);
        self.options.set(OptionFlags::SELF_SIGNED, self_signed);
        self
    }

    #[cfg(feature = "tls-server")]
    pub fn certificate_input_file(&self) -> Option<InputFile> {
        self.cert_and_key
            .as_ref()
            .map(|cert_and_key| cert_and_key.into())
    }
}

fn main() {}
