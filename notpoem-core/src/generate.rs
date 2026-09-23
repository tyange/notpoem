use rand::seq::IndexedRandom;
use std::collections::HashMap;

pub fn generate(chain: &HashMap<String, Vec<String>>, start: &str, len: usize) -> String {
    let mut rng = rand::rng();
    let mut words = vec![start.to_string()];
    let mut current = start.to_string();

    for _ in 1..len {
        let Some(candidates) = chain.get(&current) else {
            break;
        };
        let Some(next) = candidates.choose(&mut rng) else {
            break;
        };
        words.push(next.clone());
        current = next.clone();
    }

    words.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chain::build_chain;

    #[test]
    fn follows_the_only_path_when_every_word_has_one_successor() {
        let chain = build_chain("하나 둘 셋 넷");
        assert_eq!(generate(&chain, "하나", 4), "하나 둘 셋 넷");
    }

    #[test]
    fn stops_early_when_the_chain_runs_out() {
        let chain = build_chain("하나 둘 셋");
        assert_eq!(generate(&chain, "하나", 10), "하나 둘 셋");
    }

    #[test]
    fn never_exceeds_requested_length() {
        let chain = build_chain("가 나 가 나 가 나 가 나");
        for _ in 0..50 {
            let poem = generate(&chain, "가", 3);
            assert_eq!(poem.split_whitespace().count(), 3);
        }
    }

    #[test]
    fn unknown_start_word_returns_just_that_word() {
        let chain = build_chain("하나 둘 셋");
        assert_eq!(generate(&chain, "없는단어", 5), "없는단어");
    }

    #[test]
    fn every_step_is_a_transition_seen_in_the_corpus() {
        let chain = build_chain("나는 오늘 하늘을 보았다 나는 오늘 바다를 보았다");
        for _ in 0..50 {
            let poem = generate(&chain, "나는", 4);
            let words: Vec<&str> = poem.split_whitespace().collect();
            for pair in words.windows(2) {
                let seen = &chain[pair[0]];
                assert!(seen.iter().any(|w| w == pair[1]));
            }
        }
    }
}
