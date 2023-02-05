# Maintainer: berinaniesh <berinaniesh@gmail.com>

pkgname=ucf-bin
_pkgname=${pkgname%-bin}
pkgver=0.1.0
pkgrel=1
pkgdesc="A universal code formatter"
arch=("x86_64" "aarch64")
url="https://github.com/berinaniesh/ucf"
license=("MIT")
depends=()
provides=("${_pkgname}")
options=("!strip" "emptydirs")
source_x86_64=("${url}/releases/download/${pkgver}/gohttpserver_${pkgver}_linux_amd64.tar.gz")
source_aarch64=("${url}/releases/download/${pkgver}/gohttpserver_${pkgver}_linux_arm64.tar.gz")
sha256sums_x86_64=('7ddbbee7268f4d89bd008351a6bf99336615587cc2a900fefb7d924442b95db2')
sha256sums_aarch64=('aca46287b1aa0caa1e71569ece86ace39a70715e47e7c7f6d01787cee3d36e04')
prepare() {
    if [ -d "${srcdir}/build" ]; then
        rm -rf ${srcdir}/build
    fi
    mkdir ${srcdir}/build
    tar -xvf ${srcdir}/gohttpserver_${pkgver}_linux_*.tar.gz -C ${srcdir}/build
}

package() {
    cd ${srcdir}/build/
    install -Dm755 "${_pkgname}" -t "${pkgdir}/usr/bin/"
}
