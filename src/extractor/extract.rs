use std::{
    fs::File,
    io::{Read, Seek, SeekFrom},
    path::PathBuf,
};

use crate::extractor::{ExtractorError, GameInfo};

const PVD_SECTOR_SIZE: u64 = 2048;
const PVD_SECTOR: u64 = 16;
const ROOT_DIRECTORY_SECTOR_AFTER_PVD: usize = 156;
const ROOT_DIRECTORY_SIZE: usize = 34;

pub fn extract(path: &str) -> Result<GameInfo, ExtractorError> {
    let serial = extract_serial_number_from(String::from(path))?;
    Ok(GameInfo::new(
        PathBuf::from(path),
        String::new(),
        Some(serial),
    ))
}

struct Extent {
    lba: u32,
    size: u32,
}

fn extract_serial_number_from(iso_path: String) -> Result<String, ExtractorError> {
    let mut file = File::open(iso_path)?;

    let root_dir = read_root_directory(&mut file)?;
    let system_cnf = find_file_in_directory(&mut file, &root_dir, "SYSTEM.CNF")
        .ok_or(ExtractorError::MissingSystemCnf)?;

    let contents = read_extent(&mut file, &system_cnf)?;
    Ok(String::from_utf8_lossy(&contents).to_string())
}

fn read_root_directory(file: &mut File) -> Result<Extent, ExtractorError> {
    let pvd = read_sector(file, PVD_SECTOR, PVD_SECTOR_SIZE)?;
    let root_directory_record = &pvd
        [ROOT_DIRECTORY_SECTOR_AFTER_PVD..ROOT_DIRECTORY_SECTOR_AFTER_PVD + ROOT_DIRECTORY_SIZE];

    Ok(extent_from_record(root_directory_record))
}

fn find_file_in_directory(
    file: &mut File,
    root_dir_extent: &Extent,
    target_name: &str,
) -> Option<Extent> {
    let data = read_extent(file, root_dir_extent).ok()?;

    let mut offset = 0usize;
    while offset < data.len() {
        let record_len = data[offset] as usize;

        if record_len == 0 {
            offset += 1;
            continue;
        }

        let record = &data[offset..offset + record_len];
        let name = record_filename(record);

        if name.eq_ignore_ascii_case(target_name) {
            return Some(extent_from_record(record));
        }

        offset += record_len;
    }

    None
}

fn extent_from_record(record: &[u8]) -> Extent {
    let lba = u32::from_le_bytes([record[2], record[3], record[4], record[5]]);
    let size = u32::from_le_bytes([record[10], record[11], record[12], record[13]]);

    Extent { lba, size }
}

fn read_extent(file: &mut File, extent: &Extent) -> Result<Vec<u8>, ExtractorError> {
    file.seek(SeekFrom::Start((extent.lba as u64) * PVD_SECTOR_SIZE))?;

    let mut buffer = vec![0u8; extent.size as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn read_sector(file: &mut File, sector: u64, sector_size: u64) -> Result<Vec<u8>, ExtractorError> {
    file.seek(SeekFrom::Start(sector * sector_size))?;

    let mut buffer = vec![0u8; sector_size as usize];
    file.read_exact(&mut buffer)?;

    Ok(buffer)
}

fn record_filename(record: &[u8]) -> String {
    let name_len = record[32] as usize;
    let name_bytes = &record[33..33 + name_len];

    String::from_utf8_lossy(name_bytes)
        .split(";")
        .next()
        .unwrap()
        .to_string()
}
