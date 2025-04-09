// tests9.rs
//
// Rust 非常擅长与 C/C++ 以及其他静态编译语言共享 FFI（外部函数接口），甚至可以在代码内部进行链接！就像下面的代码一样，它通过 extern 块实现这一点。
//
// `extern` 关键字后面的短字符串表示导入函数使用的 ABI（应用二进制接口）。在这个练习中使用的是 "Rust"，当然也存在其他的变体，比如标准 C 的 "C"，或 Windows 使用的 "stdcall"。
//
// 在 extern 块中声明要导入的函数时，用分号（;）结束函数签名，而不是使用花括号。一些属性可以应用到这些函数声明上以修改链接行为，比如 `#[link_name = ".."]`，它可以修改实际使用的符号名。
//
// 如果你想将一个符号导出到链接环境中，也可以在函数定义前加上 `extern` 关键字，并指定 ABI 字符串。Rust 函数的默认 ABI 就是 "Rust"，所以如果你只是在不同 Rust 模块之间链接，甚至可以省略整个 extern。
//
// Rust 默认会对函数名进行“符号重整”（mangling），这与 C++ 的做法类似。为了禁止这种行为并使函数可以通过名称进行链接，可以使用属性 `#[no_mangle]`。
//
// 在这个练习中，你的任务是让测试代码可以调用模块 Foo 中的 `my_demo_function`。`my_demo_function_alias` 是 `my_demo_function` 的别名，所以测试用例中的两行代码应该调用的是同一个函数。
//
// 你**不能**修改任何现有代码，除了添加两行属性。


extern "Rust" {
    fn my_demo_function(a: u32) -> u32;
    #[link_name="my_demo_function"]
    fn my_demo_function_alias(a: u32) -> u32;
}

mod Foo {
    // No `extern` equals `extern "Rust"`.
    #[no_mangle]
    fn my_demo_function(a: u32) -> u32 {
        a
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        // The externally imported functions are UNSAFE by default
        // because of untrusted source of other languages. You may
        // wrap them in safe Rust APIs to ease the burden of callers.
        //
        // SAFETY: We know those functions are aliases of a safe
        // Rust function.
        unsafe {
            my_demo_function(123);
            my_demo_function_alias(456);
        }
    }
}
