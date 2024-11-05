import { parseArgs } from "@std/cli/parse-args";
import { denoPlugins } from "@luca/esbuild";
import * as esbuild from "@x/esbuild";

const usage = "Usage: deno run --allow-run --allow-env --allow-read bundle.ts --entryPoint=<path_to_ts_file>, --outputFolder=<path_to_output_folder>";

const flags = parseArgs(Deno.args, {
  "entryPoint": null,
  "outputFolder": null
});

async function validateArguments(flags) {
  if (!flags.entryPoint || !flags.outputFolder) {
    console.error("Missing required flags.\n" + usage);
    Deno.exit(1);
  } else {
    try {
      const entryPoint = await Deno.stat(flags.entryPoint);
      if (!entryPoint.isFile) {
        console.error("--entryPoint is not a file.\n" + usage);
        Deno.exit(1);
      }
      const outputFolder = await Deno.stat(flags.outputFolder);
      if (!outputFolder.isDirectory) {
        console.error("--outputFolder is not a folder or does not exist.\n" + usage);
        Deno.exit(1);
      }
      return [flags.entryPoint, flags.outputFolder];
    } catch (error) {
      console.error("Error: " + error + "\n" + usage);
      Deno.exit(1);
    }
  }
}

const [entryPoint, outputFolder] = await validateArguments(flags);

if (!entryPoint || !outputFolder) {
  console.error("Missing required flags.\n" + usage);
  Deno.exit(1);
} else {
  esbuild.build({
    plugins: [...denoPlugins()],
    entryPoints: [entryPoint],
    outdir: outputFolder,
    bundle: true,
    platform: "browser",
    format: "esm",
    target: "esnext",
    minify: true,
    sourcemap: true,
    treeShaking: true,
  });
  await esbuild.stop();
}