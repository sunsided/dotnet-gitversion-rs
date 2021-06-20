include!(concat!(env!("OUT_DIR"), "/gitversion.rs"));

fn main() {
    // Use the "global" constant.
    println!("Display:      {}", GIT_VERSION);
    println!("Debug:        {:?}", GIT_VERSION);
    println!("SHA:          {}", GIT_VERSION.sha);
    println!("Commit:       {}", GIT_VERSION.commit_date);

    // The GitVersion::build() const function allows you to obtain
    // the struct as a constant.
    const GV: GitVersion = GitVersion::new();
    println!("Branch name:  {}", GV.branch_name);

    // Alternatively you can use the Default trait to obtain a new instance.
    let gv = GitVersion::default();
    println!("Short commit: {}", gv.short_sha);
}
