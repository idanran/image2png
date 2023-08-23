import { spawn } from 'child_process'
import { join } from 'path'

export function conver(input: string, output: string): Promise<void> {
    return new Promise((resolve, reject) => {
        const args = ['-i', input, '-o', output]
        const child = spawn(join(__dirname, './bimg-cli.exe'), args)
        let data = ''
        child.stdout.on('data', chunk => data += chunk.toString())
        child.on('exit', () => {
            if (data.includes('png')) {
                resolve()
            } else {
                reject('转换失败')
            }
        })
    })
}