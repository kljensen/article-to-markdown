# 🎉 Article to Markdown 📝

A small Rust program to convert web articles to Markdown effortlessly.

## ✨ Features

1. 🖥️ Accepts a URL from the command line.
2. 📥 Fetches the URL content.
3. 📑 Extracts the main article using [Readability](https://github.com/kumabook/readability).
4. 📝 Converts the content to Markdown with [htmd](https://github.com/letmutex/htmd).

## 🤔 Why Use This?

Easily get article content into AI/LLMs for your projects.

## 🔧 Build Instructions

```sh
cargo build --release
```

## 🚀 Usage Example

Convert a [NYTimes article about SBF](https://www.nytimes.com/2024/02/27/technology/sam-bankman-fried-fraud-ftx.html) to Markdown:

```sh
./target/release/article-to-markdown 'https://www.nytimes.com/2024/02/27/technology/sam-bankman-fried-fraud-ftx.html'
```

### 📄 Output

```markdown
# Sam Bankman-Fried Seeks Lenient Sentence and to Appeal Conviction - The New York Times

Since [Sam Bankman-Fried](https://www.nytimes.com/2023/11/02/technology/sam-bankman-fried-rise-crash.html) was [convicted of fraud](https://www.nytimes.com/2023/11/02/technology/sam-bankman-fried-fraud-trial-ftx.html) last year, he has hired a new lawyer known for courtroom showmanship. A group of sympathetic law professors has pushed for a reappraisal of his actions. And his parents have turned for help to former employees of FTX, the collapsed cryptocurrency exchange he founded.

From a federal detention center in Brooklyn, Mr. Bankman-Fried, 31, has continued to fight his case behind the scenes, as he aims for a lenient sentence and prepares to appeal his conviction. On Tuesday, his lawyers filed a legal memo in U.S. District Court in Manhattan, arguing that he should receive a prison sentence of between five and a quarter and six and a half years.

Mr. Bankman-Fried is “deeply, deeply sorry” for “the pain he caused over the last two years,” the memo said. “His sole focus after the collapse of FTX was making customers whole.”

The filing was a crucial step before Mr. Bankman-Fried’s sentencing on March 28, when the federal judge overseeing his case, Lewis A. Kaplan, will decide how long to imprison the one-time billionaire on charges that carry a maximum sentence of 110 years. But it was only one prong of a long-shot strategy orchestrated by Mr. Bankman-Fried’s family and friends to reverse his conviction and engineer a public reappraisal of his leadership at FTX.
```
