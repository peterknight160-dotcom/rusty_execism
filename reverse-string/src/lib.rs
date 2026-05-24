
 extern crate unicode_reverse;
 use unicode_reverse::reverse_grapheme_clusters_in_place;
pub fn reverse(input:  &str) -> String {
    let  mut local_str = String::from(input);
   reverse_grapheme_clusters_in_place( &mut local_str);
   local_str
   
   
}


