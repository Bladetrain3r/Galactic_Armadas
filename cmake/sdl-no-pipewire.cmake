# Loaded via CMAKE_TOOLCHAIN_FILE before SDL's project() call.
# This file does not name a system or a compiler.
#
# Bundled SDL 2.0 still passes a struct pw_proxy * to pw_node_enum_params.
# PipeWire 1.4 made that argument a struct pw_node *, and GCC 14 treats the
# mismatch as an error. This scene does not play audio. PulseAudio and ALSA
# stay available when their headers are installed.
set(SDL_PIPEWIRE OFF CACHE BOOL "Use Pipewire audio" FORCE)
set(SDL_PIPEWIRE_SHARED OFF CACHE BOOL "Dynamically load Pipewire support" FORCE)
