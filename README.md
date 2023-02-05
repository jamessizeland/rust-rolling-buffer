# rust-rolling-buffer

Attempt to build an efficient 'lossy fifo' buffer to use as a graphing buffer for data.

It should accept all primitive data types.

## Examples

Two ways to read all of the data from the buffer, as a simple vector, or as a lazy Iterator to use more flexibly later.

```Rust
let mut buff = RollingBuffer::new(20);

for num in 0..40 {
    buff.add(num);
    let vec_out = buff.values();
    let iter_out: Vec<_> = buff.values_iter().cloned().collect();
    println!("len: {} - {:?}", buff.len(), vec_out);
    println!("len: {} - {:?}", buff.len(), iter_out);
    assert_eq!(vec_out, iter_out);
}
```

Can combine the Iterator output with the enumerate method to get a vector of values with their index position.

```Rust
let mut buff = RollingBuffer::new(20);

for num in 0..40 {
    buff.add(num);
}
let data: Vec<(usize, _)> = buff.values_iter().cloned().enumerate().collect();
println!("{:?}", data);
```

Works on other data types too

```Rust
let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
let mut buff = RollingBuffer::new(20);

for num in 0..40 {
    let num = alphabet.chars().nth(num).unwrap_or('Z');
    buff.add(num);
    let vec_out = buff.values();
    let iter_out: Vec<_> = buff.values_iter().cloned().collect();
    assert_eq!(vec_out, iter_out);
}
```

Complex data types accepted with some derive macros

```Rust
#[derive(Default, Clone, Copy)]
struct Data {
    x: f32,
    y: f32,
}

let mut buff = RollingBuffer::new(20);

for num in 0..40_u8 {
    let num: f32 = num.into();
    let point = Data {
        x: num,
        y: num * 5.3,
    };
    buff.add(point);
    let vec_out = buff.values();
    let iter_out: Vec<_> = buff.values_iter().cloned().collect();
}
```
