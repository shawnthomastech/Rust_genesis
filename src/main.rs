use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    // let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    // for stream in listener.incoming() {
    //     let stream = stream.unwrap();

    //     handle_connection(stream);
    //     thisisafunction(10.0);
    // }
    list_files_and_save("./test","./output.txt");
    if let Err(e) = insert_content_into_html("output.txt", "index.html") {
        eprintln!("Error: {}", e);
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&mut stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();

//     if request_line == "GET / HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = fs::read_to_string("hello.html").unwrap();
//         let length = contents.len();

//         let response = format!(
//             "{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}"
//         );

//         stream.write_all(response.as_bytes()).unwrap();
//     } else {
//         //blah
//     }
// }

// TODAY I ADDED A FILE TO THE CODEBASE TO ACTUALLY TRY TO GET A VISUAL ON THE DOCUMENT


fn list_files_and_save(path: &str, output_file: &str) -> Result<(), std::io::Error> {
    let paths = fs::read_dir(path)?;

    let mut output = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(output_file)?;

    for path in paths {
        let path = path?.path();

        if path.extension().and_then(std::ffi::OsStr::to_str) == Some("txt") {
            let contents = fs::read_to_string(&path)?;
            writeln!(output, "File: {}\nContents:\n{}\n [PLACEHOLDER]", path.display(), contents)?;
        }
    }

    Ok(())
}

fn insert_content_into_html(text_file: &str, html_file: &str) -> Result<(), std::io::Error> {
    let text_content = fs::read_to_string(text_file)?;

    let mut html_content = fs::read_to_string(html_file)?;
    html_content = html_content.replace("[PLACEHOLDER]", &text_content);

    fs::write(html_file, html_content)?;
    Ok(())
}


//once again sealing things with an update
//just to seal things with an update 

// Today I need to make sure that the stylesheet I am using is actually working on the front-end of the app


