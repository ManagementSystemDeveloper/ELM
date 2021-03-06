use rocket::fairing::{Fairing, Info, Kind};
use rocket::{http::Header, http::Method, http::Status, Request, Response};

pub struct CorsFairing;

impl Fairing for CorsFairing {
    fn on_response(&self, request: &Request, response: &mut Response) {
        // Add CORS headers to allow all origins to all outgoing requests
        response.set_header(rocket::http::Header::new(
            "Access-Control-Allow-Origin",
            "*",
        ));
        response.adjoin_header(Header::new("Access-Control-Allow-Headers", "idToken"));
        response.adjoin_header(Header::new("Access-Control-Allow-Headers", "content-type"));
        response.adjoin_header(Header::new("Access-Control-Allow-Headers", "file-name"));

        // Respond to all `OPTIONS` requests with a `204` (no content) status
        if response.status() == Status::NotFound && request.method() == Method::Options {
            response.set_status(Status::NoContent);
        }
    }

    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }
}
