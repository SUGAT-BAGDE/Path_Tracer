use crate::accumulators::TileAccumulator;
use crate::sampler::Sampler;

type RenderJob = Box<dyn FnOnce(&mut Sampler) -> TileAccumulator + Send + 'static>;
type RenderJobResult = TileAccumulator;

pub(crate) mod threadpool;
pub(crate) mod worker;

pub(crate) use threadpool::Threadpool;
