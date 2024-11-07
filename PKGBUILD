# Maintainer: Your Name <youremail@example.com>
pkgname=liz-shortcut-helper
pkgver=0.1.1
pkgrel=1
pkgdesc="A Rust-based fast helper to remember, customize and autorun shortcuts or commands for Linux."
arch=('x86_64')
url="https://github.com/philia897/liz-shortcut-helper"
license=('GPL-3.0')  # Replace with your license
depends=('rofi' 'ydotool')
makedepends=('cargo')
source=("$pkgname-$pkgver.tar.gz::$url/archive/refs/tags/v$pkgver.tar.gz"
        "bluebird.service")
sha256sums=('SKIP' 'SKIP')  # Replace SKIP with actual checksums

build() {
    cd "$srcdir/$pkgname-$pkgver"
    cargo build --release
}

package() {
    cd "$srcdir/$pkgname-$pkgver"
    
    # Install the executables
    install -Dm755 "target/release/liz" "$pkgdir/usr/bin/liz"
    install -Dm755 "target/release/bluebird" "$pkgdir/usr/bin/bluebird"

    # Install the systemd service file
    install -Dm644 "$srcdir/bluebird.service" "$pkgdir/usr/lib/systemd/system/bluebird.service"
    
    # Ensure the user config directory exists
    install -dm755 "$pkgdir/etc/skel/.config/liz"
    
    # Copy default runtime data (or config) to /etc/skel (which is used for user home dirs)
    install -Dm644 "$srcdir/$pkgname-$pkgver/config/default_config.toml" "$pkgdir/etc/skel/.config/liz/bluebird.toml"
}

post_install() {
    # Enable the systemd service upon installation (optional)
    systemctl enable bluebird.service
}

post_remove() {
    # Disable the service upon package removal (optional)
    systemctl disable bluebird.service
}
