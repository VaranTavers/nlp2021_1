
pub struct Settings {
    pub ignored_words: Vec<&'static str>,
    pub notters: Vec<&'static str>,
    pub separators: Vec<&'static str>, 
    pub min_occurence: i32,
    pub max_occurence: i32,
}

impl Default for Settings {
    
    fn default() -> Self {
        Settings {
            ignored_words: vec!["\"", "", "i", "me", "my", "myself", "we", "our", "ours", "ourselves", "you", "your", "yours", "yourself", "yourselves", "he", "him", "his", "himself", "she", "her", "hers", "herself", "it", "its", "itself", "they", "them", "their", "theirs", "themselves", "what", "which", "who", "whom", "this", "that", "these", "those", "am", "is", "are", "was", "were", "be", "been", "being", "have", "has", "had", "having", "do", "does", "did", "doing", "a", "an", "the", "and", "but", "if", "or", "because", "as", "until", "while", "of", "at", "by", "for", "with", "about", "against", "between", "into", "through", "during", "before", "after", "above", "below", "to", "from", "up", "down", "in", "out", "on", "off", "over", "under", "again", "further", "then", "once", "here", "there", "when", "where", "why", "how", "all", "any", "both", "each", "few", "more", "most", "other", "some", "such", "only", "own", "same", "so", "than", "too", "very", "s", "t", "can", "will", "just", "should", "now"],
            notters: vec!["no", "not", "doesn't", "don't", "neither", "nor", "shouldn't", "couldn't", "wouldn't", "won't", "isn't", "aren't", "nobody", "none", "nothing"],
            separators: vec![",", ":", ";", "."],
            min_occurence: 30,
            max_occurence: 2000,
        }
     }
}
