use unicode_reverse::reverse_grapheme_clusters_in_place;
pub fn reverse(input: &str) -> String {
    let output = &mut input.to_string();
    reverse_grapheme_clusters_in_place(output);
    output.to_string()
}
