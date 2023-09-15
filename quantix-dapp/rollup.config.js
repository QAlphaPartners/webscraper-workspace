import rollup_nre from '@rollup/plugin-node-resolve';
import rollup_tsc from '@rollup/plugin-typescript';

export default [
  {
    input: './src/main.ts',
    output: {
      file: './dist/js/app-bundle.js',
      format: 'iife',
      name: 'bundle',
      sourcemap: true
    },
    plugins: [
      // rollup_cjs(),
      rollup_nre(),
      rollup_tsc({
        tsconfig: './tsconfig.json',
        compilerOptions: {
          declaration: false,
          declarationDir: null
        }
      }),
      // terser({ compress: false, mangle: false, format: { comments: false } })
    ]
  }
]

