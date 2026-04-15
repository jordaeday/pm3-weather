use rsntp::SntpClient;

pub fn sync_time_from_ntp() -> Result<(), Box<dyn std::error::Error>> {
  let client = SntpClient::new();
  let result = client.synchronize("pool.ntp.org:123")?;

  let unix_time = result.datetime().unix_timestamp()?;

  let ts = libc::timespec {
      tv_sec: unix_time.as_secs() as libc::time_t,
      tv_nsec: 0,
  };

  unsafe {
    if libc::clock_settime(libc::CLOCK_REALTIME, &ts) != 0 {
        return Err(Box::new(std::io::Error::last_os_error()));
    }
  };

  Ok(())
}