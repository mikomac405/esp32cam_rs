use esp32_nimble::utilities::mutex::Mutex;
use esp32_nimble::utilities::BleUuid;
use esp32_nimble::{BLEAdvertising, BLEServer, NimbleProperties};
use lazy_static::lazy_static;
use log::{info, warn};

pub static UUID_BLE_SERVICE_STR: &str = "io.test.ble"; // up-to 16 bytes
pub static UUID_BLE_CTRL_CHAR_STR: &str = "control"; // up-to 16 bytes

lazy_static! {
    pub static ref UUID_BLE_SERVICE: BleUuid = str_to_uuid(UUID_BLE_SERVICE_STR);
    pub static ref UUID_BLE_CTRL_CHAR: BleUuid = str_to_uuid(UUID_BLE_CTRL_CHAR_STR);
}

pub fn str_to_uuid(s: &str) -> BleUuid {
    let mut arr = [0u8; 16];
    for (idx, char) in s.as_bytes().iter().enumerate() {
        if idx < 16 {
            arr[idx] = *char
        } else {
            warn!("uuid string is longer than 16 bytes!");
            break;
        }
    }
    BleUuid::from_uuid128(arr)
}

pub fn ble_advertise_config(
    name: &str,
    server: &mut BLEServer,
    advertising: &Mutex<BLEAdvertising>,
) {
    server.on_connect(|server, desc| {
        server
            .update_conn_params(desc.conn_handle(), 24, 48, 0, 60)
            .expect("server.update_conn_params");
    });
    server.on_disconnect(|_desc, reason| {
        info!("Client disconnected ({:?})", reason);
    });
    let service = server.create_service(*UUID_BLE_SERVICE);

    let writable_characteristic = service.lock().create_characteristic(
        *UUID_BLE_CTRL_CHAR,
        NimbleProperties::READ | NimbleProperties::WRITE,
    );
    writable_characteristic
        .lock()
        .on_read(move |_, _| {
            ::log::info!("Read from writable characteristic.");
        })
        .on_write(|args| {
            ::log::info!(
                "Wrote to writable characteristic: {:?} -> {:?}",
                args.current_data(),
                args.recv_data()
            );
        });

    advertising
        .lock()
        .set_data(
            esp32_nimble::BLEAdvertisementData::new()
                .name(name)
                .add_service_uuid(*UUID_BLE_SERVICE),
        )
        .unwrap();
}

// pub async fn do_ble_scan(
//     ble_scan: &mut BLEScan,
// ) -> Result<Vec<esp32_nimble::BLEAdvertisedDevice>, anyhow::Error> {
//     let (tx, rx) = tokio::sync::oneshot::channel::<()>();
//     let mut tx = Some(tx);

//     ble_scan
//         .active_scan(true)
//         .filter_duplicates(true)
//         .limited(false)
//         .interval(100)
//         .window(99)
//         .on_completed(move || {
//             let tx = tx.take();
//             if let Some(tx) = tx {
//                 let _ = tx.send(());
//             }
//         });
//     ble_scan
//         .start(10000)
//         .await
//         .map_err(|e| anyhow!("ble_scan.start: {:?}", e))?;

//     select! {
//         _ = sleep(Duration::from_secs(15)) => {
//             bail!("ble scan timed out!");
//         }
//         _ = rx => {
//             info!("Scan finished");
//         }
//     };
//     let result = ble_scan.get_results().cloned().collect::<Vec<_>>();

//     Ok(result)
// }
