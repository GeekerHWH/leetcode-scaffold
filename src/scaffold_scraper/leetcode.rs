use reqwest;
use scraper::{Html, Selector};

pub async fn get_leetcode_problem_description(url: String) -> String {
    let body = reqwest::get(url).await.unwrap().text().await.unwrap();

    let html = Html::parse_document(&body);
    let selector = Selector::parse("meta[name='description']").unwrap();
    if let Some(element) = html.select(&selector).next() {
        if let Some(content) = element.value().attr("content") {
            return content
                .replace("\\u{a0}", " ") // 替换 \u{a0} 字面文本
                .replace("\\n", "\n") // 替换 \n 字面文本
                .replace("\u{a0}", " ") // 替换实际的不间断空格字符
                .trim()
                .to_string();
        }
    }

    "无法获取题目描述".to_string()
}
