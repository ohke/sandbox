use crossbeam::channel;
use futures::task;
use futures::task::ArcWake;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::time::{Duration, Instant};

struct Delay {
    when: Instant,
    waker: Option<Arc<Mutex<Waker>>>,
}

impl Future for Delay {
    type Output = &'static str;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if let Some(waker) = &self.waker {
            // タイマースレッド実行済みなら、保存されているwakerと現在のタスクのwakerが一致するか確認
            let mut waker = waker.lock().unwrap();

            if !waker.will_wake(cx.waker()) {
                // 一致しない場合、wakerを更新する
                *waker = cx.waker().clone();
            }
        } else {
            // 初回なら、タイマースレッドをspawnする
            let waker = Arc::new(Mutex::new(cx.waker().clone()));
            self.waker = Some(waker.clone());

            let when = self.when;
            thread::spawn(move || {
                // 一定時間経過までsleep (ビジーループを避けるため)
                let now = Instant::now();
                if now < when {
                    thread::sleep(when - now);
                }

                // 続きの処理を行うために、ランタイムへタスクを実行予約する
                let waker = waker.lock().unwrap();
                waker.wake_by_ref();
            });
        }

        if Instant::now() >= self.when {
            println!("Hello world");
            // 処理が完了して、Output型の値を返す
            Poll::Ready("done")
        } else {
            // 処理が未完了
            Poll::Pending
        }
    }
}

struct Task {
    future: Mutex<Pin<Box<dyn Future<Output = ()> + Send>>>,
    executor: channel::Sender<Arc<Task>>,
}

impl Task {
    fn poll(self: Arc<Self>) {
        let waker = task::waker(self.clone());
        let mut cx = Context::from_waker(&waker);

        let mut future = self.future.try_lock().unwrap();

        let _ = future.as_mut().poll(&mut cx);
    }

    // 与えられた future で新しいタスクをspawn
    fn spawn<F>(future: F, sender: &channel::Sender<Arc<Task>>)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Arc::new(Task {
            future: Mutex::new(Box::pin(future)),
            executor: sender.clone(), // 送信チャネルを渡す
        });

        let _ = sender.send(task);
    }
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        // 送信チャネルからプッシュ
        arc_self.executor.send(arc_self.clone());
    }
}

struct MiniTokio {
    // crossbeam::channel は Send + Sync
    scheduled: channel::Receiver<Arc<Task>>,
    sender: channel::Sender<Arc<Task>>,
}

impl MiniTokio {
    fn new() -> Self {
        let (sender, scheduled) = channel::unbounded();
        MiniTokio { scheduled, sender }
    }

    fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Task::spawn(future, &self.sender);
    }

    fn run(&mut self) {
        // チャネルからスケジュールされた (= wakeされた) タスクを受信したらpoll
        while let Ok(task) = self.scheduled.recv() {
            task.poll();
        }
    }
}

fn main() {
    let mut mini_tokio = MiniTokio::new();

    mini_tokio.spawn(async {
        let when = Instant::now() + Duration::from_millis(1000);
        let future = Delay { when, waker: None };

        let out = future.await;
        assert_eq!(out, "done");

        std::process::exit(0);
    });

    mini_tokio.run();
}
