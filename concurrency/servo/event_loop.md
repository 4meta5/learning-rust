# Glorified For Loop

*Servo consists of multiple concurrent components, running potentially in various processes, and at the scale of the system as a whole, those components are seen communicating with each other via (ipc-)channels and running their internal logic as event-loops.*

A "tab" in your browser consists of a given "script event-loop" component running a web-page (or several) in a process. 
* a different process maintains the central hub -- "constellation" -- which owns senders to all the "script event loops" that are currently running (and shares a sender to itself with each of them)
* in another thread (but the same process), the "embedder" is running (the actual browser UI)

When the "script event-loop" wants to navigate one of the webpages it manages, the "script event-loop" will send a message to the "constellation" to ask it to navigate that "page". 

## Non Event-Loop
"non" indicates it has nothing to do with async I/O

```rust
pub fn run(&mut self) -> bool {
    let received = select! {
        recv(self.port, event) => {
            match event {
                Some(msg) => Some(msg),
                // Our sender has been dropped, quit.
                None => return false,
            }
        },
        recv(crossbeam_channel::::after(Duration::from_millis(100))) => None,
    };
    if let Some(msg) = received {
        self.handle_msg(msg);
        while let Some(another_msg) = self.port.try_recv() {
            // Handle any other incoming messages,
            // before performing a hang checkpoint.
            self.handle_msg(another_msg);
        }
    }
    self.perform_a_hang_monitor_checkpoint();
    true
}
```

That's a *non-event loop*. This is how you start it:

```rust
/// Start a new hang monitor worker, and return a handle to register components for monitoring.
pub fn init(constellation_chan: IpcSender<HangAlert>) -> Box<BackgroundMonitorRegister> {
    let (sender, port) = channel();
    let _ = thread::Builder::new().spawn(move || {
        let mut monitor = { BackgroundMonitorWorker::new(constellation_chan, port) };
        while monitor.run() {
            // Monitoring until all senders have been dropped...
        }
    });
    Box::new(HangMonitorRegister { sender })
}
```

The value proposition is the simplicity of the event loop's sequential processing in isolation. The logic follows the pattern: "for each message you receive, do 1, and then 2, and then maybe 3 if this or that, and son...".

The background hang monitor's state is shown below. The state of the monitor consists mainly of a bunch of `MonitoredComponent` and mutating those shouldn't require any locking.

```rust
struct MonitoredComponent {
    thread_id: MonitoredThreadId,
    last_activity: Instant,
    last_annotation: Option<HangAnnotation>,
    transient_hang_timeout: Duration,
    permanent_hang_timeout: Duration,
    sent_transient_alert: bool,
    sent_permanent_alert: bool,
    is_waiting: bool,
}

pub struct BackgroundMonitorWorker {
    monitored_components: HashMap<MonitoredComponentId, MonitoredComponent>,
    constellation_chan: IpcSender<HangAlert>,
    port: Receiver<(MonitoredComponentId, MonitoredComponentMsg)>,
}
```

Mutation can utilize a simple `&mut` because all operations occur in the same thread in a sequential fashion. The following "hang monitor checkpoint" is performed at each iteration of the non event-loop, after incoming messages have been handled:

```rust
fn perform_a_hang_monitor_checkpoint(&mut self) {
    for (component_id, mut monitored) in self.monitored_components.iter_mut() {
        if monitored.is_waiting {
            continue;
        }
        let last_annotation = monitored.last_annotation.unwrap();
        if monitored.last_activity.elapsed() > monitored.permanent_hang_timeout {
            if monitored.sent_permanent_alert {
                continue;
            }
            let profile = unsafe { suspend_and_sample_thread(monitored.thread_id) };
            let _ = self.constellation_chan.send(HangAlert::Permanent(
                component_id.clone(),
                last_annotation,
                profile,
            ));
            monitored.sent_permanent_alert = true;
            continue;
        }
        if monitored.last_activity.elapsed() > monitored.transient_hang_timeout {
            if monitored.sent_transient_alert {
                continue;
            }
            let _ = self
                .constellation_chan
                .send(HangAlert::Transient(component_id.clone(), last_annotation));
            monitored.sent_transient_alert = true;
        }
    }
}
```

How is the message passing handled?

```rust
fn handle_msg(&mut self, msg: (MonitoredComponentId, MonitoredComponentMsg)) {
    match msg {
        (
            component_id,
            MonitoredComponentMsg::Register(
                thread_id,
                transient_hang_timeout,
                permanent_hang_timeout,
            ),
        ) => {
            let component = MonitoredComponent {
                thread_id,
                last_activity: Instant::now(),
                last_annotation: None,
                transient_hang_timeout,
                permanent_hang_timeout,
                sent_transient_alert: false,
                sent_permanent_alert: false,
                is_waiting: true,
            };
            assert!(
                self.monitored_components
                    .insert(component_id, component)
                    .is_none(),
                "This component was already registered for monitoring."
            );
        },
        (component_id, MonitoredComponentMsg::NotifyActivity(annotation)) => {
            let mut component = self
                .monitored_components
                .get_mut(&component_id)
                .expect("Received NotifyActivity for an unknown component");
            component.last_activity = Instant::now();
            component.last_annotation = Some(annotation);
            component.sent_transient_alert = false;
            component.sent_permanent_alert = false;
            component.is_waiting = false;
        },
        (component_id, MonitoredComponentMsg::NotifyWait) => {
            let mut component = self
                .monitored_components
                .get_mut(&component_id)
                .expect("Received NotifyWait for an unknown component");
            component.last_activity = Instant::now();
            component.sent_transient_alert = false;
            component.sent_permanent_alert = false;
            component.is_waiting = true;
        },
    }
}
```

An earlier iteration of the background monitor shared a bunch of raw `Sender`s around the system for:
1. registering components for monitoring
2. notifying the monitor of the start of an activity by a registered component
3. notifying the monitor of the start of a registered component going into "waiting" mode

Instead, we use two traits now -- one dealing with registering components:
```rust
/// A handle to register components for hang monitoring,
/// and to receive a means to communicate with the underlying hang monitor worker.
pub trait BackgroundHangMonitorRegister: BackgroundHangMonitorClone + Send {
    /// Register a component for hang monitoring:
    /// to be called from within the thread to be monitored for hangs.
    fn register_component(
        &self,
        component: MonitoredComponentId,
        transient_hang_timeout: Duration,
        permanent_hang_timeout: Duration,
    ) -> Box<BackgroundHangMonitor>;
}
```

The second, returned by the "register" method of the first, manages sending notifications to the monitor from an component already registered for monitoring.

```rust
/// Proxy methods to communicate with the background hang monitor
pub trait BackgroundHangMonitor {
    /// Notify the start of handling an event.
    fn notify_activity(&self, annotation: HangAnnotation);
    /// Notify the start of waiting for a new event to come in.
    fn notify_wait(&self);
}
```

1. The whole background hang monitor, and the methods of communication with it, remain hidden from the rest of the system
2. we can out those traits in a minimal crate used by the rest of the system, whereas the background hang monitor can live in it's own crate that only a few other crates in the system rely on; this is nice because, if you make a change to the monitor crate, you do not need to recompile all the crates using the traits...

## Thread Sampling

