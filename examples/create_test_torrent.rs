// Helper utility to create a test .torrent file
use std::collections::HashMap;
use torrent_client::bencode::{BencodeValue, encode};
use std::fs;
use sha1::{Sha1, Digest};

fn main() {
    // Create a small test file
    let test_data = b"Hello, BitTorrent! This is a test file for our torrent client.";
    fs::write("test_file.txt", test_data).expect("Failed to write test file");
    
    // Calculate piece hash
    let mut hasher = Sha1::new();
    hasher.update(test_data);
    let piece_hash: [u8; 20] = hasher.finalize().into();
    
    // Build info dictionary
    let mut info_dict = HashMap::new();
    info_dict.insert("name".to_string(), BencodeValue::String(b"test_file.txt".to_vec()));
    info_dict.insert("piece length".to_string(), BencodeValue::Integer(16384));
    info_dict.insert("length".to_string(), BencodeValue::Integer(test_data.len() as i64));
    info_dict.insert("pieces".to_string(), BencodeValue::String(piece_hash.to_vec()));
    
    // Build root dictionary
    let mut root = HashMap::new();
    root.insert("announce".to_string(), BencodeValue::String(b"http://tracker.example.com:8000/announce".to_vec()));
    root.insert("info".to_string(), BencodeValue::Dict(info_dict));
    
    // Encode to bencode
    let torrent_data = encode(&BencodeValue::Dict(root));
    
    // Write .torrent file
    fs::write("test.torrent", torrent_data).expect("Failed to write torrent file");
    
    println!("Created test.torrent and test_file.txt");
}
