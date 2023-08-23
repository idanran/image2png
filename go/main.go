package main

import (
	"flag"
	"fmt"
	"os"

	"github.com/h2non/bimg"
)

func main() {
	// 定义命令行参数
	var input string
	var output string
	flag.StringVar(&input, "i", "", "输入图片文件的路径")
	flag.StringVar(&output, "o", "", "输出图片文件的路径")
	flag.Parse()

	// 检查参数是否有效
	if input == "" || output == "" {
		fmt.Println("请输入有效的参数")
		flag.Usage()
		os.Exit(1)
	}

	buffer, err := bimg.Read(input)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
	}

	newImage, err := bimg.NewImage(buffer).Convert(bimg.PNG)
	if err != nil {
		fmt.Fprintln(os.Stderr, err)
	}

	imgType := bimg.NewImage(newImage).Type()

	// 4. 写入文件
	err = os.WriteFile(output, newImage, 0o666)
	if err != nil {
		fmt.Println("写入图片文件失败:", err)
		os.Exit(2)
	}

	fmt.Println("图片已转换为:", imgType)
}
