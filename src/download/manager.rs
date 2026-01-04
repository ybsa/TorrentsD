use anyhow::{Result, Context};
use tokio::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::net::SocketAddr;
use crate::torrent::Metainfo;
use crate::peer::{PeerConnection, Message};
use crate::storage::Storage;
use super::piece::Piece;

pub struct DownloadManager {
    metainfo: Arc<Metainfo>,
    pieces: Arc<Mutex<Vec<Piece>>>,
    storage: Arc<Mutex<Storage>>,
    peer_id: [u8; 20],
}

impl DownloadManager {
    pub fn new(metainfo: Metainfo, output_dir: &str, peer_id: [u8; 20]) -> Result<Self> {
        let num_pieces = metainfo.num_pieces();
        let piece_length = metainfo.info.piece_length;
        let total_length = metainfo.info.total_length;
        
        let mut pieces = Vec::with_capacity(num_pieces);
        for i in 0..num_pieces {
            let length = if i == num_pieces - 1 {
                // Last piece might be smaller
                total_length - (i * piece_length)
            } else {
                piece_length
            };
            
            let hash = *metainfo.piece_hash(i).unwrap();
            pieces.push(Piece::new(i, length, hash));
        }
        
        let storage = Storage::new(&metainfo, output_dir)?;
        
        Ok(DownloadManager {
            metainfo: Arc::new(metainfo),
            pieces: Arc::new(Mutex::new(pieces)),
            storage: Arc::new(Mutex::new(storage)),
            peer_id,
        })
    }
    
    pub async fn download_from_peers(&self, peers: Vec<SocketAddr>) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(100);
        
        // Spawn tasks for each peer
        for peer_addr in peers.iter().take(5) { // Limit to 5 simultaneous peers
            let tx = tx.clone();
            let metainfo = Arc::clone(&self.metainfo);
            let pieces = Arc::clone(&self.pieces);
            let peer_id = self.peer_id;
            let peer_addr = *peer_addr;
            
            tokio::spawn(async move {
                let _ = download_from_peer(peer_addr, metainfo, pieces, peer_id, tx).await;
            });
        }
        
        drop(tx); // Drop the original sender
        
        // Collect completed pieces
        while let Some((piece_index, data)) = rx.recv().await {
            let mut storage = self.storage.lock().unwrap();
            storage.write_piece(piece_index, &data)
                .context("Failed to write piece")?;
            
            println!("✓ Piece {}/{} complete", piece_index + 1, self.metainfo.num_pieces());
        }
        
        Ok(())
    }
    
    pub fn is_complete(&self) -> bool {
        let pieces = self.pieces.lock().unwrap();
        pieces.iter().all(|p| p.is_complete())
    }
    
    pub fn progress(&self) -> (usize, usize) {
        let pieces = self.pieces.lock().unwrap();
        let complete = pieces.iter().filter(|p| p.is_complete()).count();
        (complete, pieces.len())
    }
}

async fn download_from_peer(
    peer_addr: SocketAddr,
    metainfo: Arc<Metainfo>,
    pieces: Arc<Mutex<Vec<Piece>>>,
    peer_id: [u8; 20],
    tx: mpsc::Sender<(usize, Vec<u8>)>,
) -> Result<()> {
    let mut conn = PeerConnection::connect(peer_addr, &metainfo.info_hash, &peer_id).await?;
    
    // Wait for bitfield or first message
    let _ = conn.receive_message().await?;
    
    // Send interested
    conn.send_interested().await?;
    
    // Wait for unchoke
    loop {
        if let Some(msg) = conn.receive_message().await? {
            if matches!(msg, Message::Unchoke) {
                break;
            }
        } else {
            return Ok(()); // Connection closed
        }
    }
    
    // Download pieces
    loop {
        // Find a piece to download
        let piece_to_download = {
            let mut pieces_guard = pieces.lock().unwrap();
            pieces_guard.iter_mut()
                .find(|p| !p.is_complete() && conn.has_piece(p.index))
                .map(|p| (p.index, p.length, p.hash))
        };
        
        if let Some((piece_index, piece_length, piece_hash)) = piece_to_download {
            // Download this piece
            let mut piece = Piece::new(piece_index, piece_length, piece_hash);
            
            while let Some((begin, length)) = piece.next_block_to_request() {
                conn.request_piece(piece_index as u32, begin as u32, length as u32).await?;
                
                // Wait for the piece message
                loop {
                    if let Some(msg) = conn.receive_message().await? {
                        if let Message::Piece { index, begin: msg_begin, data } = msg {
                            if index as usize == piece_index && msg_begin as usize == begin {
                                piece.add_block(begin, data);
                                break;
                            }
                        }
                    } else {
                        return Ok(()); // Connection closed
                    }
                }
            }
            
            // Verify and save the piece
            if piece.verify() {
                if let Some(data) = piece.data() {
                    // Update the shared pieces
                    {
                        let mut pieces_guard = pieces.lock().unwrap();
                        pieces_guard[piece_index] = piece.clone();
                    }
                    
                    // Send to storage
                    let _ = tx.send((piece_index, data)).await;
                }
            }
        } else {
            // No more pieces to download from this peer
            break;
        }
    }
    
    Ok(())
}
