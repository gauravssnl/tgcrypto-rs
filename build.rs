fn main() {
    cc::Build::new()
        .include("tgcrypto/tgcrypto")
        .file("tgcrypto/tgcrypto/aes256.c")
        .file("tgcrypto/tgcrypto/cbc256.c")
        .file("tgcrypto/tgcrypto/ctr256.c")
        .file("tgcrypto/tgcrypto/ige256.c")
        .compile("libtgcrypto");
}
