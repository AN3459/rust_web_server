# rust_web_server

# init

```shell
cargo init
```

# 代码格式化

1.在项目根目录中创建 rustfmt.toml 文件

2.在这个文件中添加以下配置
max_width = 100 // 设置最大行宽为 100 个字符
tab_spaces = 4 // 设置缩进宽度为 4 个空格
edition = "2018" // 设置 Rust 版本（根据实际项目版本进行调整）
use_small_heuristics = "Max" // 设置换行策略
newline_style = "Auto" // 设置换行符风格，根据平台自动选择

3.设置保存代码时自动格式化

1.通过 Ctrl + , 或者 Cmd + , 打开设置 2.选择 "File" -> "Preferences" -> "Settings"。 3.在右上角点击 "Open Settings (JSON)"，这将打开一个 JSON 文件，你可以在其中添加配置。 4.添加以下配置：

"editor.formatOnSave": true,
"rustfmt.configPath": ".rustfmt.toml"
