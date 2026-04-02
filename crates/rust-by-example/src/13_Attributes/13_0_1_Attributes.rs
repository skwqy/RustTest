/*
   属性是应用于某些模块、crate 或项的元数据（metadata）。这元数据可以用来：

      1、条件编译代码
      2、设置 crate 名称、版本和类型（二进制文件或库）
      3、禁用 lint （警告）
      4、启用编译器的特性（宏、全局导入（glob import）等）
      5、链接到一个非 Rust 语言的库
      6、标记函数作为单元测试
      7、标记函数作为基准测试的某个部分
   
   当属性作用于:
      1、整个 crate 时，它们的语法为 #![crate_attribute]，
      2、模块时，语法为 #[item_attribute]（注意少了感叹号 !）。
      3、项时，语法为 #[item_attribute]（注意少了感叹号 !）。

   属性可以接受参数，有不同的语法形式：

      1、#[attribute = "value"]
      2、#[attribute(key = "value")]
      3、#[attribute(value)]
   
   属性可以多个值，它们可以分开到多行中：
      1、#[attribute(value, value2)]
      2、#[attribute(value, value2, value3, value4, value5)]
*/






