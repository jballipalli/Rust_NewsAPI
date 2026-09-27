use crate::NewsApiError;
use crate::NewsApiResponse;
use crate::options;
use crate::{Category, Country, Language, SearchIn};
use core::fmt;
use std::collections::{HashMap, HashSet};
use url::Url;

static BASE_URL: &str = "https://newsapi.org/v2/";

pub struct Everything;
impl private::SupportSources for Everything {}
impl private::SupportLanguage for Everything {}

pub struct TopHeadlines;
impl private::SupportSources for TopHeadlines {}
impl private::SupportCategory for TopHeadlines {}
impl private::SupportCountry for TopHeadlines {}

pub struct Sources;
impl private::SupportCategory for Sources {}
impl private::SupportCountry for Sources {}
impl private::SupportLanguage for Sources {}

pub struct NewsApiClient<State> {
    api_key: String,

    url: Url,

    parameters: HashMap<String, String>,

    is_built: bool,

    state: std::marker::PhantomData<State>,
}

impl<State> fmt::Display for NewsApiClient<State> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "url: {}", self.get_url())?;
        writeln!(f, "parameters: {:#?}", self.get_parameters())?;
        Ok(())
    }
}

pub trait CommonTrait {
    fn get_url(&self) -> &str
    where
        Self: Sized;

    fn get_parameters(&self) -> &HashMap<String, String>
    where
        Self: Sized;

    fn get_parameters_mut(&mut self) -> &mut HashMap<String, String>
    where
        Self: Sized;

    fn show_api_key(&self) -> &str
    where
        Self: Sized;

    fn page_size(&mut self, size: u8) -> Result<&mut Self, NewsApiError>
    where
        Self: Sized;

    fn page(&mut self, page: usize) -> &mut Self
    where
        Self: Sized;

    fn q(&mut self, q: &str) -> Result<&mut Self, NewsApiError>
    where
        Self: Sized;
}

pub trait CountryTrait: Sized {
    fn country(&mut self, country: Country) -> &mut Self;
}

impl<T> CountryTrait for NewsApiClient<T>
where
    T: private::SupportCountry,
{
    fn country(&mut self, country: Country) -> &mut Self {
        self.get_parameters_mut().insert(
            "country".to_string(),
            options::COUNTRY_LOOKUP[country].to_string(),
        );
        self
    }
}

pub trait CategoryTrait: Sized {
    fn category(&mut self, category: Category) -> &mut Self;
}

impl<T> CategoryTrait for NewsApiClient<T>
where
    T: private::SupportCategory,
{
    fn category(&mut self, category: Category) -> &mut Self {
        self.get_parameters_mut()
            .insert("category".into(), format!("{category:?}").to_lowercase());
        self
    }
}

pub trait SourcesTrait: Sized {
    fn sources(&mut self, source: Vec<String>) -> Result<&mut Self, NewsApiError>;
}

impl<T> SourcesTrait for NewsApiClient<T>
where
    T: private::SupportSources,
{
    /// # Arguments
    /// source : `Vec<String>`
    ///
    /// # Description
    /// This function adds sources query to the url.
    /// Expects to provide valid source id's in a `Vec<String>`. max length is 20.
    ///
    /// # Error
    /// Raises `NewsApiError::ParamError`, if length of the vector is greater than 20.
    fn sources(&mut self, source: Vec<String>) -> Result<&mut Self, NewsApiError> {
        match source.len() < 21 {
            true => {
                self.get_parameters_mut()
                    .insert("sources".to_string(), source.join(","));
                Ok(self)
            }
            false => Err(NewsApiError::ParamError {
                param: "sources".to_string(),
                message: "cannot provide more than 20 source ids.".to_string(),
            }),
        }
    }
}

pub trait LanguageTrait: Sized {
    fn language(&mut self, language: Language) -> &mut Self;
}

impl<T> LanguageTrait for NewsApiClient<T>
where
    T: private::SupportLanguage,
{
    fn language(&mut self, language: Language) -> &mut Self {
        self.get_parameters_mut().insert(
            "language".to_string(),
            options::LANGUAGE_LOOKUP[language].to_string(),
        );
        self
    }
}

impl<State> CommonTrait for NewsApiClient<State> {
    fn get_url(&self) -> &str {
        &self.url.as_str()
    }

    fn get_parameters(&self) -> &HashMap<String, String> {
        &self.parameters
    }

    fn get_parameters_mut(&mut self) -> &mut HashMap<String, String> {
        &mut self.parameters
    }

    fn show_api_key(&self) -> &str {
        &self.api_key
    }

    /// # Arguments
    /// size: u8
    ///
    /// # Description
    /// inserts pageSize into the ```self.parameters``` hashmap.
    ///
    /// As per newsapi.org documentation, number of results to return per page (request). 20 is the default, 100 is the maximum.
    ///
    /// # Error
    /// Raise `NewsApiError::ParamError`, when size is out of bounds.
    ///
    fn page_size(&mut self, size: u8) -> Result<&mut Self, NewsApiError> {
        match (1..=100).contains(&size) {
            true => {
                self.parameters
                    .insert("pageSize".to_string(), size.to_string());
                Ok(self)
            }
            false => Err(NewsApiError::ParamError {
                param: "pageSize".to_string(),
                message: "range out of bound. size must be within 1 and 100".to_string(),
            }),
        }
    }

    /// # Arguments
    /// page: usize
    ///
    /// # Description
    /// Inserts the page number into the `self.parameters` hashmap.
    ///
    /// This selects which page of results to return. The page number is
    /// included in the request as the `page` query parameter. Use this to page through the results if the total results found is greater than the page size.
    ///
    fn page(&mut self, page: usize) -> &mut Self {
        self.parameters.insert("page".to_string(), page.to_string());
        self
    }

    /// # Arguments
    /// q : String
    ///
    /// # Description
    /// expect `q` to be a valid URL-encoded string.
    /// This function does not check the string URL encoded validation.
    ///
    /// # Error
    /// Raise `NewsApiError::ParamError`, as q is limited to 500 chars
    fn q(&mut self, q: &str) -> Result<&mut Self, NewsApiError> {
        match q.len() < 501 {
            true => {
                self.parameters.insert("q".to_string(), q.to_string());
                Ok(self)
            }
            false => Err(NewsApiError::ParamError {
                param: "q".to_string(),
                message: "length exceeds 500 char limit".to_string(),
            }),
        }
    }
}

impl NewsApiClient<Everything> {
    pub fn everything(api_key: String) -> Self {
        let base = Url::parse(BASE_URL).unwrap();
        let url = base.join("everything").unwrap();
        let parameters: HashMap<String, String> = HashMap::new();

        Self {
            api_key,
            url,
            parameters,
            state: std::marker::PhantomData::<Everything>,
            is_built: false,
        }
    }

    /// # Argument
    /// search_in : `&str`
    ///
    /// # Description
    /// adds searchIn and vectors elements to the parameters.
    /// It convert SearchIn -> &'static str before adding them to the hashmap.
    ///
    /// # Error
    /// Returns `NewsApiError::ParamError`, when provided with empty vector or contains duplicate values
    pub fn search_in(&mut self, search_in: Vec<SearchIn>) -> Result<&mut Self, NewsApiError> {
        if search_in.is_empty() {
            return Err(NewsApiError::ParamError {
                param: "searchIn".to_string(),
                message: "provided empty vector".to_string(),
            });
        }

        let mut seen = HashSet::new();
        for value in &search_in {
            if !seen.insert(*value) {
                return Err(NewsApiError::ParamError {
                    param: "searchIn".to_string(),
                    message: "duplicate values are not allowed".to_string(),
                });
            }
        }

        self.parameters.insert(
            "searchIn".to_string(),
            search_in
                .iter()
                .map(|s| s.as_str())
                .collect::<Vec<_>>()
                .join(","),
        );

        Ok(self)
    }

    pub fn domains(&mut self, domains: &str) -> Result<&mut Self, NewsApiError> {
        if domains.is_empty() {
            return Err(NewsApiError::ParamError {
                param: "domain".to_string(),
                message: "provided empty string".to_string(),
            });
        }

        self.parameters
            .insert("domains".to_string(), domains.to_owned());

        Ok(self)
    }

    pub fn exclude_domains(&mut self, domains: &str) -> Result<&mut Self, NewsApiError> {
        if domains.is_empty() {
            return Err(NewsApiError::ParamError {
                param: "excludeDomains".to_string(),
                message: "provided empty string".to_string(),
            });
        }

        self.parameters
            .insert("excludeDomains".to_string(), domains.to_owned());

        Ok(self)
    }
}

impl NewsApiClient<TopHeadlines> {
    pub fn top_headlines(api_key: String) -> Self {
        let base = Url::parse(BASE_URL).unwrap();
        let url = base.join("top-headlines").unwrap();

        let parameters: HashMap<String, String> = HashMap::new();

        Self {
            api_key,
            url,
            parameters,
            is_built: false,
            state: std::marker::PhantomData::<TopHeadlines>,
        }
    }
}

impl NewsApiClient<Sources> {
    pub fn sources(api_key: String) -> Self {
        let base = Url::parse(BASE_URL).unwrap();
        let url = base.join("top-headlines/sources").unwrap();

        let parameters: HashMap<String, String> = HashMap::new();

        Self {
            api_key,
            url,
            parameters,
            is_built: false,
            state: std::marker::PhantomData::<Sources>,
        }
    }
}

impl<State> NewsApiClient<State> {
    /// Build the url.
    ///
    /// updates the url if there are any additional parameters are provided.
    pub fn build(&mut self) -> &Self {
        if self.get_parameters().is_empty() || self.is_built {
            return self;
        }

        self.url
            .query_pairs_mut()
            .extend_pairs(self.parameters.iter());

        self.is_built = true;

        self
    }

    /// fetches `NewsApiResponse` using ureq crate.
    ///
    /// if `self.is_built = false`, calls `self.build()` method to update the url.
    pub fn fetch(mut self) -> Result<NewsApiResponse, NewsApiError> {
        if !self.is_built {
            self.build();
        }

        let response: String = ureq::get(self.get_url())
            .call()
            .map_err(|e| NewsApiError::BadRequest(e))?
            .body_mut()
            .read_to_string()
            .map_err(|e| NewsApiError::FailedToParseIntoString(e))?;

        let articles = serde_json::from_str::<NewsApiResponse>(response.as_str())
            .map_err(|e| NewsApiError::FailedToParseIntoJSON(e))?;

        Ok(articles)
    }
}

mod private {
    pub trait SupportCountry: Sized {}
    pub trait SupportCategory: Sized {}
    pub trait SupportLanguage: Sized {}
    pub trait SupportSources: Sized {}
}
