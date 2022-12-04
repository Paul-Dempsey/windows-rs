use std::collections::HashMap;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, RwLock};
use windows::{core::*, Devices::Enumeration::*, Foundation::*};

// This sample tracks devices as they are added and removed, 
// maintaining a list of current devices. 

struct MyDeviceInfo {
    name: HSTRING, // Friendlier name for a device
    id: HSTRING, // Device Id which can be used to open the device
}

// Atomics and locks required to keep MyDevices Sync + Send
struct MyDevices {
    ready: AtomicBool,
    changed: AtomicBool,
    data: RwLock<HashMap<String, MyDeviceInfo>>,
}

impl MyDevices {
    /// Only useable in DeviceWatcher events via Arc, so we provide an Arc-based constructor
    pub fn new_arc() -> Arc<Self> {
        Arc::new(MyDevices {
            ready: AtomicBool::new(false),
            changed: AtomicBool::new(false),
            data: RwLock::new(HashMap::new()),
        })
    }

    /// Returns true when initial enumeration is complete
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }

    /// Returns true when a device has been added or removed after
    /// the initial enumeration is complete (or the changed flag
    /// is explicitly set).
    pub fn is_changed(&self) -> bool {
        self.changed.load(Ordering::SeqCst)
    }

    /// Set changed flag, returning previous state
    pub fn set_changed(&self, flag: bool) -> bool {
        self.changed.swap(flag, Ordering::SeqCst)
    }

    /// Project current list of devices
    pub fn devices(&self) -> Vec<MyDeviceInfo> {
        let hash = self.data.read().unwrap();
        hash.values()
            .map(|value| MyDeviceInfo {
                name: value.name.clone(),
                id: value.id.clone(),
            })
            .collect()
    }

    fn on_enumeration_complete(&self) {
        // for demo purposes, print message when initial enumeration is finished
        println!("enumeration complete");
        self.ready.store(true, Ordering::SeqCst);
    }

    fn on_added(&self, name: HSTRING, id: HSTRING) {
        if self.ready.load(Ordering::SeqCst) {
            // for demo purposes, print the name of devices added after the initial enumeration
            println!("Added {name}");
        }

        let key = id.to_string_lossy();
        let mut hash = self.data.write().unwrap();
        hash.insert(key, MyDeviceInfo { name, id });
        _ = self.set_changed(true);
    }

    fn on_removed(&self, id: HSTRING) {
        let key = id.to_string_lossy();
        // for demo purposes, print name of devices removed after the initial enumeration
        if self.ready.load(Ordering::SeqCst) {
            let readonly_hash = self.data.read().unwrap();
            if let Some(d) = readonly_hash.get(&key) {
                println!("Removed {}", d.name);
            } else {
                println!("Removed {}", key);
            }
        }
        let mut hash = self.data.write().unwrap();
        _ = hash.remove(&key); // discard removed device info
        _ = self.set_changed(true);
    }
}

fn main() -> Result<()> {

    // For this demo, we watch all devices and maintain a list
    // of devices as they come and go.
    // If you don't need to monitor all devices on the system, it's
    // recommended to use DeviceInformation::CreateWatcherAqsFilter
    // to reduce the scope.
    //
    // To monitor only MIDI input devices, for example:
    //
    // ```rust
    // use windows::Devices::Midi::MidiInPort;
    // let watcher = DeviceInformation::CreateWatcherAqsFilter(&MidiInPort::GetDeviceSelector()?)?;
    // ```
    let watcher = DeviceInformation::CreateWatcher()?;
    let handler = MyDevices::new_arc();

    // Wire up the watcher's event handlers.
    // Each event requires its own MyDevices reference moved into the closure

    let client = handler.clone();
    watcher.Added(&TypedEventHandler::<DeviceWatcher, DeviceInformation>::new(
        move |_, info| {
            if let Some(info) = info.as_ref() {
                client.on_added(info.Name()?, info.Id()?);
            }
            Ok(())
        },
    ))?;

    let client = handler.clone();
    watcher.Stopped(&TypedEventHandler::<DeviceWatcher, IInspectable>::new(
        move |_, item| {
            if let Some(item) = item.as_ref() {
                client.on_enumeration_complete();
                // for demo purposes, print the event
                println!("Stopped {}", item.GetRuntimeClassName()?);
            }
            Ok(())
        },
    ))?;

    let client = handler.clone();
    watcher.Removed(
        &TypedEventHandler::<DeviceWatcher, DeviceInformationUpdate>::new(move |_, update| {
            if let Some(info) = update.as_ref() {
                client.on_removed(info.Id()?);
            }
            Ok(())
        }),
    )?;

    // Since MyDevices don't need to capture any data for this event,
    // we don't clone the handler or capture anything in the closure.
    // This handler could have been omitted, but presented here to show
    // the syntax for an 'updated' event handler.
    watcher.Updated(
        &TypedEventHandler::<DeviceWatcher, DeviceInformationUpdate>::new(|_, update| {
            if let Some(info) = update.as_ref() {
                println!("Updated {}", info.Id()?);
            }
            Ok(())
        }),
    )?;

    let client = handler.clone();
    watcher.EnumerationCompleted(&TypedEventHandler::new(move |_, _| {
        client.on_enumeration_complete();
        Ok(())
    }))?;

    watcher.Start()?;

    // wait until end of initial enumeration
    let interval = std::time::Duration::new(1, 0);
    while !handler.is_ready() && match watcher.Status()? {
        DeviceWatcherStatus::EnumerationCompleted => false,
        DeviceWatcherStatus::Aborted => false,
        DeviceWatcherStatus::Stopped => false,
        _ => true,
    } {
        std::thread::sleep(interval);
    }

    // print the list of devices
    println!("Devices:");
    for device in handler.devices() {
        println!("{} | {}", device.name, device.id);
    }

    // pause for input
    // While waiting for input, the event handlers will print
    // the devices that are added or removed.
    // One way to observe this is to plug in or unplug any USB device
    // before pressing ENTER.
    println!("Press ENTER to exit.");
    let mut buf = String::new();
    _ = std::io::stdin().read_line(&mut buf);

    if handler.is_changed() {
        println!("Changed Devices:");
        for device in handler.devices() {
            println!("{} | {}", device.name, device.id);
        }
    }
    Ok(())
}
