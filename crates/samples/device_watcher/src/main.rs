use windows::{core::*, Devices::Enumeration::*};

// This sample tracks devices as they are added and removed,
// maintaining a list of current devices.
//
// The hardest part of making use of a DeviceWatcher is the
// plumbing for event handlers, which this sample provides.
// See the MyDevices::hook_up_events function, and make note
// of the thread-safe members in the MyDevices struct.

mod my_devices;
use my_devices::*;

fn main() -> Result<()> {
    // If you don't need to monitor all devices on the system, the API
    // documentation recommends to use DeviceInformation::CreateWatcherAqsFilter
    // to reduce the scope of devices to be watched.
    //
    // To monitor only MIDI input devices, for example:
    //
    // ```rust
    // let watcher = DeviceInformation::CreateWatcherAqsFilter(&MidiInPort::GetDeviceSelector()?)?;
    // ```
    let watcher = DeviceInformation::CreateWatcher()?; // watch all devices
    let handler = MyDevices::new_arc();
    MyDevices::hook_up_events(&watcher, &handler)?;
    watcher.Start()?;

    //
    // wait for initial enumeration to finish
    //
    let interval = std::time::Duration::new(1, 0);
    while !handler.is_ready()
        && match watcher.Status()? {
            DeviceWatcherStatus::EnumerationCompleted => false,
            DeviceWatcherStatus::Aborted => false,
            DeviceWatcherStatus::Stopped => false,
            _ => true,
        }
    {
        std::thread::sleep(interval);
    }

    // print the list of devices
    println!("Device name: Id");
    println!("------------ ------------");
    for device in handler.devices() {
        print_device(&device);
    }

    println!("");
    println!("As devices are added and removed, they will be shown below.");
    println!("Plug or unplug any USB device to see it in action.");
    println!("");
    println!("Waiting for changes...");
    println!("-- Press ENTER to exit --");
    // pause for input
    _ = std::io::stdin().read_line(&mut String::new());

    if handler.is_changed() {
        println!("Current Devices:");
        for device in handler.devices() {
            print_device(&device);
        }
    } else {
        println!("No changes");
    }
    Ok(())
}

fn print_device(info: &MyDeviceInfo) {
    println!("{}: {}", info.name, info.id);
}
