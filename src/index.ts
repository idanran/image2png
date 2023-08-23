import { spawn } from 'child_process'
import { join } from 'path'

export function conver(input: string, output: string): Promise<void> {
    return new Promise((resolve, reject) => {
        const args = [input, output]
        const child = spawn(join(__dirname, './image-cli.exe'), args)
        let data = ''
        child.stdout.on('data', chunk => data += chunk.toString())
        child.on('exit', () => {
            if (data.includes('dimensions')) {
                resolve()
            } else {
                reject('转换失败')
            }
        })
    })
}