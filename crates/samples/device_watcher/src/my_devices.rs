use std::collections::HashMap;
use std::sync::{atomic::AtomicBool, atomic::Ordering, Arc, RwLock};
use windows::{core::*, Devices::Enumeration::*, Foundation::*};

pub struct MyDeviceInfo {
    pub name: HSTRING, // Friendlier name for a device
    pub id: HSTRING,   // Device Id which can be used to open the device
}

// Atomics and locks keep MyDevices Sync + Send, as required
// by the TypedEventHandler's trait bounds.
pub struct MyDevices {
    ready: AtomicBool,
    changed: AtomicBool,
    data: RwLock<HashMap<String, MyDeviceInfo>>,
}

impl MyDevices {
    /// MyDevices is useable in DeviceWatcher events only via Arc, so we
    /// provide an Arc-based constructor
    pub fn new_arc() -> Arc<Self> {
        Arc::new(MyDevices {
            ready: AtomicBool::new(false),
            changed: AtomicBool::new(false),
            data: RwLock::new(HashMap::new()),
        })
    }

    // Wire up the watcher's event handlers.
    pub fn hook_up_events(watcher: &DeviceWatcher, handler: &Arc<MyDevices>) -> Result<()> {
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

        let client = handler.clone();
        watcher.Updated(
            &TypedEventHandler::<DeviceWatcher, DeviceInformationUpdate>::new(move |_, update| {
                if let Some(info) = update.as_ref() {
                    client.on_updated(info.Id()?);
                }
                Ok(())
            }),
        )?;

        let client = handler.clone();
        watcher.EnumerationCompleted(&TypedEventHandler::new(move |_, _| {
            client.on_enumeration_complete();
            Ok(())
        }))?;

        Ok(())
    }

    //
    // App-specific event handling
    //

    fn on_enumeration_complete(&self) {
        // for demo purposes, print message when initial enumeration is finished
        println!("enumeration complete");
        self.ready.store(true, Ordering::SeqCst);
        _ = self.set_changed(false);
    }

    fn on_added(&self, name: HSTRING, id: HSTRING) {
        if self.ready.load(Ordering::SeqCst) {
            // for demo purposes, print the name of devices added after the initial enumeration
            println!("Added {}", name);
        }

        let key = id.to_string_lossy();
        let mut hash = self.data.write().unwrap();
        hash.insert(key, MyDeviceInfo { name, id });
        _ = self.set_changed(true);
    }

    fn on_updated(&self, id: HSTRING) {
        if self.ready.load(Ordering::SeqCst) {
            println!("Updated {}", id)
        }
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

    //
    // public methods
    //

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

    /// Make a current list of devices
    pub fn devices(&self) -> Vec<MyDeviceInfo> {
        let hash = self.data.read().unwrap();
        hash.values()
            .map(|value| MyDeviceInfo {
                name: value.name.clone(),
                id: value.id.clone(),
            })
            .collect()
    }

}
