use crate::endpoint::*;

pub trait SupportCategory {}
pub trait SupportCountry {}
pub trait SupportDateRange {}
pub trait SupportFilterSource {}
pub trait SupportLanguage {}
pub trait SupportPagination {}
pub trait SupportQuery {}
pub trait SupportSearchIn {}
pub trait SupportSortBy {}
pub trait SupportSources {}

impl SupportCategory for TopHeadlines {}
impl SupportCategory for Sources {}

impl SupportCountry for TopHeadlines {}
impl SupportCountry for Sources {}

impl SupportDateRange for Everything {}

impl SupportFilterSource for Everything {}

impl SupportLanguage for Everything {}
impl SupportLanguage for Sources {}

impl SupportPagination for Everything {}
impl SupportPagination for TopHeadlines {}

impl SupportQuery for Everything {}
impl SupportQuery for TopHeadlines {}

impl SupportSearchIn for Everything {}

impl SupportSortBy for Everything {}

impl SupportSources for Everything {}
impl SupportSources for TopHeadlines {}
