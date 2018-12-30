use either::Either;

pub struct Stats {
    samples: Vec<f32>,
}

impl Stats {
    pub fn new() -> Stats {
        Stats {
            samples: Vec::new(),
        }
    }

    pub fn add_sample(&mut self, sample: f32) {
        self.samples.push(sample);
    }

    pub fn stats(&self, last_opt: Option<usize>) -> (usize, f32, f32) {
        let mean_iter = if let Some(last) = last_opt {
            Either::Left(self.samples.iter().rev().take(last))
        }
        else {
            Either::Right(self.samples.iter())
        };

        let len = mean_iter.len();

        let std_dev_iter = mean_iter.clone();

        let mut numerator = 0.0;
        for sample in mean_iter {
            numerator += sample;
        }
        let mean = numerator / len as f32;

        let mut deviation_sqr = 0.0;
        for sample in std_dev_iter {
            deviation_sqr += (sample - mean) * (sample - mean);
        }
        let std_dev = (deviation_sqr / (len as f32 - 1.0)).sqrt();

        (len, mean, std_dev)
    }
}
