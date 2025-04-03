# This Makefile is meant to be used only in development environments.

.DEFAULT_GOAL := _bruh

TARGET = app

_bruh:
	@echo "You didn't read the README, did you?"

c:
	@cmake -E remove_directory build && echo "[INFO]: Removed build directory."
	@cmake -E remove_directory .cache && echo "[INFO]: Removed .cache directory."
	@cmake -E remove_directory bin && echo "[INFO]: Removed .cache directory."

bud:
	@cmake -B build -S . --preset unix-debug
	@cmake --build build

bur:
	@cmake -B build -S . --preset unix-release
	@cmake --build build

bwd:
	@cmake -B build -S . --preset windows-debug
	@cmake --build build

bwr:
	@cmake -B build -S . --preset windows-release
	@cmake --build build

r:
	@./build/${TARGET}