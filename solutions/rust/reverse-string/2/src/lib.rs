use unicode_reverse::reverse_grapheme_clusters_in_place as reverse_grapheme;

pub fn reverse(input: &str) -> String {
    let mut output = input.to_string();
    reverse_grapheme(&mut output);
    output
}
