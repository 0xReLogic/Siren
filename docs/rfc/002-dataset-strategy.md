# RFC-002: Dataset Selection & Scraping Policy

- **Status**: Draft
- **Author**: Allen Elzayn, Cascade
- **Date**: 2025-08-04

## 1. Summary

This document outlines the strategy for sourcing, collecting, processing, and versioning the datasets required to train and evaluate the Siren phishing detection model. A high-quality, diverse dataset is critical for the model's performance and robustness.

## 2. Data Sources

We will use a combination of public feeds for phishing URLs and large-scale crawls for legitimate URLs.

### Phishing Data Sources:

- **PhishTank**: A free, community-driven database of phishing URLs. Provides a high volume of verified phishing sites.
- **URLhaus (by abuse.ch)**: A project from abuse.ch focused on sharing malicious URLs used for malware distribution. We will filter for phishing-related URLs.
- **OpenPhish**: Provides community and commercial phishing intelligence feeds. We will start with their free community feed.

### Legitimate Data Sources:

- **Common Crawl**: A massive, open repository of web crawl data. We will sample from it to get a diverse set of legitimate websites.
- **Tranco List**: A research-oriented top sites list that is more transparent and resistant to manipulation than commercial lists. We will use this to source popular, trusted domains.

## 3. Data Scraping & Collection Policy

To ensure ethical and responsible data collection, all scraping activities will adhere to the following policies:

- **User-Agent**: All requests will use a descriptive `User-Agent` string (e.g., `Siren-Data-Collector/1.0 (+https://github.com/0xReLogic/Siren)`).
- **Respect `robots.txt`**: Our scraper will parse and obey the rules specified in a site's `robots.txt` file.
- **Rate Limiting**: Implement a polite crawling speed with delays between requests to a single domain to avoid impacting site performance.
- **Passive Collection**: We will only fetch the HTML content of a URL. We will **not** submit any forms or attempt to interact with the page in any way.
- **Data Privacy**: No personally identifiable information (PII) will be intentionally collected or stored.

## 4. Data Processing Pipeline

The raw data will be processed through a standardized pipeline:

1.  **Ingestion**: Fetch URLs from the selected sources.
2.  **Scraping**: Download the raw HTML content for each URL.
3.  **Deduplication**: Hash the HTML content to identify and remove duplicate pages.
4.  **Labeling**: Assign a label (`phish` or `legit`) to each unique page.
5.  **Feature Extraction**: (To be defined in a later RFC) Extract relevant features from the URL and HTML.
6.  **Splitting**: Divide the final dataset into three splits for model development:
    -   **80% Training Set**: Used to train the model.
    -   **10% Validation Set**: Used for hyperparameter tuning.
    -   **10% Test Set**: A held-out set used for final performance evaluation.

## 5. Data Versioning and Storage

- **Versioning**: We will use **DVC (Data Version Control)** to version our datasets. DVC allows us to store metadata about the datasets in Git while keeping the large data files in a separate storage backend. This makes our experiments reproducible.
- **Storage**: The actual data files will be stored in an S3-compatible object storage solution, such as AWS S3 or a self-hosted MinIO instance.
