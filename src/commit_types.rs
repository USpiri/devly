use std::{ fmt, fs, env };
use serde::{ Deserialize, Serialize };

#[derive( Debug, Serialize, Deserialize )]
pub struct CommitType {
    pub name: String,
    pub emoji: String,
    pub description: String,
    pub release: bool,
}

impl fmt::Display for CommitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {:<15}: {}", self.emoji, self.name, self.description)
    }
}

pub fn load_commit_types() -> Result<Vec<CommitType>, Box<dyn std::error::Error>> {

    // Instalation path
    let mut current_exe = env::current_exe().expect("No se puede obtener la ruta actual");
    current_exe.pop();
    current_exe.push("devly.toml");

    // Reading devly.toml file
    let content = fs::read_to_string(current_exe)?;
    let data: toml::Value = content.parse()?;
    let commit_types = data
        .get("CommitType")
        .ok_or("Missing [CommitType] section")?
        .as_array()
        .ok_or("Invalid [CommitType] section")?
        .iter()
        .map(|ct| {
            ct.clone().try_into::<CommitType>().map_err(|e| {
                format!("Failed to parse CommitType: {}", e).into()
            })
        })
        .collect::<Result<Vec<CommitType>, Box<dyn std::error::Error>>>()?;

    Ok(commit_types)
}