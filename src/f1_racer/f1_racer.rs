use rand::Rng;

pub struct F1Racer {
    name: String,
    completed_laps: u8,
    laps: u8,
    best_lap_time: u8,
    lap_times: Vec<u8>,
}

impl F1Racer {
    pub fn new(name: String, laps: u8) -> Self {
        let mut rng = rand::thread_rng();
        let lap_times: Vec<u8> = vec![rng.gen(), rng.gen(), rng.gen(), rng.gen(), rng.gen()];
        println!("{}, lap times: {:?}", name, lap_times);
        F1Racer {
            name,
            completed_laps: 0,
            laps,
            best_lap_time: 255,
            lap_times,
        }
    }

    pub fn do_lap(&mut self) {
        println!(
            "{} is doing lap {} of {}",
            self.name, self.completed_laps, self.laps
        );
        let lap_time = self.lap_times.pop();

        if lap_time.is_some() && lap_time.unwrap() < self.best_lap_time {
            self.best_lap_time = lap_time.unwrap();
        }

        self.completed_laps += 1;
    }
}

impl std::future::Future for F1Racer {
    type Output = u8;
    fn poll(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        println!("Thread assign is id: {:?}", std::thread::current().id());
        if self.completed_laps < self.laps {
            self.get_mut().do_lap();
            cx.waker().wake_by_ref();
            return std::task::Poll::Pending;
        }
        println!("{} finished in {} seconds", self.name, self.best_lap_time);
        std::task::Poll::Ready(self.best_lap_time)
    }
}
