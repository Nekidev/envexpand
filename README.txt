A simple program to expand environment variables in a string.

Usage: ee [OPTIONS] [INPUT]

Arguments:
  [INPUT]  The input string to expand. If not provided, the program will read from STDIN until EOF.

Options:
  -v, --variables <VARIABLES>  One or more custom variables to use in the expansion. Each variable should be in the format NAME=VALUE.
  -n, --no-env                 Only use the variables provided with the --variables flag, ignoring any environment variables.
  -u, --use-empty              If set, variables that are not found will be replaced with an empty string instead of being left unchanged.
  -h, --help                   Print help
  -V, --version                Print version
  