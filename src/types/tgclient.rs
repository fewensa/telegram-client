//use std::sync::Arc;
//
//use rtdlib::types::*;
//
//use crate::api::Api;
//
//pub type ListenerOn<T> = Box<T>;
//pub type ListenerSave<T> = Option<Arc<T>>;
//
//
//pub type LFnUpdate = Fn((&Api, &Box<Update>)) + Send + Sync + 'static;
//pub type LFnOptionBool = Fn((&Api, &OptionValueBoolean)) + Send + Sync + 'static;
//pub type LFnOptionEmpty = Fn((&Api, &OptionValueEmpty)) + Send + Sync + 'static;
//pub type LFnOptionInteger = Fn((&Api, &OptionValueInteger)) + Send + Sync + 'static;
//pub type LFnOptionString = Fn((&Api, &OptionValueString)) + Send + Sync + 'static;
//
