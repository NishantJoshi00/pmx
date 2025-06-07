pub fn list(storage: &crate::storage::Storage) -> crate::Result<()> {
    let profile_list = storage.list_repos()?;

    if profile_list.is_empty() {
        println!("No profiles found.");
        return Ok(());
    }

    profile_list
        .iter()
        .for_each(|profile| println!("{}", profile));
    Ok(())
}


pub fn copy_profile(path: &str, storage: &crate::storage::Storage) -> crate::Result<()> {
    use arboard::Clipboard;
    use std::fs;
    
    let profile_path = storage.get_repo_path(path)?;
    let content = fs::read_to_string(&profile_path)?;
    
    let mut clipboard = Clipboard::new()?;
    clipboard.set_text(content)?;
    
    println!("Profile content copied to clipboard: {}", path);
    Ok(())
}

pub fn completion(shell: &crate::cli::Shell) -> crate::Result<()> {
    match shell {
        crate::cli::Shell::Zsh => {
            const ZSH_COMPLETION: &str = include_str!("../../completions/_pmx");
            print!("{}", ZSH_COMPLETION);
        }
    }
    Ok(())
}
