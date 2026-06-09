use crate::server::ConfigFileGetter;

pub fn SecurityCheck() -> Result<(), String> {
    // Config file exists
    println!("\t## Checking config file...");
    match ConfigFileGetter() {
        Ok(DATA) => {
            println!("\t\t### {}", DATA.fileFeedback);
            drop(DATA.file);
        }
        Err(e) => {
            return Err(format!("Error getting config file: {:?}", e));
        }
    };

    // Ok
    return Ok(());
}
