//#![windows_subsystem = "windows"]

extern crate ws;
extern crate chrono;
extern crate dotenv;

use chrono::Local;
use dotenv::dotenv;
use local_ipaddress;

struct Router {
    sender: ws::Sender,
    inner: Box<dyn ws::Handler>,
}

impl ws::Handler for Router {
    fn on_shutdown(&mut self) {
        self.inner.on_shutdown()
    }

    fn on_open(&mut self, shake: ws::Handshake) -> ws::Result<()> {
        self.inner.on_open(shake)
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        self.inner.on_message(msg)
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        self.inner.on_close(code, reason)
    }

    fn on_error(&mut self, err: ws::Error) {
        self.inner.on_error(err)
    }

    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let out = self.sender.clone();

        match req.resource() {
            "/" => {
                self.inner = Box::new(PLCReceiver)
            }
            "/sender" => {
                self.inner = Box::new(PLCSender { ws: out })
            }
            _ => {
                self.inner = Box::new(NotFound)
            }
        }

        self.inner.on_request(req)
    }
}

struct NotFound;

impl ws::Handler for NotFound {
    fn on_request(&mut self, req: &ws::Request) -> ws::Result<ws::Response> {
        let mut res = ws::Response::from_request(req)?;
        res.set_status(404);
        res.set_reason("Not Found");
        Ok(res)
    }
}


struct PLCSender {
    ws: ws::Sender,
}

impl ws::Handler for PLCSender {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("[{} System] Bluetooth sender connected", Local::now().format("%Y-%m-%d %H:%M:%S"));
        Ok(())
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        println!("[{} Bluetooth sender] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), msg);
        self.ws.broadcast(msg)
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        println!("[{} System] Bluetooth sender closed", Local::now().format("%Y-%m-%d %H:%M:%S"));
    }
}

struct PLCReceiver;

impl ws::Handler for PLCReceiver {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        println!("[{} System] PLCReceiver connected", Local::now().format("%Y-%m-%d %H:%M:%S"));
        Ok(())
    }

    fn on_message(&mut self, _: ws::Message) -> ws::Result<()> {
        Ok(())
    }

    fn on_close(&mut self, _: ws::CloseCode, _: &str) {
        println!("[{} System] PLCReceiver closed", Local::now().format("%Y-%m-%d %H:%M:%S"));
    }
}

fn main() {
    dotenv().ok();

    let ip = dotenv::var("LAN_IP")
        .unwrap_or(local_ipaddress::get().unwrap());

    let port = dotenv::var("LAN_PORT")
        .unwrap_or("3012".to_string());

    println!("[{} System] Bridge server started\r\n\
    PLCReceiver please connect to: ws://{}:{}\r\n\
    Bluetooth sender please connect to: ws://{}:{}/sender",
             Local::now().format("%Y-%m-%d %H:%M:%S"),
             ip, port,
             ip, port
    );

    ws::listen(format!("{}:{}", ip, port), |out| {
        Router {
            sender: out,
            inner: Box::new(PLCReceiver),
        }
    }).unwrap();
}

