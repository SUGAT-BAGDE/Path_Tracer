use crate::accumulator::Accumulator;
use crate::sampler::Sampler;

type RenderJob = Box<dyn FnOnce(&mut Sampler) -> Accumulator + Send + 'static>;

pub(crate) mod threadpool;
pub(crate) mod worker;

pub(crate) use threadpool::Threadpool;
