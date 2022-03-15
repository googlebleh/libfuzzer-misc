When running with `-jobs` set to a value `>1`, libFuzzer redirects output to a
file. This may be undesirable when the code **under test** produces enough
output logging to fill the disk.

This is a `LD_PRELOAD`able library to hook `system()`, used by libFuzzer to
spawn child fuzzing processes, so we can control the IO redirection.
