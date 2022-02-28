定义了一个`trait Null`, 该trait提供`is_null`方法用于判空。

`u8`、`u16`、`u32`, `None`等类型实现了Null, 如果你有一个`u32`类型的属性可能为空，并且您保证该数字不会达到它的最大值，那么你无需使用`Option<u32>`, 你可以用`u32`的最大值作为`null`，`is_null`发现`self`为u32::MAX时，将返回true