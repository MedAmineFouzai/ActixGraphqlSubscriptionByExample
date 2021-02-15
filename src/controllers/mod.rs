mod simple_broker;
use simple_broker::SimpleBroker;
mod schema;
use schema::{UserObject};
use async_graphql::{Context, Enum, Object, Result, Schema, Upload,Subscription, ID};
use futures::lock::Mutex;
use futures::{Stream, StreamExt};
use slab::Slab;
use std::sync::Arc;
use std::fs::File;
#[derive(Debug)]
pub struct MyToken(pub String);


pub type MessageSchema = Schema<QueryRoot, MutationRoot, SubscriptionRoot>;

pub type Storage = Arc<Mutex<Slab<UserObject>>>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
	async fn messages(&self, ctx: &Context<'_>) -> Vec<UserObject> {
		let messages = ctx.data_unchecked::<Storage>().lock().await;
		messages.iter().map(|(_, msg)| msg).cloned().collect()
	}


}

pub struct MutationRoot;

#[Object]
impl MutationRoot {
	async fn send_msg(&self, ctx: &Context<'_>,username:String ,msg:Option<String>,file: Option<Upload>) -> ID {
		let mut store = ctx.data_unchecked::<Storage>().lock().await;
		let entry = store.vacant_entry();
		let id: ID = entry.key().into();
		//note file saving in not async need to use async file write
		let file =match file {
			Some(file)=>{
				let mut upload=file.value(ctx).unwrap();
				let  saved_file = Some(File::create(format!("static/uploads/{}",upload.filename)).unwrap());
				let mut  _saved= Some(std::io::copy(&mut upload.content, &mut saved_file.unwrap()));
				format!("/media/static/uploads/{}",upload.filename)
			}, 
			None=>"".to_string()
		};
		let msg=match  msg {
			Some(msg)=>msg,
			None=>"".to_string()
		};
	
		let user =UserObject{
			id:id.clone(),
			username:username,
			text:Some(msg),
			src:Some(file)
		};
		entry.insert(user);
		SimpleBroker::publish(StreamChanged {
			mutation_type: MutationType::Created,
			id: id.clone(),
		});
		id
	}

	
}

#[derive(Enum, Eq, PartialEq, Copy, Clone)]
enum MutationType {
	Created
}

#[derive(Clone)]
struct StreamChanged {
	mutation_type: MutationType,
	id: ID,
}

#[Object]
impl StreamChanged {
	async fn mutation_type(&self) -> MutationType {
		self.mutation_type
	}

	async fn id(&self) -> &ID {
		&self.id
	}

	async fn user(&self, ctx: &Context<'_>) -> Result<Option<UserObject>> {
		let message = ctx.data_unchecked::<Storage>().lock().await;
		let id = self.id.parse::<usize>()?;
		Ok(message.get(id).cloned())
	}
}

pub struct SubscriptionRoot;

#[Subscription]
impl SubscriptionRoot {
	

	async fn subscribe(&self, mutation_type: Option<MutationType>) -> impl Stream<Item = StreamChanged> {
		SimpleBroker::<StreamChanged>::subscribe().filter(move |event| {
			let res = if let Some(mutation_type) = mutation_type {
				event.mutation_type == mutation_type
			} else {
				true
			};
			async move { res }
		})
	}
}

