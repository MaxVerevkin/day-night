# day-night

A simple program that executes shell commands based on the time of day

## Options

```
❯ day-night --help
day-night 0.1.0
Max Verevkin <me@maxverevkin.tk>
A simple program that executes shell commands based on the time of day

USAGE:
    day-night [FLAGS] [OPTIONS] --lat <latitude> --lon <longitude>

FLAGS:
        --always-run    If set, a command will be run on each update. Otherwise, a command will be run only when
                        changing states (from night to day and vice versa)
    -h, --help          Prints help information
    -V, --version       Prints version information

OPTIONS:
        --day <day>              A shell command to run in day time
        --interval <interval>    The interval between updates in seconds [default: 600]
        --lat <latitude>         Your geographical latitude
        --lon <longitude>        Your geographical longitude
        --night <night>          A shell command to run in night time
```
