# Current Task

BBD-WAL-013 REOPENED — user reports empty native window.
Actual running binary matches accepted build; desktop supplies both Wayland and X11.
Winit Wayland hide/show is a no-op, missed by prior Xvfb acceptance.
Active test-source authorization: SOL_BBD_WAL_013_BLANK_WINDOW_TESTS_01.md.
Reviewer fixes architecture to X11/XWayland broker launch and requires actual rendered
control screenshot plus blank-content falsification. No production edit until red.
Preserve unrelated npm/policy changes. No installer work or owner relay.
