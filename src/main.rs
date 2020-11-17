use strikeout::scan;

fn main() {
    // when no cache
    let mut file_set = scan::get_file_list().unwrap_or_default();
    let list = scan::scan_new_file("src", &mut file_set);
    for file in list.into_iter() {
        println!("{} @ {}", file.file_name().to_str().unwrap(), file.path().to_str().unwrap());
    }
    println!("    ####    \nfile set:\n{:?}", file_set);
    if let Err(e) = scan::store_file_list(&file_set) {
        println!("File list cache failed.\n{}", e);
    };
}
