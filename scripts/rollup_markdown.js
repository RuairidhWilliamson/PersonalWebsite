const { createFilter } = require("rollup-pluginutils");
const { marked } = require('marked');
const { resolve, extname, join } = require('path');
const fs = require('fs');
const crypto = require('crypto');
const sharp = require('sharp');

function markdown(opts = {}) {
    const {
        include = ["**/*.md"],
        exclude,
        emit = true,
        publicPath = '/client/',
    } = opts;
    if (!include) {
        throw Error("include option should be specified");
    }
    const mdFilter = createFilter(include, exclude);
    const imgFilter = createFilter(['**/*.png', '**/*.jpg', '**/*.gif']);
    const copies = Object.create(null);

    function processImg(path) {
        let ext = extname(path);
        if (['.png', '.jpg', '.gif'].includes(ext)) {
            ext = '.webp';
        }
        const hash = crypto.createHash('sha1').update(path).digest('hex').substring(0, 16);
        const dest = `${hash}${ext}`;
        copies[path] = dest;

        return `${publicPath}${dest}`;
    }

    return {
        name: "markdown",
        load(id) {
            this.addWatchFile(id);
            if (!imgFilter(id)) return;
            return `export default "${processImg(id)}"`;
        },
        transform(code, id) {
            this.addWatchFile(id);
            if (mdFilter(id)) {
                const md = code;
                const replaced_md = md.replace(/!\[(.+)\]\((.+)\)/g, function(_, $1, $2) {
                    const path = resolve($2);
                    const newPath = processImg(path);
                    return `![${$1}](${newPath})`;
                });
                const html = JSON.stringify(marked.parse(replaced_md));


                return {
                    code: `export default ${html};`,
                    map: { mappings: "" }
                };
            }
        },
        generateBundle: async function(outputOptions) {
            if (!emit) return;
            const base = outputOptions.dir;
            await fs.promises.mkdir(base, {recursive: true});
            await Promise.all(Object.keys(copies).map(async src => {
                const dest = copies[src];
                const output = join(base, dest);
                if (imgFilter(src)) {
                    return sharp(src, {animated: true}).resize(600).webp().toFile(output);
                } else {
                    return fs.promises.copyFile(src, output);
                }
            }));
        },
    };
}

exports.markdown = markdown;
