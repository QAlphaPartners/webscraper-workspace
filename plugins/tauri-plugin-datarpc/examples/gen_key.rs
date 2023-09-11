use anyhow::Result;
use rand::RngCore;
use tauri_plugin_datarpc::crypt::{pwd, EncryptContent};

const DEMO_PWD: &str = "welcome";
const SALT: &str = "demo1_pwd_salt";

fn main() -> Result<()> {
	let mut key = [0u8; 64]; // 512 bits = 64 bytes
	rand::thread_rng().fill_bytes(&mut key);
	println!("\nGenerated key for HMAC:\n{key:?}");

	let b64u = base64_url::encode(&key);
	println!("\nKey b64u encoded:\n{b64u}");

	
	let pwd = pwd::encrypt_pwd(&EncryptContent {
		content: DEMO_PWD.to_string(),
		salt: SALT.to_string(),
	})?;

	println!("\nencrypt_pwd= {pwd}\nfor pwd= {DEMO_PWD} with salt= {SALT}\n");

	Ok(())
}
