use std::fs;

pub fn generate_index_html(current_dir: &str, name: &str) {
    let wasm_file = format!("{}_bg.wasm", name);
    let js_file = format!("{}.js", name);

    let index_content = format!(
        r#"<!DOCTYPE html>
<html lang="en-US">

<head>
  <!-- Title -->
  <title>{} App</title>

  <!-- Disable Zooming -->
  <meta name="viewport" content="width=device-width, initial-scale=1.0, user-scalable=no">

  <!-- Mobile -->
  <meta name=HandheldFriendly content="True" />
  <meta name=mobile-web-app-capable content="yes" />



  <!-- Imports -->

  <script type="module">
    // Use ES module import syntax to import functionality from the module
    // that we have compiled.
    //
    // Note that the `default` import is an initialization function which
    // will "boot" the module and make it ready to use. Currently browsers
    // don't support natively imported WebAssembly as an ES module, but
    // eventually the manual initialization won't be required!
    import init from './{}';

    async function run() {{
      // First detect if we have WebGPU available. Our client only works with WebGPU.
      if (!navigator.gpu) {{
        console.error("WebGPU NOT supported. The game client will not work.");
        renderUnsupportedContent();
        return;
      }} else {{
        console.info("WebGPU supported.");
      }}


      //SUPPORTED SECTION --splash and spinner, client
      // ... (rest of the supported content code)

      // Now we need to actually load the wasm file, so we use the
      // default export to inform it where the wasm file is located on the
      // server, and then we wait on the returned promise to wait for the
      // wasm to be loaded.
      //
      // It may look like this: await init('./pkg/without_a_bundler_bg.wasm');
      // but there is also a handy default inside `init` function, which uses
      // `import.meta` to locate the wasm file relatively to js file.
      //
      // Note that instead of a string you can also pass in any of the
      // following things:
      //
      // * `WebAssembly.Module`
      //
      // * `ArrayBuffer`
      //
      // * `Response`
      //
      // * `Promise` which returns any of the above, e.g. `fetch("./path/to/wasm")`
      //
      // This gives you complete control over how the module is loaded
      // and compiled.
      //
      // Also note that the promise, when resolved, yields the wasm module's
      // exports which is the same as importing the `*_bg` module in other
      // modes
      await init("./{}");
    }}

    run();
  </script>
</head>

<body>
</body>

</html>"#,
        name, js_file, wasm_file
    );

    let index_path = format!("{}/www/index.html", current_dir);

    // Write the content to index.html
    if let Err(e) = fs::write(&index_path, index_content) {
        eprintln!("Error generating index.html: {:?}", e);
    }
}
