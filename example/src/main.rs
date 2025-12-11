use p12_keystore::KeyStore;

fn main() {
    KeyStore::from_pkcs12(&[], "1").unwrap();
}
