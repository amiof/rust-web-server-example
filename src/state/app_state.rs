use std::sync::Arc;


pub struct AppState {
    pub message: String,
}

pub fn init_state()->Arc<AppState>{
   Arc::new(AppState{
       message: "Hello, world!".to_string(),
   }) 
}
