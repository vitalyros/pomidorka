# Maintainer: Vitaly Roslov 22212673+vitalyros@users.noreply.github.com
pkgname=pomidorka
pkgver=0.1.0
pkgrel=1
makedepends=('rust' 'cargo')
arch=('i686' 'x86_64' 'armv6h' 'armv7h')
pkgdesc="Console timer for Pomodoro Technique"
url="https://github.com/vitalyros/pomidorka"
source=('https://media.githubusercontent.com/media/vitalyros/pomidorka/master/alarm.mp3')
md5sums=("51377e112e2b3a0b8650369aee9261ad")
license=('MIT')
package() {
    cargo install --root="$pkgdir/usr/" $pkgname
    install -D --mode=644 --owner=root --group=root $srcdir/alarm.mp3 "$pkgdir/usr/share/$pkgname/"
}
