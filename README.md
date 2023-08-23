# image2png

[![npm](https://img.shields.io/npm/v/image2png?style=flat-square)](https://www.npmjs.com/package/image2png)

一个文件格式转换工具，支持WebP、JPEG转换为PNG。

## API
```ts
function conver(input: string, output: string): Promise<void>
```

input 和 output 为文件地址。

## Build

```
yarn build
```
然后把编码器 (image-cli.exe) 放入 lib 目录。

## Build `image-cli.exe`

```
cd rs
cargo build -r
```