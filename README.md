# image2png

[![npm](https://img.shields.io/npm/v/image2png?style=flat-square)](https://www.npmjs.com/package/image2png)

## API
```ts
function conver(input: string, output: string): Promise<void>
```

input 和 output 为文件地址。

## Build

```
yarn build
```
然后把编码器 (bimg-cli.exe) 放入 lib 目录。

## Build `bimg-cli.exe`

https://github.com/h2non/bimg/issues/435#issuecomment-1336550063

```
cd go
go build -ldflags '-w -s'
upx bimg-cli.exe
```