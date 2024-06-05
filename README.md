# rs-svc
Rust service wrapper that run on Linux

## Examples
See `examples`
```rust
use rs_svc::svc::service::Service;

struct MyService;

impl Service for MyService {
    fn init(&self) -> anyhow::Result<()> {
        println!("init");
        Ok(())
    }

    // must be non-blocking
    fn start(&self) -> anyhow::Result<()> {
        std::thread::spawn(move || {
            println!("start")
        });
        Ok(())
    }

    fn stop(&self) -> anyhow::Result<()> {
        print!("stop");
        Ok(())
    }
}


fn main() {
    let my_svc = MyService;
    rs_svc::svc::service::run(&my_svc).unwrap()
}
```