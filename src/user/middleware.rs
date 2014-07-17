use std::any::AnyRefExt;
use std::fmt::Show;

use conduit_middleware;
use conduit::Request;
use conduit_cookie::RequestSession;

use app::RequestApp;
use super::User;
use util::errors::{CargoResult, Unauthorized, CargoError};

pub struct Middleware;

impl conduit_middleware::Middleware for Middleware {
    fn before(&self, req: &mut Request) -> Result<(), Box<Show>> {
        let id = match req.session().find_equiv(&"user_id")
                          .and_then(|s| from_str(s.as_slice())) {
            Some(id) => id,
            None => return Ok(()),
        };
        let user = match User::find(&req.app().db(), id) {
            Ok(user) => user,
            Err(..) => return Ok(()),
        };

        req.mut_extensions().insert("crates.io.user", box user);
        Ok(())
    }
}

pub trait RequestUser<'a> {
    fn user(self) -> CargoResult<&'a User>;
}

impl<'a> RequestUser<'a> for &'a Request {
    fn user(self) -> CargoResult<&'a User> {
        match self.extensions().find_equiv(&"crates.io.user").and_then(|r| {
            r.as_ref::<User>()
        }) {
            Some(user) => Ok(user),
            None => Err(Unauthorized.box_error()),
        }
    }
}