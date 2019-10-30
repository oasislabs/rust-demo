import typescript from 'rollup-plugin-typescript';

export default {
    input: './app/src/index.ts',
    output: {
        name: 'SealedAuction',
        file: 'lib/bundle.js',
        format: 'umd',
    },
    plugins: [typescript()],
};
