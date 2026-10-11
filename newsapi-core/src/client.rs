use crate::NewsApiError;
use crate::capabilities::*;
use crate::endpoint::*;
use crate::errors::Result;
use crate::options::*;
use crate::parameters::Parameters;

use chrono::{NaiveDate, NaiveDateTime, Utc};
use core::fmt;
use std::marker::PhantomData;
use url::Url;

#[derive(Debug, PartialEq)]
pub struct NewsApiClient<State> {
    api_key: String,
    parameters: Parameters,
    state: std::marker::PhantomData<State>,
}

impl<S: Endpoint> fmt::Display for NewsApiClient<S> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "url: {:#?}", self.url())?;
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

    fn build_url(&self) -> Result<Url> {
        if self.parameters.values().contains_key("sources")
            && (self.parameters.values().contains_key("country")
                || self.parameters.values().contains_key("category"))
        {
            return Err(NewsApiError::BuildError);
        }

        let mut url = Url::parse(format!("{}/{}", S::BASE_URL, S::PATH).as_str()).unwrap();

        url.query_pairs_mut()
            .extend_pairs(self.parameters.values().iter());

        Ok(url)
    }

    pub fn show_api_key(&self) -> &str {
        self.api_key.as_ref()
    }

    pub fn url(&self) -> String {
        self.build_url().unwrap().to_string()
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

impl<S: SupportDateRange> NewsApiClient<S> {
    /// set from query parameter to the request
    ///
    /// # Examples
    ///
    /// ```
    /// let client = NewsApiClient::everything("api_key")
    ///             .from("2026-10-01")?;
    /// ```
    pub fn from(mut self, datetime: impl AsRef<str>) -> Result<Self> {
        let date = Self::validate(datetime.as_ref(), "from")?;
        self.parameters.set("from", date);
        Ok(self)
    }

    /// set to query parameter to the request
    ///
    /// # Examples
    ///
    /// ```
    /// let client = NewsApiClient::everything("api-key")
    ///         .to("2026-10-09T15:30:00")?;
    ///```
    pub fn to(mut self, datetime: impl AsRef<str>) -> Result<Self> {
        let date = Self::validate(datetime.as_ref(), "to")?;
        self.parameters.set("to", date);
        Ok(self)
    }

    fn validate(input: &str, param: &'static str) -> Result<String> {
        let now = Utc::now();
        let today = now.date_naive();

        // Accepted format: YYYY-MM-DD
        if let Ok(date) = NaiveDate::parse_from_str(input, "%Y-%m-%d") {
            // Enforce the exact format, including zero-padding.
            if date.format("%Y-%m-%d").to_string() != input {
                return Err(NewsApiError::ParamError {
                    param: param,
                    message: "Invalid date format. format to YYYY-MM-DD",
                });
            }

            if date > today {
                return Err(NewsApiError::ParamError {
                    param,
                    message: "Provided future date",
                });
            }

            return Ok(date.format("%Y-%m-%d").to_string());
        }

        // Accepted format: YYYY-MM-DDTHH:MM:SS
        if let Ok(datetime) = NaiveDateTime::parse_from_str(input, "%Y-%m-%dT%H:%M:%S") {
            // Enforce the exact format.
            if datetime.format("%Y-%m-%dT%H:%M:%S").to_string() != input {
                return Err(NewsApiError::ParamError {
                    param,
                    message: "Invalid DateTime format. Format to YYYY-MM-DDTHH:MM:SS",
                });
            }

            // This comparison treats the timezone-free timestamp as UTC.
            if datetime > now.naive_utc() {
                return Err(NewsApiError::ParamError {
                    param,
                    message: "Provided future DateTime",
                });
            }

            return Ok(datetime.format("%Y-%m-%dT%H:%M:%S").to_string());
        }

        Err(NewsApiError::BuildError)
    }
}

impl<S: SupportFilterSource> NewsApiClient<S> {
    /// Sets the domains used to filter the request.
    ///
    /// Doesn't validate the provided `domains` string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use newsapi_core::NewsApiClient;
    ///
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .domains("bbc.com,techcrunch.com,engadget.com")
    /// ```
    pub fn domains(mut self, domains: impl Into<String>) -> Self {
        self.parameters.set("domains", domains);
        self
    }

    /// Sets the domains used to filter the request.
    ///
    /// Doesn't validate the provided `domains` string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use newsapi_core::NewsApiClient;
    ///
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .exclude_domains("bbc.com,techcrunch.com,engadget.com")
    /// ```
    pub fn exclude_domains(mut self, domains: impl Into<String>) -> Self {
        self.parameters.set("excludeDomains", domains);
        self
    }
}

impl<S: SupportLanguage> NewsApiClient<S> {
    /// Adds language query to the request
    ///
    /// # Examples
    ///
    /// ```
    /// let client = NewsApiClient::top_headlines("api-key")
    ///              .language(Language::English);
    /// ```
    pub fn language(mut self, language: Language) -> Self {
        self.parameters.set("language", language.as_str());
        self
    }
}

impl<S: SupportPagination> NewsApiClient<S> {
    /// Set pageSize query parameter to the request
    ///
    /// # Errors
    /// if provided `size` is out of range. `size` must be within `1` and `100`
    ///
    /// # Examples
    ///
    /// ```
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .page_size(20);
    /// ```
    pub fn page_size(mut self, size: u8) -> Result<Self> {
        match (1..=100).contains(&size) {
            true => {
                self.parameters.set("pageSize", size.to_string());
                Ok(self)
            }
            false => {
                return Err(NewsApiError::ParamError {
                    param: "pageSize",
                    message: "range out of bounds. size must be within 1 and 100",
                });
            }
        }
    }

    /// Set page query to the request
    ///
    /// # Examples
    ///
    /// ```
    /// let client = NewsApiCLient::top_headlines("api-key")
    ///             .page(2);
    /// ```
    pub fn page(mut self, page: u8) -> Self {
        self.parameters.set("page", page.to_string());
        self
    }
}

impl<S: SupportQuery> NewsApiClient<S> {
    /// expect `q` to be a valid URL-encoded string.
    /// This function does not check the string URL encoded validation.
    ///
    /// # Error
    /// Raise `NewsApiError::ParamError`, as q is limited to 500 chars.
    ///
    /// # Examples
    /// ```
    /// let client = NewsApiCLient::top_headlines("api-key")
    ///             .q("artificial+inteliigence")
    /// ```
    pub fn q(mut self, q: impl Into<String>) -> Result<Self> {
        let q = q.into();
        match q.len() < 501 {
            true => {
                self.parameters.set("q", q);
                Ok(self)
            }
            false => Err(NewsApiError::ParamError {
                param: "q",
                message: "length exceeds 500 char limit",
            }),
        }
    }
}

impl<S: SupportSearchIn> NewsApiClient<S> {
    /// sets searchIn query parameter to the request
    ///
    /// # Errors
    ///
    /// Raises `NewsApiError` if `search` is empty or length > 3
    ///
    /// # Examples
    /// ```
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .search_in(vec![SearchIn::Title, SearchIn::Content]);
    /// ```
    pub fn search_in(mut self, search: Vec<SearchIn>) -> Result<Self> {
        match search.len() < 4 {
            true => {
                self.parameters.set(
                    "searchIn",
                    search
                        .iter()
                        .map(|s| s.as_str())
                        .collect::<Vec<&str>>()
                        .join(","),
                );
                Ok(self)
            }
            false => {
                return Err(NewsApiError::ParamError {
                    param: "searchIn",
                    message: "provided empty vector or vector contains duplicate values",
                });
            }
        }
    }
}

impl<S: SupportSortBy> NewsApiClient<S> {
    /// sets sortBy query parameter to the request
    ///
    /// # Examples
    ///
    /// ```
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .sort_by(SortBy::Popularity);
    /// ```
    pub fn sort_by(mut self, sort: SortBy) -> Self {
        self.parameters.set("sortBy", sort.as_str());
        self
    }
}

impl<S: SupportSources> NewsApiClient<S> {
    /// This function adds sources query to the url.
    ///
    /// Expects to provide valid source id's in a `source` vector. max length is 20.
    ///
    /// # Error
    /// Raises `NewsApiError::ParamError`, if length of the vector is greater than 20.
    ///
    /// # Examples
    /// ```
    /// let client = NewsApiClient::top_headlines("api-key")
    ///             .sources(vec!["bbc-news","business-insider"])
    /// ```
    pub fn sources(mut self, source: Vec<impl Into<String>>) -> Result<Self> {
        match source.len() < 21 {
            true => {
                self.parameters.set(
                    "sources",
                    source
                        .into_iter()
                        .map(|s| s.into())
                        .collect::<Vec<String>>()
                        .join(","),
                );
                Ok(self)
            }
            false => Err(NewsApiError::ParamError {
                param: "sources",
                message: "cannot provide more than 20 source ids.",
            }),
        }
    }
}
