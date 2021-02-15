use async_graphql::{ Object,ID};
use serde::{Deserialize,Serialize};


// #[derive(Debug,Clone,Deserialize,Serialize)]
// pub struct FileObject {
//     pub src:String,
// }
// #[derive(Debug,Clone,Deserialize,Serialize)]
// pub struct MessageObject{
//     pub msg:String,
// }

#[derive(Debug,Clone,Deserialize,Serialize)]
pub struct UserObject {
    pub id:ID,
    pub username:String,
    pub text:Option<String>,
    pub src:Option<String>,

}

#[Object]
impl UserObject {
    async fn id(&self) -> &str {
        &self.id
    }

    async fn username(&self) -> &str {
        &self.username
    }

    async fn text(&self) -> &str {
        &self.text.as_ref().unwrap()
    }
    async fn src(&self) -> &str {
        &self.src.as_ref().unwrap()
    }

}

// #[Object]
// impl MessageObject {
//     async fn id(&self) -> &str {
//         &self.id
//     }
//     async fn msg(&self) -> &str {
//         &self.msg
//     }

//     async fn file(&self) -> &FileObject {
//         &self.file
//     }

// }

// #[Object]
// impl FileObject {
//     async fn id(&self) -> &str {
//         &self.id
//     }
//     async fn filename(&self) ->&str {
//         &self.filename
//     }

//     async fn src(&self) -> &str {
//         &self.src
//     }

// }






