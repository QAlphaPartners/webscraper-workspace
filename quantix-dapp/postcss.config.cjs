// postcss.config.cjs
const postcssImport = require('postcss-import');
const postcssNesting = require('postcss-nesting');
const autoprefixer = require('autoprefixer');
const cssnano = require('cssnano');

module.exports=(ctx)=> ({
  from: ctx.from,
  to: ctx.to,
  plugins: [
    postcssImport({
      path: ['src/pcss'], // specify the path to the pcss folder
    }),
    postcssNesting,
    autoprefixer,
    // cssnano({
    //   preset: 'default',
    // }),
  ],
});
