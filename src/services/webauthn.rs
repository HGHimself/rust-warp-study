use uuid::Uuid;
use webauthn_rs::prelude::*;

pub struct Webauthn {
    webauthn: webauthn_rs::Webauthn,
}

impl Webauthn {
    pub fn new() -> Self {
        let rp_id = "localhost";
        let rp_origin = Url::parse("http://localhost:4000").expect("Invalid URL");
        let mut builder = WebauthnBuilder::new(rp_id, &rp_origin).expect("Invalid configuration");

        let webauthn = builder.build().expect("Invalid configuration");

        Self { webauthn }
    }

    // {"publicKey":{"rp":{"name":"localhost","id":"localhost"},"user":{"id":decodeBase64("g4omdj3ITNC5xSqWEYE7VQ").buffer,"name":"user_name","displayName":"user_display_name"},"challenge":decodeBase64("M1c8uLwCqTwzG9bTBfwZNEl6naZT5DomnqLRQ5yEB24").buffer,"pubKeyCredParams":[{"type":"public-key","alg":-7},{"type":"public-key","alg":-257}],"timeout":60000,"attestation":"none","authenticatorSelection":{"requireResidentKey":false,"userVerification":"preferred"},"extensions":{"uvm":true,"credProps":true}}}

    pub fn register(
        &self,
        user_name: &str,
        user_display_name: &str,
    ) -> WebauthnResult<(CreationChallengeResponse, PasskeyRegistration)> {
        self.webauthn
            .start_passkey_registration(Uuid::new_v4(), user_name, user_display_name, None)
    }

    // pub fn finish_register(&self) {
    //     self.webauthn.finish_passkey_registration(reg, state)
    // }
}
