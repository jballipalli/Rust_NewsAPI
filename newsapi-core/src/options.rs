#[derive(Debug)]
pub enum SearchIn {
    Title,
    Description,
    Content,
}

impl SearchIn {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Content => "content",
            Self::Description => "description",
            Self::Title => "title",
        }
    }
}

#[derive(Debug)]
pub enum SortBy {
    Relevancy,
    Popularity,
    PublishedAt,
}

impl SortBy {
    pub const fn as_str(&self) -> &'static str {
        match self {
            SortBy::PublishedAt => "publishedAt",
            SortBy::Popularity => "popularity",
            SortBy::Relevancy => "relevancy",
        }
    }
}

#[derive(Debug)]
pub enum Language {
    Arabic,
    German,
    English,
    Spanish,
    French,
    Hebrew,
    Italian,
    Dutch,
    Norwegian,
    Portuguese,
    Russian,
    Swedish,
    Urdu,
    Chinese,
}

impl Language {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Language::Arabic => "ar",
            Language::German => "de",
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Hebrew => "he",
            Language::Italian => "it",
            Language::Dutch => "nl",
            Language::Norwegian => "no",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Swedish => "sv",
            Language::Urdu => "ud",
            Language::Chinese => "zh",
        }
    }
}

#[derive(Debug)]
pub enum Country {
    Argentina,
    Australia,
    Austria,
    Belgium,
    Brazil,
    Bulgaria,
    Canada,
    China,
    Colombia,
    Cuba,
    CzechRepublic,
    Egypt,
    France,
    Germany,
    Greece,
    HongKong,
    Hungary,
    India,
    Indonesia,
    Ireland,
    Israel,
    Italy,
    Japan,
    Latvia,
    Lithuania,
    Malaysia,
    Mexico,
    Morocco,
    Netherlands,
    NewZealand,
    Nigeria,
    Norway,
    Philippines,
    Poland,
    Portugal,
    Romania,
    Russia,
    SaudiArabia,
    Serbia,
    Singapore,
    Slovakia,
    Slovenia,
    SouthAfrica,
    SouthKorea,
    Sweden,
    Switzerland,
    Taiwan,
    Thailand,
    Turkiye,
    UAE,
    Ukraine,
    UnitedKingdom,
    UnitedStates,
    Venezuela,
}

impl Country {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Country::Argentina => "ar",
            Country::Australia => "au",
            Country::Austria => "at",
            Country::Belgium => "be",
            Country::Brazil => "br",
            Country::Bulgaria => "bg",
            Country::Canada => "ca",
            Country::China => "cn",
            Country::Colombia => "co",
            Country::Cuba => "cu",
            Country::CzechRepublic => "cz",
            Country::Egypt => "eg",
            Country::France => "fr",
            Country::Germany => "de",
            Country::Greece => "gr",
            Country::HongKong => "hk",
            Country::Hungary => "hu",
            Country::India => "in",
            Country::Indonesia => "id",
            Country::Ireland => "ie",
            Country::Israel => "il",
            Country::Italy => "it",
            Country::Japan => "jp",
            Country::Latvia => "lv",
            Country::Lithuania => "lt",
            Country::Malaysia => "my",
            Country::Mexico => "mx",
            Country::Morocco => "ma",
            Country::Netherlands => "nl",
            Country::NewZealand => "nz",
            Country::Nigeria => "ng",
            Country::Norway => "no",
            Country::Philippines => "ph",
            Country::Poland => "pl",
            Country::Portugal => "pt",
            Country::Romania => "ro",
            Country::Russia => "ru",
            Country::SaudiArabia => "sa",
            Country::Serbia => "rs",
            Country::Singapore => "sg",
            Country::Slovakia => "sk",
            Country::Slovenia => "si",
            Country::SouthAfrica => "za",
            Country::SouthKorea => "kr",
            Country::Sweden => "se",
            Country::Switzerland => "ch",
            Country::Taiwan => "tw",
            Country::Thailand => "th",
            Country::Turkiye => "tr",
            Country::UAE => "ae",
            Country::Ukraine => "ua",
            Country::UnitedKingdom => "gb",
            Country::UnitedStates => "us",
            Country::Venezuela => "ve",
        }
    }
}

#[derive(Debug)]
pub enum Category {
    Business,
    Entertainment,
    General,
    Health,
    Science,
    Sports,
    Technology,
}

impl Category {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Category::Business => "business",
            Category::Entertainment => "entertainment",
            Category::General => "general",
            Category::Health => "health",
            Category::Science => "science",
            Category::Sports => "sports",
            Category::Technology => "technology",
        }
    }
}
