//#![windows_subsystem = "windows"]

extern crate web_view;
use web_view::*;

fn main() {
    let webview = web_view::builder()
        .title("Evernote")
        .content(Content::Url("https://www.evernote.com/client/web"))
        .size(1180, 768)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .build()
        .unwrap();

    let handle = webview.handle();
    handle.dispatch(move |webview| {
        webview.eval(JS)
    }).unwrap();

    webview.run().unwrap();

}

const JS: &str = r#"
Object.defineProperty(navigator, 'userAgent', {
    get() {
        return "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/74.0.3709.0 Safari/537.36"
    }
})
"#;