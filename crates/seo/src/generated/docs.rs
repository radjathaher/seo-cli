// AUTO-GENERATED. DO NOT EDIT.
#![allow(dead_code)]

pub static DOCS_MAP: &[(&str, &str)] = &[
    ("ai-optimization", "AI Optimization API provides data for keyword discovery, conversational optimization, and LLM benchmarking via LLM Responses, AI Keyword Data, and LLM Mentions."),
    ("ai-optimization/ai-keyword-data", "AI Keyword Data API provides AI search volume estimates and intent insights based on keyword usage in AI tools."),
    ("ai-optimization/llm-mentions", "LLM Mentions API provides keyword, brand, and website mentions in LLMs with metrics like AI search volume and impressions."),
    ("ai-optimization/llm-responses", "LLM Responses API generates structured responses from LLMs (ChatGPT, Claude, Gemini, Perplexity) for specified inputs."),
    ("app-data", "App Data API provides rankings, ratings, reviews, and listings for apps on Google Play and the App Store."),
    ("app-data/apple", "Apple App Data API provides app metadata, rankings, and reviews for the App Store."),
    ("app-data/google", "Google App Data API provides app rankings, lists, details, and reviews for Google Play."),
    ("appendix", "Appendix endpoints provide account usage and pricing details plus API status and error code information."),
    ("backlinks", "Backlinks API provides inbound link, referring domain, and referring page data from a live index, with filtering and bulk endpoints."),
    ("business-data", "Business Data API provides public business info and reviews from Google, Trustpilot, and Tripadvisor, plus social media endpoints."),
    ("business-data/business-listings", "Business Listings API provides listings data from the in-house Business Listings Database, including Google Maps entities and category aggregations."),
    ("business-data/google", "Google Business Data API provides public business info and reviews from Google Search, Google Hotels, and Google Business Profile."),
    ("business-data/social-media", "Social Media API provides interaction data for URLs on Facebook, Pinterest, and Reddit using platform embeds."),
    ("content-analysis", "Content Analysis API finds citations for a target keyword or brand and analyzes sentiment, polarity, and trends."),
    ("content-generation", "Content Generation API produces and transforms text, including generation, paraphrasing, grammar checks, and summaries."),
    ("dataforseo-labs", "DataForSEO Labs API delivers keyword, SERP, and domain data from in-house databases, grouped by search engine, and uses Live method."),
    ("dataforseo-labs/amazon", "DataForSEO Labs API delivers keyword, SERP, and domain data from in-house databases, grouped by search engine, and uses Live method."),
    ("dataforseo-labs/apple", "DataForSEO Labs API delivers keyword, SERP, and domain data from in-house databases, grouped by search engine, and uses Live method."),
    ("dataforseo-labs/bing", "DataForSEO Labs API delivers keyword, SERP, and domain data from in-house databases, grouped by search engine, and uses Live method."),
    ("dataforseo-labs/google", "DataForSEO Labs API delivers keyword, SERP, and domain data from in-house databases, grouped by search engine, and uses Live method."),
    ("domain-analytics", "Domain Analytics API provides traffic, technology, and Whois insights, grouped into Technologies and Whois APIs."),
    ("domain-analytics/technologies", "Domain Analytics Technologies API identifies technologies used by websites and provides stats by domain or technology."),
    ("domain-analytics/whois", "Domain Analytics Whois API provides Whois data with backlink, ranking, and traffic info."),
    ("keywords-data", "Keywords Data API provides keyword analysis data for Google and Bing sources with Standard and Live methods."),
    ("keywords-data/bing", "Bing Keywords Data API provides search volume, keywords for site/keywords, audience estimation, and keyword performance using Bing Ads data."),
    ("keywords-data/clickstream-data", "Clickstream Data API provides clickstream-based search volume and keyword insights as an alternative to Google Ads volume, with Live method."),
    ("keywords-data/google-ads", "Google Ads Keywords Data API provides search volume, keywords for site/keywords, and ad traffic metrics using Google Ads data."),
    ("merchant", "Merchant API provides e-commerce data for product discovery, pricing, and seller analysis across Google Shopping and Amazon."),
    ("merchant/amazon", "Amazon Merchant API provides Amazon organic/paid listings plus product and ASIN details."),
    ("merchant/google", "Google Shopping API provides product listing data, product details, prices, and sellers data for Google Shopping."),
    ("on-page", "OnPage API crawls websites with customizable parameters to evaluate on-page SEO and site health, with task-based retrieval."),
    ("serp", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/baidu", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/bing", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/google", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/naver", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/seznam", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/yahoo", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
    ("serp/youtube", "SERP API provides search results for major engines (Google, Bing, YouTube, Yahoo, Baidu, Naver, Seznam) with Regular, Advanced, and HTML functions plus Standard and Live methods."),
];

pub fn lookup(path: &[String]) -> Option<&'static str> {
    let mut end = path.len();
    while end > 0 {
        let key = path[..end].join("/");
        if let Some((_, v)) = DOCS_MAP.iter().find(|(k, _)| *k == key) {
            return Some(*v);
        }
        end -= 1;
    }
    None
}
