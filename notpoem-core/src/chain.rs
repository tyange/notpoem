use std::collections::HashMap;

pub fn build_chain(corpus: &str) -> HashMap<String, Vec<String>> {
    let words: Vec<&str> = corpus.split_whitespace().collect();
    let mut chain: HashMap<String, Vec<String>> = HashMap::new();

    for pair in words.windows(2) {
        let (current, next) = (pair[0], pair[1]);
        chain
            .entry(current.to_string())
            .or_default()
            .push(next.to_string());
    }

    chain
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_chain_maps_each_word_to_the_ones_that_followed_it() {
        let chain = build_chain("나는 오늘 하늘을 보았다 나는 오늘 바다를 보았다");

        let after_now = chain.get("나는").unwrap();
        assert_eq!(after_now, &vec!["오늘".to_string(), "오늘".to_string()]);

        let after_today = chain.get("오늘").unwrap();
        assert_eq!(after_today, &vec!["하늘을".to_string(), "바다를".to_string()]);
    }

    #[test]
    fn last_word_has_no_entry_as_a_key() {
        let chain = build_chain("하나 둘 셋");
        assert!(!chain.contains_key("셋"));
    }
}
