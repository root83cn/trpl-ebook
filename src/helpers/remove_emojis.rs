use regex::Regex;

pub fn remove_emojis(input: &str) -> String {
    // cf. http://stackoverflow.com/a/29115826/1254484
    let emojis = Regex::new("[\u{203c}\u{2049}\u{20e3}\u{2122}\u{2139}\u{2194}-\u{2199}\u{21a9}-\u{21aa}\u{231a}-\u{231b}\u{23e9}-\u{23ec}\u{23f0}\u{23f3}\u{24c2}\u{25aa}-\u{25ab}\u{25b6}\u{25c0}\u{25fb}-\u{25fe}\u{2600}-\u{2601}\u{260e}\u{2611}\u{2614}-\u{2615}\u{261d}\u{263a}\u{2648}-\u{2653}\u{2660}\u{2663}\u{2665}-\u{2666}\u{2668}\u{267b}\u{267f}\u{2693}\u{26a0}-\u{26a1}\u{26aa}-\u{26ab}\u{26bd}-\u{26be}\u{26c4}-\u{26c5}\u{26ce}\u{26d4}\u{26ea}\u{26f2}-\u{26f3}\u{26f5}\u{26fa}\u{26fd}\u{2702}\u{2705}\u{2708}-\u{270c}\u{270f}\u{2712}\u{2714}\u{2716}\u{2728}\u{2733}-\u{2734}\u{2744}\u{2747}\u{274c}\u{274e}\u{2753}-\u{2755}\u{2757}\u{2764}\u{2795}-\u{2797}\u{27a1}\u{27b0}\u{2934}-\u{2935}\u{2b05}-\u{2b07}\u{2b1b}-\u{2b1c}\u{2b50}\u{2b55}\u{3030}\u{303d}\u{3297}\u{3299}\u{1f004}\u{1f0cf}\u{1f170}-\u{1f171}\u{1f17e}-\u{1f17f}\u{1f18e}\u{1f191}-\u{1f19a}\u{1f1e7}-\u{1f1ec}\u{1f1ee}-\u{1f1f0}\u{1f1f3}\u{1f1f5}\u{1f1f7}-\u{1f1fa}\u{1f201}-\u{1f202}\u{1f21a}\u{1f22f}\u{1f232}-\u{1f23a}\u{1f250}-\u{1f251}\u{1f300}-\u{1f320}\u{1f330}-\u{1f335}\u{1f337}-\u{1f37c}\u{1f380}-\u{1f393}\u{1f3a0}-\u{1f3c4}\u{1f3c6}-\u{1f3ca}\u{1f3e0}-\u{1f3f0}\u{1f400}-\u{1f43e}\u{1f440}\u{1f442}-\u{1f4f7}\u{1f4f9}-\u{1f4fc}\u{1f500}-\u{1f507}\u{1f509}-\u{1f53d}\u{1f550}-\u{1f567}\u{1f5fb}-\u{1f640}\u{1f645}-\u{1f64f}\u{1f680}-\u{1f68a}]").unwrap();

    emojis.replace_all(input, "")
}

#[test]
fn emoji_removal() {
    assert_eq!(remove_emojis("emojis? 😅"), "emojis? ");
}
