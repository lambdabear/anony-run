use nix::{
    sys::memfd::{memfd_create, MemFdCreateFlag},
    unistd::fexecve,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use std::{ffi::CString, fs::File, io::Write, os::fd::FromRawFd};

pub fn run(bin: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let rand_string: String = thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    let raw_fd = memfd_create(
        CString::new(rand_string.as_str())?.as_c_str(),
        MemFdCreateFlag::MFD_CLOEXEC,
    )?;

    // println!("RawFD: {}", raw_fd);

    let mut file = unsafe { File::from_raw_fd(raw_fd) };

    file.write_all(bin)?;

    let args: Vec<CString> = std::env::args()
        .map(|arg| CString::new(arg).unwrap())
        .collect();

    let vars: Vec<CString> = std::env::vars()
        .map(|(mut var, value)| {
            var.extend(['='].iter());
            var.extend(value.chars());

            // println!("ENV: {}", &var);

            CString::new(var).unwrap()
        })
        .collect();

    if let Err(errno) = fexecve(raw_fd, args.as_slice(), &vars) {
        println!("fexecve error: {:?}", errno);
    }

    Ok(())
}
