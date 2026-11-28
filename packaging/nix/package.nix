{ lib
, rustPlatform
, fetchFromGitHub
, src ? null
}:

rustPlatform.buildRustPackage rec {
  pname = "kktypr";
  version = "0.3.0";

  src = if src != null then src else fetchFromGitHub {
    owner = "kk376";
    repo = "kktypr";
    rev = "v${version}";
    hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
  };

  cargoLock = {
    lockFile = ../../Cargo.lock;
  };

  meta = with lib; {
    description = "Terminal touch typing academy and developer code syntax trainer in pure Rust";
    homepage = "https://github.com/kk376/kktypr";
    license = with licenses; [ mit asl20 ];
    maintainers = [ "Kushagra Kumar (kk376)" ];
    mainProgram = "kktypr";
    platforms = platforms.unix;
  };
}
