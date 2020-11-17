use strikeout::scan::scan_new_file;
use strikeout::scan::get_old_file_list;

fn main() {
    let mut file_set = get_old_file_list();
    let list = scan_new_file("src", &mut file_set);
    for file in list.into_iter() {
        println!("{} @ {}", file.file_name().to_str().unwrap(), file.path().to_str().unwrap());
    }
    println!("    ####    \nfile set:\n{:?}", file_set);
}
