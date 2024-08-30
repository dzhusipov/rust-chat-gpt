use reqwest::Client;
use regex::Regex;

use crate::models::models::{ChatGPTRequest, ChatGPTMessage, ChatGPTResponse};

pub async fn get_chatgpt_response(api_key: &str, prompt: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let request = ChatGPTRequest {
        model: "gpt-4o-mini".to_string(),
        messages: vec![ChatGPTMessage {
            role: "user".to_string(),
            content: prompt.to_string(),
        }],
    };

    // log::info!("prompt: {}", prompt);

    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&request)
        .send()
        .await?
        .json::<ChatGPTResponse>()
        .await?;
    // log::info!("prompt {:?}", response);
    Ok(response.choices.first().unwrap().message.content.clone())
}

pub fn escape_markdown_v2(text: &str) -> String {
    text
        // .replace('\\', r"\\")
        // .replace('_', r"\_")
        // .replace('*', r"\*")
        // .replace('[', r"\[")
        // .replace(']', r"\]")
        // .replace('(', r"\(")
        // .replace(')', r"\)")
        // .replace('~', r"\~")
        // .replace('`', r"\`")
        // .replace('>', r"\>")
        // .replace('<', r"\<")  // Экранируем символы < и >
        // .replace('#', r"\#")
        // .replace('+', r"\+")
        // .replace('-', r"\-")
        // .replace('=', r"\=")
        // .replace('|', r"\|")
        // .replace('{', r"\{")
        // // .replace('}', r"\}")
        // .replace('.', r"\.")
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        // .replace('!', r".")
}

pub fn bold_html(text: &str) -> String {
    let re = Regex::new(r"\*\*(.+?)\*\*").unwrap();
    re.replace_all(text, "<b>$1</b>").to_string()
}

pub fn replace_code_markers(text: &str) -> String {
    let escaped_text = escape_markdown_v2(text);
    let escaped_text = bold_html(&escaped_text); // Применяем функцию для жирного шрифта
    // log::info!("{}", escaped_text);
    // Используем регулярное выражение для поиска всех вхождений ``` с опциональным указанием языка.
    let re = Regex::new(r"```([\w-]+)?").unwrap();

    // Переменная для хранения результата
    let mut result = String::new();
    let mut last_end = 0;
    let mut replace_next = true;

    // Проходим по всем вхождениям ```.
    for mat in re.captures_iter(&escaped_text) {
        // Получаем индексы начала и конца текущего вхождения.
        let m = mat.get(0).unwrap(); // Само вхождение ```
        let start = m.start();
        let end = m.end();

        // Добавляем текст до текущего вхождения
        result.push_str(&escaped_text[last_end..start]);

        // Получаем название языка, если оно указано
        let language = mat.get(1).map_or("", |lang| lang.as_str());

        // Заменяем первое и второе вхождения на нужные строки.
        if replace_next {
            // Открывающий тег <pre> с указанием языка
            result.push_str(&format!(r#"<pre language="{}">"#, language));
        } else {
            // Закрывающий тег </pre>
            result.push_str("</pre>");
        }

        // Инвертируем флаг для чередования замен
        replace_next = !replace_next;
        last_end = end;
    }

    // Добавляем оставшийся текст после последнего вхождения
    result.push_str(&escaped_text[last_end..]);

    result
}
