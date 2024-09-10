use std::{
    error::Error,
    fs::{self, File},
    io::Write,
};

pub struct Config {
    pub name: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let name = match args.next() {
            Some(arg) => arg,
            None => return Err("project name missing"),
        };

        Ok(Config { name })
    }
}

fn create_dirs(name: &str) -> Result<String, Box<dyn Error>> {
    let full_dir_name = format!("{name}/src");
    let _ = fs::create_dir_all(&full_dir_name)?;

    Ok(full_dir_name)
}

fn create_files(location: &str) -> Result<(), Box<dyn Error>> {
    let mut main_file = File::create(format!("{location}/main.c"))?;
    let _ = File::create(format!("{location}/lib.h"))?;

    let mut makefile = File::create("Makefile")?;
    let main_content = r#"
        #include <stdio.h>

        int main(){
            printf("Hey there!!!\n");
            return 0;
        }
    "#;

    let make_content = r#"
    # Variables
    CC = gcc                 # Compiler
    CFLAGS = -Wall -Wextra   # Compiler flags for warnings
    TARGET = main            # Output executable name

    # Source files
    SRCS = main.c            # List of source files
    OBJS = $(SRCS:.c=.o)     # List of object files (auto-generated from SRCS)

    # Default target to build the executable
    all: $(TARGET)

    # Link object files to create the executable
    $(TARGET): $(OBJS)
        $(CC) $(CFLAGS) -o $(TARGET) $(OBJS)

    # Compile source files into object files
    %.o: %.c
        $(CC) $(CFLAGS) -c $< -o $@

    # Clean up generated files
    clean:
        rm -f $(OBJS) $(TARGET)

    # Phony targets to prevent conflicts with files named 'all' or 'clean'
    .PHONY: all clean

    "#;

    main_file.write_all(main_content.as_bytes())?;
    makefile.write_all(make_content.as_bytes())?;
    Ok(())
}

pub fn init(config: Config) -> Result<(), Box<dyn Error>> {
    let dir_name = create_dirs(&config.name)?;
    create_files(&dir_name)?;
    Ok(())
}
