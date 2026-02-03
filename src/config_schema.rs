// 注意：KnConfig 结构体已被移除，因为当前代码中未使用
// 如果将来需要配置文件支持，可以重新添加

/// 占位符结构体，保持模块存在
#[allow(dead_code)]
pub struct ConfigSchema;

#[allow(dead_code)]
impl ConfigSchema {
    /// 生成默认配置示例（保留以备将来使用）
    pub fn example() -> &'static str {
        r#"# KN Configuration Example
# This file format is not currently used but reserved for future use
# default_agent = npm
# global_agent = npm
# auto_update = true
# log_level = info
"#
    }
}

// 测试已移除，因为KnConfig结构体不再使用
// 如果将来重新实现配置功能，可以恢复测试
