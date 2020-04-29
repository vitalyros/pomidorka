# Maintainer: Vitaly Roslov 22212673+vitalyros@users.noreply.github.com
pkgname=pomidorka
pkgver=0.1.2
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Console timer for Pomodoro Technique"
url="https://github.com/vitalyros/pomidorka"
source=('https://media.githubusercontent.com/media/vitalyros/pomidorka/master/alarm.ogg')
md5sums=("c3b049b592fff80cef530b0253277e76")
license=('MIT')
package() {
    cargo install --root="$pkgdir/usr/" "$pkgname"
    mkdir -p "$pkgdir/usr/share/$pkgname/"
    install -D -m644 --owner=root --group=root "$srcdir/alarm.ogg" "$pkgdir/usr/share/$pkgname/"
}
