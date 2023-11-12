pub mod program {
    use crate::f1_racer;
    use crate::functions::sleep_for::sleep_for_secs;

    pub async fn f1racer_run() {
        let racer01 = f1_racer::f1_racer::F1Racer::new("Racer01".to_string(), 5);
        let racer02 = f1_racer::f1_racer::F1Racer::new("Racer02".to_string(), 5);
        let racer03 = f1_racer::f1_racer::F1Racer::new("Racer03".to_string(), 5);

        let handle01 = tokio::spawn(racer01);
        let handle02 = tokio::spawn(racer02);
        let handle03 = tokio::spawn(racer03);
        let handle04 = tokio::spawn(sleep_for_secs(1));

        println!("Racer 01: {:?}", handle01.await);
        println!("Racer 02: {:?}", handle02.await);
        println!("Racer 03: {:?}", handle03.await);

        handle04.await;
    }
}
