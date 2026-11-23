//! Deterministic behavior-driven test verifier for async channel deadlock challenge.

use tokio::sync::mpsc;
use tokio::time::{timeout, Duration};

#[tokio::test]
async fn test_concurrent_producers_no_deadlock() {
    let (tx, mut rx) = mpsc::channel::<u32>(100);

    let producer_handles: Vec<_> = (0..16)
        .map(|i| {
            let tx = tx.clone();
            tokio::spawn(async move {
                for j in 0..100 {
                    tx.send(i * 100 + j).await.expect("send failed");
                }
            })
        })
        .collect();

    let consumer = tokio::spawn(async move {
        let mut count = 0;
        while let Some(_val) = rx.recv().await {
            count += 1;
            if count == 1600 {
                break;
            }
        }
        count
    });

    for handle in producer_handles {
        handle.await.expect("producer panicked");
    }

    let result = timeout(Duration::from_secs(5), consumer).await;
    assert!(result.is_ok(), "Deadlock detected: consumer timed out");
    assert_eq!(result.unwrap().unwrap(), 1600);
}
