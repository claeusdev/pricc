use clap::Parser;
use std::{
    fs::{self},
    path::PathBuf,
};

// Import all templates
const TEMPLATE_MAIN: &str = include_str!("../templates/main.c.template");
const TEMPLATE_HEADER: &str = include_str!("../templates/header.h.template");
const TEMPLATE_MAKEFILE: &str = include_str!("../templates/Makefile.template");
const TEMPLATE_README: &str = include_str!("../templates/README.md.template");
const TEMPLATE_GITIGNORE: &str = include_str!("../templates/gitignore.template");
const TEMPLATE_TEST: &str = include_str!("../templates/test.c.template");

#[derive(Debug)]
pub enum PriccError {
    ConfigError(String),
    IoError(std::io::Error),
    TemplateError(String),
}

impl std::error::Error for PriccError {}
impl std::fmt::Display for PriccError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            PriccError::ConfigError(msg) => write!(f, "Configuration error: {}", msg),
            PriccError::IoError(e) => write!(f, "IO error: {}", e),
            PriccError::TemplateError(msg) => write!(f, "Template error: {}", msg),
        }
    }
}

impl From<std::io::Error> for PriccError {
    fn from(error: std::io::Error) -> Self {
        PriccError::IoError(error)
    }
}

#[derive(Debug)]
pub struct PriccConfig {
    pub name: String,
    pub author: Option<String>,
    pub proj_version: String,
    pub description: Option<String>,
    pub c_standard: CStandard,
    pub include_tests: bool,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum CStandard {
    C89,
    C99,
    C11,
    C17,
}

impl CStandard {
    fn to_flag(&self) -> &'static str {
        match self {
            CStandard::C89 => "-std=c89",
            CStandard::C99 => "-std=c99",
            CStandard::C11 => "-std=c11",
            CStandard::C17 => "-std=c17",
        }
    }
}

impl Default for PriccConfig {
    fn default() -> Self {
        Self {
            name: String::new(),
            author: None,
            proj_version: "0.1.0".to_string(),
            description: None,
            c_standard: CStandard::C11,
            include_tests: false,
        }
    }
}

impl PriccConfig {
    pub fn new(name: String) -> Result<Self, PriccError> {
        if name.is_empty() {
            return Err(PriccError::ConfigError(
                "Project name cannot be empty".to_string(),
            ));
        }

        Ok(Self {
            name,
            ..Default::default()
        })
    }

    pub fn with_author(mut self, author: String) -> Self {
        self.author = Some(author);
        self
    }

    pub fn with_tests(mut self) -> Self {
        self.include_tests = true;
        self
    }
}

struct Template<'a> {
    content: &'a str,
}

impl<'a> Template<'a> {
    fn new(content: &'a str) -> Self {
        Self { content }
    }

    fn render(&self, vars: &[(&str, &str)]) -> String {
        let mut result = self.content.to_string();
        for (key, value) in vars {
            result = result.replace(&format!("{{{{{}}}}}", key), value);
        }
        result
    }
}

pub struct ProjectBuilder {
    config: PriccConfig,
    root_dir: PathBuf,
}

impl ProjectBuilder {
    pub fn new(config: PriccConfig) -> Self {
        Self {
            root_dir: PathBuf::from(&config.name),
            config,
        }
    }

    pub fn build(&self) -> Result<(), PriccError> {
        self.create_directory_structure()?;
        self.create_source_files()?;
        self.create_build_files()?;
        self.create_docs()?;
        if self.config.include_tests {
            self.create_test_structure()?;
        }
        self.init_git()?;

        Ok(())
    }

    fn create_directory_structure(&self) -> Result<(), PriccError> {
        let dirs = ["src", "include", "docs", "build", "bin"];

        for dir in dirs.iter() {
            fs::create_dir_all(self.root_dir.join(dir))?;
        }

        Ok(())
    }

    fn create_source_files(&self) -> Result<(), PriccError> {
        // Create main.c
        let main_template = Template::new(TEMPLATE_MAIN);
        let main_content = main_template.render(&[
            ("name", &self.config.name),
            (
                "author",
                self.config.author.as_deref().unwrap_or("Anonymous"),
            ),
            (
                "description",
                self.config.description.as_deref().unwrap_or("A C project"),
            ),
        ]);

        fs::write(self.root_dir.join("src").join("main.c"), main_content)?;

        // Create header file
        let header_template = Template::new(TEMPLATE_HEADER);
        let header_content = header_template.render(&[
            ("name", &self.config.name),
            ("guard", &self.config.name.to_uppercase()),
            (
                "author",
                self.config.author.as_deref().unwrap_or("Anonymous"),
            ),
            (
                "description",
                self.config.description.as_deref().unwrap_or("A C project"),
            ),
        ]);

        fs::write(
            self.root_dir
                .join("include")
                .join(format!("{}.h", self.config.name)),
            header_content,
        )?;

        Ok(())
    }

    fn create_build_files(&self) -> Result<(), PriccError> {
        let makefile_template = Template::new(TEMPLATE_MAKEFILE);
        let makefile_content = makefile_template.render(&[
            ("name", &self.config.name),
            ("cstandard", self.config.c_standard.to_flag()),
        ]);

        fs::write(self.root_dir.join("Makefile"), makefile_content)?;

        Ok(())
    }

    fn create_docs(&self) -> Result<(), PriccError> {
        let readme_template = Template::new(TEMPLATE_README);
        let readme_content = readme_template.render(&[
            ("name", &self.config.name),
            (
                "description",
                self.config.description.as_deref().unwrap_or("A C project"),
            ),
            (
                "author",
                self.config.author.as_deref().unwrap_or("Anonymous"),
            ),
            ("version", &self.config.proj_version),
        ]);

        fs::write(self.root_dir.join("README.md"), readme_content)?;

        Ok(())
    }

    fn create_test_structure(&self) -> Result<(), PriccError> {
        fs::create_dir_all(self.root_dir.join("tests"))?;

        let test_template = Template::new(TEMPLATE_TEST);
        let test_content = test_template.render(&[("name", &self.config.name)]);

        fs::write(
            self.root_dir.join("tests").join("test_main.c"),
            test_content,
        )?;

        Ok(())
    }

    fn init_git(&self) -> Result<(), PriccError> {
        let gitignore_template = Template::new(TEMPLATE_GITIGNORE);
        let gitignore_content = gitignore_template.render(&[("name", &self.config.name)]);

        fs::write(self.root_dir.join(".gitignore"), gitignore_content)?;

        std::process::Command::new("git")
            .arg("init")
            .current_dir(&self.root_dir)
            .output()
            .map_err(|e| PriccError::IoError(e))?;

        Ok(())
    }
}

#[derive(Parser)]
#[command(
    name = "pricc",
    about = "A C project generator",
    version,
    author = "Your Name <your.email@example.com>"
)]
pub struct Cli {
    /// Name of the project
    #[arg(required = true)]
    name: String,

    /// Author of the project
    #[arg(short, long)]
    author: Option<String>,

    /// Description of the project
    #[arg(short, long)]
    description: Option<String>,

    /// C standard to use
    #[arg(short, long, value_enum, default_value_t = CStandard::C11)]
    standard: CStandard,

    /// Include test setup
    #[arg(short, long)]
    tests: bool,

    /// Project version number
    #[arg(short = 'v', long = "proj-version", default_value = "0.1.0")]
    proj_version: String,
}

impl From<Cli> for PriccConfig {
    fn from(cli: Cli) -> Self {
        let mut config = PriccConfig::new(cli.name).expect("Invalid project name");

        if let Some(author) = cli.author {
            config = config.with_author(author);
        }

        if cli.tests {
            config = config.with_tests();
        }

        config.description = cli.description;
        config.proj_version = cli.proj_version;
        config.c_standard = cli.standard;

        config
    }
}

pub fn run() -> Result<(), PriccError> {
    let cli = Cli::parse();
    let config: PriccConfig = cli.into();
    let builder = ProjectBuilder::new(config);
    builder.build()
}
