# montools
Simple CLI Monitor Tools written in Rust

[Similar C++ verion](https://github.com/xTeamStanly/montools/tree/cpp)

## monb - Monitor Brightness
### Argument example

Set all monitors brightness to 10
```
monb.exe 10
monb.exe /*:10
```

Set specific monitor/s to brightness
```
monb.exe /1:20 - set monitor 1 brightness to 20
monb.exe /1:10 /2:30 - set monitor 1 brightness to 10, monitor 2 brightness to 30
```

Set specific monitor/s to brightness, but all others to different brightness
(Set monitor 1 brightness to 10, monitor 2 brightness to 30, set all other monitors brightness to 20)
```
monb.exe /1:10 /2:30 20
monb.exe 20 /1:10 /2:30
monb.exe /1:10 20 /2:30
monb.exe /*:20 /1:10 /2:30
```
