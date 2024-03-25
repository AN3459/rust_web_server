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

# 迁移

sea-orm-cli migrate init -d src/common/database/migration

sea-orm-cli migrate generate -d src/common/database/migration user_table

sea-orm-cli migrate -d src/common/database/migration up

DATABASE_URL=postgresql://test:test@http://localhost/test cargo run --release -- migrations run

## 设置环境变量

export DATABASE_URL=postgresql://postgres:test@localhost:5432/postgres

## 执行迁移

sea-orm-cli migrate -d src/common/database/migration up

## 生成实体

sea-orm-cli generate entity -o src/common/database/db_entity --with-serde both

为了让生成的实体支持日期类型 uuid 类型 json 类型 需要在 sea-orm 的 feature 中添加以下内容
features = ["sqlx-postgres", "runtime-tokio-rustls", "macros", "debug-print","with-chrono","with-json","with-uuid"]

## neo4j

1.  docker pull neo4j

2.  > docker run -d --name container_name \ //-d 表示容器后台运行 --name 指定容器名字
        -p 7474:7474 -p 7687:7687 \  //映射容器的端口号到宿主机的端口号
        -v /home/neo4j/data:/data \  //把容器内的数据目录挂载到宿主机的对应目录下
        -v /home/neo4j/logs:/logs \  //挂载日志目录
        -v /home/neo4j/conf:/var/lib/neo4j/conf   //挂载配置目录
        -v /home/neo4j/import:/var/lib/neo4j/import \  //挂载数据导入目录
        --env NEO4J_AUTH=neo4j/password \  //设定数据库的名字的访问密码
        neo4j //指定使用的镜像

docker run -d --name neo4j_test -p 7474:7474 -p 7687:7687 -v /home/neo4j/data:/data -v /home/neo4j/logs:/logs -v /home/neo4j/conf:/var/lib/neo4j/conf -v /home/neo4j/import:/var/lib/neo4j/import --env NEO4J_AUTH=neo4j/password neo4j

## gremlim

1.首次启动

docker run --name janusgraph-default janusgraph/janusgraph:latest

docker run --rm --link janusgraph-default:janusgraph -e GREMLIN_REMOTE_HOSTS=janusgraph \
 -it janusgraph/janusgraph:latest ./bin/gremlin.sh

2.第二次启动

docker start janusgraph-default

docker run --rm --link janusgraph-default:janusgraph -e GREMLIN_REMOTE_HOSTS=janusgraph \
 -it janusgraph/janusgraph:latest ./bin/gremlin.sh


## 容器中需要执行的指令
cargo install sea-orm-cli
rustup component add rustfmt