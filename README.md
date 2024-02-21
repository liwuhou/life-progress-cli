# life-progress-cli

```shell
Usage: life-progress-cli [OPTIONS] [COMMAND]

Commands:
  search  Search the country in the list
  view    View the country detail
  config  Custom config file path
  help    Print this message or the help of the given subcommand(s)

Options:
  -b, --birthday <BIRTHDAY>  Birthday, eg: 2024-02-20|20240220|unix_timestamp
  -c, --config <CONFIG>      Custom config file path
  -g, --gender <GENDER>      Gender, eg: 0: female, 1: male
  -n, --nation <NATION>      Nation, use search command to find your nation
  -h, --help                 Print help
  -V, --version              Print version
```

Sample Example

```shell
$ life-progress-cli -b 2020-10-24

You spent 1215 days, completed 4.46% of life progress, still have 26006 days left. enjoy!
```

You can use `life-progress-cli search` your country in the list.

```shell
$ life-progress-cli search china

People's Republic of China { Average: 77.72, Male: 75, Female: 80.7 }

$ life-progress-cli search korea

North Korea { Average: 71.77, Male: 67.88, Female: 75.88 }
South Korea { Average: 82.97, Male: 79.88, Female: 86.24 }
```
