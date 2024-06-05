use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::flag;
use signal_hook::iterator::SignalsInfo;
use signal_hook::iterator::exfiltrator::WithOrigin;

pub trait Service {
    fn init(&self) -> anyhow::Result<()> {
        Ok(())
    }

    /// start is called after init, This method must be non-blocking.
    fn start(&self) -> anyhow::Result<()> {
        Ok(())
    }
    fn stop(&self) -> anyhow::Result<()> {
        Ok(())
    }
}

pub fn run(svc: &dyn Service) -> anyhow::Result<()> {
    // term_now
    let term_now = Arc::new(AtomicBool::new(false));
    for sig in TERM_SIGNALS {
        flag::register_conditional_shutdown(*sig, 1, Arc::clone(&term_now))?;
        flag::register(*sig, Arc::clone(&term_now))?;
    }

    // consume signals
    let mut sigs = vec![];
    sigs.extend(TERM_SIGNALS);
    let mut signals = SignalsInfo::<WithOrigin>::new(&sigs)?;

    // init service
    svc.init()?;
    // start service
    svc.start()?;

    // waiting for signals
    for info in &mut signals {
        eprintln!("Received a signal {:?}", info);
        match info.signal {
            term_sig => {
                eprintln!("Terminating...");
                assert!(TERM_SIGNALS.contains(&term_sig));
                break;
            }
        }
    }

    // stop service
    svc.stop()
}