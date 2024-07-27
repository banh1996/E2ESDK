use rsa::{pkcs1::EncodeRsaPublicKey, pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey}, Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};
use std::fs::File;
use std::io::{self, Error, ErrorKind, Read, Write};
use std::path::Path;
use std::sync::{Arc, Mutex};

// Define the E2eCyber trait
pub trait E2eCyber {
    fn init(&mut self, priv_key_path: &Path, pub_key_path: &Path) -> Result<(), io::Error>;
    fn generate_pairkey(&mut self, priv_key_path: &Path, pub_key_path: &Path) -> Result<(), io::Error>;
    fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>, io::Error>;
    fn decrypt(&self, encrypted_message: &[u8]) -> Result<Vec<u8>, io::Error>;
}

pub struct E2eRSA2K {
    isinit: Arc<Mutex<bool>>,
    privkey: Option<Arc<Mutex<RsaPrivateKey>>>,
    pubkey: Option<Arc<Mutex<RsaPublicKey>>>,
}

/*****************************************************************************************************************
 * Start trait E2eRSA2K implementation for E2eCyber
 *****************************************************************************************************************/
impl E2eCyber for E2eRSA2K {

/*****************************************************************************************************************
 *  e2esdk::init function
 *  brief        Init necessary configuration for SDK
 *  details      If you already owned private/public key, free to call this function
 *               Otherwise, call generate_pairkey function to generate pair-key firstly
 *  \param[in]   priv_key_path: path to existed private key, expect PEM format
 *  \param[in]   pub_key_path: path to existed public key, expect PEM format
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
fn init(&mut self, priv_key_path: &Path, pub_key_path: &Path) -> Result<(), io::Error> {
    // let isinit = self.isinit.lock().unwrap();
    // if *isinit {
    //     return Err(io::Error::new(io::ErrorKind::AlreadyExists, "E2eRSA2K is already initialized"));
    // }

    // Read the private key from the specified path
    let priv_key = {
        let mut file = File::open(priv_key_path)?;
        let mut priv_pem = String::new();
        file.read_to_string(&mut priv_pem)?;
        RsaPrivateKey::from_pkcs8_pem(&priv_pem)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    };
    let priv_key = Arc::new(Mutex::new(priv_key));

    // Read the public key from the specified path
    let pub_key = {
        let mut file = File::open(pub_key_path)?;
        let mut pub_pem = String::new();
        file.read_to_string(&mut pub_pem)?;
        RsaPublicKey::from_public_key_pem(&pub_pem)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    };
    let pub_key = Arc::new(Mutex::new(pub_key));

    // Verify the key pair, since cannot get modulus with rsa lib, try encrypt/decrypt to verify pair key
    let test_data = b"!!Qualgo!!";
    let encrypted_data = {
        let pub_key = pub_key.lock().unwrap();
        pub_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, test_data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    };
    let decrypted_data = {
        let priv_key = priv_key.lock().unwrap();
        priv_key.decrypt(Pkcs1v15Encrypt, &encrypted_data)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?
    };
    if decrypted_data != test_data {
        return Err(io::Error::new(io::ErrorKind::Other, "Key pair verification failed"));
    }

    // Assign to the struct fields
    self.privkey = Some(priv_key);
    self.pubkey = Some(pub_key);

    // Modify the isinit variable
    let mut isinit_guard = self.isinit.lock().unwrap();
    *isinit_guard = true;

    Ok(())
}

/*****************************************************************************************************************
 *  e2esdk::generate_pairkey function
 *  brief        Generate private/public key to input paths
 *  details      Call this function if you do not have a pair key for setting
 *  \param[in]   priv_key_path: path to private key need to be stored, expect PEM format
 *  \param[in]   pub_key_path: path to public key need to be stored, expect PEM format
 *  \param[out]  -
 *  \precondition: -
 *  \reentrant:  FALSE
 *  \return      Error code if any
 ****************************************************************************************************************/
fn generate_pairkey(&mut self, priv_key_path: &Path, pub_key_path: &Path) -> Result<(), io::Error> {
    let mut rng = rand::thread_rng(); //we can improve random mecha by using HSM instead
    let bits = 2048;

    //Generate pair key
    let priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
    let pub_key = RsaPublicKey::from(&priv_key);

    // Save the private key to the specified path
    {
        let priv_pem = priv_key.to_pkcs8_pem(Default::default()).unwrap();
        let mut file = File::create(priv_key_path)?;
        file.write_all(priv_pem.as_bytes())?;
    }

    // Save the public key to the specified path
    {
        let pub_pem = pub_key.to_public_key_pem(Default::default()).unwrap();
        let mut file = File::create(pub_key_path)?;
        file.write_all(pub_pem.as_bytes())?;
    }

    Ok(())
}

fn encrypt(&self, message: &[u8]) -> Result<Vec<u8>, io::Error> {
    let isinit = self.isinit.lock().unwrap();
    if !*isinit {
        return Err(io::Error::new(io::ErrorKind::Other, "E2eRSA2K is not initialized"));
    }
    let pub_key = self.pubkey.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Public key not found"))?;
    let pub_key = pub_key.lock().unwrap();
    pub_key.encrypt(&mut rand::thread_rng(), Pkcs1v15Encrypt, message)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}

fn decrypt(&self, encrypted_message: &[u8]) -> Result<Vec<u8>, io::Error> {
    let isinit = self.isinit.lock().unwrap();
    if !*isinit {
        return Err(io::Error::new(io::ErrorKind::Other, "E2eRSA2K is not initialized"));
    }
    let priv_key = self.privkey.as_ref().ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Private key not found"))?;
    let priv_key = priv_key.lock().unwrap();
    priv_key.decrypt(Pkcs1v15Encrypt, encrypted_message)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))
}
}
/*****************************************************************************************************************
 * End trait E2eRSA2K implementation for E2eCyber
 *****************************************************************************************************************/

/*****************************************************************************************************************
 * Start implementation for E2eRSA2K object
 *****************************************************************************************************************/
impl E2eRSA2K {
pub fn new() -> Self {
    E2eRSA2K {
        isinit: Arc::new(Mutex::new(false)),
        privkey: None,
        pubkey: None,
    }
}

pub fn new_with_key_files(priv_key_path: &str, pub_key_path: &str) -> Self {
    let mut instance = E2eRSA2K::new();
    if let Err(e) = instance.init(Path::new(priv_key_path), Path::new(pub_key_path)) {
        eprintln!("Failed to initialize with keys: {}", e);
    }
    instance
}

pub fn is_initialized(&self) -> bool {
    *self.isinit.lock().unwrap()
}

}
/*****************************************************************************************************************
 * End implementation for E2eRSA2K object
 *****************************************************************************************************************/