AS=nasm # Assembly compiler
ASFLAGS=-f elf64 -gdwarf # Assembly flags
LD=ld # Linker
LDFLAGS=#-m elf_x86_64 # Linker flags
SOURCES=$(wildcard ./src/*.asm) # Sources
OBJECTS=$(SOURCES:.asm=.o) # Object files
EXECUTABLE=aoc # Program name

# Check version
all: $(SOURCES) $(EXECUTABLE)

# Create executable
$(EXECUTABLE): $(OBJECTS)
	$(LD) $(LDFLAGS) $(OBJECTS) -o $@

# Compile assembly program
$(OBJECTS): $(SOURCES)
	$(AS) $(ASFLAGS) $(SOURCES)

# Clean folder
clean:
	rm src/*.o $(EXECUTABLE)

# Clean folder
run: all
	./aoc
