include!(concat!(env!("OUT_DIR"), "/gitversion.rs"));

fn main() {
    println!("Display: {}", GIT_VERSION);
    println!("Debug:   {:?}", GIT_VERSION);
    println!("SHA:     {}", GIT_VERSION.sha);
    println!("Commit:  {}", GIT_VERSION.commit_date);
}
