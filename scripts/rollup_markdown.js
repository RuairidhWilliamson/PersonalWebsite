const { createFilter } = require("rollup-pluginutils");
const { marked } = require('marked');
const { resolve, extname, join } = require('path');
const fs = require('fs');
const crypto = require('crypto');

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
    const filter = createFilter(include, exclude);
    const copies = Object.create(null);

    return {
        name: "markdown",

        transform(code, id) {
            if (filter(id)) {
                const md = code;
                const replaced_md = md.replace(/!\[(.+)\]\((.+)\)/g, function(_, $1, $2) {
                    const path = resolve($2);

                    const ext = extname(path);
                    const hash = crypto.createHash('sha1').update(path).digest('hex').substring(0, 16);
                    const dest = `${hash}${ext}`;
                    copies[path] = dest;

                    return `![${$1}](${publicPath}${dest})`;
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

                return fs.promises.copyFile(src, output);
            }));
        }
    };
}

exports.markdown = markdown;