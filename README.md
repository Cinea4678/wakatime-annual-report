# WakaTime年度报告

一个美观、便捷、易用的WakaTime年度报告生成工具。

<img width="1552" alt="截图示例" src="https://github.com/Cinea4678/wakatime-annual-report/assets/45115933/cff8012b-d084-4c7e-b2d0-e5292247f0f1">

## 安装

前往Release页面下载对应自己操作系统版本的程序即可。

## 使用步骤

1. 前往[WakaTime设置中心](https://wakatime.com/settings/account)导出自己的完整记录，如下所示：
   ![点击Export](https://github.com/Cinea4678/wakatime-annual-report/assets/45115933/1ed8fc3b-272b-4ffb-9a76-0eefa404e823)
2. 在终端运行程序，并带上参数`-i <记录的文件地址>`，如下所示：
   <img width="962" alt="" src="https://github.com/Cinea4678/wakatime-annual-report/assets/45115933/85c6667a-04ce-48b1-ad93-23191a923d74">
3. 打开链接，查看结果。

## 参数文档

```
Usage: wakatime-annual-report.exe [OPTIONS] --input <INPUT>

Options:
  -i, --input <INPUT>          Input file path
      --no-serve               Don't serve by the program
      --year <YEAR>            The year to generate the report
      --time-zone <TIME_ZONE>  Your time zone, default as 8 (UTC+8) [default: 8.0]
  -t, --timeout <TIMEOUT>      Timeout of the WakaTime's time calculating. See WakaTime's FAQ for more info [default: 900.0]
      --host <HOST>            The host address of the web server [default: 127.0.0.1]
  -p, --port <PORT>            The host port of the web server [default: 8031]
  -h, --help                   Print help
  -V, --version                Print version
```

详细说明：
- `input`: WakaTime导出的JSON文件的本地位置。
- `no-serve`: 不提供网页服务，而是将结果输出到当前目录并退出。
- `year`: 需要生成年度报告的年份，可以不填。若不填将在程序运行时请求用户输入。
- `time-zone`: 用户所在的时区，该选项默认为中国标准时间（CST）。
- `timeout`: 记时粒度，默认使用WakaTime官方的默认值。参见<a>https://wakatime.com/faq#timeout</a>。
- `host`: 网页服务时的服务主机。
- `port`: 网页服务时的服务端口。

## 计划中的工作

作者正在准备期末考试周，无法马上完善这些功能点，如有不周敬请谅解

1. 完善报告内容
2. 实现超大型Json的流式读取
3. 改进代码架构，探索前后端结合的其他方式
   
