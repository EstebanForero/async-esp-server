use esp_println::println;
use picoserve::{io::Read, request::Path, response::ResponseWriter};

struct TimedResponseWriter<'r, W> {
    path: Path<'r>,
    response_writer: W,
}

impl<'r, W: ResponseWriter> ResponseWriter for TimedResponseWriter<'r, W> {
    type Error = W::Error;

    async fn write_response<
        R: Read<Error = Self::Error>,
        H: picoserve::response::HeadersIter,
        B: picoserve::response::Body,
    >(
        self,
        connection: picoserve::response::Connection<'_, R>,
        response: picoserve::response::Response<H, B>,
    ) -> Result<picoserve::ResponseSent, Self::Error> {
        let status_code = response.status_code();

        let result = self
            .response_writer
            .write_response(connection, response)
            .await;

        println!("Path: {}; Status Code: {}", self.path, status_code,);

        result
    }
}

struct CorsResponseWriter<W> {
    response_writer: W,
}

impl<W: ResponseWriter> ResponseWriter for CorsResponseWriter<W> {
    type Error = W::Error;

    async fn write_response<
        R: Read<Error = Self::Error>,
        H: picoserve::response::HeadersIter,
        B: picoserve::response::Body,
    >(
        self,
        connection: picoserve::response::Connection<'_, R>,
        response: picoserve::response::Response<H, B>,
    ) -> Result<picoserve::ResponseSent, Self::Error> {
        let new_response = response
            .with_header("Access-Control-Allow-Origin", "*")
            .with_header(
                "Access-Control-Allow-Methods",
                "GET, POST, PUT, DELETE, OPTIONS",
            )
            .with_header("Access-Control-Allow-Headers", "*");

        self.response_writer
            .write_response(connection, new_response)
            .await
    }
}

pub struct CorsLayer;

impl<State, PathParameters> picoserve::routing::Layer<State, PathParameters> for CorsLayer {
    type NextState = State;
    type NextPathParameters = PathParameters;

    async fn call_layer<
        'a,
        R: Read + 'a,
        NextLayer: picoserve::routing::Next<'a, R, Self::NextState, Self::NextPathParameters>,
        W: ResponseWriter<Error = R::Error>,
    >(
        &self,
        next: NextLayer,
        state: &State,
        path_parameters: PathParameters,
        _request_parts: picoserve::request::RequestParts<'_>,
        response_writer: W,
    ) -> Result<picoserve::ResponseSent, W::Error> {
        next.run(
            state,
            path_parameters,
            CorsResponseWriter { response_writer },
        )
        .await
    }
}
