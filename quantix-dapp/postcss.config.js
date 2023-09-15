import postcssImport from 'postcss-import';
import postcssNesting from 'postcss-nesting';
import autoprefixer from 'autoprefixer';
import cssnano from 'cssnano';

export default {
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
}
