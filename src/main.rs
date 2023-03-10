extern crate winreg;
use std::fs;
use std::io;
use std::path::Path;
use winreg::enums::*;
use winreg::RegKey;

fn main() -> io::Result<()> {
    println!("Reading some system info...");
    let hklm = RegKey::predef(HKEY_LOCAL_MACHINE);
    let cur_ver = hklm.open_subkey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion")?;
    let pf: String = cur_ver.get_value("ProgramFilesDir")?;
    let dp: String = cur_ver.get_value("DevicePath")?;
    println!("ProgramFiles = {}\nDevicePath = {}", pf, dp);
    let info = cur_ver.query_info()?;
    println!("info = {:?}", info);
    let mt = info.get_last_write_time_system();
    println!(
        "last_write_time as winapi::um::minwinbase::SYSTEMTIME = {}-{:02}-{:02} {:02}:{:02}:{:02}",
        mt.wYear, mt.wMonth, mt.wDay, mt.wHour, mt.wMinute, mt.wSecond
    );

    let lastorder = fs::read_to_string("lastread.txt")?;
    println!("lastorder = {}", lastorder);

    // enable `chrono` feature on `winreg` to make this work
    // println!(
    //     "last_write_time as chrono::NaiveDateTime = {}",
    //     info.get_last_write_time_chrono()
    // );

    println!("And now lets write something...");
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let path = Path::new("Software").join("WinregRsExample1");
    let (key, disp) = hkcu.create_subkey(&path)?;

    match disp {
        REG_CREATED_NEW_KEY => println!("A new key has been created"),
        REG_OPENED_EXISTING_KEY => println!("An existing key has been opened"),
    }

    // key.set_value("TestSZ", &"written by Rust")?;
    let sz_val: String = key.get_value("KEYGEN_ACCOUNT_ID")?;
    // key.delete_value("TestSZ")?;
    println!("TestSZ = {}", sz_val);

    fs::write("lastread.txt", sz_val.to_string())?;

    // key.set_value("TestDWORD", &1234567890u32)?;
    // let dword_val: u32 = key.get_value("TestDWORD")?;
    // println!("TestDWORD = {}", dword_val);

    // key.set_value("TestQWORD", &1234567891011121314u64)?;
    // let qword_val: u64 = key.get_value("TestQWORD")?;
    // println!("TestQWORD = {}", qword_val);

    // key.create_subkey("sub\\key")?;
    // hkcu.delete_subkey_all(&path)?;

    // println!("Trying to open nonexistent key...");
    // hkcu.open_subkey(&path).unwrap_or_else(|e| match e.kind() {
    //     io::ErrorKind::NotFound => panic!("Key doesn't exist"),
    //     io::ErrorKind::PermissionDenied => panic!("Access denied"),
    //     _ => panic!("{:?}", e),
    // });
    Ok(())
}