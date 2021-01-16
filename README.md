# Pastebin Clone



A pastebin clone made using [Rocket](https://rocket.rs) Framework in [Rust](https://rust-lang.org). Pastebin is a place where you can dump your code and access it later using a unique URL.  



# Features

1. Send you bins using post request at `/post` . For eg, if you are hosting on localhost, you can send the request as

   ```bash
   curl -d "some data" http://localhost:8000
   # From file
   curl --data-binary @filename http://localhost:8000
   ```

   Refer to this article https://ec.haxx.se/http/http-post on various ways to post data using curl. 

   or, on Powershell as -

   ```powershell
    echo "from powershell" | Invoke-RestMethod -Uri "http://localhost:8000" -Method Post
    # Alternatively, you can post a file directly as well
    Invoke-RestMethod -Uri "https://paste.rs" -Method Post -InFile .\file.txt
   ```

   

   **The server would respond with the location of your bin.**  

2. To get a bin back, send a request on `/<binID>`. For eg, if the server is currently serving on localhost port 8000, and the bin id is XeP then the URL would be `http://localhost:8000/XeP`.

   You can also use curl to get the data. 

   ```bash
   curl http://localhost:8000/XeP
   ```

   



# Build

To simply build it you can use  `cargo build` for the development version. For the optimized and release version, use `--release` flag.

Or, you can build and run using `cargo run` command. 



# Features to Implement

1. Adding a form where users can manually input new pastes. 
2. Support deletion of pastes.



Made by following the guide at: https://rocket.rs/v0.4/guide/



