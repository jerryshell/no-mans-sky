# No Man's Sky

无用户时自动运行，监测到用户登录时自动终止并清理环境

**⚠️免责声明：本项目仅供学习交流使用，如作他用所承受的任何直接、间接法律责任一概与作者无关，下载使用即代表使用者同意上述事实**

```
./no-mans-sky --help
no-mans-sky 0.2.0
github.com/jerryshell/no-mans-sky

USAGE:
    no-mans-sky [OPTIONS]

OPTIONS:
    -h, --help                                                         Print help information
    -k, --kill-at-unix-timestamp-secs <KILL_AT_UNIX_TIMESTAMP_SECS>    [default: -1]
    -V, --version                                                      Print version information
```

## 示例

以下命令将会使程序在 2077 年 1 月 1 日 12 时 0 分 0 秒终止

```bash
 nohup ./no-mans-sky -k 3376699200 > /dev/null &
```

## 可执行文件下载

https://github.com/jerryshell/no-mans-sky/releases

## 开源许可证

[GNU Affero General Public License v3.0](https://choosealicense.com/licenses/agpl-3.0/)
