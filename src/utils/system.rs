use windows::Win32::Storage::FileSystem::GetDiskFreeSpaceExW;
use windows::core::HSTRING;

pub fn get_disk_free_space(path: &str) -> u64 {
    let mut free_bytes_available: u64 = 0;
    let mut total_number_of_bytes: u64 = 0;
    let mut total_number_of_free_bytes: u64 = 0;

    let path_hstring = HSTRING::from(path);
    unsafe {
        if GetDiskFreeSpaceExW(
            &path_hstring,
            Some(&mut free_bytes_available),
            Some(&mut total_number_of_bytes),
            Some(&mut total_number_of_free_bytes),
        ).is_ok() {
            free_bytes_available
        } else {
            0
        }
    }
}

pub fn get_c_drive_free_space() -> u64 {
    get_disk_free_space("C:\\")
}
