use indoc::indoc;
use puzzle_core::error::cli_error;

/// 查看帮助文档
pub fn help() {
    let message = indoc! {
        "
        全部用法:
        puzzle build
            --path=<project-path>                     * 项目路径

            --features=<option1,option2,...>            功能选项
                ansi-color                              开启终端 Ansi 颜色
                all                                     开启以上功能
                none                                    关闭以上功能          [默认]

            --exports=<option1,option2,...>             导出选项
                ast                                     导出 AST json
                all                                     开启以上功能
                none                                    关闭以上功能          [默认]

            --infos=<option1,option2,...>               日志信息选项
                progress                                开启显示程序进度
                ignore                                  开启显示忽略规则
                file                                    开启显示文件分析详情
                all                                     开启以上功能
                none                                    关闭以上功能          [默认]

        puzzle version                                  查看 Puzzle CLI 以及第三方依赖版本信息

        puzzle help                                     查看 Puzzle CLI 帮助文档

        注: * 表示必传参数
        "
    };
    println!("{}", message);
}

/// 查看版本
pub fn version() {
    let message = indoc! {
        "
        ┌──────────────────────────────┐
        │        Puzzle v0.0.1         │
        │     Rust v1.93.1 Runtime     │
        ├──────────────────────────────┤
        │ • indoc               v2.0.7 │
        └──────────────────────────────┘
        "
    };
    println!("{}", message);
}

/// 未知命令
pub fn unknown() {
    cli_error("未知命令: 请使用 help 命令查看使用帮助手册")
}
