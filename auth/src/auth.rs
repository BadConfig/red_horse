use diesel::PgConnection;
use std::sync::Arc;
use crate::db::{
    AuthSecret,
    Auth,
};

#[derive(Clone)]
pub struct EngineAuth {
    secret: Arc<AuthSecret>,
}

pub struct Roles {
   data: EngineAuth,
   role: String,
}

impl EngineAuth {
    pub fn new(secret: Arc<AuthSecret>) -> Self {
        EngineAuth {
            secret,
        }
    }
    pub fn role(&self, role: &str) -> Roles {
        Roles {
            data: (*self).clone(),
            role: role.to_owned(),
        }
    }   
}

impl actix_web::guard::Guard for Roles {
    fn check(&self, request: &actix_http::RequestHead) -> bool {
        use std::ops::Index;
        let role = request.uri.path();
        let role: Vec<&str> = role
            .split('/')
            .collect();
        let role = role.index(2);
        println!("{}",role);
        let token = match request.headers.get("jwt") {
            Some(t) => t,
            None => return false,
        };
        println!("{:?}",token);
        let user = match token.to_str() {
            Ok(token) => match Auth::validate_jwt(token,self.data.secret.as_ref()) {
                Ok(auth) => auth,
                Err(_) => return false,
            },
            Err(_) => return false,
        };
        println!("{:?}",user);
        let d = user.roles.contains(&role.to_string());
        println!("{:?}",d);
        d
    }
}
