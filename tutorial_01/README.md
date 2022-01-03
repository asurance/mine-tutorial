# 项目准备和基本设置

1. `Rust`安装  
   略
2. `amethyst`一些概念简单介绍
   1. 状态
      amethyst将整个游戏视为状态机来看,同时内置了栈来管理状态,不过本教程中只用到一个状态,就不深入解释了. 
   2. ecs (entity + component + system)系统 + 资源(resource)
      1. entity 就是实体,更准确的说是实体的id,通过id能找到对应的component
      2. component 就是组件,主要作用是存放实体上数据的,供system使用
      3. system 就是系统,是逻辑的主要区域,通过对指定compnonet和resource的访问来进行数据的更新和操作
      4. resource 资源,这个资源不局限于图片资源一类的,像扫雷游戏中的计时器也可以用resource的方式实现,个人理解resource是为了解决某些数据在游戏中只会存在一份,与其为此单独创建一个实体去存放,不如直接以资源的方式直接访问方便.
   3. 详细内容还请参考[官方文档](https://book.amethyst.rs/book/stable/concepts/intro)

3. 项目依赖安装
   1. 使用`cargo new miner`命令创建新项目并将创建的miner作为之后的工作目录
   2. 在`Cargo.toml`中将依赖库添加
      ``` toml
      [dependencies.amethyst]
      version = "0.15"
      features = ["vulkan"]  // 苹果电脑应该需要为metal,但我电脑不是,所以没测试过
      ```
   3. 运行`cargo run`等到控制台打印`Hello,world!`说明依赖安装完成

4. 项目基本设置
   1. 创建一个`Mine`的状态
      ``` Rust
      struct Mine;
      impl SimpleState for Mine;
      ``` 
   2. `main`函数修改
      ``` Rust
      fn main() -> amethyst::Result<()> {

        // amethyst日志输出
        amethyst::start_logger(Default::default());
        
        // 获取资源根目录,assets目录会在之后存放图片资源
        let app_root = amethyst::utils::application_root_dir()?;
        let assets_root = app_root.join("assets");

        // system一类的设置,之前新建的system都会链式添加这后面
        let game_data = GameDataBuilder::default();

        // 创建应用并运行
        let mut game = Application::new(assets_root, Mine, game_data)?;
        game.run();

        Ok(())
      }
      ``` 
   3. `cargo run`运行,此时会发现控制台会有amethyst自带日志信息,并且程序不会自动退出.
5. 创建最基本的窗口
   1. `main`函数修改,添加基本窗口配置
      ``` Rust
      // ...
      let game_data = GameDataBuilder::default().with_bundle(
          RenderingBundle::<DefaultBackend>::new().with_plugin(
              RenderToWindow::from_config(DisplayConfig {
                  title: "扫雷".to_string(),
                  dimensions: Some((800, 600)),
                  ..Default::default()
              })
              .with_clear([0.0, 0.0, 0.0, 1.0]),
          ),
      )?;
      // ...
      ```
    2. `cargo run`运行,现在有了一个纯黑的窗口.

附: 如果你的编辑器不能自动导入包,那么需要在`main.rs`开始添加如下代码
``` Rust
use amethyst::{
    prelude::*,
    renderer::{types::DefaultBackend, RenderToWindow, RenderingBundle},
    window::DisplayConfig,
    GameDataBuilder,
};
```