CFLAGS = -Wall -Wextra -g
TARGET := searchit

CODE_FOLDER := src
BUILD_FOLDER := build
DEPS_FOLDER := deps
SOURCE_C_CODE := $(wildcard $(CODE_FOLDER)/*.c)

MUPDF_DIR := $(DEPS_FOLDER)/mupdf
MUPDF_INCLUDE := $(MUPDF_DIR)/include
MUPDF_LIB := $(MUPDF_DIR)/build/release

CFLAGS += -I$(MUPDF_INCLUDE)
LDLIBS := -L$(MUPDF_LIB) -lmupdf -lmupdf-third -lm -lpthread -ldl

.PHONY: build clean run

build:
	mkdir -p $(BUILD_FOLDER)
	gcc $(CFLAGS) -O0 $(SOURCE_C_CODE) -o $(BUILD_FOLDER)/$(TARGET) $(LDLIBS)

clean:
	rm -r $(BUILD_FOLDER)

run:
	$(BUILD_FOLDER)/$(TARGET)
