use std::{fs::File, io::Write};

fn main()-> std::io::Result<()> {
    let mut new_file = File::create("./makefile")?;
    
    let template_makefile = r#"# Compiler
CC = gcc

# Compiler flags (uncomment and customize as needed)
# CFLAGS = -Wall -Wextra -O2

# Source files (add your source files here)
# SRC = main.c file1.c file2.c

# Object files (automatically generated from SRC)
# OBJ = $(SRC:.c=.o)

# Output executable name (customize as needed)
# OUT = myprogram

# Include directories (uncomment and add paths as needed)
# INC_DIRS = -I/path/to/include/dir1 -I/path/to/include/dir2

# Library directories (uncomment and add paths as needed)
# LIB_DIRS = -L/path/to/lib/dir1 -L/path/to/lib/dir2

# Libraries to link (uncomment and add libraries as needed)
# LIBS = -lmylib1 -lmylib2

# Default target (uncomment if you want to specify a default target)
# all: $(OUT)

# Build rules (uncomment and customize as needed)
# $(OUT): $(OBJ)
# 	$(CC) $(CFLAGS) $(INC_DIRS) $(LIB_DIRS) -o $(OUT) $(OBJ) $(LIBS)

# Clean rule (uncomment if you want to provide a clean target)
# clean:
# 	rm -f $(OUT) $(OBJ)

# .PHONY targets (uncomment if you want to declare phony targets)
# .PHONY: all clean
"#;
    new_file.write_all(template_makefile.as_bytes())?;
    Ok(())
}
