# Yew ANSI

ANSI escape code rendering for the web

This library doesn't cover all ANSI escape codes, only SGR parameters.

Supported:

- bold
- italic
- underline
- foreground and background colours:
  - 3/4 bit (named colours with "bright" modifier)
  - 8-bit (256-color palette)
  - 24-bit (full RGB)
