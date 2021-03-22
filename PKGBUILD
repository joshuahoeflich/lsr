#!/usr/bin/env bash
# Maintainer: Your Name <josh@joshuahoeflich.com>
pkgname=lsr
pkgver=1.0.0
pkgrel=1
pkgdesc="List files recursively."
arch=('x86-64')
url=""
license=('MIT')
groups=()
depends=()
makedepends=('cargo')
optdepends=()
provides=()
conflicts=()
replaces=()
backup=()
options=()
install=
changelog=
source=($pkgname-$pkgver.tar.gz)
noextract=()
md5sums=() #autofill using updpkgsums

build() {
  cd "$pkgname-$pkgver"
  cargo build --release
}

package() {
  cd "$pkgname-$pkgver"
  cp target/release/lsr "$pkgdir"
}
