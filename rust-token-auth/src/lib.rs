use proxy_wasm::traits::*;
use proxy_wasm::types::*;

proxy_wasm::main! {{
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(MyHttpContext) });
}}

struct MyHttpContext;

impl Context for MyHttpContext {}

impl HttpContext for MyHttpContext {
    fn on_http_request_headers(&mut self, _: usize, _: bool) -> Action {
        // Always be a friendly proxy.
        self.add_http_request_header("Message", "hello");
        self.set_http_request_header("Welcome", Some("warm"));
        return Action::Continue;
    }

    fn on_http_response_headers(&mut self, _: usize, _: bool) -> Action {
        // Add a custom header
        self.add_http_response_header("x-srv-ext", "added");
        // Conditionally add to a header value.
        let msg = self.get_http_response_header("Message");
        if msg.unwrap_or_default() == "foo" {
            self.add_http_response_header("Message", "bar");
        }
        // Unconditionally remove a header.
        self.set_http_response_header("Welcome", None);
        return Action::Continue;
    }
}
