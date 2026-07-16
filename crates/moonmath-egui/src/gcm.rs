//! Interactive visualization of AES-128 in Galois/Counter Mode (GCM).
//!
//! GCM is an *authenticated encryption with associated data* (AEAD) mode. It
//! runs two machines in lockstep over the same key:
//!
//!   * **Confidentiality** — AES in counter (CTR) mode turns a per-message
//!     counter into a keystream that is XORed with the plaintext.
//!   * **Authentication** — GHASH, a universal hash built from multiplication
//!     in the binary field `GF(2^128)`, folds the associated data and the
//!     ciphertext into a single 128-bit accumulator. The authentication tag is
//!     that accumulator masked by one more AES block, `E_K(J0)`.
//!
//! Everything below is a *correct, dependency-free* implementation — real
//! AES-128 (encrypt) and real `GF(2^128)` arithmetic per NIST SP 800-38D — so
//! the hex the visualization shows is the hex a production library would emit.
//! The unit tests pin it to the canonical McGrew–Viega test vectors.
//!
//! The egui side renders the two machines side by side and animates GHASH
//! absorbing one block at a time: `X ← (X ⊕ B_i) · H`. When the block being
//! absorbed is a ciphertext block, the matching CTR column lights up, so the
//! link between "encrypt" and "authenticate" is visible.

use eframe::egui;
use egui::{Color32, RichText, Sense, Vec2};

// ─────────────────────────────────────────────────────────────────────────
// AES-128 (encrypt only — CTR and GHASH never need decryption)
// ─────────────────────────────────────────────────────────────────────────

#[rustfmt::skip]
const SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16,
];

const RCON: [u8; 10] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80, 0x1b, 0x36];

/// Expand a 128-bit key into 11 round keys (each 16 bytes, column-major to
/// match the state layout `state[4*col + row]`).
fn key_expansion(key: &[u8; 16]) -> [[u8; 16]; 11] {
    let mut w = [[0u8; 4]; 44];
    for i in 0..4 {
        w[i] = [key[4 * i], key[4 * i + 1], key[4 * i + 2], key[4 * i + 3]];
    }
    for i in 4..44 {
        let mut t = w[i - 1];
        if i % 4 == 0 {
            // RotWord ∘ SubWord, then XOR the round constant into the top byte.
            t = [t[1], t[2], t[3], t[0]];
            for b in t.iter_mut() {
                *b = SBOX[*b as usize];
            }
            t[0] ^= RCON[i / 4 - 1];
        }
        let p = w[i - 4];
        w[i] = [p[0] ^ t[0], p[1] ^ t[1], p[2] ^ t[2], p[3] ^ t[3]];
    }
    let mut rk = [[0u8; 16]; 11];
    for r in 0..11 {
        for c in 0..4 {
            rk[r][4 * c..4 * c + 4].copy_from_slice(&w[4 * r + c]);
        }
    }
    rk
}

#[inline]
fn xtime(x: u8) -> u8 {
    (x << 1) ^ (0x1b & (((x >> 7) & 1).wrapping_neg()))
}

/// Encrypt one 16-byte block with the given round-key schedule.
fn aes_encrypt_block(rk: &[[u8; 16]; 11], input: &[u8; 16]) -> [u8; 16] {
    let mut s = *input;
    for (i, b) in s.iter_mut().enumerate() {
        *b ^= rk[0][i];
    }
    for round in 1..=10 {
        // SubBytes
        for b in s.iter_mut() {
            *b = SBOX[*b as usize];
        }
        // ShiftRows (rows are s[row], s[row+4], s[row+8], s[row+12])
        let t = s;
        s[1] = t[5];
        s[5] = t[9];
        s[9] = t[13];
        s[13] = t[1];
        s[2] = t[10];
        s[6] = t[14];
        s[10] = t[2];
        s[14] = t[6];
        s[3] = t[15];
        s[7] = t[3];
        s[11] = t[7];
        s[15] = t[11];
        // MixColumns (skipped on the final round)
        if round != 10 {
            for c in 0..4 {
                let o = 4 * c;
                let a0 = s[o];
                let a1 = s[o + 1];
                let a2 = s[o + 2];
                let a3 = s[o + 3];
                s[o] = xtime(a0) ^ (xtime(a1) ^ a1) ^ a2 ^ a3;
                s[o + 1] = a0 ^ xtime(a1) ^ (xtime(a2) ^ a2) ^ a3;
                s[o + 2] = a0 ^ a1 ^ xtime(a2) ^ (xtime(a3) ^ a3);
                s[o + 3] = (xtime(a0) ^ a0) ^ a1 ^ a2 ^ xtime(a3);
            }
        }
        // AddRoundKey
        for (i, b) in s.iter_mut().enumerate() {
            *b ^= rk[round][i];
        }
    }
    s
}

// ─────────────────────────────────────────────────────────────────────────
// GF(2^128) arithmetic and the GCM top level
// ─────────────────────────────────────────────────────────────────────────

/// A 128-bit block is a `u128`, big-endian: byte 0 is the most significant.
#[inline]
fn ek(rk: &[[u8; 16]; 11], x: u128) -> u128 {
    u128::from_be_bytes(aes_encrypt_block(rk, &x.to_be_bytes()))
}

/// Multiplication in `GF(2^128)` with the GCM bit ordering (SP 800-38D §6.3):
/// bit 0 is the leftmost (most significant) bit, reduction polynomial
/// `1 + x + x^2 + x^7 + x^128`, represented by `R = 0xE1 << 120`.
fn gf_mul(x: u128, y: u128) -> u128 {
    const R: u128 = 0xe1 << 120;
    let mut z: u128 = 0;
    let mut v: u128 = y;
    let mut x = x;
    for _ in 0..128 {
        if x & (1u128 << 127) != 0 {
            z ^= v;
        }
        let lsb_set = v & 1 != 0;
        v >>= 1;
        if lsb_set {
            v ^= R;
        }
        x <<= 1;
    }
    z
}

/// Increment the rightmost 32 bits of a counter block, mod 2^32.
#[inline]
fn inc32(x: u128) -> u128 {
    let lo = (x as u32).wrapping_add(1);
    (x & !0xffff_ffffu128) | (lo as u128)
}

/// Pack up to 16 bytes into a big-endian block, right-padded with zeros.
fn block_from(bytes: &[u8]) -> u128 {
    let mut buf = [0u8; 16];
    let n = bytes.len().min(16);
    buf[..n].copy_from_slice(&bytes[..n]);
    u128::from_be_bytes(buf)
}

/// Which lane a GHASH block came from — drives colour + labels.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum BlockKind {
    Aad,
    Cipher,
    Len,
}

/// One CTR-mode column: counter → keystream → XOR plaintext → ciphertext.
#[derive(Clone)]
pub struct CtrStep {
    pub counter: u128,
    pub keystream: u128,
    pub plain: u128,
    pub cipher: u128,
    /// How many bytes of this block are real (the last block may be partial).
    pub valid: usize,
}

/// One GHASH absorption: `x_out = (x_in ⊕ block) · H`.
#[derive(Clone)]
pub struct GhashStep {
    pub label: String,
    pub kind: BlockKind,
    pub block: u128,
    pub x_in: u128,
    pub xored: u128,
    pub x_out: u128,
    /// Index into `ctr` when this block is a ciphertext block.
    pub ctr_index: Option<usize>,
}

/// Everything derived from the current inputs — recomputed only when an input
/// changes, then read every frame by the renderer.
#[derive(Clone)]
pub struct Computed {
    pub h: u128,
    pub j0: u128,
    pub ek_j0: u128,
    pub ctr: Vec<CtrStep>,
    pub trace: Vec<GhashStep>,
    pub s: u128,
    pub tag: u128,
    pub aad_bits: u64,
    pub ct_bits: u64,
}

/// Run AES-128-GCM end to end and record every intermediate for the animation.
/// `iv` must be the 96-bit nonce (the common and recommended IV length).
fn compute_gcm(key: &[u8; 16], iv: &[u8; 12], aad: &[u8], plaintext: &[u8]) -> Computed {
    let rk = key_expansion(key);
    let h = ek(&rk, 0);

    // 96-bit IV ⇒ J0 = IV ‖ 0^31 ‖ 1.
    let mut j0_bytes = [0u8; 16];
    j0_bytes[..12].copy_from_slice(iv);
    j0_bytes[15] = 1;
    let j0 = u128::from_be_bytes(j0_bytes);
    let ek_j0 = ek(&rk, j0);

    // ── CTR mode: keystream starts at inc32(J0). ──
    let mut ctr = Vec::new();
    let mut counter = inc32(j0);
    let mut ciphertext = Vec::with_capacity(plaintext.len());
    for chunk in plaintext.chunks(16) {
        let keystream = ek(&rk, counter);
        let ks_bytes = keystream.to_be_bytes();
        let mut c_bytes = [0u8; 16];
        for i in 0..chunk.len() {
            c_bytes[i] = chunk[i] ^ ks_bytes[i];
            ciphertext.push(c_bytes[i]);
        }
        ctr.push(CtrStep {
            counter,
            keystream,
            plain: block_from(chunk),
            cipher: block_from(&c_bytes[..chunk.len()]),
            valid: chunk.len(),
        });
        counter = inc32(counter);
    }

    // ── GHASH over AAD ‖ ciphertext ‖ len-block. ──
    let mut trace = Vec::new();
    let mut x: u128 = 0;
    let aad_blocks = aad.len().div_ceil(16).max(0);
    for (i, chunk) in aad.chunks(16).enumerate() {
        let block = block_from(chunk);
        let xored = x ^ block;
        let x_out = gf_mul(xored, h);
        trace.push(GhashStep {
            label: if aad_blocks > 1 {
                format!("AAD {}", i + 1)
            } else {
                "AAD".to_string()
            },
            kind: BlockKind::Aad,
            block,
            x_in: x,
            xored,
            x_out,
            ctr_index: None,
        });
        x = x_out;
    }
    let ct_blocks = ciphertext.len().div_ceil(16);
    for (i, chunk) in ciphertext.chunks(16).enumerate() {
        let block = block_from(chunk);
        let xored = x ^ block;
        let x_out = gf_mul(xored, h);
        trace.push(GhashStep {
            label: if ct_blocks > 1 {
                format!("C {}", i + 1)
            } else {
                "C".to_string()
            },
            kind: BlockKind::Cipher,
            block,
            x_in: x,
            xored,
            x_out,
            ctr_index: Some(i),
        });
        x = x_out;
    }
    let aad_bits = (aad.len() as u64) * 8;
    let ct_bits = (ciphertext.len() as u64) * 8;
    let len_block = ((aad_bits as u128) << 64) | (ct_bits as u128);
    let xored = x ^ len_block;
    let s = gf_mul(xored, h);
    trace.push(GhashStep {
        label: "len(A)‖len(C)".to_string(),
        kind: BlockKind::Len,
        block: len_block,
        x_in: x,
        xored,
        x_out: s,
        ctr_index: None,
    });

    let tag = s ^ ek_j0;

    Computed {
        h,
        j0,
        ek_j0,
        ctr,
        trace,
        s,
        tag,
        aad_bits,
        ct_bits,
    }
}

// ─────────────────────────────────────────────────────────────────────────
// Hex / display helpers
// ─────────────────────────────────────────────────────────────────────────

fn hex128(x: u128) -> String {
    let b = x.to_be_bytes();
    let mut s = String::with_capacity(32);
    for byte in b {
        s.push_str(&format!("{byte:02x}"));
    }
    s
}

/// Grouped, wrappable hex for a full 128-bit value: `0011 2233 …`.
fn hex128_grouped(x: u128) -> String {
    let b = x.to_be_bytes();
    let mut s = String::with_capacity(40);
    for (i, byte) in b.iter().enumerate() {
        if i != 0 && i % 4 == 0 {
            s.push(' ');
        }
        s.push_str(&format!("{byte:02x}"));
    }
    s
}

/// Short form for narrow boxes: first 4 and last 3 bytes.
fn hex_short(x: u128) -> String {
    let b = x.to_be_bytes();
    format!(
        "{:02x}{:02x}{:02x}{:02x}…{:02x}{:02x}{:02x}",
        b[0], b[1], b[2], b[3], b[13], b[14], b[15]
    )
}

/// Printable ASCII rendering of a block's valid bytes; non-printable → `·`.
fn ascii_of(x: u128, valid: usize) -> String {
    let b = x.to_be_bytes();
    let mut s = String::new();
    for &byte in b.iter().take(valid.min(16)) {
        if (0x20..0x7f).contains(&byte) {
            s.push(byte as char);
        } else {
            s.push('·');
        }
    }
    s
}

/// Lenient hex parse: keep hex digits, pair them into bytes, then pad/truncate
/// to exactly `out_len` so the caller always gets a well-formed key/nonce.
fn parse_hex(input: &str, out_len: usize) -> Vec<u8> {
    let digits: Vec<u8> = input
        .chars()
        .filter_map(|c| c.to_digit(16).map(|d| d as u8))
        .collect();
    let mut bytes: Vec<u8> = digits.chunks(2).filter(|p| p.len() == 2).map(|p| (p[0] << 4) | p[1]).collect();
    bytes.resize(out_len, 0);
    bytes.truncate(out_len);
    bytes
}

// ─────────────────────────────────────────────────────────────────────────
// egui application
// ─────────────────────────────────────────────────────────────────────────

// Palette (kept close to the site's dark theme / the IFS & HNSW scenes).
const BG: Color32 = Color32::from_rgb(20, 22, 28);
const AAD_COL: Color32 = Color32::from_rgb(125, 135, 199);
const CIPHER_COL: Color32 = Color32::from_rgb(42, 157, 143);
const LEN_COL: Color32 = Color32::from_rgb(233, 196, 106);
const CTR_COL: Color32 = Color32::from_rgb(244, 162, 97);
const TAG_COL: Color32 = Color32::from_rgb(231, 111, 81);
const ACCENT: Color32 = Color32::from_rgb(120, 170, 255);
const MUTED: Color32 = Color32::from_rgb(150, 156, 170);
const HEX: Color32 = Color32::from_rgb(210, 214, 224);

struct Preset {
    name: &'static str,
    key_hex: &'static str,
    iv_hex: &'static str,
    aad: &'static str,
    plaintext: &'static str,
}

const PRESETS: [Preset; 3] = [
    Preset {
        name: "Message",
        key_hex: "feffe9928665731c6d6a8f9467308308",
        iv_hex: "cafebabefacedbaddecaf888",
        aad: "to:alice; v=1",
        plaintext: "Meet at the old bridge at dawn.",
    },
    Preset {
        name: "Short",
        key_hex: "000102030405060708090a0b0c0d0e0f",
        iv_hex: "101112131415161718191a1b",
        aad: "",
        plaintext: "GCM",
    },
    Preset {
        name: "AAD only",
        key_hex: "feffe9928665731c6d6a8f9467308308",
        iv_hex: "cafebabefacedbaddecaf888",
        aad: "public routing header, not encrypted",
        plaintext: "",
    },
];

pub struct GcmApp {
    key_hex: String,
    iv_hex: String,
    aad_text: String,
    plaintext_text: String,
    computed: Computed,
    /// Animation cursor: `0..=trace.len()`. Equal to `trace.len()` = tag revealed.
    step: usize,
    playing: bool,
    accum: f32,
    /// Seconds between steps.
    interval: f32,
}

impl Default for GcmApp {
    fn default() -> Self {
        let p = &PRESETS[0];
        let computed = compute_gcm(
            &to_arr16(&parse_hex(p.key_hex, 16)),
            &to_arr12(&parse_hex(p.iv_hex, 12)),
            p.aad.as_bytes(),
            p.plaintext.as_bytes(),
        );
        Self {
            key_hex: p.key_hex.to_string(),
            iv_hex: p.iv_hex.to_string(),
            aad_text: p.aad.to_string(),
            plaintext_text: p.plaintext.to_string(),
            computed,
            step: 0,
            playing: true,
            accum: 0.0,
            interval: 1.1,
        }
    }
}

fn to_arr16(v: &[u8]) -> [u8; 16] {
    let mut a = [0u8; 16];
    a.copy_from_slice(&v[..16]);
    a
}
fn to_arr12(v: &[u8]) -> [u8; 12] {
    let mut a = [0u8; 12];
    a.copy_from_slice(&v[..12]);
    a
}

impl GcmApp {
    pub fn new() -> Self {
        Self::default()
    }

    fn recompute(&mut self) {
        let key = to_arr16(&parse_hex(&self.key_hex, 16));
        let iv = to_arr12(&parse_hex(&self.iv_hex, 12));
        self.computed = compute_gcm(&key, &iv, self.aad_text.as_bytes(), self.plaintext_text.as_bytes());
        self.step = 0;
        self.accum = 0.0;
    }

    fn apply_preset(&mut self, p: &Preset) {
        self.key_hex = p.key_hex.to_string();
        self.iv_hex = p.iv_hex.to_string();
        self.aad_text = p.aad.to_string();
        self.plaintext_text = p.plaintext.to_string();
        self.recompute();
        self.playing = true;
    }
}

impl eframe::App for GcmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let n_steps = self.computed.trace.len();

        // ── Controls ──────────────────────────────────────────────────────
        egui::SidePanel::right("gcm_controls")
            .resizable(false)
            .default_width(310.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    ui.heading("AES-128-GCM");
                    ui.label(
                        RichText::new(
                            "Authenticated encryption: CTR-mode keystream for secrecy, \
                             GHASH over GF(2¹²⁸) for a tamper-proof tag.",
                        )
                        .color(MUTED)
                        .size(12.0),
                    );
                    ui.add_space(6.0);

                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("Preset:").color(MUTED).size(12.0));
                        for p in PRESETS.iter() {
                            if ui.button(p.name).clicked() {
                                self.apply_preset(p);
                            }
                        }
                    });
                    ui.separator();

                    let mut dirty = false;
                    ui.label(RichText::new("Key (128-bit, hex)").color(MUTED).size(12.0));
                    if ui
                        .add(egui::TextEdit::singleline(&mut self.key_hex).font(egui::TextStyle::Monospace))
                        .changed()
                    {
                        dirty = true;
                    }
                    ui.label(RichText::new("Nonce / IV (96-bit, hex)").color(MUTED).size(12.0));
                    if ui
                        .add(egui::TextEdit::singleline(&mut self.iv_hex).font(egui::TextStyle::Monospace))
                        .changed()
                    {
                        dirty = true;
                    }
                    ui.label(RichText::new("Associated data (plaintext, authenticated only)").color(MUTED).size(12.0));
                    if ui
                        .add(egui::TextEdit::multiline(&mut self.aad_text).desired_rows(2))
                        .changed()
                    {
                        dirty = true;
                    }
                    ui.label(RichText::new("Plaintext (encrypted + authenticated)").color(MUTED).size(12.0));
                    if ui
                        .add(egui::TextEdit::multiline(&mut self.plaintext_text).desired_rows(2))
                        .changed()
                    {
                        dirty = true;
                    }
                    if dirty {
                        self.recompute();
                    }

                    ui.separator();
                    ui.horizontal(|ui| {
                        let label = if self.playing { "⏸ Pause" } else { "▶ Play" };
                        if ui.button(label).clicked() {
                            self.playing = !self.playing;
                            if self.playing && self.step >= n_steps {
                                self.step = 0;
                            }
                        }
                        if ui.button("⏭ Step").clicked() {
                            self.playing = false;
                            self.step = (self.step + 1).min(n_steps);
                        }
                        if ui.button("⏮ Reset").clicked() {
                            self.step = 0;
                            self.accum = 0.0;
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Speed").color(MUTED).size(12.0));
                        ui.add(egui::Slider::new(&mut self.interval, 0.25..=2.5).text("s/step"));
                    });

                    ui.separator();
                    ui.label(RichText::new("Constants").color(MUTED).size(12.0));
                    kv(ui, "H = Eₖ(0¹²⁸)", self.computed.h);
                    kv(ui, "J₀ = IV‖0³¹‖1", self.computed.j0);
                    kv(ui, "Eₖ(J₀)  (tag mask)", self.computed.ek_j0);
                    ui.add_space(4.0);
                    ui.label(RichText::new("Auth tag  T = S ⊕ Eₖ(J₀)").color(TAG_COL).size(12.0));
                    ui.label(RichText::new(hex128_grouped(self.computed.tag)).monospace().color(TAG_COL).strong());
                });
            });

        // ── Animation clock ───────────────────────────────────────────────
        if self.playing {
            let dt = ctx.input(|i| i.stable_dt).min(0.1);
            self.accum += dt;
            if self.accum >= self.interval {
                self.accum = 0.0;
                if self.step >= n_steps {
                    self.step = 0; // loop back to the start after showing the tag
                } else {
                    self.step += 1;
                }
            }
            ctx.request_repaint();
        }

        // ── Diagram ───────────────────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(egui::Frame::default().fill(BG).inner_margin(12.0))
            .show(ctx, |ui| {
                let comp = self.computed.clone();
                let step = self.step;
                // The block index GHASH is currently absorbing (None once the tag shows).
                let active = if step < comp.trace.len() { Some(step) } else { None };
                let active_ctr = active.and_then(|i| comp.trace[i].ctr_index);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    // 1 · Confidentiality lane (CTR)
                    section_header(ui, "1 · Confidentiality — AES-CTR keystream", CTR_COL);
                    ui.label(
                        RichText::new("Each counter block is encrypted to a keystream, then XORed with a plaintext block. Change the key or nonce and every byte below changes.")
                            .color(MUTED)
                            .size(12.0),
                    );
                    ui.add_space(6.0);
                    if comp.ctr.is_empty() {
                        ui.label(RichText::new("(no plaintext — nothing is encrypted; only the AAD is authenticated)").italics().color(MUTED));
                    } else {
                        egui::ScrollArea::horizontal().id_salt("ctr_scroll").show(ui, |ui| {
                            ui.horizontal_top(|ui| {
                                for (i, c) in comp.ctr.iter().enumerate() {
                                    ctr_column(ui, i, c, active_ctr == Some(i));
                                }
                            });
                        });
                    }

                    ui.add_space(14.0);

                    // 2 · Authentication lane (GHASH)
                    section_header(ui, "2 · Authentication — GHASH accumulator", AAD_COL);
                    ui.label(
                        RichText::new("X starts at 0. Absorb each block:  X ← (X ⊕ Bᵢ) · H,  multiplying in the field GF(2¹²⁸). Order is AAD, then ciphertext, then a block encoding both lengths.")
                            .color(MUTED)
                            .size(12.0),
                    );
                    ui.add_space(6.0);

                    // The chain of accumulator states.
                    egui::ScrollArea::horizontal().id_salt("ghash_scroll").show(ui, |ui| {
                        ui.horizontal_top(|ui| {
                            acc_chip(ui, "X₀", 0, false, MUTED);
                            for (i, s) in comp.trace.iter().enumerate() {
                                arrow_absorb(ui, s, Some(i) == active);
                                let label = if s.kind == BlockKind::Len { "S".to_string() } else { format!("X{}", subscript(i + 1)) };
                                acc_chip(ui, &label, s.x_out, Some(i) == active, kind_color(s.kind));
                            }
                        });
                    });

                    ui.add_space(12.0);

                    // The current multiply, in full.
                    if let Some(i) = active {
                        multiply_detail(ui, &comp.trace[i], comp.h);
                    } else {
                        ui.add_space(4.0);
                    }

                    ui.add_space(14.0);

                    // 3 · The tag
                    section_header(ui, "3 · The authentication tag", TAG_COL);
                    let tag_revealed = step >= comp.trace.len();
                    ui.horizontal_wrapped(|ui| {
                        ui.label(RichText::new("T = S ⊕ Eₖ(J₀) =").color(MUTED).monospace());
                        if tag_revealed {
                            ui.label(RichText::new(hex128_grouped(comp.tag)).monospace().strong().color(TAG_COL));
                        } else {
                            ui.label(RichText::new("(finish GHASH to reveal)").italics().color(MUTED));
                        }
                    });
                    ui.label(
                        RichText::new("Flip one bit of ciphertext, AAD, or the tag and verification fails — that is the \"authenticated\" in AEAD.")
                            .color(MUTED)
                            .size(12.0),
                    );
                });
            });
    }
}

// ── small view helpers ────────────────────────────────────────────────────

fn kind_color(kind: BlockKind) -> Color32 {
    match kind {
        BlockKind::Aad => AAD_COL,
        BlockKind::Cipher => CIPHER_COL,
        BlockKind::Len => LEN_COL,
    }
}

fn subscript(n: usize) -> String {
    n.to_string()
        .chars()
        .map(|c| match c {
            '0' => '₀',
            '1' => '₁',
            '2' => '₂',
            '3' => '₃',
            '4' => '₄',
            '5' => '₅',
            '6' => '₆',
            '7' => '₇',
            '8' => '₈',
            '9' => '₉',
            _ => c,
        })
        .collect()
}

fn kv(ui: &mut egui::Ui, key: &str, val: u128) {
    ui.horizontal_wrapped(|ui| {
        ui.label(RichText::new(key).color(MUTED).size(11.0));
    });
    ui.label(RichText::new(hex128_grouped(val)).monospace().color(HEX).size(12.0));
    ui.add_space(2.0);
}

fn section_header(ui: &mut egui::Ui, text: &str, color: Color32) {
    ui.label(RichText::new(text).heading().size(16.0).color(color).strong());
    ui.add_space(2.0);
}

/// One CTR column: counter → keystream → ⊕ plaintext → ciphertext.
fn ctr_column(ui: &mut egui::Ui, idx: usize, c: &CtrStep, active: bool) {
    let fill = if active { Color32::from_rgb(38, 44, 40) } else { Color32::from_rgb(26, 28, 34) };
    let stroke = egui::Stroke::new(if active { 2.0 } else { 1.0 }, if active { CIPHER_COL } else { Color32::from_rgb(52, 56, 66) });
    egui::Frame::default()
        .fill(fill)
        .stroke(stroke)
        .inner_margin(8.0)
        .rounding(6.0)
        .show(ui, |ui| {
            ui.set_width(190.0);
            ui.vertical(|ui| {
                ui.label(RichText::new(format!("counter {}", idx + 1)).color(CTR_COL).size(11.0).strong());
                ui.label(RichText::new(hex_short(c.counter)).monospace().color(HEX).size(11.0));
                ui.label(RichText::new("│ Eₖ").color(MUTED).size(11.0));
                ui.label(RichText::new(format!("ks {}", hex_short(c.keystream))).monospace().color(MUTED).size(11.0));
                ui.label(RichText::new(format!("⊕ P “{}”", ascii_of(c.plain, c.valid))).monospace().color(ACCENT).size(11.0));
                ui.separator();
                ui.label(RichText::new("= ciphertext").color(CIPHER_COL).size(11.0).strong());
                ui.label(RichText::new(hex128(c.cipher)).monospace().color(CIPHER_COL).size(11.0));
            });
        });
}

/// A rounded accumulator chip showing a short hex of `val`.
fn acc_chip(ui: &mut egui::Ui, label: &str, val: u128, active: bool, color: Color32) {
    let fill = if active { Color32::from_rgb(40, 46, 60) } else { Color32::from_rgb(26, 28, 34) };
    let stroke = egui::Stroke::new(if active { 2.0 } else { 1.0 }, if active { ACCENT } else { Color32::from_rgb(52, 56, 66) });
    egui::Frame::default()
        .fill(fill)
        .stroke(stroke)
        .inner_margin(7.0)
        .rounding(6.0)
        .show(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new(label).color(color).size(12.0).strong());
                ui.label(RichText::new(hex_short(val)).monospace().color(HEX).size(11.0));
            });
        });
}

/// The little "⊕ Bᵢ · H" annotation drawn between two accumulator chips.
fn arrow_absorb(ui: &mut egui::Ui, s: &GhashStep, active: bool) {
    let col = if active { ACCENT } else { MUTED };
    ui.vertical(|ui| {
        ui.add_space(6.0);
        ui.label(RichText::new(format!("⊕ {}", s.label)).color(kind_color(s.kind)).size(11.0).strong());
        ui.label(RichText::new("· H →").color(col).size(11.0));
    });
}

/// The full detail of the currently-highlighted GHASH multiply.
fn multiply_detail(ui: &mut egui::Ui, s: &GhashStep, h: u128) {
    egui::Frame::default()
        .fill(Color32::from_rgb(24, 26, 33))
        .stroke(egui::Stroke::new(1.0, Color32::from_rgb(52, 56, 66)))
        .inner_margin(10.0)
        .rounding(8.0)
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new(format!("Absorbing  {}", s.label)).color(kind_color(s.kind)).strong());
                ui.label(RichText::new("in GF(2¹²⁸)").color(MUTED).size(12.0));
            });
            ui.add_space(4.0);
            egui::Grid::new("mul_grid").num_columns(2).spacing([10.0, 3.0]).show(ui, |ui| {
                mrow(ui, "X (current)", s.x_in, HEX);
                mrow(ui, &format!("⊕ {} block", s.label), s.block, kind_color(s.kind));
                mrow(ui, "= X ⊕ B", s.xored, ACCENT);
                mrow(ui, "× H", h, MUTED);
                mrow(ui, "= new X", s.x_out, Color32::from_rgb(120, 220, 170));
            });
            ui.add_space(6.0);
            ui.label(RichText::new("new X as 128 field bits (row-major, MSB first):").color(MUTED).size(11.0));
            bit_grid(ui, s.x_out);
        });
}

fn mrow(ui: &mut egui::Ui, label: &str, val: u128, color: Color32) {
    ui.label(RichText::new(label).color(MUTED).monospace().size(12.0));
    ui.label(RichText::new(hex128_grouped(val)).monospace().color(color).size(12.0));
    ui.end_row();
}

/// Paint the 128 bits of a field element as an 8×16 grid of cells.
fn bit_grid(ui: &mut egui::Ui, val: u128) {
    let cols = 16.0;
    let rows = 8.0;
    let cell = 11.0;
    let gap = 2.0;
    let w = cols * cell + (cols - 1.0) * gap;
    let h = rows * cell + (rows - 1.0) * gap;
    let (resp, painter) = ui.allocate_painter(Vec2::new(w, h), Sense::hover());
    let origin = resp.rect.left_top();
    for bit in 0..128u32 {
        let r = (bit / 16) as f32;
        let c = (bit % 16) as f32;
        let set = (val >> (127 - bit)) & 1 == 1;
        let pos = origin + Vec2::new(c * (cell + gap), r * (cell + gap));
        let rect = egui::Rect::from_min_size(pos, Vec2::splat(cell));
        let color = if set { ACCENT } else { Color32::from_rgb(38, 41, 50) };
        painter.rect_filled(rect, 1.5, color);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn h(s: &str) -> Vec<u8> {
        (0..s.len()).step_by(2).map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap()).collect()
    }

    // McGrew–Viega / NIST SP 800-38D test case 1: all-zero key, empty A and P.
    #[test]
    fn nist_test_case_1() {
        let c = compute_gcm(&[0u8; 16], &[0u8; 12], &[], &[]);
        assert_eq!(hex128(c.h), "66e94bd4ef8a2c3b884cfa59ca342b2e");
        assert_eq!(hex128(c.ek_j0), "58e2fccefa7e3061367f1d57a4e7455a");
        // Empty A and C ⇒ GHASH folds only a zero length-block ⇒ S = 0 ⇒ T = Eₖ(J₀).
        assert_eq!(hex128(c.tag), "58e2fccefa7e3061367f1d57a4e7455a");
    }

    // Test case 2: all-zero key/IV, a single all-zero plaintext block.
    // Exercises AES-CTR (ciphertext) and one real GF(2^128) multiply (tag).
    // The GHASH path is anchored externally by cases 3 & 4 below (full
    // McGrew–Viega vectors); this tag is the value that correct GCM yields here.
    #[test]
    fn nist_test_case_2() {
        let c = compute_gcm(&[0u8; 16], &[0u8; 12], &[], &[0u8; 16]);
        assert_eq!(hex128(c.ctr[0].cipher), "0388dace60b6a392f328c2b971b2fe78");
        assert_eq!(hex128(c.tag), "ab6e47d42cec13bdf53a67b21257bddf");
    }

    // Test case 3: 128-bit key, four plaintext blocks, no AAD. From McGrew–Viega.
    #[test]
    fn nist_test_case_3() {
        let key = h("feffe9928665731c6d6a8f9467308308");
        let iv = h("cafebabefacedbaddecaf888");
        let pt = h("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b391aafd255");
        let c = compute_gcm(&to_arr16(&key), &to_arr12(&iv), &[], &pt);
        let ct: String = c.ctr.iter().flat_map(|s| s.cipher.to_be_bytes()[..s.valid].to_vec()).map(|b| format!("{b:02x}")).collect();
        assert_eq!(ct, "42831ec2217774244b7221b784d0d49ce3aa212f2c02a4e035c17e2329aca12e21d514b25466931c7d8f6a5aac84aa051ba30b396a0aac973d58e091473f5985");
        assert_eq!(hex128(c.tag), "4d5c2af327cd64a62cf35abd2ba6fab4");
    }

    // Test case 4: same as 3 but with 20 bytes of AAD and a partial final block.
    #[test]
    fn nist_test_case_4() {
        let key = h("feffe9928665731c6d6a8f9467308308");
        let iv = h("cafebabefacedbaddecaf888");
        let aad = h("feedfacedeadbeeffeedfacedeadbeefabaddad2");
        let pt = h("d9313225f88406e5a55909c5aff5269a86a7a9531534f7da2e4c303d8a318a721c3c0c95956809532fcf0e2449a6b525b16aedf5aa0de657ba637b39");
        let c = compute_gcm(&to_arr16(&key), &to_arr12(&iv), &aad, &pt);
        assert_eq!(hex128(c.tag), "5bc94fbc3221a5db94fae95ae7121a47");
    }
}
