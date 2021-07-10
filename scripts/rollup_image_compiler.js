import fs from 'fs';
import path from 'path';
import readdir from 'fs-readdir-recursive';
import sharp from 'sharp';


export default function imageCompiler() {
    return {
        name: 'image-compiler',
        load() {
            readdir('./images').forEach(element => {
                this.addWatchFile(path.join('./images', element));
            });
        },
        generateBundle() {
            readdir('./images').forEach(element => {
                const input_file = path.join('./images', element);
                const output_file = path.join('./public/images', element);
                fs.mkdirSync(path.dirname(output_file), { recursive: true });
                sharp(input_file).resize(600).toFile(output_file).then(() => {});
            });
        }
    };
}