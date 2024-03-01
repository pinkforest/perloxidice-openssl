fn main() {

 
    

}

#[cfg(test)]
mod test {

    #[test]
    fn sha512() {
        use openssl::hash::{Hasher, MessageDigest};
        
        let mut hasher = Hasher::new(MessageDigest::sha256())?;
        hasher.update(b"test")?;
        hasher.update(b"this")?;
        let digest: &[u8] = &hasher.finish()?;
        
        let expected = hex::decode("9740e652ab5b4acd997a7cca13d6696702ccb2d441cca59fc6e285127f28cfe6")?;
        assert_eq!(digest, expected);
    }
    
}
