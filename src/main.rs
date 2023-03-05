extern crate fsuipc;

use std::io;
use std::process;

use fsuipc::user::*;
use fsuipc::*;

fn main() {
    match run() {
        Ok(_) => process::exit(0),
        Err(e) => {
            println!("IO Error: {:?}", e);
            process::exit(-1);
        }
    }
}

fn run() -> io::Result<()> {
    let mut handle = UserHandle::new()?;
    let mut session = handle.session();

    // FDR data as required by CARS - https://tc.canada.ca/en/corporate-services/acts-regulations/list-regulations/canadian-aviation-regulations-sor-96-433/standards/standard-625-aircraft-equipment-maintenance-standard/standard-625-schedule-2-aeroplane-flight-data-recorder-fdr-specifications-canadian-aviation

    // Time
    let mut hour = 0u8;
    let mut minute = 0u8;
    let mut second = 0u8;
    session.read(0x023b, &mut hour)?;
    session.read(0x023c, &mut minute)?;
    session.read(0x023a, &mut second)?;

    // Altitude
    let mut altitude = 0u8; // Altitude in meters
    session.read(0x0570, &mut altitude)?;
    let altitude_feet: f32 = (altitude as f32) / 3.28;

    // IAS
    let mut ias = 0u8;
    session.read(0x02bc, &mut ias)?;

    // Magnetic heading
    let mut heading = 0u32;
    session.read(0x0580, &mut heading)?;
    heading = heading * (360 / (65536 * 65536)); // Convert to degrees

    // Acceleration
    // Stab trim position
    // N1 / EPR / Torque & RPM
    // Flap position
    // Lat
    // Long

    session.process()?;

    Ok(())
}
