import { defineConfig } from 'vite';

export default defineConfig({
  build: {
    target: 'esnext',
    lib: {
      entry: './index.ts',
      name: 'transmission'
    },
  },
});
