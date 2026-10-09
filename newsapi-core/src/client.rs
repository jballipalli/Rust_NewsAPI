use crate::NewsApiError;
use crate::capabilities::*;
use crate::endpoint::*;
use crate::errors::Result;
use crate::options::*;
use crate::parameters::Parameters;
use core::fmt;
use std::marker::PhantomData;

#[derive(Debug, PartialEq)]
pub struct NewsApiClient<State> {
    api_key: String,
    parameters: Parameters,
    state: std::marker::PhantomData<State>,
}

impl<State> fmt::Display for NewsApiClient<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "url: {}", self.url())?;
        Ok(())
    }
}

impl NewsApiClient<Everything> {
    pub fn everything(api_key: impl Into<String>) -> Self {
        Self::new(api_key)
    }
}

impl NewsApiClient<TopHeadlines> {
    pub fn top_headlines(api_key: impl Into<String>) -> Self {
        Self::new(api_key)
    }
}

impl NewsApiClient<Sources> {
    pub fn sources(api_key: impl Into<String>) -> Self {
        Self::new(api_key)
    }
}

impl<S: Endpoint> NewsApiClient<S> {
    fn new(api_key: impl Into<String>) -> Self {
        Self {
            api_key: api_key.into(),
            parameters: Parameters::new(),
            state: PhantomData::<S>,
        }
    }
}

impl<S: SupportCategory> NewsApiClient<S> {
    /// Sets the category used to filter the request.
    ///
    /// # Examples
    ///
    /// ```
    ///
    /// use newsapi_core::{Category, NewsApiClient}; ///
    ///
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .category(Category::Technology);
    ///
    /// ```

    pub fn category(mut self, category: Category) -> Self {
        self.parameters.set("category", category.as_str());
        self
    }
}

impl<S: SupportCountry> NewsApiClient<S> {
    /// Sets the country used to filter the request.
    ///
    /// The country is represented by its ISO 3166-1 alpha-2 country code
    /// when sent to NewsAPI.
    ///
    /// # Examples
    ///
    ///```
    /// # use newsapi_core::{Country, NewsApiClient};
    ///
    /// let client = NewsApiClient::top_headlines("api-key")
    ///                 .country(Country::India);
    /// ```
    pub fn country(mut self, country: Country) -> Self {
        self.parameters.set("country", country.as_str());
        self
    }
}

impl<S: SupportFilterSource> NewsApiClient<S> {
    /// Sets the domains used to filter the request.
    ///
    /// # Examples
    ///
    /// ```
    /// # use newsapi_core::NewsApiClient;
    ///
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .domains("bbc.com,techcrunch.com,engadget.com")
    /// ```
    pub fn domains(mut self, domains: impl Into<String>) -> Result<Self> {
        let domain = domains.into();
        match !domain.is_empty() {
            true => {
                self.parameters.set("domains", domain);
                Ok(self)
            }
            false => {
                return Err(NewsApiError::ParamError {
                    param: "domains",
                    message: "received empty string",
                });
            }
        }
    }

    /// Sets the domains used to filter the request.
    ///
    /// # Examples
    ///
    /// ```
    /// # use newsapi_core::NewsApiClient;
    ///
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .exclude_domains("bbc.com,techcrunch.com,engadget.com")
    /// ```
    pub fn exclude_domains(mut self, domains: impl Into<String>) -> Result<Self> {
        let domain = domains.into();

        match !domain.is_empty() {
            true => {
                self.parameters.set("excludeDomains", domain);
                Ok(self)
            }
            false => {
                return Err(NewsApiError::ParamError {
                    param: "excludeDomains",
                    message: "received empty string",
                });
            }
        }
    }
}
