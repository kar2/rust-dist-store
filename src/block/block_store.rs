use std::net::{TcpListener, TcpStream};
use std::str;
use pickledb::{PickleDb, PickleDbDumpPolicy, SerializationMethod};
use sha2::{Sha256, Digest};

const LISTENER_PORT: String = "127.0.0.1:7878";
const BLOCK_SIZE: u16 = 4096;

pub fn startTcpListener() {
    let listener = TcpListener::bind(LISTENER_PORT).unwrap();

    // Start PickleDB
    let mut db = PickleDb::new("blockstore.db", PickleDbDumpPolicy::AutoDump, SerializationMethod::Json);

    for stream in listener.incoming() {
        // Later: queue streams in thread?
        handle_client_request(stream)?;
    }
}

fn handle_client_request(stream: TcpStream) -> Result<String, io::Error> {
    // Parse TCP Stream

    // 
}

fn put_block(buf: &[u8], db: &PickleDb) -> Result<i32, io::Error> {
    // Byte string from buf
    // TODO: Check if utf8 vs utf8_lossy makes a difference
    let mut byte_string = String::from_utf8_lossy(buf);
    let mut hasher = Sha256::new(byte_string);
    hasher.update(byte_string);
    let hash = hasher.finalize();

    // Put into db
    db.set(hash, byte_string);
    return hash;
}

fn get_block(hash: i32) -> Result<[u8], io::Error> {

}

fn get_hashlist(hashlist: [i32]) -> Result<[i32], io::Error> {

}