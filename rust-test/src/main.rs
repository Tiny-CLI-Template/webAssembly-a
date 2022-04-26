


fn main() {
    /* 基本输出语句 */
    println!("Hello, world!");

    /* 无符号8位 bit, 分别有 8 16 32 64 128 */
    let a : u8 = 1;
    /* 有符号8位 bit 分别有 8 16 32 64 128 */
    let b : i8 = -1;

    /* 长度取决于所运行的目标平台 ,32位系统就是32 */
    let c : usize = 1;

    let d : isize = 1;

    /* 浮点形 32位 */
    let e : f32 = 6.44;
    /* 浮点形 64位 */
    let e : f64 = 6.44;
    /* 默认为高精度的 f64 */
    let e = 6.44;

    /* 布尔类型 */
    let b = false;
    let b : bool = true;

    /* char类型 */
    let c = "5";
}


