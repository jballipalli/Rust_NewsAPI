use enum_map::{Enum, EnumMap, enum_map};
use serde::{self, Deserialize, Serialize};
use std::sync::LazyLock;

pub static COUNTRY_LOOKUP: LazyLock<EnumMap<Country, &'static str>> = LazyLock::new(|| {
    enum_map! {
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
        Country::Venuzuela => "ve",
    }
});

pub static LANGUAGE_LOOKUP: LazyLock<EnumMap<Language, &'static str>> = LazyLock::new(|| {
    enum_map! {
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
});

#[derive(Debug, Serialize, Deserialize)]
pub enum SearchIn {
    #[serde(rename = "title")]
    Title,
    #[serde(rename = "description")]
    Description,
    #[serde(rename = "content")]
    Content,
}

impl SearchIn {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Content => "content",
            Self::Description => "description",
            Self::Title => "title",
        }
    }
}

#[derive(Debug, Enum)]
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

#[derive(Debug, Enum)]
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
    Venuzuela,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Category {
    #[serde(rename = "business")]
    Business,
    #[serde(rename = "entertainment")]
    Entertainment,
    #[serde(rename = "general")]
    General,
    #[serde(rename = "health")]
    Health,
    #[serde(rename = "science")]
    Science,
    #[serde(rename = "sports")]
    Sports,
    #[serde(rename = "technology")]
    Technology,
}
