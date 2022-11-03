use ndarray::prelude as na;
use ndarray_rand::rand_distr::{Normal, Uniform};
use ndarray_rand::RandomExt;

pub trait Planner: Sized {
    type WalkStepParams;
    fn fit_walkparams(&mut self) -> Self::WalkStepParams;
    fn set_walkparams(&mut self, params: Self::WalkStepParams);
    fn get_walkparams(&self, params: Self::WalkStepParams);

    fn into_sampler<S: Sampler>(self) -> S;

    fn prepare<S: Sampler>(mut self) -> S {
        let params = self.fit_walkparams();
        self.set_walkparams(params);
        self.into_sampler::<S>()
    }
}

pub trait Sampler: Iterator {}

struct LatticeSampler<D: na::Dimension> {
    sigma: f64,
    walker: na::Array<f64, na::Dim<D>>,
    pdf: fn(f64) -> f64,
    ncorr: usize,
}

impl<D> Iterator for LatticeSampler<D>
where
    na::Dim<D>: na::Dimension,
    D: na::Dimension
{
    type Item = na::Array<f64, na::Dim<D>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut new = na::Array::<f64, _>::zeros(self.walker.raw_dim());
        let mut prob = na::Array::zeros(self.walker.raw_dim());
        for _ in 0..self.ncorr {
            let steps = na::Array::random(self.walker.raw_dim(), Normal::new(0., self.sigma).unwrap());
            let new = &self.walker + steps;
            
            na::Zip::from(&mut prob)
                .and(&self.walker)
                .and(&new)
                .for_each(|m, o, n| *m = self.pdf(n) / (self.pdf(o) + f64::EPSILON));
            
            //let alpha = Array::random(self.walker.shape(), Uniform::new(0., 1.));
            //let mask = prob > alpha;
            //self.walker[mask] = new[mask];
        };
        Some(new)
    }
}

//impl<D> Planner for LatticeWalker<D> {
//	type WalkStepParams = (f64, usize);
//	fn fit_walkparams(&mut self) -> Self::WalkStepParams {
//		let mut counter = 0;
//		while counter < 100 {
//			let steps = Array::random(self.walker.shape(), Normal::new(0., self.sigma));
//			let new =  self.walker + steps;
//			let prob = new.mapv(self.pdf) / (self.walker.mapv(self.pdf) + f64::EPSILON);
//			let alpha = Array::random(self.walker.shape(), Uniform::new(0., 1.));
//			let mask = prob > alpha;
//			self.walker[mask] = new[mask];
//			let passed_ratio = f64::abs(mask.sum() - (self.walker.size as f64 * 0.6));
//			if passed_ratio > 0.1 {
//
//			} else {
//				counter += 1;
//			}
//		}
//
//		(self.eps, 100);
//	}
//}
